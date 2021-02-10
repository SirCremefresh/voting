extern crate voting;
#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use self::voting::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use voting::schema::votings::dsl::*;

    let connection = establish_connection();
    let results = votings
        .limit(5)
        .load::<Voting>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.voting_id);
        println!("-----------\n");
        println!("{}", post.name);
    }
}