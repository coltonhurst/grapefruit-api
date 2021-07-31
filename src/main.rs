use std::error::Error;
use std::str;
use rocket::http::Method;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, routes};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use base64::{encode, decode};
use substring::Substring;

#[macro_use]
extern crate rocket;
extern crate base64;

/* ----- Models ------ */

#[derive(Deserialize, Serialize)]
struct Member {
    authorization: String,
    email: Option<String>,
    username: Option<String>,
    guid: Option<String>,
    new_authorization: Option<String>
}
/*
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
*/
/* ----- /v1/member ----- */

fn decode_auth(encoded: &String) -> (String, String) {
  let auth = base64::decode(encoded).unwrap();
  let auth_string = match str::from_utf8(&auth) {
    Ok(v) => v,
    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
  };
  let index_of_colon = auth_string.find(':').unwrap();

  return (auth_string.substring(index_of_colon+1, auth_string.chars().count()).to_string(),
          auth_string.substring(0, index_of_colon).to_string());
}

/*
  Requires:
  - email
  - password
*/
#[post("/v1/login", data = "<member>")]
fn login(member: Json<Member>) -> Json<Member> {
    let (auth_email, auth_password) = decode_auth(&member.authorization);

    if auth_email.eq("kotrunga@gmail.com") && auth_password.eq("pass") {
      println!("The username matches!");
    }

    let random_member = Member {
        authorization: member.authorization.clone(),
        email: member.email.clone(),
        username: member.username.clone(),
        guid: Some(String::from("fake-guid")),
        new_authorization: None
    };

    return Json(random_member);


    /*

    authorization: String,
    email: Option<String>,
    username: Option<String>,
    guid: Option<String>,
    new_authorization: Option<String>

    */
}

/*
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
*/
/* ----- APP START ----- */

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let allowed_origins = AllowedOrigins::some_exact(&["http://127.0.0.1:8081"]);

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
                login,
                /*post_member,
                put_member,
                delete_member,
                get_post,
                post_post,
                put_post,
                delete_post*/
            ],
        )
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
