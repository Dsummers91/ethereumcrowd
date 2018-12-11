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
    reddit_posts (id) {
        id -> Uuid,
        reddit_id -> Uuid,
        post_id -> Varchar,
        body -> Varchar,
    }
}

joinable!(reddit -> people (person_id));
joinable!(reddit_posts -> reddit (reddit_id));

allow_tables_to_appear_in_same_query!(
    people,
    reddit,
    reddit_posts,
);
