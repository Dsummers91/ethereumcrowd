use db::Conn as DbConn;
use rocket_contrib::Json;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel;
use rocket::response::status::NoContent;
use diesel::prelude::*;
use schema::people::dsl::*;
use schema::people;

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Associations)]
#[table_name = "people"]
pub struct Person {
    pub id: i32,
    pub name: String
}

#[derive(Deserialize, Insertable, Serialize)] 
#[table_name = "people"]
pub struct NewPerson {
    pub name: String
}

#[get("/<person>")]
pub fn get(person: String, conn: DbConn) -> Json<Person> {
    let person_request = people.filter(name.eq(person)).get_result::<Person>(&*conn);
    Json(person_request.unwrap())
}

#[get("/")]
pub fn list(conn: DbConn) -> Json<Vec<Person>> {
    let person_request = people.load::<Person>(&*conn);
    Json(person_request.unwrap())
}

#[post("/", format = "application/json", data = "<person>")]
fn create(person: Json<NewPerson>, conn: DbConn) -> Json<Person> {
    let new_person = diesel::insert_into(people)
        .values(&person.into_inner())
        .get_result(&*conn);
    Json(new_person.unwrap())
}
