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
                                                        apis::database::edit_post,
                                                        apis::database::delete_post])
                    .register(apis::routes::ROOT, catchers![apis::catchers::unprocessable_content,
                                                            apis::catchers::bad_request,
                                                            apis::catchers::unauthorized,
                                                            apis::catchers::internal_server_error,
                                                            apis::catchers::not_found])
}