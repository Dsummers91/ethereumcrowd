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

    conn.execute("DROP TABLE person", &[]).unwrap();
    conn.execute("CREATE TABLE person (
                      id              SERIAL PRIMARY KEY,
                      name            VARCHAR NOT NULL,
                      data            BYTEA
                 )", &[]).unwrap();

    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
    &[&me.name, &me.data]).unwrap();
    for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2),
        };
        println!("Found person {}: {}", person.id, person.name);
    }

    rocket::ignite().mount("/person", routes![
                                            person::get_person, 
                                            person::create_person
                                      ]).launch();
}
