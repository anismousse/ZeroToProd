use crate::routes::health_check::health_check;
use crate::routes::subscriptions::subscriptions;

use rocket::figment::Figment;
use rocket::{Build, Rocket};
use std::net::TcpListener;

use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("newsletter")]
pub struct Newsletter(sqlx::PgPool);

pub fn startup(config: &Figment) -> Result<Rocket<Build>, std::io::Error> {
    let server = rocket::custom(config)
        .mount("/", routes![health_check, subscriptions])
        .attach(Newsletter::init());
    Ok(server)
}

pub fn build_rocket_config(port_input: Option<u16>, db_url: Option<String>) -> Figment {
    // Get available port
    let port = match port_input {
        Some(value) => value,
        None => match TcpListener::bind("127.0.0.1:0") {
            Ok(listener) => listener.local_addr().unwrap().port(),
            Err(_) => panic!("No port available"),
        },
    };

    // Get DB url if one is provided. If not extract it from the ENV variables.
    let url = match db_url {
        Some(value) => value,
        None => dotenv!("DATABASE_URL").into(),
    };

    // Building configuration object for Rocket
    rocket::Config::figment().merge(("port", port)).merge((
        "databases.newsletter",
        rocket_db_pools::Config {
            url,
            min_connections: None,
            max_connections: 1024,
            connect_timeout: 3,
            idle_timeout: None,
        },
    ))
}
