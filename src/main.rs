extern crate rocket;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::{build_rocket_config, startup};

#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    // Redirect all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to set logger");

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        "zero2prod".into(),
        // Output the formatted spans to stdout.
        std::io::stdout,
    );

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    // Set subscriber that will be used to process spans.
    set_global_default(subscriber).expect("Failed to set subscriber");

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
