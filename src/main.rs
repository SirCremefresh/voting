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

use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, ContentType, Method};
use std::io::Cursor;

pub struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON) {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type, Authorization"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(""));
        }
    }
}
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
    rocket::ignite()
        .manage(postgre_connection_poll)
        .mount(
            "/api",
            routes![
                poll::get_active_poll,
                poll::set_active_poll,
                voting::create_voting,
                voting::cors_create_voting,
                voting::get_voting,
                voting::cors_get_voting,
                vote::set_vote,
                voter::create_voter,
                voter::cors_create_voter,
                voter::get_voter_info,
            ],
        )
        .register(catchers![routes::unauthorized])
        .attach(CORS())
        .launch();
}
