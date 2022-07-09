extern crate rocket;
use zero2prod::{build_rocket_config, startup};

#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    // Building configuration object for Rocket
    let config = build_rocket_config();

    // launch Rocket
    startup(&config)
        .expect("Failed to bind address")
        .launch()
        .await;
}
