extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL NOT SET");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connection to {}", database_url))
}

pub mod schema;
pub mod models;
