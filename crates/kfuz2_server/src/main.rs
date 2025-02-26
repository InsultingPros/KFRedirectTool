use kfuz2_server::ServerErrors;
use kfuz2_server::{config::load_config, server::run_server};

#[tokio::main]
/// # Errors
/// _
/// # Panics
/// _
pub async fn main() -> Result<(), ServerErrors> {
    let res = load_config("").await?;
    dbg!(
        &res.config.ip4,
        &res.config.port,
        &res.config.cache_memory_limit,
        &res.config.disk_cache_limit,
        &res.servers
    );

    run_server(&res).await
}
