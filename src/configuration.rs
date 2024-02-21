//! src/configuration

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration reader
    let source = config::File::new("configuration.yaml", config::FileFormat::Yaml);
    let settings = config::Config::builder().add_source(source).build()?;

    // Try to convert the configuration values it read into our Settings type
    settings.try_deserialize::<Settings>()
}
