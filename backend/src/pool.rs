use diesel;
use r2d2;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;

type DbType = diesel::pg::PgConnection;

use r2d2_diesel::ConnectionManager;
type Pool = r2d2::Pool<ConnectionManager<DbType>>;
type PoolConn = r2d2::PooledConnection<ConnectionManager<DbType>>;

pub struct DbConn(pub PoolConn);

impl Deref for DbConn {
    type Target = DbType;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

pub fn init(database_url: &str) -> Pool {
    println!("Connect to database_url: {}", database_url);
    let manager = ConnectionManager::<DbType>::new(database_url);
    r2d2::Pool::new(manager)
        .map_err(|err| {
            println!("Error creating Database Poll. err: {:?}", err);
            err
        })
        .expect("db pool")
}
