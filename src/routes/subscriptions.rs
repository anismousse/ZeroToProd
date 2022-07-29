use regex::Regex;
use rocket::form::Form;
use rocket::form::{self, Error};

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
pub fn subscriptions(subscriber: Form<Subscriber<'_>>) -> String {
    format!(" {} - {}", subscriber.name, subscriber.email)
}
