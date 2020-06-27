pub mod model;
pub mod schema;
pub mod auth;

use rocket::*;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use diesel::result::Error;
use self::model::User;
use super::db;
use self::auth::ApiKey;

use jwt::{Header, Registered, Token};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use crypto::sha2::Sha256;


#[post("/register", format = "application/json", data = "<user>")]
fn create(user: Json<User>, connection: db::Connection) -> Result<status::Created<Json<User>>, Status> {
//    let insert = User { ..user.into_inner() };
    User::create(user.into_inner(), &connection)
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
    User::read(id, &connection)
        .map(|item| Json(json!(item)))
        .map_err(|_| Status::NotFound)
}

#[put("/<id>", data = "<user>")]
fn update(id: String, user: Json<User>, connection: db::Connection) -> Json<JsonValue> {
    let update = User {  ..user.into_inner() };
    Json(json!({
        "success": User::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: String, connection: db::Connection) -> Json<JsonValue> {
    Json(json!({
        "success": User::delete(id, &connection)
    }))
}

#[get("/sensitive")]
fn sensitive(key: ApiKey) -> String {
    format!("Hello, you have been identified as {}", key.0)
}

#[derive(Serialize, Deserialize)]
struct Credentials {
   username: String,
   password: String
}

#[post("/login", data = "<credentials>")]
fn login(credentials: Json<Credentials>, connection: db::Connection) ->  Result<Json<JsonValue>, Status> {
    let header: Header = Default::default();
    let username = credentials.username.to_string();
    let password = credentials.password.to_string();
    // Expiration of the token is set to two weeks
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let two_weeks_from_now: u64 = since_the_epoch.as_secs() + 1209600 as u64;

        match User::by_username_and_password(username, password, &connection) {
        None => {
            Err(Status::NotFound)
        },
        Some(user) => {
            let claims = Registered {
                exp: Some(two_weeks_from_now),
                sub: Some(user.id.into()),
                ..Default::default()
            };
            let token = Token::new(header, claims);

            token.signed(b"secret_key", Sha256::new())
                .map(|message| Json(json!({ "success": true, "token": message })))
                .map_err(|_| Status::InternalServerError)
        }
    }
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/user", routes![read_one,  create, update, delete, info, info_error, sensitive])
        .mount("/auth", routes![login])
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
