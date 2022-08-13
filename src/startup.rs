use crate::configuration::ApplicationSettings;
use crate::routes::health_check::health_check;
use crate::routes::subscriptions::subscriptions;

use rocket::figment::Figment;
use rocket::{Build, Rocket};

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

pub fn build_rocket_config(app_settings: &ApplicationSettings, db_url: Option<String>) -> Figment {

    // Get DB url if one is provided. If not extract it from the ENV variables.
    let url = match db_url {
        Some(value) => value,
        None => dotenv!("DATABASE_URL").into(),
    };

    // Building configuration object for Rocket
    rocket::Config::figment()
        .merge(("address", app_settings.host.to_string()))
        .merge(("port", app_settings.port))
        .merge((
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
