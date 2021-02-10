#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use serde::{Deserialize, Serialize};
use rocket_contrib::json::Json;

#[derive(Deserialize, Serialize)]
struct Task {
    description: String,
    complete: bool,
}

#[get("/todo")]
fn new() -> Json<Task> {
    Json(Task {
        description: String::from("pussies eater"),
        complete: true,
    })
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}!", name.as_str())
}

fn main() {
    rocket::ignite().mount("/", routes![index, hello,new]).launch();
}
