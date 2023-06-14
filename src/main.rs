#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

mod apis;

#[get("/")]
fn index() -> &'static str {
    "Hello, BEC!"
}

#[get("/<str>")]
fn hello(str: String) -> String {
    format!("Hello, {}!", str)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(apis::routes::ROOT, routes![index, hello])
                    .mount(apis::routes::ROOT, routes![apis::themoviedb::search_movie_name,
                                                        apis::themoviedb::search_movie_id,
                                                        apis::themoviedb::search_actor_name,
                                                        apis::themoviedb::search_actor_id,
                                                        apis::themoviedb::get_genres,
                                                        apis::themoviedb::get_trending])
                    .mount(apis::routes::ROOT, routes![apis::bec::am_i_logged_in])
                    .mount(apis::routes::ROOT, routes![apis::database::get_users,
                                                        apis::database::am_i_following,
                                                        apis::database::follow,
                                                        apis::database::unfollow,
                                                        apis::database::posts,
                                                        apis::database::did_i_review,
                                                        apis::database::post,
                                                        apis::database::get_post,
                                                        apis::database::get_user_posts,
                                                        apis::database::edit_post,
                                                        apis::database::delete_post,
                                                        apis::database::comments,
                                                        apis::database::comment,
                                                        apis::database::get_comment,
                                                        apis::database::edit_comment,
                                                        apis::database::delete_comment,
                                                        apis::database::delete_user,
                                                        apis::database::like_comment,
                                                        apis::database::like_post,
                                                        apis::database::likes_comment,
                                                        apis::database::likes_post,
                                                        apis::database::get_like_comment,
                                                        apis::database::get_like_post,
                                                        apis::database::delete_like_comment,
                                                        apis::database::delete_like_post,
                                                        apis::database::get_leaderboard,
                                                        apis::database::get_notification,
                                                        apis::database::delete_notification])
                    .mount(apis::routes::ROOT,routes![apis::openai::ask_gpt])
                    .register(apis::routes::ROOT, catchers![apis::catchers::unprocessable_content,
                                                            apis::catchers::bad_request,
                                                            apis::catchers::unauthorized,
                                                            apis::catchers::internal_server_error,
                                                            apis::catchers::not_found])
}

#[cfg(test)]
mod tests {
    use rocket::local::asynchronous::Client;
    use rocket::http::{ContentType, Status, Header};
    use serde_json::json;
    use super::*;

    #[test]
    // Sanity test
    fn basic_test() {
        assert_eq!(1+1, 2);
    }

    #[tokio::test]
    // Testing connection with the database
    async fn database_test() {
        let ret = sqlx::query_as!(
            apis::model::user::DbOptionInt,
            r#"SELECT 1 AS cnt"#,
            ).fetch_one(&*(apis::postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
                error!("Couldn't insert data! {}", e);
                apis::model::user::DbOptionInt{cnt: None}
            });

        match ret.cnt {
            Some(cnt) => {
                assert_eq!(cnt, 1);
            }
            None => {
                assert!(false);
            }
        }
    }

    #[tokio::test]
    // Test the unprocessable_content catcher by making a post request with a malformed body
    async fn unprocessable_content_test() {
        let rocket = rocket::build()
                .mount("/", routes![apis::themoviedb::search_movie_name])
                .register("/", catchers![apis::catchers::unprocessable_content]);
        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let response = client.post("/search/movieName")
            .header(Header::new("Content-Type", "application/json"))
            .body(json!({
                "movie_name": true
            }).to_string())
            .dispatch().await;

        assert_eq!(response.status(), Status::UnprocessableEntity);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
    }

    #[tokio::test]
    // Test the bad_request catcher by making a bearer-protected request without a bearer token
    async fn bad_request_test() {
        let rocket = rocket::build()
                .mount("/", routes![apis::bec::am_i_logged_in])
                .register("/", catchers![apis::catchers::bad_request]);
        let client = Client::tracked(rocket).await.expect("valid rocket instance");
        let response = client.get("/amILoggedIn").dispatch().await;

        assert_eq!(response.status(), Status::BadRequest);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
    }

    #[tokio::test]
    // Test the unauthorized catcher by providing a dummy bearer token
    async fn unauthorized_test() {
        let rocket = rocket::build()
                .mount("/", routes![apis::bec::am_i_logged_in])
                .register("/", catchers![apis::catchers::unauthorized]);
        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let response = client.get("/amILoggedIn").header(Header::new("Bearer", "1234")).dispatch().await;

        assert_eq!(response.status(), Status::Unauthorized);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
    }

    #[tokio::test]
    // Test the not_found catcher by getting a route that is not registered
    async fn not_found_test() {
        let rocket = rocket::build()
                        .mount("/", routes![hello])
                        .register("/", catchers![apis::catchers::not_found]);
        let client = Client::tracked(rocket).await.expect("valid rocket instance");
        let response = client.get("/a/b").dispatch().await;

        assert_eq!(response.status(), Status::NotFound);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
    }
}