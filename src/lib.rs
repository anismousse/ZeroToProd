#[macro_use]
extern crate rocket;

use std::net::TcpListener;

use regex::Regex;
use rocket::form::Form;
use rocket::form::{self, Error};
use rocket::http::Status;
use rocket::Config;
use rocket::{Build, Rocket};

#[derive(FromForm)]
struct Subscriber<'r> {
    #[field(validate = omits("no"))]
    name: &'r str,
    #[field(validate = omits("no"))]
    #[field(validate = validate_email())]
    email: &'r str,
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

#[get("/health_check")]
async fn health_check() -> Status {
    Status::Ok
}

#[post("/subscriptions", data = "<subscriber>")]
fn subscriptions(subscriber: Form<Subscriber<'_>>) -> String {
    format!(" {} - {}", subscriber.name, subscriber.email)
}

pub fn startup(config: &rocket::Config) -> Result<Rocket<Build>, std::io::Error> {
    let server = rocket::custom(config).mount("/", routes![health_check, subscriptions]);
    Ok(server)
}

pub fn startup_default() -> Rocket<Build> {
    rocket::build().mount("/", routes![health_check, subscriptions])
}

pub fn build_rocket_config() -> rocket::Config {
    // Get available port
    let port = match TcpListener::bind("127.0.0.1:0") {
        Ok(listener) => listener.local_addr().unwrap().port(),
        Err(_) => panic!("No port available"),
    };

    // Building configuration object for Rocket
    Config {
        port,
        ..Config::debug_default()
    }
}
