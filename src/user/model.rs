#![allow(proc_macro_derive_resolution_fallback)]
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use super::schema::users;
use argon2::{self, Config};

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub password: String
}

impl User {
    pub fn create(user: User, connection: &PgConnection) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(&InsertableUser::from_user(user))
            .get_result(connection)
    }

    pub fn read(id: String, connection: &PgConnection) -> QueryResult<Vec<User>> {
        if id != "".to_string() {
            users::table.find(id).load::<User>(connection)
        } else {
            users::table.order(users::id).load::<User>(connection)
        }
    }

    pub fn by_username_and_password(username_: String, password_: String, connection: &PgConnection) -> Option<User> {
        let salt = b"somesalt";
        let config = Config::default();
        let hash = argon2::hash_encoded(&password_.as_bytes(), salt, &config).unwrap();
        println!("Hashed password {:?}", &hash);
        let res = users::table
            .filter(users::id.eq(username_))
            .filter(users::password.eq(hash))
            .order(users::id)
            .first(connection);
        match res {
            Ok(user) => Some(user),
            Err(_) => {
                None
            }
        }
    }

    pub fn update(id: String, user: User, connection: &PgConnection) -> bool {
        diesel::update(users::table.find(id)).set(&user).execute(connection).is_ok()
    }

    pub fn delete(id: String, connection: &PgConnection) -> bool {
        diesel::delete(users::table.find(id)).execute(connection).is_ok()
    }
}

#[derive(Insertable)]
#[table_name = "users"]
struct InsertableUser {
    id: String,
    password: String,
}

impl InsertableUser {

    fn from_user(user: User) -> InsertableUser {
        let salt = b"somesalt";
        let config = Config::default();
        let hash = argon2::hash_encoded(&user.password.as_bytes(), salt, &config).unwrap();
        InsertableUser {
            id: user.id,
            password: hash,
        }
    }
}