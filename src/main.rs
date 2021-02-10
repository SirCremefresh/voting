#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;

#[derive(Deserialize)]
struct Task {
    description: String,
    complete: bool
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
    rocket::ignite().mount("/", routes![index, hello]).launch();
}
