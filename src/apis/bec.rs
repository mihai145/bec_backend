use rocket::http::{Status, ContentType};
use serde_json::json;
use crate::apis::auth;

#[get("/amILoggedIn")]
pub async fn am_i_logged_in(_bearer: auth::bearer::Bearer<'_>) -> (Status, (ContentType, String)) {
    (Status::Accepted, (ContentType::JSON, json!({
        "ok": true,
        "reason": String::from("You are logged in")
    }).to_string()))
}