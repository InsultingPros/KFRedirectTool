// pub mod cache;
pub mod config;
pub mod server;

#[derive(thiserror::Error, Debug)]
pub enum ServerErrors {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    TomlSerError(#[from] toml::ser::Error),
    #[error(transparent)]
    TomlDeError(#[from] toml::de::Error),
}
