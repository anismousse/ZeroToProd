use rocket::http::Status;
use rocket::local::blocking::Client;
use rocket::tokio;
use zero2prod::{build_rocket_config, startup, startup_default};

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

#[tokio::test]
async fn health_check() {
    let address_port = spawn_app();
    let client = reqwest::Client::new();

    let url = format!("{}/health_check", &address_port);
    // let err_msg = "Can not execute the request";

    let response = client.get(url).send().await.unwrap();
    //.expect(err_msg);

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

fn spawn_app() -> String {
    // Building configuration object for Rocket
    let config = build_rocket_config();
    let server = startup(&config).expect("Failed to bind address");
    let _ = tokio::spawn(server.launch());
    format!("http://{}:{}", config.address, &config.port.to_string())
}
