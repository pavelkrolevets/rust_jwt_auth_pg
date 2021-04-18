
#![feature(plugin)]
#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
extern crate rocket;


extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate secstr;
extern crate argon2;

#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket::http::Method;

mod db;
mod user;

fn main() {
    dotenv().ok();

    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Get, Method::Post, Method::Patch]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(true);
    
    let mut rocket = rocket::ignite().attach(cors.to_cors().unwrap())
        .manage(db::connect());
    rocket = user::mount(rocket);
    rocket.launch();
}