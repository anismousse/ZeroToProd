extern crate rocket;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::{build_rocket_config, startup};

#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    let configuration = get_configuration().expect("Fail to load the configuration");

    // Building configuration object for Rocket
    let config = build_rocket_config(
        Some(configuration.application_port),
        Some(configuration.database.connection_string()),
    );

    // launch Rocket
    startup(&config)
        .expect("Failed to bind address")
        .launch()
        .await;
}
