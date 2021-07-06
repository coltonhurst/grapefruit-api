use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;

/* ----- API Contracts ------ */

#[derive(Deserialize, Serialize)]
struct MemberContract {
    email: String,
    username: String,
    guid: Option<String>,
}

/* ----- /v1/member ----- */

/*
    Get member details. Requires Authorization
    header.
*/
#[get("/v1/member/<guid>")]
fn get_member(guid: &str) -> Json<MemberContract> {
    // fake member GET
    let random_member = MemberContract {
        email: String::from("joe@gmail.com"),
        username: String::from("joe"),
        guid: Some(String::from(guid))
    };

    return Json(random_member);
}

/*
    Create a new member! Provide the username
    and email, and it will return the
    same object back with the member guid.
    Authorization header required, username
    in header should match username from body.

    Expects:
    - Authorization header
      - Basic
      - username:password
      - encoded base64
    - The following JSON body:
    {
        "username": "joe",
        "email": "joe@gmail.com",
    }

    Returns:
    - The following JSON body:
    {
        "username": "joe",
        "email": "joe@gmail.com",
        "guid": "af9f428b-4314-4bf2-b65e-84056822044a"
    }
*/
#[post("/v1/member", data = "<member>")]
fn post_member(mut member: Json<MemberContract>) -> Json<MemberContract> {
    // fake member creation
    member.guid = Some(String::from("af9f428b-4314-4bf2-b65e-84056822044a"));
    return member;
}

/*
    Update a member. The guid determines the member,
    and the Authorization header authenticates.
    Returns the updated values.

    Expects:
    - Authorization header
      - Basic
      - username:password
      - encoded base64
    - The following JSON body:
    {
        "username": "joe",
        "email": "joe2@gmail.com"
    }

    Returns:
    - The following JSON body:
    {
        "username": "joe",
        "email": "joe2@gmail.com",
        "guid": "af9f428b-4314-4bf2-b65e-84056822044a"
    }
*/
#[put("/v1/member/<guid>", data = "<member>")]
fn put_member(guid: &str, mut member: Json<MemberContract>) -> Json<MemberContract> {
    // fake member update
    member.guid = Some(String::from(guid));
    return member;
}

/*
    Delete a member account. Requires the
    Authorization header.

    Expects:
    - Authorization header
*/
#[delete("/v1/member/<guid>")]
fn delete_member(guid: &str) -> String {
    format!("Deleting the member with guid: {}", guid)
}

/* ----- APP START ----- */

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_member, post_member, put_member, delete_member])
}
