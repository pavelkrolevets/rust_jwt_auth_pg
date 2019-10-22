
#![feature(plugin)]
#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;


extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;

mod db;
mod user;

fn main() {
    dotenv().ok();

    let mut rocket = rocket::ignite()
        .manage(db::connect());
    rocket = user::mount(rocket);
    rocket.launch();
}