use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

#[macro_use] extern crate rocket;

#[derive(Deserialize)]
struct Member {
    email: String,
    username: String,
    password: String,
}

/* ----- GET ----- */

#[get("/v1/member/<guid>")]
fn get_member(guid: &str) -> String {
    format!("Return the member with guid id: {}", guid)
}

/* ----- POST (CREATE) ----- */

#[post("/v1/member/<username>")]
fn post_member(username: &str) -> String {
    format!("Creating the member with username: {}", username)
}

#[post("/v2/member", data = "<member>")]
fn post_member_v2(member: Json<Member>) -> String {
    format!("Creating the member {}", member.email)
}

/* ----- PUT (UPDATE) ----- */

#[put("/v1/member/<guid>")]
fn put_member(guid: &str) -> String {
    format!("Updating the member with guid: {}", guid)
}

#[put("/v2/member/<guid>", data="<member>")]
fn put_member_v2(guid: &str, member: Json<Member>) -> String {
    format!("Updating guid {}, username is now {}", guid, member.username)
}

/* ----- DELETE ----- */

#[delete("/v1/member/<guid>")]
fn delete_member(guid: &str) -> String {
    format!("Deleting the member with guid: {}", guid)
}

/* ----- APP START ----- */

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_member, post_member, put_member, delete_member, post_member_v2, put_member_v2])
}





