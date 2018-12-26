#[cfg(test)]

mod tests {
    extern crate diesel;
    extern crate postgres;
    use r2d2_diesel::ConnectionManager;

    use std::env;
    use person::*;
    use db::*;
    use uuid::Uuid;
    use diesel::result::Error;
    use diesel::Connection as DConn;
    use self::postgres::{Connection, TlsMode};
    use schema::people::dsl::*;
    use schema::{people, reddit, reddit_posts};



    #[test]
    fn should_return_correct_information() {
        dotenv::from_filename("../.env").ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let connection = r2d2::Pool::builder().build(manager).expect("db pool").get();

        assert!(connection.is_ok());

        let conn = Conn(connection.unwrap());

        conn.test_transaction::<_, Error, _>(|| {
            diesel::insert_into(people)
                .values((name.eq("Ruby"), id.eq(Uuid::new_v4())))
                .execute(&*conn)?;

            let all_names = people.select(name).load::<String>(&*conn)?;
            assert_eq!(vec!["Ruby"], all_names);

            Ok(())
        });
    }
}
