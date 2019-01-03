#![allow(proc_macro_derive_resolution_fallback)]

use db::Conn as DbConn;
use rocket_contrib::json::Json;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel;
use diesel::prelude::*;
use schema::people::dsl::*;
use schema::{people, twitter, reddit, reddit_posts, reddit_comments};
use reddit::Reddit;
use reddit::reddit_post::RedditPost;
use reddit::reddit_comment::RedditComment;
use uuid::Uuid;

mod test;

#[derive(Deserialize, Serialize, Insertable, PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Associations)]
#[table_name = "people"]
pub struct Person {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, Queryable, Deserialize)]
pub struct PersonDetails {
    pub name: String,
    pub reddit_username: Option<String>,
    pub twitter_username: Option<String>,
}

#[derive(Deserialize)]
pub struct NewPerson {
    pub name: String
}

#[derive(Serialize, Queryable, Deserialize)]
pub struct PersonRedditPosts {
    pub title: String,
    pub body: String,
    pub score: i32,
}

pub fn attach_uuid(person: NewPerson) -> Person {
    Person{name: person.name, id: Uuid::new_v4()}
}

/// Get ID of a person by name
pub fn get_id(n: String, conn: &DbConn) -> Uuid {
    people.filter(name.ilike(n)).select(id).first::<Uuid>(&**conn).unwrap()
}

#[get("/<person>")]
pub fn get(person: String, conn: DbConn) -> Json<PersonDetails> {
    let person_request = people
        .filter(name.ilike(person))
        .left_outer_join(reddit::table)
        .left_outer_join(twitter::table)
        .select((name, reddit::dsl::username.nullable(), twitter::dsl::username.nullable()))
        .get_result::<PersonDetails>(&*conn);
    Json(person_request.unwrap())
}

#[get("/<person>/posts")]
pub fn get_posts(person: String, conn: DbConn) -> Json<Vec<PersonRedditPosts>> {
    let person_request = people
        .filter(name.ilike(person))
        .inner_join(reddit::table
                    .inner_join(reddit_posts::table))
        .select((reddit_posts::dsl::title, reddit_posts::dsl::body, reddit_posts::dsl::score))
        .load::<PersonRedditPosts>(&*conn);
    Json(person_request.unwrap())
}

#[get("/<person>/comments")]
pub fn get_comments(person: String, conn: DbConn) -> Json<Vec<PersonRedditPosts>> {
    let person_request = people
        .filter(name.ilike(person))
        .inner_join(reddit::table
                    .inner_join(reddit_comments::table))
        .select((reddit_comments::dsl::submission_title, reddit_comments::dsl::body, reddit_comments::dsl::score))
        .load::<PersonRedditPosts>(&*conn);
    Json(person_request.unwrap())
}

#[get("/")]
pub fn list(conn: DbConn) -> Json<Vec<Person>> {
    let person_request = people.load::<Person>(&*conn);
    Json(person_request.unwrap())
}

#[post("/", format = "application/json", data = "<person>")]
pub fn create(person: Json<NewPerson>, conn: DbConn) -> Json<Person> {
    let new_person = attach_uuid(person.into_inner());
    let nperson = diesel::insert_into(people)
        .values(&new_person)
        .get_result(&*conn);
    Json(nperson.unwrap())
}
