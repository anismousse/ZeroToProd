extern crate rocket;
use rocket::Config;
use std::net::TcpListener;
use zero2prod::startup;

#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    // Get available port
    let port = match TcpListener::bind("127.0.0.1:0") {
        Ok(listener) => listener.local_addr().unwrap().port(),
        Err(_) => panic!("No port available"),
    };

    // Building configuration object for Rocket
    let config = Config {
        port,
        ..Config::debug_default()
    };

    // launch Rocket
    startup(config).launch().await;
}
