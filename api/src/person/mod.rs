use postgres::{Connection, TlsMode};
use rocket_contrib::Json;

#[derive(PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Associations)]
#[table_name = "people"]
pub struct Person {
    pub id: Option<i32>,
    pub name: String
}


#[get("/<name>")]
pub fn get_person(conn: DbConn, name: String) -> Json<Person> {
    let conn = Connection::connect("postgres://postgres:postgres@localhost:5432/test", TlsMode::None).unwrap();
    let rows = &conn.query("SELECT id, name FROM people WHERE name = $1", &[&name]).unwrap();

    let row = rows.get(0);

    let person = Person {
        id: row.get(0),
        name: row.get(1),
    };
    Json(person)
}


#[get("/")]
pub fn get_people() -> Json<Vec<Person>> {
    let conn = Connection::connect("postgres://postgres:postgres@localhost:5432/test", TlsMode::None).unwrap();
    let mut people: Vec<Person> = vec![];

    let rows = &conn.query("SELECT id, name FROM people", &[]).unwrap();

    let row = rows.get(0);
    for row in rows {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
        };
        people.push(person);
    };
    Json(people)
}

#[post("/", format = "application/json", data = "<person>")]
pub fn create_person(person: Json<Person>) -> Json<Person> {
    let conn = Connection::connect("postgres://postgres:postgres@localhost:5432/test", TlsMode::None).unwrap();
    conn.execute("INSERT INTO people (name) VALUES ($1)", 
        &[&person.name]).unwrap();
    person
}

