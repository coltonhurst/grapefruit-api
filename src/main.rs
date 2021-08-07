use base64::{decode, encode};
use rocket::http::Method;
use rocket::serde::json::Json;
use rocket::serde::ser::StdError;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio;
use rocket::{get, routes};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::str;
use substring::Substring;
use tokio_postgres::{Error, NoTls};

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
    new_authorization: Option<String>,
    error: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct UpdateMember {
    authorization: String,
    email: Option<String>,
    username: Option<String>,
    guid: Option<String>,
    new_authorization: String,
    error: Option<String>,
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

fn decode_auth(encoded: &String) -> (String, String) {
    let auth = base64::decode(encoded).unwrap();
    let auth_string = match str::from_utf8(&auth) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let index_of_colon = auth_string.find(':').unwrap();

    return (
        auth_string.substring(0, index_of_colon).to_string(),
        auth_string
            .substring(index_of_colon + 1, auth_string.chars().count())
            .to_string(),
    );
}

#[post("/v1/login", data = "<member>")]
async fn login(member: Json<Member>) -> Json<Member> {
    let (auth_email, auth_password) = decode_auth(&member.authorization);

    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=admin dbname=grapefruit",
        NoTls,
    )
    .await
    .unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let mut return_member = Member {
        authorization: member.authorization.clone(),
        email: Some(String::from("")),
        username: Some(String::from("")),
        guid: Some(String::from("")),
        new_authorization: None,
        error: Some(String::from("")),
    };

    let mut guid_from_db: String = String::from("");
    // when passing the vars in the query, can do something like this with as_ref()
    // "SELECT id, email, username, pass, guid FROM public.members WHERE email=$1 AND pass=$2",
    // &[&member.email.as_ref(), &auth_password],
    for row in client
        .query(
            "SELECT email, username, guid FROM public.members WHERE email=$1 AND pass=$2",
            &[&auth_email, &auth_password],
        )
        .await
        .unwrap()
    {
        return_member.email = row.get(0);
        return_member.username = row.get(1);
        guid_from_db = row.get(2);

        println!("found member: {:?}", return_member.email);
    }

    return_member.guid = Some(guid_from_db.clone());

    if (guid_from_db.eq("")) {
        return_member.error = Some("incorrect email or password".to_string());
    }

    return Json(return_member);
}

#[put("/v1/member/<guid>", data = "<member>")]
async fn put_member(guid: &str, member: Json<UpdateMember>) -> Json<UpdateMember> {
    let (auth_email, auth_password) = decode_auth(&member.authorization);
    let (new_auth_email, new_auth_password) = decode_auth(&member.new_authorization);
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=admin dbname=grapefruit",
        NoTls,
    )
    .await
    .unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let mut return_member = UpdateMember {
        authorization: member.authorization.clone(),
        email: Some(String::from("")),
        username: member.username.clone(),
        guid: Some(String::from("")),
        new_authorization: String::from(""),
        error: Some(String::from("")),
    };

    for row in client
        .query(
            "UPDATE public.members SET email=$1, pass=$2 WHERE guid=$3",
            &[&new_auth_email, &new_auth_password, &member.guid.as_ref()],
        )
        .await
        .unwrap()
    {
    }

    /*let mut guid_from_db: String = String::from("");
    for row in client
        .query(
            "SELECT guid FROM public.members WHERE $1",
            &[&new_auth_email],
        )
        .await
        .unwrap()
    {
      guid_from_db = row.get(0);

      println!("found member: {:?}", return_member.email);
    }*/

    return_member.email = Some(new_auth_email.clone());
    return_member.username = member.username.clone();
    return_member.guid = member.guid.clone();
    return_member.new_authorization = member.new_authorization.clone();

    /*if (guid_from_db.eq("")) {
        return_member.error = Some("error trying to update the account".to_string());
    }*/

    return Json(return_member);
}

/*
#[post("/v1/member", data = "<member>")]
fn post_member(mut member: Json<Member>) -> Json<Member> {
    let (auth_email, auth_password) = decode_auth(&member.authorization);
    let mut error: String = "".to_string();

    let random_member = Member {
        authorization: member.authorization.clone(),
        email: member.email.clone(),
        username: member.username.clone(),
        guid: Some(String::from("0c50569f-3a4e-4703-b4c9-f46515adeb54")),
        new_authorization: None,
        error: Some(error),
    };

    return Json(random_member);
}
*/

/*
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
                login,
                put_member,
                /*delete_member,
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
