use db::Conn as DbConn;
use rocket_contrib::Json;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel;
use rocket::response::status::NoContent;
use diesel::prelude::*;
use schema::reddit::dsl::*;
use person::{Person, get_id as get_person_id};
use schema::{people, reddit};

use uuid::Uuid;

pub mod reddit_post;
mod test;

#[derive(Identifiable, Insertable, Serialize, Deserialize, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Person, foreign_key = "person_id")]
#[table_name = "reddit"]
pub struct Reddit {
    pub id: Uuid,
    pub person_id: Uuid,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
#[table_name = "reddit"]
pub struct NewReddit {
    pub person: String,
    pub username: String,
}

pub fn populate_new_redditor(r: NewReddit, conn: &DbConn) -> Reddit {
    let person_uid = get_person_id(r.person, &*conn);
    Reddit{person_id: person_uid, username: r.username, id: Uuid::new_v4()}
}

pub fn get_id(n: String, conn: &DbConn) -> Uuid {
    reddit.filter(username.eq(n)).select(id).first::<Uuid>(&**conn).unwrap()
}

#[get("/")]
pub fn list(conn: DbConn) -> Json<Vec<Reddit>> {
    let person_request = reddit.load::<Reddit>(&*conn);
    Json(person_request.unwrap())
}

#[post("/", format = "application/json", data = "<reddit_name>")]
pub fn create(reddit_name: Json<NewReddit>, conn: DbConn) -> Json<Reddit> {
    let nreddit = populate_new_redditor(reddit_name.into_inner(), &conn);
    let new_reddit = diesel::insert_into(reddit)
        .values(&nreddit)
        .get_result(&*conn);
    Json(new_reddit.unwrap())
}
