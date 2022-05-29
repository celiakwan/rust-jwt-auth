#![feature(decl_macro)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use dotenv::dotenv;
use rocket::config::{Config, Environment, LoggingLevel};

mod db;
mod handlers;
mod models;
mod schema;

fn main() {
    dotenv().ok();

    db::init();

    let config = Config::build(Environment::Development)
        .address("127.0.0.1")
        .port(8080)
        .log_level(LoggingLevel::Debug)
        .finalize()
        .unwrap();
    rocket::custom(config)
        .manage(db::init())
        .mount("/", handlers::routes::api_routes())
        .launch();
}
