use kfuz2_server::{config::load_config, server::run_server};

#[tokio::main]
/// # Errors
/// _
/// # Panics
/// _
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let res = load_config("").await?;
    dbg!(
        &res.ip4,
        &res.port,
        &res.cache_memory_limit,
        &res.disk_cache_limit,
        &res.server
    );

    run_server(&res).await
}
