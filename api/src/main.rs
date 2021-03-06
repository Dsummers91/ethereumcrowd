#![feature(plugin)]
#![feature(custom_attribute)]
#![feature(proc_macro_hygiene, decl_macro)]
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
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

mod db;
mod schema;
mod person;
mod reddit;
mod twitter;

use rocket::response::NamedFile;
use rocket::Rocket;
use std::path::{Path, PathBuf};
use dotenv::dotenv;
use std::env;

fn rocket() -> Rocket {
    dotenv::from_filename("../.env").ok();
    let cors_options = rocket_cors::CorsOptions::default();
    let default = cors_options.to_cors().unwrap();

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
               person::get_posts,
        ])
        .mount("/reddit", routes![
               reddit::create,
               reddit::list,
               reddit::get,
               reddit::reddit_post::create,
               reddit::reddit_post::list,
               reddit::reddit_post::get,
               reddit::reddit_comment::create,
               reddit::reddit_comment::list,
               reddit::reddit_comment::get,
        ])
        .mount("/twitter", routes![
               twitter::create,
               twitter::list,
//               twitter::get,
        ])
}

/// Launch our webserver
fn main() {
    rocket().launch();
}
