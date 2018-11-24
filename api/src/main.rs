#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate postgres;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod person;

use postgres::{Connection, TlsMode};
use person::{Person};

fn main() {
    let conn = Connection::connect("postgres://postgres:postgres@localhost:5432", TlsMode::None).unwrap();
    rocket::ignite().mount("/person", routes![
                                            person::get_person, 
                                            person::create_person
                                      ]).launch();
}
