use rocket::form::{self, Error, Form};
use rocket::http::Status;
use rocket_db_pools::{sqlx, Connection};

use chrono::Utc;
use regex::Regex;
use uuid::Uuid;

use crate::startup::Newsletter;

use tracing::{self};

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

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(subscriber, db),
    fields(
    request_id = %Uuid::new_v4(),
    subscriber_email = %subscriber.email,
    subscriber_name= %subscriber.name
    )
)]
#[post("/subscriptions", data = "<subscriber>")]
pub async fn subscriptions(db: Connection<Newsletter>, subscriber: Form<Subscriber<'_>>) -> Status {
    match insert_subscriptions(db, subscriber).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

#[tracing::instrument(name = "Saving a new subscriber", skip(subscriber, db))]
pub async fn insert_subscriptions(
    mut db: Connection<Newsletter>,
    subscriber: Form<Subscriber<'_>>,
) -> Result<(), sqlx::Error> {
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
    sqlx::query(&sql_query)
        .execute(&mut *db)
        // .instrument(query_span)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute the query: {:?}", e);
            e
        })?;
    Ok(())
}
