extern crate rocket;
use zero2prod::configuration::get_configuration;
use zero2prod::{build_rocket_config, startup};

#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    let configuration = get_configuration().expect("Fail to load the configuration");

    // Building configuration object for Rocket
    let config = build_rocket_config(Some(configuration.application_port));

    // launch Rocket
    startup(&config)
        .expect("Failed to bind address")
        .launch()
        .await;
}
