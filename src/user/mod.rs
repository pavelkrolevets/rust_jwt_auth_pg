pub mod model;
pub mod schema;
pub mod auth;
extern crate hmac;
extern crate sha2;

use rocket::*;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use diesel::result::Error;
use rocket_contrib::serve::{StaticFiles};
use self::model::User;
use super::db;
use self::auth::ApiKey;

use hmac::{Hmac, NewMac};
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use sha2::Sha384;
use std::collections::BTreeMap;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
// use crypto::sha2::Sha256;
use uuid::Uuid;
use std::str;

#[post("/register", format = "application/json", data = "<credentials>")]
fn create(credentials: Json<Credentials>, connection: db::Connection) -> Result<status::Created<Json<User>>, Status> {
   let insert = User { 
       id: Uuid::new_v4(),
       email: credentials.email.to_string(),
       password: credentials.password.to_string()
 };
    User::create(insert, &connection)
        .map(|user| person_created(user))
        .map_err(|error| error_status(error))
}

#[get("/info")]
fn info(key: ApiKey) -> Json<JsonValue> {
    Json(json!(
        {
            "success": true,
            "message": key.0
        }
    ))
}

#[get("/info", rank = 2)]
fn info_error() -> Json<JsonValue> {
    Json(json!(
        {
            "success": false,
            "message": "Not authorized"
        }
    ))
}

#[get("/<id>")]
fn read_one(_key: ApiKey, id: String, connection: db::Connection) -> Result<Json<JsonValue>, Status> {
    User::read(Uuid::parse_str(&id).unwrap(), &connection)
        .map(|item| Json(json!(item)))
        .map_err(|_| Status::NotFound)
}

#[put("/<id>", data = "<user>")]
fn update(id: String, user: Json<User>, connection: db::Connection) -> Json<JsonValue> {
    let update = User {  ..user.into_inner() };
    Json(json!({
        "success": User::update(Uuid::parse_str(&id).unwrap(), update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: String, connection: db::Connection) -> Json<JsonValue> {
    Json(json!({
        "success": User::delete(Uuid::parse_str(&id).unwrap(), &connection)
    }))
}

#[get("/sensitive")]
fn sensitive(key: ApiKey) -> String {
    format!("Hello, you have been identified as {}", key.0)
}

#[derive(Serialize, Deserialize)]
struct Credentials {
   email: String,
   password: String
}

#[post("/login", data = "<credentials>")]
fn login(credentials: Json<Credentials>, connection: db::Connection) ->  Result<Json<JsonValue>, Status> {
   
    let email = credentials.email.to_string();
    let password = credentials.password.to_string();
    // Expiration of the token is set to two weeks
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let two_weeks_from_now: u64 = since_the_epoch.as_secs() + 1209600 as u64;

        match User::by_email_and_password(email, password, &connection) {
        None => {
            Err(Status::NotFound)
        },
        Some(user) => {
            // let claims = Registered {
            //     exp: Some(two_weeks_from_now),
            //     sub: Some(user.id.to_hyphenated().to_string()),
            //     ..Default::default()
            // };

            let key: Hmac<Sha384> = Hmac::new_varkey(b"some-secret").unwrap();
            let header = Header {
                algorithm: AlgorithmType::Hs384,
                ..Default::default()
            };

            let mut claims = BTreeMap::new();
            claims.insert("sub", Some(user.id.to_hyphenated().to_string()));

            let token = Token::new(header, claims);

            token.sign_with_key(&key)
                .map(|message| Json(json!({ "success": true, "token": message.as_str() })))
                .map_err(|_| Status::InternalServerError)
        }
    }
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/user", routes![read_one,  create, update, delete, info, info_error, sensitive])
        .mount("/auth", routes![login])
        .mount("/", StaticFiles::from("www"))
}

fn person_created(user: User) -> status::Created<Json<User>> {
    status::Created(
        format!("{host}:{port}/user/{name}", host = host(), port = port(), name = user.id).to_string(),
        Some(Json(user)))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}
