use db::Conn as DbConn;
use rocket_contrib::json::Json;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel;
use diesel::prelude::*;
use schema::twitter::dsl::*;
use person::{Person, get_id as get_person_id};
use schema::{people, twitter};
use uuid::Uuid;

#[derive(Identifiable, Insertable, Serialize, Deserialize, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Person, foreign_key = "person_id")]
#[table_name = "twitter"]
pub struct Twitter {
    pub id: Uuid,
    pub person_id: Uuid,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewTwitter {
    pub person: String,
    pub username: String,
}

pub fn populate_new_twitteror(r: NewTwitter, conn: &DbConn) -> Twitter {
    let person_uid = get_person_id(r.person, &*conn);
    Twitter{person_id: person_uid, username: r.username, id: Uuid::new_v4()}
}

pub fn get_id(n: String, conn: &DbConn) -> Uuid {
    twitter.filter(username.eq(n)).select(id).first::<Uuid>(&**conn).unwrap()
}

#[get("/")]
pub fn list(conn: DbConn) -> Json<Vec<Twitter>> {
    let person_request = twitter.load::<Twitter>(&*conn);
    Json(person_request.unwrap())
}

#[post("/", format = "application/json", data = "<twitter_name>")]
pub fn create(twitter_name: Json<NewTwitter>, conn: DbConn) -> Json<Twitter> {
    let ntwitter = populate_new_twitteror(twitter_name.into_inner(), &conn);
    let new_twitter = diesel::insert_into(twitter)
        .values(&ntwitter)
        .get_result(&*conn);
    Json(new_twitter.unwrap())
}
