#[macro_use] extern crate rocket;

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
                    .register(apis::routes::ROOT, catchers![apis::catchers::unprocessable_content,
                                                            apis::catchers::not_found])
}