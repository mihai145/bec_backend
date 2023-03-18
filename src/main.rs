#[macro_use] extern crate rocket;

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
    rocket::build().mount("/", routes![index])
                    .mount("/", routes![hello])
}