#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]
#![feature(custom_derive)]
#![allow(proc_macro_derive_resolution_fallback)]

/// ORM and config
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

/// Connection pooling
extern crate r2d2;
extern crate r2d2_diesel;

// Web server
extern crate rocket;
extern crate rocket_contrib;

mod db;
mod schema;
mod person;
mod reddit;

use rocket::response::NamedFile;
use rocket::Rocket;
use std::path::{Path, PathBuf};
use dotenv::dotenv;
use std::env;

fn rocket() -> Rocket {
    dotenv::from_filename("../.env").ok();
    let default = rocket_cors::Cors::default();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Initializes database pool with r2d2.
    let pool = db::init_pool(database_url);

    rocket::ignite().manage(pool)
        .attach(default)
        .mount("/people", routes![
               person::get,
               person::create,
               person::list,
               person::get_comments,
        ])
        .mount("/reddit", routes![
               reddit::create,
               reddit::list,
               reddit::reddit_post::create,
               reddit::reddit_post::list,
               reddit::reddit_post::get,
        ])
}

/// Launch our webserver
fn main() {
    rocket().launch();
}
