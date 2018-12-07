
#![feature(custom_derive)]
use postgres::{Connection, TlsMode};
use rocket_contrib::Json;

#[derive(Serialize, Deserialize, FromForm)]
pub struct Reddit {
    pub id: Option<i32>,
    pub person: String,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct Create_Reddit {
    pub id: Option<i32>,
    pub person_id: i32,
    pub username: String,
}

#[get("/")]
pub fn get_reddit_users() -> Json<Vec<Reddit>> {
    let conn = Connection::connect("postgres://postgres:postgres@localhost:5432/template1", TlsMode::None).unwrap();
    let mut people: Vec<Reddit> = vec![];

    let rows = &conn.query("SELECT r.id, p.name, r.username FROM Reddit r
                            INNER JOIN person p ON p.id = r.person_id", &[]).unwrap();

    let row = rows.get(0);
    for row in rows {
        let person = Reddit {
            id: row.get(0),
            person: row.get(1),
            username: row.get(2)
        };
        people.push(person);
    };
    Json(people)
}

#[post("/", format = "application/json", data = "<person>")]
pub fn create_reddit_user(person: Json<Create_Reddit>) -> Json<Create_Reddit> {
    let conn = Connection::connect("postgres://postgres:postgres@localhost:5432/template1", TlsMode::None).unwrap();
    conn.execute("INSERT INTO Reddit (username, person_id) VALUES ($1, $2)", 
        &[&person.username, &person.person_id]).unwrap();
    person
}
