use std::vec;

use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::asynchronous::Client;
use sqlx::{Connection, PgConnection};
use zero2prod::configuration::get_configuration;
use zero2prod::configuration::Settings;
use zero2prod::startup::{build_rocket_config, startup};

async fn spawn_rocket_client(configuration: &Settings) -> Client {
    // Building configuration object for Rocket
    let config = build_rocket_config(Some(configuration.application_port));

    Client::tracked(startup(&config).unwrap()).await.unwrap()
}

//#[rocket::tokio::test]
#[tokio::test]
async fn test_subscriptions_with_valid_form_data_rocket_test() {
    // Get configuration
    let configuration = get_configuration().expect("Fail to load the configuration");
    // launch rocket client
    let client = spawn_rocket_client(&configuration);
    // launch connection to the data base
    let connection_string = configuration.database.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to the data base.");

    let body = "name=Akin%20Mousse&email=anismousse%40gmail.com";
    let cl = client.await;
    let response = cl
        .post("/subscriptions")
        .header(ContentType::Form)
        .body(body)
        .dispatch();

    assert_eq!(response.await.status(), Status::Ok);

    // checks that the users is in the database
    let saved = sqlx::query!(
        "SELECT email, name FROM subscriptions ",)
    .fetch_one(&mut connection)
    .await
    .expect("Failed to fetch saved subscribers.");

    assert_eq!(saved.name, "Akin Mousse");
    assert_eq!(saved.email, "anismousse@gmail.com");
}

#[tokio::test]
async fn test_subscriptions_with_invalid_form_data_rocket_test() {
    let configuration = get_configuration().expect("Fail to load the configuration");
    let client = spawn_rocket_client(&configuration);
    let cl = client.await;
    let test_cases = vec![
        ("missing email", "name=Akin%20Mousse"),
        ("missing name", "email=anismousse%40gmail.com"),
        ("missing name and email", ""),
        ("an incorrect email", "email=anismousse%40gm%40gmail.com"),
    ];

    for (err_msg, incorrect_body) in test_cases {
        let response = cl
            .post("/subscriptions")
            .header(ContentType::Form)
            .body(&incorrect_body)
            .dispatch();

        assert_eq!(
            response.await.status(),
            Status::UnprocessableEntity,
            "The API did not fail with 422 despite {} in the payload.",
            err_msg
        );
    }
}
