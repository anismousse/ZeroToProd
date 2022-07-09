#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::{Build, Rocket};

#[get("/health_check")]
async fn health_check() -> Status {
    Status::Ok
}

pub fn startup(config: rocket::Config) -> Rocket<Build> {
  rocket::custom(&config).mount("/", routes![health_check])
}


pub fn startup_default() -> Rocket<Build> {
  rocket::build().mount("/", routes![health_check])
}

