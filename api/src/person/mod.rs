#![allow(proc_macro_derive_resolution_fallback)]

use db::Conn as DbConn;
use rocket_contrib::Json;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel;
use rocket::response::status::NoContent;
use diesel::prelude::*;
use schema::people::dsl::*;
use schema::{people, reddit};
use reddit::Reddit;

use uuid::Uuid;

#[derive(Deserialize, Serialize, Insertable, PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Associations)]
#[table_name = "people"]
pub struct Person {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize)] 
#[table_name = "people"]
pub struct NewPerson {
    pub name: String
}

pub fn attach_uuid(person: NewPerson) -> Person {
    Person{name: person.name, id: Uuid::new_v4()}
}

#[get("/<person>")]
pub fn get(person: String, conn: DbConn) -> Json<Person> {
    let person_request = people
        .filter(name.eq(person))
        .get_result::<Person>(&*conn);
    Json(person_request.unwrap())
}

#[get("/")]
pub fn list(conn: DbConn) -> Json<Vec<(Person, Option<Reddit>)>> {
    let person_request = people
                            .left_outer_join(reddit::table)
                            .load::<(Person, Option<Reddit>)>(&*conn);
    Json(person_request.unwrap())
}

#[post("/", format = "application/json", data = "<person>")]
fn create(person: Json<NewPerson>, conn: DbConn) -> Json<Person> {
    let new_person = attach_uuid(person.into_inner());
    let nperson = diesel::insert_into(people)
        .values(&new_person)
        .get_result(&*conn);
    Json(nperson.unwrap())
}
