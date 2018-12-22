use db::Conn as DbConn;
use rocket_contrib::Json;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel;
use rocket::response::status::NoContent;
use diesel::prelude::*;
use schema::reddit_comments::dsl::*;
use schema::{people, reddit, reddit_comments};
use reddit::Reddit;
use uuid::Uuid;
use reddit::get_id;

#[derive(Identifiable, Insertable, Serialize, Deserialize, Queryable, Associations, PartialEq)]
#[belongs_to(Reddit, foreign_key = "reddit_id")]
#[table_name = "reddit_comments"]
pub struct RedditComment {
    pub id: Uuid,
    pub reddit_id: Uuid,
    pub comment_id: String,
    pub body: String,
    pub score: i32,
    pub subreddit: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewRedditComment {
    pub comment_id: String,
    pub username: String,
    pub body: String,
    pub score: i32,
    pub subreddit: String,
}

pub fn populate_new_reddit_comment(r: NewRedditComment, conn: &DbConn) -> RedditComment {
    let reddit_uid = get_id(r.username, &*conn);
    RedditComment{comment_id: r.comment_id, reddit_id: reddit_uid, score: r.score, subreddit: r.subreddit, body: r.body, id: Uuid::new_v4()}
}

#[get("/comments")]
pub fn list(conn: DbConn) -> Json<Vec<RedditComment>> {
    let person_request = reddit_comments.load::<RedditComment>(&*conn);
    Json(person_request.unwrap())
}

#[get("/comments/<username>")]
pub fn get(username: String, conn: DbConn) -> Json<Vec<RedditComment>> {
    let reddit_uid = get_id(username, &conn);
    let person_request = reddit_comments.filter(reddit_id.eq(reddit_uid)).load::<RedditComment>(&*conn);
    Json(person_request.unwrap())
}

#[post("/comments", format = "application/json", data = "<reddit_comment>")]
pub fn create(reddit_comment: Json<NewRedditComment>, conn: DbConn) -> Json<RedditComment> {
    let nreddit = populate_new_reddit_comment(reddit_comment.into_inner(), &conn);
    let new_reddit = diesel::insert_into(reddit_comments)
        .values(&nreddit)
        .get_result(&*conn);
    Json(new_reddit.unwrap())
}
