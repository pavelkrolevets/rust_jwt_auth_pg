use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};

// use crypto::sha2::Sha256;

use hmac::{Hmac, NewMac};
use jwt::{AlgorithmType, Header, Token, VerifyWithKey};
use sha2::Sha384;
use std::collections::BTreeMap;


pub struct ApiKey(pub String);

pub fn read_token(incoming: &str) -> Result<String, String> {
    let token_str = Token::parse_unverified(incoming)
        .map_err(|_| "Unable to parse key".to_string())?;

    let key: Hmac<Sha384> = Hmac::new_varkey(b"some-secret")
        .map_err(|_| "Unable to parse key".to_string())?;
    
    let token: Token<Header, BTreeMap<String, String>, _> = token_str.verify_with_key(&key)
        .map_err(|_| "Token not valid".to_string())?;
    let header = token.header();
    let claims = token.claims();

    if header.algorithm == AlgorithmType::Hs384 {
        Ok(claims["sub"].clone())
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