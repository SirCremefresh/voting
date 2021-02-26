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

mod actions;
mod dtos;
mod models;
mod pool;
mod routes;
pub mod schema;
pub mod schema_custom;
mod utils;
mod validators;

//use diesel;
use dotenv::dotenv;
use std::env;

use routes::{poll, vote, voter, voting};

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
        .mount(
            "/api",
            routes![
                poll::get_active_poll,
                poll::set_active_poll,
                voting::create_voting,
                voting::get_voting,
                vote::set_vote,
                voter::create_voter,
                voter::get_voter_info,
            ],
        )
        .register(catchers![routes::unauthorized])
        .launch();
}
