#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate rocket_cors;
extern crate postgres;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod person;
mod reddit;

use postgres::{Connection, TlsMode};
use person::{Person};
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, AllowedHeaders};


fn main() {
    let conn = Connection::connect("postgres://postgres:postgres@localhost:5432/template1", TlsMode::None).unwrap();

    let default = rocket_cors::Cors::default();

    let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost:4200"]);
    
    assert!(failed_origins.is_empty());

    // You can also deserialize this
    let options = rocket_cors::Cors {
        allowed_origins: AllowedOrigins::all(), 
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    };

    rocket::ignite().mount("/people", routes![
                           person::get_person, 
                           person::create_person,
                           person::get_people,
                    ])
                    .mount("/reddit", routes![
                           reddit::get_reddit_users, 
                           reddit::create_reddit_user,
                    ])

                    .attach(default)
                    .launch();
}
