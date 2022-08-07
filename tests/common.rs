use sqlx::Executor;
use sqlx::{Connection, PgConnection, PgPool};
use once_cell::sync::Lazy;
use rocket::local::asynchronous::Client;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, Settings};
use zero2prod::startup::{build_rocket_config, startup};
use zero2prod::telemetry::{get_subscriber, init_subscriber};

pub struct TestApp {
    pub client: Client,
    pub bd_pool: PgPool,
    pub configuration: Settings,
}

// Ensure that the `tracing` stack is only initialized once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber("test".into(), "debug".into());
    init_subscriber(subscriber);
});

pub async fn spawn_rocket_client() -> TestApp {
    // Setting up tracing
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    // Get configuration
    let mut configuration = get_configuration().expect("Fail to load the configuration");

    // Update the database configuration to use the test database
    configuration.database.database_name = format!("TEST_DB_{}", Uuid::new_v4().to_string());

    let connection_pool = configure_data_base(&configuration).await;

    // Building Rocket's config object.
    let config = build_rocket_config(
        Some(configuration.application_port),
        Some(configuration.database.connection_string()),
    );

    TestApp {
        client: Client::tracked(startup(&config).unwrap()).await.unwrap(),
        bd_pool: connection_pool,
        configuration,
    }
}

async fn configure_data_base(configuration: &Settings) -> PgPool {
    // Connect to POSTGRES
    let connection_string = configuration.database.connection_string_without_db();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to the database.");

    // Create the Test database
    connection
        .execute(
            format!(
                r#"CREATE DATABASE "{}";"#,
                &configuration.database.database_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to create database.");

    let new_connection_string = configuration.database.connection_string();
    let connection_pool = PgPool::connect(&new_connection_string)
        .await
        .expect("Failed to create database pool.");

    // Execute migration
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

pub async fn delete_test_data_base(configuration: Settings) -> () {
    // Get configuration
    // let configuration = &app.configuration;

    let connection_string = configuration.database.connection_string_without_db();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to the database.");

    let query = format!(
        r#"
        SELECT pg_terminate_backend(pg_stat_activity.pid)
        FROM pg_stat_activity
        WHERE pg_stat_activity.datname = '{}'
        AND pid <> pg_backend_pid();
    "#,
        &configuration.database.database_name
    );
    connection
        .execute(query.as_str())
        .await
        .expect("Failed to create database.");

    let query = format!(
        r#"
      DROP DATABASE "{}";
  "#,
        &configuration.database.database_name
    );
    connection
        .execute(query.as_str())
        .await
        .expect("Failed to create database.");
}
