pub mod controllers;
pub mod entities;
pub mod middlewares;
pub mod utils;

use std::time::Duration;

use dotenv::dotenv;
use graphul::{middleware, Graphul, http::Methods};
use sea_orm::{ConnectOptions, Database, DbErr, DatabaseConnection};
use tracing::log::LevelFilter;

use crate::{
    middlewares::tracing::tracing_middleware, controllers::test::TestController,
};


#[derive(Clone)]
pub struct AppState {
    db: DatabaseConnection
}

#[tokio::main]
async fn main() {
    let Ok(app_state) = setup().await else {
        panic!("Something happened setupping the backend state")
    };

    let mut app = Graphul::share_state(app_state);
    // router
    
    app.get("/", || async { "hello world!" });
    let mut test_routes = app.group("test");
    test_routes.resource("", TestController);
    test_routes.get("/:id", TestController::get_by_id);
    test_routes.patch("/:id", TestController::patch);
    test_routes.delete("/:id", TestController::delete);

    app.middleware(middleware::from_fn(tracing_middleware));

    app.run("127.0.0.1:8000").await;
}

async fn setup() -> Result<AppState, DbErr> {
    dotenv().ok();
    tracing_subscriber::fmt().init();
    let mariadb_uri = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut opt = ConnectOptions::new(mariadb_uri.to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(LevelFilter::Info);
    
    let database = Database::connect(opt).await?;

    let app_state = AppState{
        db: database
    };

    Ok(app_state)
}
