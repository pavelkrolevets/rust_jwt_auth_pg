use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};

use crypto::sha2::Sha256;

use hmac::{Hmac, NewMac};
use jwt::{AlgorithmType, Header, Token, VerifyWithKey};
use sha2::Sha384;
use std::collections::BTreeMap;


pub struct ApiKey(pub String);

pub fn read_token(key_inc: &str) -> Result<String, String> {
    let token = Token::<Header>::parse(key_inc)
        .map_err(|_| "Unable to parse key".to_string())?;
    let key: Hmac<Sha256> = Hmac::new_varkey(b"some-secret")?;
    if token.verify_with_key(&key) {
        token.claims.sub.ok_or("Claims not valid".to_string())
    } else {
        Err("Token not valid".to_string())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();
        if keys.len() != 1 {
            return Outcome::Forward(());
        }
        match read_token(keys[0]) {
            Ok(claim) => Outcome::Success(ApiKey(claim)),
            Err(_) => Outcome::Forward(())
        }
    }
}