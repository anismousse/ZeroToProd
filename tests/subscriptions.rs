use rocket::http::Status;
use rocket::local::blocking::Client;
use zero2prod::{build_rocket_config, startup, startup_default};

#[test]
fn subscriptions_rocket_test() {
    let client = Client::tracked(startup_default()).unwrap();
    let body = "name=Akin%20Mousse&email=anismousse%40gmail.com";
    let response = client.get("/subscriptions").body(body).dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn subscriptions_test() {
    let config = build_rocket_config();
    let body = "name=Akin%20Mousse&email=anismousse%40gmail.com";
    let client = Client::tracked(startup(&config).unwrap()).unwrap();

    // Test for the health_check endpoint
    let response = client.get("/subscriptions").body(body).dispatch();
    assert_eq!(response.status(), Status::Ok);
}
