use rocket::http::Status;
use rocket::local::blocking::Client;
// use rocket::tokio;
use zero2prod::startup::{build_rocket_config, startup, startup_default};

#[test]
fn health_check_rocket_test() {
    let client = Client::tracked(startup_default()).unwrap();

    // Test for the health_check endpoint
    let response = client.get("/health_check").dispatch();
    assert_eq!(response.status(), Status::Ok);

    // Test for a non existing endpoint
    let response = client.get("/toto").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn health_check_test() {
    let config = build_rocket_config(None);

    let client = Client::tracked(startup(&config).unwrap()).unwrap();

    // Test for the health_check endpoint
    let response = client.get("/health_check").dispatch();
    assert_eq!(response.status(), Status::Ok);

    // Test for a non existing endpoint
    let response = client.get("/toto").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}
