use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Sftp {
    pub host: [u8;4],
    pub port: u16,
    pub user: String,
    pub pswd: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub sftp: Sftp,
}

impl Settings {

    pub fn new() -> Result<Self, ConfigError> {

        let s = Config::builder()
            .add_source(File::with_name("config/default").required(true))
            .add_source(Environment::default().separator("_"))
            .build()?;

        s.try_deserialize()
    }

}
