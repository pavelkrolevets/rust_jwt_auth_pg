
#![feature(plugin)]
extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;


mod db;
mod user;

fn main() {
    let mut rocket = rocket::ignite()
        .manage(db::connect());
    rocket = user::mount(rocket);
    rocket.launch();
}