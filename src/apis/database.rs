use sqlx::{postgres::PgPoolOptions};
use crate::apis::model;
use rocket::http::{Status, ContentType};
use serde_json::json;

#[get("/users")]
pub async fn get_users() -> (Status, (ContentType, String)) {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");
    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
    let pool = PgPoolOptions::new()
    .max_connections(100)
    .connect(&db_url)
    .await.expect("Unable to connect to Postgres");


    // let url = "postgres://doadmin:AVNS_2h3CfD61P1Ta8u2923d@db-postgresql-fra1-82985-do-user-13782354-0.b.db.ondigitalocean.com:25060/defaultdb";
    // let conn = sqlx::postgres::PgConnection::connect(url).await;
    
    // let row = query.fetch_one(conn).await;
    // let user = model::user::User {
    //              id: row.get("id"),
    //              nickname: row.get("nickname"),
    //              email: row.get("email"),
    //          };
    // let results = rows.iter().map(|row| {
    //     model::user::User {
    //         id: row.get("id"),
    //         nickname: row.get("nickname"),
    //         email: row.get("email"),
    //     }
    // }).collect();

    let users = sqlx::query_as!(
        model::user::User,
        r#"SELECT nickname AS "nickname!", email AS "email!" FROM users"#,
        ).fetch_all(&pool).await.unwrap_or_else(|e| {
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