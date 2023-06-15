use rocket::http::{Status, ContentType};
use serde_json::json;
use crate::apis::auth;

// endpoint to check if a user is logged in
// this endpoint triggers the request guard associated with the bearer token
// if the request guard grants the request, we will return a succesful response (ie the user is logged in)
#[get("/amILoggedIn")]
pub async fn am_i_logged_in(_bearer: auth::bearer::Bearer<'_>) -> (Status, (ContentType, String)) {
    (Status::Accepted, (ContentType::JSON, json!({
        "ok": true,
        "reason": String::from("You are logged in")
    }).to_string()))
}