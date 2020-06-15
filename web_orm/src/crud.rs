extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use self::models::*;
use self::diesel::prelude::*;

pub fn get_posts() {
    use diesel_demo::schema::posts::dsl::*;

    let connection = establish_connection;
}
