## Rocket.rs + diesel.rs + Postgresql  - restful JWT auth API boilerplate

An example how to set up Rust server restful API with JWT authentication and ORM under the hood.

- User passwords hashed ([argon2](https://crates.io/crates/rust-argon2))

## Requirements

1. Configure Rust to satisfy rocket.rs [dependencies](https://github.com/marcocastignoli/rust_rocket_api_authentication) (Rust nightly build needed)
2. Install Diesel ORM CLI and [dependencies](http://diesel.rs/guides/getting-started/) 
    ```bash
    cargo install diesel_cli
    ```
3. Install and create Postgresql database. More info [here](https://www.codementor.io/engineerapart/getting-started-with-postgresql-on-mac-osx-are8jcopb)
4. Configure project environment variable in `.env` file taking username and passwork from Posgresql step
    ```
    DATABASE_URL=postgres://username:password@localhost/database_name
    ```

## Installation

1. First run the migration to create tables and schema
    ```bash
    diesel migration run
    ``` 
2. Compile wasm front end
    ```bash
    cd www
    wasm-pack build --target web
    cd ..
    ```
3. Compile the code and run
    ```bash
    cargo run
    ```
###### If everything was installed right and compiles without errors you should see Rocekt server listening at http://localhost:8001

## API rout JWT protection

Routs can be protected through JWT check in the message Header

```rust
#[get("/sensitive")]
fn sensitive(key: ApiKey) -> String {
    format!("Hello, you have been identified as {}", key.0)
}
```

## API

###### JWT token life is set to 2 weeks. Can be changed 

### /user/register
```bash
curl -X POST \
  http://localhost:8001/user/register \
  -H 'content-type: application/json' \
  -d '{ "email": "some@email.com",
        "password": "12345"
}'
```

### /auth/login
Get a jwt token for the user
```bash
curl -X POST \
  http://localhost:8001/auth/login \
  -H 'content-type: application/json' \
  -d '{ "email": "some@email.com",
        "password": "12345"
}'
```
### /user
Call a protected route with a JWT in the HEADER `authentication` (use the token returned from the /auth/login API)
```bash
curl -X GET \
  http://localhost:8001/user/sensitive \
  -H 'authentication: eyJ0eXAiOiJKV1QiLCJraWQiOm51bGwsImFsZyI6IkhTMjU2In0.eyJpc3MiOm51bGwsInN1YiI6InRlc3QiLCJhdWQiOm51bGwsImV4cCI6MTU3MzAyNzg5MSwibmJmIjpudWxsLCJpYXQiOm51bGwsImp0aSI6bnVsbH0.DJ5tb/ic91oULyMjZMeam9kMU31sxGSxSnTmTppUhdA'
```



## Based on previous work: 
1. https://github.com/marcocastignoli/rust_rocket_api_authentication
2. https://github.com/sean3z/rocket-diesel-rest-api-example
3. https://medium.com/sean3z/building-a-restful-crud-api-with-rust-1867308352d8
4. https://dzone.com/articles/creating-a-rest-api-in-rust-using-rocket-and-diese