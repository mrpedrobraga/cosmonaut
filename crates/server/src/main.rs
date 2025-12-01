#[macro_use]
extern crate rocket;

use rocket::get;

mod message;
mod user;
mod world;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello_world, sum])
}

#[get("/")]
fn hello_world() -> String {
    "Hello, World".into()
}

#[get("/sum/<a>/<b>")]
fn sum(a: i32, b: i32) -> String {
    format!("{} + {} = {}", a, b, a + b)
}
