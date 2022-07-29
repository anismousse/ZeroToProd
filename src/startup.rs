use crate::routes::health_check::health_check;
use crate::routes::subscriptions::subscriptions;

use rocket::Config;
use rocket::{Build, Rocket};
use std::net::TcpListener;

pub fn startup(config: &rocket::Config) -> Result<Rocket<Build>, std::io::Error> {
    let server = rocket::custom(config).mount("/", routes![health_check, subscriptions]);
    Ok(server)
}

pub fn startup_default() -> Rocket<Build> {
    rocket::build().mount("/", routes![health_check, subscriptions])
}

pub fn build_rocket_config(port_input: Option<u16>) -> rocket::Config {
    // Get available port
    let port = match port_input {
        Some(value) => value,
        None => match TcpListener::bind("127.0.0.1:0") {
            Ok(listener) => listener.local_addr().unwrap().port(),
            Err(_) => panic!("No port available"),
        },
    };
    // let port = match TcpListener::bind("127.0.0.1:0") {
    //     Ok(listener) => listener.local_addr().unwrap().port(),
    //     Err(_) => panic!("No port available"),
    // };

    // Building configuration object for Rocket
    Config {
        port,
        ..Config::debug_default()
    }
}
