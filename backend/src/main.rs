use routes::lab3_route;
use utilities::CORS;

#[macro_use]
extern crate rocket;

pub mod lab3;
pub mod routes;
pub mod utilities;

#[launch]
async fn rocket() -> _ {
    rocket::build().attach(CORS).mount("/", routes![lab3_route])
}
