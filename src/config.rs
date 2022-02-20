use config::{Config, Environment};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u32,
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> AppConfig {
        let mut config: Config = Config::new();
        config.merge(Environment::default())
            .expect("Error reading environment");

        config.try_into()
            .expect("Error parsing config")
    }
}