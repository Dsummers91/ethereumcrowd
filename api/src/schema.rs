table! {
    people (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

table! {
    person (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

table! {
    person_reddit (id) {
        id -> Int4,
        reddit_username -> Nullable<Varchar>,
        person_id -> Nullable<Int4>,
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
    test (id) {
        id -> Int4,
        num -> Nullable<Int4>,
        data -> Nullable<Varchar>,
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
    person,
    person_reddit,
    reddit,
    reddit_comments,
    reddit_posts,
    test,
    twitter,
);
