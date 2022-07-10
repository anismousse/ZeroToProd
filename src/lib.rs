#[macro_use]
extern crate rocket;

// use std::net::TcpListener;

use rocket::http::Status;
use rocket::Config;
use rocket::{Build, Rocket};

#[get("/health_check")]
async fn health_check() -> Status {
    Status::Ok
}

pub fn startup(config: &rocket::Config) -> Result<Rocket<Build>, std::io::Error> {
    let server = rocket::custom(config).mount("/", routes![health_check]);
    Ok(server)
}

pub fn startup_default() -> Rocket<Build> {
    rocket::build().mount("/", routes![health_check])
}

pub fn build_rocket_config() -> rocket::Config {
    // Get available port
    // let port = match TcpListener::bind("127.0.0.1:0") {
    //     Ok(listener) => listener.local_addr().unwrap().port(),
    //     Err(_) => panic!("No port available"),
    // };

    // Building configuration object for Rocket
    Config {
        port: 8000,
        ..Config::debug_default()
    }
}
