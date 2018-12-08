table! {
    people (id) {
        id -> Int4,
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
        id -> Int4,
        person_id -> Int4,
        username -> Varchar,
    }
}

table! {
    reddit_posts (id) {
        id -> Int4,
        reddit_id -> Int4,
        post_id -> Int4,
        body -> Varchar,
        create_time -> Timestamp,
    }
}

table! {
    test (id) {
        id -> Int4,
        num -> Nullable<Int4>,
        data -> Nullable<Varchar>,
    }
}

joinable!(reddit -> people (person_id));
joinable!(reddit_posts -> reddit (reddit_id));

allow_tables_to_appear_in_same_query!(
    people,
    person,
    person_reddit,
    reddit,
    reddit_posts,
    test,
);
