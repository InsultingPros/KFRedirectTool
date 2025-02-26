use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use kfuz2_server::{ServerErrors, config::CONFIG, server::handle_request};
use std::net::{IpAddr, SocketAddr};
use tokio::net::TcpListener;

#[tokio::main]
/// # Errors
/// _
/// # Panics
/// _
pub async fn main() -> Result<(), ServerErrors> {
    dbg!(
        &CONFIG.ip4,
        &CONFIG.port,
        &CONFIG.cache_memory_limit,
        &CONFIG.disk_cache_limit,
        &CONFIG.server
    );

    let addr: SocketAddr = SocketAddr::new(IpAddr::V4(CONFIG.ip4), CONFIG.port);
    let listener = TcpListener::bind(addr).await?;
    let http = http1::Builder::new();
    let graceful = hyper_util::server::graceful::GracefulShutdown::new();
    let mut signal = std::pin::pin!(shutdown_signal());

    let redirect_dir = &CONFIG.server.get("test_server").unwrap().redirect_directory;
    // Create a directory for files if it doesn't exist
    if !redirect_dir.exists() {
        std::fs::create_dir(redirect_dir).expect("Failed to create files directory");
    }

    println!("Server running on http://{addr}");
    println!("Press Ctrl+C to stop the server");

    loop {
        tokio::select! {
            Ok((stream, _)) = listener.accept() => {
                let io = TokioIo::new(stream);
                let conn = http.serve_connection(io, service_fn(handle_request));
                // watch this connection
                let fut = graceful.watch(conn);
                tokio::spawn(async move {
                    if let Err(e) = fut.await {
                        eprintln!("Error serving connection: {:?}", e);
                    }
                });
            }

            _ = &mut signal => {
                drop(listener);
                eprintln!("Graceful shutdown signal received.");
                // stop the accept loop
                break;
            }
        }
    }

    tokio::select! {
        _ = graceful.shutdown() => {
            eprintln!("All connections gracefully closed.");
        },
        _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
            eprintln!("Timed out wait for all connections to close.");
        }
    }

    Ok(())
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}
