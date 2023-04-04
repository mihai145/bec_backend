use std::str;
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use jsonwebtoken::{decode, Algorithm, Validation, DecodingKey};

use crate::apis::auth::models;

async fn get_public_key() -> Option<models::Key> {
    let response = reqwest
        ::get("https://dev-jc1flmgwmyky8n0k.us.auth0.com/.well-known/jwks.json")
        .await.unwrap();

    match response.json::<models::PublicKey>().await {
        Ok(res) => Some(res.keys[0].clone()),
        Err(_) => None
    }
}

pub struct Bearer<'r>(&'r str);

#[derive(Debug)]
pub enum BearerError {
    Missing,
    Invalid
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Bearer<'r> {
    type Error = BearerError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("bearer") {
            None => Outcome::Failure((Status::BadRequest, BearerError::Missing)),
            Some(bearer) => {
                print!("{}", bearer);

                // Should cache the public key... But HOW? I gave up, Rust won
                let public_key = get_public_key().await;
                let (n, e);

                match public_key {
                    Some(key) => {
                        n = key.n;
                        e = key.e;
                    }
                    None => {
                        print!("Error while getting the public key");
                        return Outcome::Failure((Status::InternalServerError, BearerError::Invalid))
                    }
                }

                let token = decode::<models::Claims>(&bearer, 
                    &DecodingKey::from_rsa_components(n.as_ref(), e.as_ref()).unwrap(), 
                    &Validation::new(Algorithm::RS256));

                match token {
                    Ok(_) => {
                        Outcome::Success(Bearer(bearer))
                    }
                    Err(e) => {
                        print!("{}", e);
                        Outcome::Failure((Status::Unauthorized, BearerError::Invalid))
                    }
                }
            }
        }
    }
}
