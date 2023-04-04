use crate::apis::routes;
use crate::apis::model;
use rocket::http::{Status, ContentType};
use rocket::Request;
use serde_json::json;

#[catch(422)]
pub fn unprocessable_content(req: &Request) -> (Status, (ContentType, String)) {
    let uri = req.uri().to_string();
    match uri.as_str() {
        routes::SEARCH_MOVIE_NAME => unprocessable_entity_response("Missing field: movieName"),
        routes::SEARCH_MOVIE_ID => unprocessable_entity_response("Missing field: movieId"),
        routes::SEARCH_ACTOR_NAME => unprocessable_entity_response("Missing field: actorName"),
        routes::SEARCH_ACTOR_ID => unprocessable_entity_response("Missing field: actorId"),
        routes::GET_TRENDING => unprocessable_entity_response("Missing field: genreId"),
        _ => unprocessable_entity_response("Could not parse your request")
    }
}

#[catch(400)]
pub fn bad_request(req: &Request) -> (Status, (ContentType, String)) {
    (Status::BadRequest, (ContentType::JSON, 
        json!(model::error::Error{
            ok: false,
            reason: String::from(req.uri().to_string() + " Bad request. Might be missing the bearer token")
    }).to_string()))
}

#[catch(401)]
pub fn unauthorized(req: &Request) -> (Status, (ContentType, String)) {
    (Status::Unauthorized, (ContentType::JSON, 
        json!(model::error::Error{
            ok: false,
            reason: String::from(req.uri().to_string() + " Unauthorized")
    }).to_string()))
}

#[catch(404)]
pub fn not_found(req: &Request) -> (Status, (ContentType, String)) {
    let req_uri = req.uri();
    let req_method = req.method();
    let error_response = json!(model::error::Error{
        ok: false,
        reason: String::from(format!("Path {req_method} {req_uri} does not exist"))
    }).to_string();
    (Status::NotFound, (ContentType::JSON, error_response))
}

#[catch(500)]
pub fn internal_server_error(req: &Request) -> (Status, (ContentType, String)) {
    (Status::InternalServerError, (ContentType::JSON, 
        json!(model::error::Error{
            ok: false,
            reason: String::from(req.uri().to_string() + " Internal server error")
    }).to_string()))
}

// Return a generic error when the body of the request cannot be parsed
fn unprocessable_entity_response(message: &str) -> (Status, (ContentType, String)) {
    (Status::UnprocessableEntity, (ContentType::JSON, 
        json!(model::error::Error{
            ok: false,
            reason: String::from(message)
        }).to_string()))
}