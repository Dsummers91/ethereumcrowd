
#![feature(custom_derive)]
use postgres::{Connection, TlsMode};
use rocket_contrib::Json;

#[derive(Serialize, Deserialize, FromForm)]
pub struct Person_Reddit {
    pub id: Option<i32>,
    pub person: String,
    #[form(field = "redditUsername")]
    pub reddit_username: String,
d}

#[derive(Serialize, Deserialize)]
pub struct Create_Person_Reddit {
    pub id: Option<i32>,
    pub person_id: i32,
    pub reddit_username: String,
}

#[get("/")]
pub fn get_reddit_users() -> Json<Vec<Person_Reddit>> {
    let conn = Connection::connect("postgres://postgres:postgres@localhost:5432/template1", TlsMode::None).unwrap();
    let mut people: Vec<Person_Reddit> = vec![];

    let rows = &conn.query("SELECT r.id, p.name, r.reddit_username FROM person_reddit r
                            INNER JOIN person p ON p.id = r.person_id", &[]).unwrap();

    let row = rows.get(0);
    for row in rows {
        let person = Person_Reddit {
            id: row.get(0),
            person: row.get(1),
            reddit_username: row.get(2)
        };
        people.push(person);
    };
    Json(people)
}

#[post("/", format = "application/json", data = "<person>")]
pub fn create_reddit_user(person: Json<Create_Person_Reddit>) -> Json<Create_Person_Reddit> {
    let conn = Connection::connect("postgres://postgres:postgres@localhost:5432/template1", TlsMode::None).unwrap();
    conn.execute("INSERT INTO person_reddit (reddit_username, person_id) VALUES ($1, $2)", 
        &[&person.reddit_username, &person.person_id]).unwrap();
    person
}
