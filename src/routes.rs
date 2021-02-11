use super::models::*;
use super::pool::DbConn;
use super::schema::votings;
use super::schema::votings::dsl::*;

use diesel::prelude::*;
use rocket_contrib::json::Json;

#[get("/votings")]
pub fn get_votings(conn: DbConn) -> Json<Vec<Voting>> {
    votings
        .limit(5)
        .load::<Voting>(&*conn)
        .map(|xs| Json(xs))
        .unwrap()
}
