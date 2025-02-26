use kfuz2_server::{ServerErrors, config::CONFIG, server::run_server};

#[tokio::main]
/// # Errors
/// _
/// # Panics
/// _
pub async fn main() -> Result<(), ServerErrors> {
    // let res = load_config().await?;
    // CONFIG. = res;
    dbg!(
        &CONFIG.ip4,
        &CONFIG.port,
        &CONFIG.cache_memory_limit,
        &CONFIG.disk_cache_limit,
        &CONFIG.server
    );

    run_server().await
}
