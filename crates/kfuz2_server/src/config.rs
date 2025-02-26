use crate::ServerErrors::{self, TomlDeError};
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use tokio::fs;

const CONFIG_NAME: &str = "kfuz2_server.toml";

// Top level struct to hold the TOML data.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConfigData {
    pub config: Config,
    /// Mapping from URL alias (e.g. "`just_server`") to the base directory.
    pub servers: Servers,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// IP to listen.
    pub ip4: Ipv4Addr,
    /// Port to listen on.
    pub port: u16,
    /// Maximum memory (in bytes) for caching compressed files.
    pub cache_memory_limit: usize,
    /// Maximum disk space (in bytes) for the optional disk cache.
    pub disk_cache_limit: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip4: Ipv4Addr::LOCALHOST,
            port: 8080,
            cache_memory_limit: 100 * 1024 * 1024, // 100 MB,
            disk_cache_limit: 500 * 1024 * 1024,   // 500 MB
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Servers {
    pub server_entry: Vec<ServerEntry>,
}

impl Default for Servers {
    fn default() -> Self {
        Self {
            server_entry: vec![ServerEntry::default()],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerEntry {
    pub url_alias: String,
    pub base_directory: String,
}

impl Default for ServerEntry {
    fn default() -> Self {
        Self {
            url_alias: String::from("just_server"), // nice_server
            base_directory: String::from("GameServers/JustServer"), // GameServers/NiceServer
        }
    }
}

/// Loads configuration from the specified file.
/// # Errors
/// _
pub async fn load_config(_: &str) -> Result<ConfigData, ServerErrors> {
    let mut contents = (fs::read_to_string(CONFIG_NAME).await).unwrap_or_else(|_| String::new());

    // println!("{contents}");

    let data: ConfigData = match toml::from_str(&contents) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(err) => {
            println!(
                "Unable to load data from `{CONFIG_NAME}`, error: {err}. Created new file for you! Go fill the fields by your taste."
            );
            contents = toml::to_string(&ConfigData::default())?;
            fs::write(CONFIG_NAME, contents).await?;
            return Err(TomlDeError(err));
        }
    };

    Ok(data)
}
