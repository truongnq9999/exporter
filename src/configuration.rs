//! src/configuration.rs

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub http: HttpSettings,
}

#[derive(serde::Deserialize, Debug)]
pub struct HttpSettings {
    pub host: String,
    pub port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let profile = std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "dev".into());

    let default_config_file = config::File::with_name("application");
    let config_file
        = config::File::with_name(&format!("application.{}", profile)).required(false);

    // warning: order matters here, the last file loaded will overwrite the previous ones
    config::Config::builder()
        // Default configuration
        .add_source(default_config_file)
        // Config values will depend on the profile and not require
        .add_source(config_file.required(false))
        // Environment variables are highest priority and will overwrite all other sources
        // The prefix is used to filter environment variables to only those that start with APP_
        // Environment variables are in the format APP_DATABASE__HOST and will be converted to database.host
        .add_source(config::Environment::with_prefix("APP").separator("__"))
        .build()?
        .try_deserialize()
}
