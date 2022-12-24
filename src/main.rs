#[macro_use]
extern crate rbatis;
extern crate rbdc;

pub mod controllers;
pub mod entities;
pub mod middlewares;

use dotenv::dotenv;
use graphul::{middleware, Graphul, http::Methods};
use once_cell::sync::Lazy;
use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;

use crate::{
    middlewares::tracing::tracing_middleware, controllers::test::TestController,
};

pub static RB: Lazy<Rbatis> = Lazy::new(Rbatis::new);

#[tokio::main]
async fn main() {
    setup().await;
    let mut app = Graphul::new();
    
    // router
    
    app.get("/", || async { "hello world!" });
    let mut test_routes = app.group("test");
    test_routes.resource("", TestController);
    test_routes.get("/:id", TestController::get_by_id);
    
    app.middleware(middleware::from_fn(tracing_middleware));

    app.run("127.0.0.1:8000").await;
}

async fn setup() {
    dotenv().ok();
    tracing_subscriber::fmt().init();
    // postgres connect info
    let mariadb_uri = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    RB.init(MysqlDriver {}, &mariadb_uri).unwrap();
    RB.try_acquire().await.unwrap();
}
