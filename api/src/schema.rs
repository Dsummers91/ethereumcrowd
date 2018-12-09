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
    reddit_posts (id) {
        id -> Uuid,
        reddit_id -> Uuid,
        post_id -> Uuid,
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
