use rocket::local::blocking::Client;
use rocket::http::Status;
use zero2prod::startup;

#[test]
fn health_check_rocket_test() {

  let client = Client::tracked(startup()).unwrap();

  // Test for the health_check endpoint
  let response = client.get("/health_check").dispatch();
  assert_eq!(response.status(), Status::Ok);

  // Test for a non existing endpoint
  let response = client.get("/toto").dispatch();
  assert_eq!(response.status(), Status::NotFound);
}
