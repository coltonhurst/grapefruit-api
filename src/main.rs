use std::error::Error;

use rocket::http::Method;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, routes};

use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[macro_use]
extern crate rocket;

/* ----- API Contracts ------ */

#[derive(Deserialize, Serialize)]
struct MemberContract {
    email: String,
    username: String,
    guid: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct PostContract {
    title: String,
    body: String,
    author: String,
    creation_date: String,
    likes: u32,
    comments: Vec<CommentContract>,
    guid: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct CommentContract {
    body: String,
    author: String,
    creation_date: String,
    likes: u32,
    guid: Option<String>,
}

/* ----- /v1/member ----- */

#[get("/v1/member/<guid>")]
fn get_member(guid: &str) -> Json<MemberContract> {
    // fake member GET
    let random_member = MemberContract {
        email: String::from("joe@gmail.com"),
        username: String::from("joe"),
        guid: Some(String::from(guid)),
    };

    return Json(random_member);
}

#[post("/v1/member", data = "<member>")]
fn post_member(mut member: Json<MemberContract>) -> Json<MemberContract> {
    // fake member creation
    member.guid = Some(String::from("af9f428b-4314-4bf2-b65e-84056822044a"));
    return member;
}

#[put("/v1/member/<guid>", data = "<member>")]
fn put_member(guid: &str, mut member: Json<MemberContract>) -> Json<MemberContract> {
    // fake member update
    member.guid = Some(String::from(guid));
    return member;
}

#[delete("/v1/member/<guid>")]
fn delete_member(guid: &str) -> String {
    format!("Deleting the member with guid: {}", guid)
}

/* ----- /v1/post ----- */

#[get("/v1/post/<guid>")]
fn get_post(guid: &str) -> Json<PostContract> {
    // fake post GET
    let random_post = PostContract {
        title: String::from("Test Post Title"),
        body: String::from(
            "Here is some text that represents the post body! For right now, it's just plain text!",
        ),
        likes: 32,
        guid: Some(String::from(guid)),
        author: String::from("coltonhurst"),
        comments: vec![CommentContract {
            body: String::from("Hello! Great post, this is my first comment!"),
            author: String::from("random002"),
            creation_date: String::from("2021-07-19"),
            likes: 12,
            guid: Some(String::from("af9f428b-4314-4bf2-b65e-84056822044a")),
        }],
        creation_date: String::from("2021-07-17"),
    };

    return Json(random_post);
}

#[post("/v1/post", data = "<post>")]
fn post_post(mut post: Json<PostContract>) -> Json<PostContract> {
    // fake post creation
    post.guid = Some(String::from("af9f428b-4314-4bf2-b65e-84056822044a"));
    post.likes = 12;
    return post;
}

#[put("/v1/post/<guid>", data = "<post>")]
fn put_post(guid: &str, mut post: Json<PostContract>) -> Json<PostContract> {
    // fake post update
    post.guid = Some(String::from(guid));
    return post;
}

#[delete("/v1/post/<guid>")]
fn delete_post(guid: &str) -> String {
    format!("Deleting the post with guid: {}", guid)
}

/* ----- APP START ----- */

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let allowed_origins = AllowedOrigins::some_exact(&["http://127.0.0.1:8080"]);

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::build()
        .mount(
            "/",
            routes![
                get_member,
                post_member,
                put_member,
                delete_member,
                get_post,
                post_post,
                put_post,
                delete_post
            ],
        )
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
