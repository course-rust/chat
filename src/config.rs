use std::{env, fs::File};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct AppConfig {
    pub server: ServerConfig,
    // pub secret_key: String,
    // pub log_level: String,
    // pub database: DatabaseConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        // Read from ./config.yaml, or /etc/config/config.yaml, or from environment variables like CHAT_CONFIG
        let ret = match (
            File::open("config.yaml"),
            File::open("/etc/config/config.yaml"),
            env::var("CHAT_CONFIG"),
        ) {
            (Ok(reader), _, _) => serde_yaml::from_reader(reader),
            (_, Ok(reader), _) => serde_yaml::from_reader(reader),
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("Config file not found"),
        };

        Ok(ret?)
    }
}
