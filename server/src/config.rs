use std::env;

use config::{ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub session_secret: String,
}

impl Config {
    /// Constructs a new [`Config`] by reading from multiple sources.
    ///
    /// In order of reading, the sources are:
    ///
    /// - `config/default.*`
    /// - `config/$RUST_ENV.*`, defaults to `config/production.*` if `RUST_ENV` is undefined
    /// - `config/local.*`
    /// - Environment variables
    ///
    /// If a configuration value is defined in multiple sources, the value in a later source will
    /// override the value in an earlier one.
    pub fn new() -> Result<Self, ConfigError> {
        let rust_env = env::var("RUST_ENV").unwrap_or_else(|_| "production".to_string());
        config::Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", rust_env)).required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::default())
            .build()?
            .try_deserialize()
    }
}
