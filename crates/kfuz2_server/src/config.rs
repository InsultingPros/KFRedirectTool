use crate::ServerErrors::{self, TomlDeError};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, net::Ipv4Addr, path::PathBuf, sync::LazyLock};

const CONFIG_NAME: &str = "kfuz2_server.toml";
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| load_config().unwrap_or_default());

// Top level struct to hold the TOML data.
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
    /// Mapping from URL alias (e.g. "`just_server`") to the base directory.
    pub server: HashMap<String, ServerEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerEntry {
    pub url_alias: String,
    pub base_directory: PathBuf,
    pub redirect_directory: PathBuf,
}

#[allow(dead_code)]
impl ServerEntry {
    fn new(url_alias: &str, base_directory: &str, redirect_directory: &str) -> Self {
        Self {
            url_alias: String::from(url_alias),
            base_directory: PathBuf::from(base_directory),
            redirect_directory: PathBuf::from(redirect_directory),
        }
    }
}

impl Default for ServerEntry {
    fn default() -> Self {
        Self {
            url_alias: "test_server".to_string(),
            base_directory: PathBuf::from("D://Games//KF Dedicated Server"),
            redirect_directory: PathBuf::from("D://Games//KF Dedicated Server//Redirect"),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip4: Ipv4Addr::LOCALHOST,
            port: 80,
            cache_memory_limit: 100 * 1024 * 1024, // 100 MB,
            disk_cache_limit: 500 * 1024 * 1024,   // 500 MB
            server: {
                let mut hmap = HashMap::new();
                // let mut server_entry = ServerEntry::new(
                //     "nice_server",
                //     "GameServers/NiceServer",
                //     "GameServers/NiceServer/Server",
                // );
                // hmap.insert(server_entry.url_alias.to_string(), server_entry);

                // server_entry = ServerEntry::new(
                //     "just_server",
                //     "GameServers/JustServer",
                //     "GameServers/JustServer/Server",
                // );
                // hmap.insert(server_entry.url_alias.to_string(), server_entry);

                let server_entry = ServerEntry::default();
                hmap.insert(server_entry.url_alias.to_string(), server_entry);

                hmap
            },
        }
    }
}

/// Loads configuration from the specified file.
/// # Errors
/// _
pub fn load_config() -> Result<Config, ServerErrors> {
    let mut config_content: String = String::new();
    match fs::read_to_string(CONFIG_NAME) {
        Ok(res) => config_content = res,
        Err(e) => {
            eprintln!("Unable to load data from `{CONFIG_NAME}`");
            eprintln!("Error: {e}");
            eprintln!(
                "Created default config file! If you want, stop the server and change the values by your taste."
            );
            let default_config_content = toml::to_string(&Config::default())?;
            fs::write(CONFIG_NAME, default_config_content)?;
        }
    }

    let data: Config = match toml::from_str(&config_content) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(err) => {
            return Err(TomlDeError(err));
        }
    };

    Ok(data)
}
