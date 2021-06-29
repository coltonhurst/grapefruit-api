#[macro_use] extern crate rocket;

#[get("/v1/member/<guid>")]
fn get_member(guid: &str) -> String {
    format!("Return the member with guid id: {}", guid)
}

#[post("/v1/member/<username>")]
fn post_member(username: &str) -> String {
    format!("Creating the member with username: {}", username)
}

#[put("/v1/member/<guid>")]
fn put_member(guid: &str) -> String {
    format!("Updating the member with guid: {}", guid)
}

#[delete("/v1/member/<guid>")]
fn delete_member(guid: &str) -> String {
    format!("Deleting the member with guid: {}", guid)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_member, post_member, put_member, delete_member])
}
