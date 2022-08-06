use rocket::http::Status;
mod common;

#[tokio::test]
async fn health_check_rocket_test() {
    let test_app = common::spawn_rocket_client().await;
    let client = test_app.client;

    // Test for the health_check endpoint
    let response = client.get("/health_check").dispatch();
    assert_eq!(response.await.status(), Status::Ok);

    // Test for a non existing endpoint
    let response = client.get("/toto").dispatch();
    assert_eq!(response.await.status(), Status::NotFound);

    common::delete_test_data_base(test_app.configuration).await;
}
