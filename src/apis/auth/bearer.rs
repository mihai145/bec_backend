use std::str;
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use jsonwebtoken::{decode, Algorithm, Validation, DecodingKey};

use crate::apis::auth::models;

use super::public_key::AUTH0_PKEY;

#[derive(Debug, Clone, Copy)]
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
                let public_key = AUTH0_PKEY.get().await;
                let (n, e);

                match public_key {
                    Some(key) => {
                        n = key.n.clone();
                        e = key.e.clone();
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

pub async fn match_sub(bearer: Bearer<'_>, id: i32) -> bool {
    let public_key = AUTH0_PKEY.get().await;
    let (n, e);

    match public_key {
        Some(key) => {
            n = key.n.clone();
            e = key.e.clone();
        }
        None => {
            print!("Error while getting the public key");
            return false;
        }
    }

    let token = decode::<models::Claims>(&bearer.0, 
        &DecodingKey::from_rsa_components(n.as_ref(), e.as_ref()).unwrap(), 
        &Validation::new(Algorithm::RS256));

    match token {
        Ok(token) => {
            if token.claims.sub.strip_prefix("auth0|").unwrap() == id.to_string() {
                return true
            }
            return false
        }
        Err(e) => {
            print!("{}", e);
            return false
        }
    }
}

pub async fn is_admin(bearer: Bearer<'_>) -> bool {
    let public_key = AUTH0_PKEY.get().await;
    let (n, e);

    match public_key {
        Some(key) => {
            n = key.n.clone();
            e = key.e.clone();
        }
        None => {
            print!("Error while getting the public key");
            return false;
        }
    }

    let token = decode::<models::Claims>(&bearer.0, 
        &DecodingKey::from_rsa_components(n.as_ref(), e.as_ref()).unwrap(), 
        &Validation::new(Algorithm::RS256));

    match token {
        Ok(token) => {
            if token.claims.https_example_com_role == "admin" {
                return true
            }
            return false
        }
        Err(e) => {
            print!("{}", e);
            return false
        }
    }
}