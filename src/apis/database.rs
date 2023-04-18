use crate::apis::model;
use rocket::http::{Status, ContentType};
use serde_json::json;
use rocket::serde::json::Json;
use crate::apis::auth;
use crate::apis::postgres;

#[post("/search/nickname", format="json", data="<body>")]
pub async fn get_users(body: Json<model::user::UserNameSearchRequest>) -> (Status, (ContentType, String)) {

    let nickname = format!("{}{}{}", '%', &body.user_name, '%');
    let users = sqlx::query_as!(
        model::user::User,
        r#"SELECT id AS "id!", nickname AS "nickname!", email AS "email!" FROM users WHERE nickname LIKE $1"#,
        nickname
        ).fetch_all(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't fetch data! {}", e);
            Vec::new()
        });

    if users.len() == 0 {
        parse_error()
    } else {
        success_response(json!(model::user::UserSearchResponse{
            ok: true,
            results: users
        }).to_string())
    }
}

#[post("/amIFollowing", format="json", data="<body>")]
pub async fn am_i_following(bearer: auth::bearer::Bearer<'_>, body: Json<model::user::FollowRequest>) -> (Status, (ContentType, String)) {
    if !auth::bearer::match_sub(bearer, body.follower_id).await {
        return (Status::Unauthorized, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("You do not have permission to act on behalf of other users")
        }).to_string()))
    }

    let count = sqlx::query_as!(
        model::user::DbCount,
        r#"SELECT COUNT(*) AS "cnt!" FROM follow WHERE follower_id = $1 AND followee_id = $2"#,
        body.follower_id, body.followee_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't fetch data! {}", e);
            model::user::DbCount{cnt: 0}
        });

    if count.cnt > 0 {
        success_response(json!(model::user::AmIFollowingResponse{
            ok: true,
            following: true
        }).to_string())
    } else {
        success_response(json!(model::user::AmIFollowingResponse{
            ok: true,
            following: false
        }).to_string())
    }
}

#[post("/follow", format="json", data="<body>")]
pub async fn follow(bearer: auth::bearer::Bearer<'_>, body: Json<model::user::FollowRequest>) -> (Status, (ContentType, String)) {
    if !auth::bearer::match_sub(bearer, body.follower_id).await {
        return (Status::Unauthorized, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("You do not have permission to act on behalf of other users")
        }).to_string()))
    }

    let ret = sqlx::query_as!(
        model::user::DbInt,
        r#"INSERT into follow VALUES ($1, $2) RETURNING follower_id AS "cnt!""#,
        body.follower_id, body.followee_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't fetch data! {}", e);
            model::user::DbInt{cnt: 0}
        });

    if ret.cnt != body.follower_id {
        (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("Database insertion failed")
        }).to_string()))
    } else {
        (Status::Accepted, (ContentType::JSON, json!(model::error::Error{
            ok: true,
            reason: String::from("Ok")
        }).to_string()))   
    }
}

#[post("/unfollow", format="json", data="<body>")]
pub async fn unfollow(bearer: auth::bearer::Bearer<'_>, body: Json<model::user::FollowRequest>) -> (Status, (ContentType, String)) {
    if !auth::bearer::match_sub(bearer, body.follower_id).await {
        return (Status::Unauthorized, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("You do not have permission to act on behalf of other users")
        }).to_string()))
    }

    let ret = sqlx::query_as!(
        model::user::DbInt,
        r#"DELETE from follow WHERE follower_id = $1 AND followee_id = $2 RETURNING follower_id AS "cnt!""#,
        body.follower_id, body.followee_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't fetch data! {}", e);
            model::user::DbInt{cnt: 0}
        });

    if ret.cnt != body.follower_id {
        (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("Database removal failed")
        }).to_string()))
    } else {
        (Status::Accepted, (ContentType::JSON, json!(model::error::Error{
            ok: true,
            reason: String::from("Ok")
        }).to_string()))   
    }
}

fn parse_error() -> (Status, (ContentType, String)) {
    let error_response = json!(model::error::Error{
        ok: false,
        reason: String::from("Internal database error!")
    }).to_string();

    (Status::InternalServerError, (ContentType::JSON, error_response))
}

// Augment the response with status code and content type
fn success_response(serialized_json: String) -> (Status, (ContentType, String)) {
    (Status::Accepted, (ContentType::JSON, serialized_json))
}