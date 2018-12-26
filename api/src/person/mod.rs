#![allow(proc_macro_derive_resolution_fallback)]

use db::Conn as DbConn;
use rocket_contrib::Json;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel;
use rocket::response::status::NoContent;
use diesel::prelude::*;
use schema::people::dsl::*;
use schema::{people, reddit, reddit_posts};
use reddit::Reddit;
use reddit::reddit_post::RedditPost;
use uuid::Uuid;

mod test;

#[derive(Deserialize, Serialize, Insertable, PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Associations)]
#[table_name = "people"]

pub struct Person {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize)]
pub struct NewPerson {
    pub name: String
}

pub fn attach_uuid(person: NewPerson) -> Person {
    Person{name: person.name, id: Uuid::new_v4()}
}

/// Get ID of a person by name
pub fn get_id(n: String, conn: &DbConn) -> Uuid {
    people.filter(name.ilike(n)).select(id).first::<Uuid>(&**conn).unwrap()
}

#[get("/<person>")]
pub fn get(person: String, conn: DbConn) -> Json<Person> {
    let person_request = people
        .filter(name.ilike(person))
        .get_result::<Person>(&*conn);
    Json(person_request.unwrap())
}

#[get("/<person>/posts")]
pub fn get_comments(person: String, conn: DbConn) -> Json<Vec<(Person, (Reddit, RedditPost))>> {
    let person_request = people
        .filter(name.ilike(person))
        .inner_join(reddit::table
                    .inner_join(reddit_posts::table))
        .load::<(Person, (Reddit, RedditPost))>(&*conn);
    Json(person_request.unwrap())
}

#[get("/")]
pub fn list(conn: DbConn) -> Json<Vec<Person>> {
    let person_request = people.load::<Person>(&*conn);
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
