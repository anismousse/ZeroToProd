use rocket::form::{self, Error, Form};
use rocket::http::Status;
use rocket_db_pools::{sqlx, Connection};

use chrono::Utc;
use regex::Regex;
use uuid::Uuid;

use crate::startup::Newsletter;

use tracing::Instrument;

#[derive(FromForm)]
pub struct Subscriber<'r> {
    #[field(validate = omits("no"))]
    name: &'r str,
    #[field(validate = omits("no"))]
    #[field(validate = validate_email())]
    email: String,
}

fn validate_email<'v>(email: &str) -> form::Result<'v, ()> {
    let email_domain_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    if !email_domain_regex.is_match(email) {
        return Err(Error::validation("invalid email address provided").into());
    }
    Ok(())
}

#[post("/subscriptions", data = "<subscriber>")]
pub async fn subscriptions(
    mut db: Connection<Newsletter>,
    subscriber: Form<Subscriber<'_>>,
) -> Status {
    let request_id = Uuid::new_v4();

    // create span for tracing purpose.
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %subscriber.email,
        subscriber_name= %subscriber.name
        );
    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!("Saving new subscriber details in the database");

    let sql_query = format!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ('{}', '{}', '{}', '{}')
    "#,
        Uuid::new_v4(),
        subscriber.email,
        subscriber.name,
        Utc::now()
    );
    match sqlx::query(&sql_query)
        .execute(&mut *db)
        .instrument(query_span)
        .await
    {
        Ok(_) => {
            Status::Ok
        }
        Err(e) => {
            tracing::error!("Failed to execute the query: {:?}", e);
            Status::InternalServerError
        }
    }
}
