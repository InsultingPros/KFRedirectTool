// pub mod cache;
pub mod compressor;
pub mod config;
pub mod html_templates;
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
