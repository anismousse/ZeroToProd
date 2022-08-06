// use std::vec;

use rocket::http::ContentType;
use rocket::http::Status;
mod common;

#[tokio::test]
async fn test_subscriptions_with_correct_form_data() {
    let test_app = common::spawn_rocket_client().await;

    let body = "name=Akin%20Mousse&email=anismousse%40gmail.com";
    let cl = test_app.client;
    let response = cl
        .post("/subscriptions")
        .header(ContentType::Form)
        .body(body)
        .dispatch();

    assert_eq!(response.await.status(), Status::Ok);

    // checks that the users is in the database
    let saved = sqlx::query!("SELECT email, name FROM subscriptions;")
        .fetch_one(&test_app.bd_pool)
        .await
        .expect("Failed to fetch saved subscribers.");

    assert_eq!(saved.name, "Akin Mousse");
    assert_eq!(saved.email, "anismousse@gmail.com");
    let _ = cl.terminate().await;
    common::delete_test_data_base(test_app.configuration).await;
}

#[tokio::test]
async fn test_subscriptions_with_incorrect_form_data() {
    let test_app = common::spawn_rocket_client().await;
    let client = test_app.client;
    let test_cases = vec![
        ("missing email", "name=Anis%20Mousse"),
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
            response.await.status(),
            Status::UnprocessableEntity,
            "The API did not fail with 422 despite {} in the payload.",
            err_msg
        );
    }
    let _ = client.terminate().await;
    common::delete_test_data_base(test_app.configuration).await;
}
