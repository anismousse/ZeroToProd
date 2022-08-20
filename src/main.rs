extern crate rocket;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::{build_rocket_config, startup};
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Fail to load the configuration");

    // Building configuration object for Rocket
    let config = build_rocket_config(
        &configuration.application,
        Some(configuration.database.connection_string()),
    );

    // launch Rocket
    startup(&config)
        .expect("Failed to bind address")
        .launch()
        .await;
}
