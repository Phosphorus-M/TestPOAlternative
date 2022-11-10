#[macro_use]
extern crate rbatis;
extern crate rbdc;

pub mod entities;
pub mod controllers;

use dotenv::dotenv;
use once_cell::sync::Lazy;
use rbatis::Rbatis;
use salvo::logging::Logger;
use salvo::prelude::*;
use rbdc_pg::driver::PgDriver;

use crate::controllers::test::{hello_world, get_test};


pub static RB: Lazy<Rbatis> = Lazy::new(Rbatis::new);


#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().init();
    let postgres_uri = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // postgres connect info
    RB.init(PgDriver {}, &postgres_uri).unwrap();
    RB.try_acquire().await.unwrap();

    // router
    let router = Router::new()
                                .hoop(Logger)
                                .get(hello_world)
                                .push(Router::with_path("tests").get(get_test));
    

    tracing::info!("Listening on http://127.0.0.1:7878");
    Server::new(TcpListener::bind("127.0.0.1:7878")).serve(router).await;
}