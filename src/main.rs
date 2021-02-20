#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
embed_migrations!("./migrations");

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate r2d2;
extern crate r2d2_diesel;

mod models;
mod pool;
mod routes;
pub mod schema;
mod utils;

//use diesel;
use dotenv::dotenv;
use std::env;

use routes::*;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let postgre_connection_poll = pool::init(&database_url);
    embedded_migrations::run(
        &*postgre_connection_poll
            .clone()
            .get()
            .expect("connection instance"),
    )
    .expect("Could run migrations");
    rocket::ignite()
        .manage(postgre_connection_poll)
        .mount("/api", routes![get_votings, create_voting, get_votings2])
        .launch();
}
