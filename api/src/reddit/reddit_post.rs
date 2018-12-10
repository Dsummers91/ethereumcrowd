#![allow(proc_macro_derive_resolution_fallback)]
#![feature(custom_derive)]

#[derive(Identifiable, Insertable, Serialize, Deserialize, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Person, foreign_key = "person_id")]
#[table_name = "reddit_posts"]
pub struct RedditPost {
    pub id: Uuid,
    pub post_ud: Uuid,
    pub reddit_id: Uuid,
    pub body: String
}

#[derive(Serialize, Deserialize)]
#[table_name = "reddit"]
pub struct NewRedditPost {
    pub post_id: Uuid,
    pub reddit_id: Uuid,
    pub body: String
}

pub fn attach_uuid(r: NewRedditPost) -> RedditPost {
    RedditPost{post_id: r.post_id, reddit_id: r.reddt_id, body: r.body, id: Uuid::new_v4()}
}

#[get("/")]
pub fn list(conn: DbConn) -> Json<Vec<Reddit>> {
    let person_request = reddit.load::<Reddit>(&*conn);
    Json(person_request.unwrap())
}

#[post("/", format = "application/json", data = "<reddit_name>")]
pub fn create(reddit_name: Json<NewReddit>, conn: DbConn) -> Json<Reddit> {
    let nreddit = attach_uuid(reddit_name.into_inner());
    let new_reddit = diesel::insert_into(reddit)
        .values(&nreddit)
        .get_result(&*conn);
    Json(new_reddit.unwrap())
}
