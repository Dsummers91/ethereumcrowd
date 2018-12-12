use db::Conn as DbConn;
use rocket_contrib::Json;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel;
use rocket::response::status::NoContent;
use diesel::prelude::*;
use schema::reddit_posts::dsl::*;
use schema::{people, reddit, reddit_posts};
use reddit::Reddit;
use uuid::Uuid;
use reddit::get_id;

#[derive(Identifiable, Insertable, Serialize, Deserialize, Queryable, Associations, PartialEq)]
#[belongs_to(Reddit, foreign_key = "reddit_id")]
#[table_name = "reddit_posts"]
pub struct RedditPost {
    pub id: Uuid,
    pub reddit_id: Uuid,
    pub post_id: String,
    pub body: String,
    pub title: String,
    pub score: i32,
    pub subreddit: String,
}

#[derive(Serialize, Deserialize)]
#[table_name = "reddit"]
pub struct NewRedditPost {
    pub post_id: String,
    pub username: String,
    pub body: String,
    pub title: String,
    pub score: i32,
    pub subreddit: String,
}

pub fn populate_new_reddit_post(r: NewRedditPost, conn: &DbConn) -> RedditPost {
    let reddit_uid = get_id(r.username, &*conn);
    RedditPost{post_id: r.post_id, reddit_id: reddit_uid, title: r.title, score: r.score, subreddit: r.subreddit, body: r.body, id: Uuid::new_v4()}
}

#[get("/posts")]
pub fn list(conn: DbConn) -> Json<Vec<RedditPost>> {
    let person_request = reddit_posts.load::<RedditPost>(&*conn);
    Json(person_request.unwrap())
}

#[get("/posts/<username>")]
pub fn get(username: String, conn: DbConn) -> Json<Vec<RedditPost>> {
    let reddit_uid = get_id(username, &conn);
    let person_request = reddit_posts.filter(reddit_id.eq(reddit_uid)).load::<RedditPost>(&*conn);
    Json(person_request.unwrap())
}

#[post("/posts", format = "application/json", data = "<reddit_post>")]
pub fn create(reddit_post: Json<NewRedditPost>, conn: DbConn) -> Json<RedditPost> {
    let nreddit = populate_new_reddit_post(reddit_post.into_inner(), &conn);
    let new_reddit = diesel::insert_into(reddit_posts)
        .values(&nreddit)
        .get_result(&*conn);
    Json(new_reddit.unwrap())
}
