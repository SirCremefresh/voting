extern crate voting;
#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use self::diesel::prelude::*;
use self::models::*;
use self::voting::*;

fn main() {
    use voting::schema::votings::dsl::*;

    let connection = establish_connection();
    votings
        .limit(5)
        .load::<Voting>(&connection)
        .expect("Error loading posts")
        .iter()
        .for_each(|voting| println!("{} -----------\n {}", voting.voting_id, voting.name));
}
