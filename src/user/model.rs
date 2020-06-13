#![allow(proc_macro_derive_resolution_fallback)]
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use super::schema::users;
use scrypt::{ScryptParams, scrypt_simple, scrypt_check};


#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String
}

impl User {
    pub fn create(user: User, connection: &PgConnection) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(&InsertableUser::from_user(user))
            .get_result(connection)
    }

    pub fn read(id: i32, connection: &PgConnection) -> QueryResult<Vec<User>> {
        if id != 0 {
            users::table.find(id).load::<User>(connection)
        } else {
            users::table.order(users::id).load::<User>(connection)
        }
    }

    pub fn by_username_and_password(username_: String, password_: String, connection: &PgConnection) -> Option<User> {
        let res = users::table
            .filter(users::username.eq(username_))
            .filter(users::password.eq(password_))
            .order(users::id)
            .first(connection);
        match res {
            Ok(user) => Some(user),
            Err(_) => {
                None
            }
        }
    }

    pub fn update(id: i32, user: User, connection: &PgConnection) -> bool {
        diesel::update(users::table.find(id)).set(&user).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &PgConnection) -> bool {
        diesel::delete(users::table.find(id)).execute(connection).is_ok()
    }
}

#[derive(Insertable)]
#[table_name = "users"]
struct InsertableUser {
    id: i32,
    username: String,
    password: String,
}

impl InsertableUser {

    fn from_user(user: User) -> InsertableUser {
        let params = ScryptParams::new(15, 8, 1).unwrap();
        let hashed_password = scrypt_simple(&user.password, &params)
            .expect("OS RNG should not fail");

        InsertableUser {
            id: user.id,
            username: user.username,
            password: hashed_password,
        }
    }
}