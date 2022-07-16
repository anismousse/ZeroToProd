use std::vec;

use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::blocking::Client;
use zero2prod::configuration::get_configuration;
use zero2prod::{build_rocket_config, startup};

fn get_rocket_client() -> Client {
    let configuration = get_configuration().expect("Fail to load the configuration");
    // Building configuration object for Rocket
    let config = build_rocket_config(Some(configuration.application_port));

    Client::tracked(startup(&config).expect("Failed to bind address")).unwrap()
}

#[test]
fn test_subscriptions_with_valid_form_data_rocket_test() {
    let client = get_rocket_client();
    let body = "name=Akin%20Mousse&email=anismousse%40gmail.com";
    let response = client
        .post("/subscriptions")
        .header(ContentType::Form)
        .body(body)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_subscriptions_with_invalid_form_data_rocket_test() {
    let client = get_rocket_client();
    let test_cases = vec![
        ("missing email", "name=Akin%20Mousse"),
        ("missing name", "email=anismousse%40gmail.com"),
        ("missing name and email", ""),
        ("an incorrect email", "email=anismousse%40gm%40gmail.com"),
    ];

    for (err_msg, incorrect_body) in test_cases {
        let response = client
            .post("/subscriptions")
            .header(ContentType::Form)
            .body(&incorrect_body)
            .dispatch();

        assert_eq!(
            response.status(),
            Status::UnprocessableEntity,
            "The API did not fail with 422 despite {} in the payload.",
            err_msg
        );
    }
}
