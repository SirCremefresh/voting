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

use chrono::Local;
use dotenv::dotenv;
use env_logger::Builder;
use log::LevelFilter;
use std::env;
use std::io::Write;

use routes::{poll, vote, voter, voting};
use rocket::Config;
use rocket::config::Environment;

fn main() {
    dotenv().ok();
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let postgre_connection_poll = pool::init(&database_url);
    embedded_migrations::run(
        &*postgre_connection_poll
            .clone()
            .get()
            .expect("connection instance"),
    )
    .expect("Could run migrations");

    let config = Config::build(Environment::Staging)
        .address(env::var("ADDRESS").unwrap_or("0.0.0.0".to_string()))
        .port(
            env::var("PORT")
                .unwrap_or("8080".to_string())
                .parse::<u16>()
                .unwrap(),
        )
        .finalize()
        .unwrap();

    rocket::custom(config)
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
