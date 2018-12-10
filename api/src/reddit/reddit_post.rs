#![allow(proc_macro_derive_resolution_fallback)]
#![feature(custom_derive)]

use db::Conn as DbConn;
use rocket_contrib::Json;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel;
use rocket::response::status::NoContent;
use diesel::prelude::*;
use schema::reddit_posts::dsl::*;
use schema::{people, reddit, reddit_posts};
use diesel::types::Timestamp;
use reddit::Reddit;


use uuid::Uuid;

#[derive(Identifiable, Insertable, Serialize, Deserialize, Queryable, Associations, PartialEq)]
#[belongs_to(Reddit, foreign_key = "reddit_id")]
#[table_name = "reddit_posts"]
pub struct RedditPost {
    pub id: Uuid,
    pub reddit_id: Uuid,
    pub post_id: String,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
#[table_name = "reddit"]
pub struct NewRedditPost {
    pub post_id: String,
    pub reddit_id: Uuid,
    pub body: String
}

pub fn attach_uuid(r: NewRedditPost) -> RedditPost {
    RedditPost{post_id: r.post_id, reddit_id: r.reddit_id, body: r.body, id: Uuid::new_v4()}
}

#[get("/posts")]
pub fn list(conn: DbConn) -> Json<Vec<RedditPost>> {
    let person_request = reddit_posts.load::<RedditPost>(&*conn);
    Json(person_request.unwrap())
}

#[post("/posts", format = "application/json", data = "<reddit_post>")]
pub fn create(reddit_post: Json<NewRedditPost>, conn: DbConn) -> Json<RedditPost> {
    let nreddit = attach_uuid(reddit_post.into_inner());
    let new_reddit = diesel::insert_into(reddit_posts)
        .values(&nreddit)
        .get_result(&*conn);
    Json(new_reddit.unwrap())
}
