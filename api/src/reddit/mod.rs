#![allow(proc_macro_derive_resolution_fallback)]
#![feature(custom_derive)]
use db::Conn as DbConn;
use rocket_contrib::Json;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel;
use rocket::response::status::NoContent;
use diesel::prelude::*;
use schema::reddit::dsl::*;
use person::Person;
use schema::{people, reddit};

use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Person, foreign_key = "person_id")]
#[table_name = "reddit"]
pub struct Reddit {
    pub id: Uuid,
    pub person_id: Uuid,
    pub username: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "reddit"]
pub struct Create_Reddit {
    pub person_id: Uuid,
    pub username: String,
}

#[get("/")]
pub fn list(conn: DbConn) -> Json<Vec<Reddit>> {
    let person_request = reddit.load::<Reddit>(&*conn);
    Json(person_request.unwrap())
}

#[post("/", format = "application/json", data = "<reddit_name>")]
pub fn create(reddit_name: Json<Create_Reddit>, conn: DbConn) -> Json<Reddit> {
    let new_reddit = diesel::insert_into(reddit)
        .values(&reddit_name.into_inner())
        .get_result(&*conn);
    Json(new_reddit.unwrap())
}
