extern crate rocket;
use zero2prod::startup;

#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    startup().launch().await;
}
