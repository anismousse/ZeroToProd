extern crate rocket;
use zero2prod::startup;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = startup().launch().await?;
    Ok(())
}
