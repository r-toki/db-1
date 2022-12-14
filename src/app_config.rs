use dotenv::dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Env {
    pub host: String,
    pub port: String,
    pub database_url: String,
}

impl Env {
    pub fn new() -> Result<Self, config::ConfigError> {
        let environment = config::Environment::default().try_parsing(true);
        let config = config::Config::builder()
            .set_default("host", "127.0.0.1")?
            .set_default("port", "8080")?
            .add_source(environment)
            .build()?;
        config.try_deserialize()
    }
}

lazy_static! {
    pub static ref ENV: Env = {
        dotenv().ok();
        Env::new().unwrap()
    };
}
