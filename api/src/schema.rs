table! {
    people (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

table! {
    reddit (id) {
        id -> Uuid,
        person_id -> Uuid,
        username -> Varchar,
    }
}

table! {
    reddit_comments (id) {
        id -> Uuid,
        reddit_id -> Uuid,
        comment_id -> Varchar,
        body -> Varchar,
        score -> Int4,
        subreddit -> Varchar,
    }
}

table! {
    reddit_posts (id) {
        id -> Uuid,
        reddit_id -> Uuid,
        post_id -> Varchar,
        body -> Varchar,
        title -> Varchar,
        score -> Int4,
        subreddit -> Varchar,
    }
}

table! {
    twitter (id) {
        id -> Uuid,
        person_id -> Uuid,
        username -> Varchar,
    }
}

joinable!(reddit -> people (person_id));
joinable!(reddit_comments -> reddit (reddit_id));
joinable!(reddit_posts -> reddit (reddit_id));
joinable!(twitter -> people (person_id));

allow_tables_to_appear_in_same_query!(
    people,
    reddit,
    reddit_comments,
    reddit_posts,
    twitter,
);
