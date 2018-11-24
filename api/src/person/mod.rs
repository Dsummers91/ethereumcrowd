use postgres::{Connection, TlsMode};
use rocket_contrib::Json;

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
}


#[get("/<name>")]
pub fn get_person(name: String) -> Json<Person> {
    let conn = Connection::connect("postgres://postgres:postgres@localhost:5432", TlsMode::None).unwrap();
    let rows = &conn.query("SELECT id, name FROM person WHERE name = $1", &[&name]).unwrap();

    let row = rows.get(0);

    let person = Person {
        id: row.get(0),
        name: row.get(1),
    };
    Json(person)
}

#[post("/", format = "application/json", data = "<person>")]
pub fn create_person(person: Json<Person>) -> Json<Person> {
    let conn = Connection::connect("postgres://postgres:postgres@localhost:5432", TlsMode::None).unwrap();
    conn.execute("INSERT INTO person (name) VALUES ($1)", 
        &[&person.name]).unwrap();
    person
}

