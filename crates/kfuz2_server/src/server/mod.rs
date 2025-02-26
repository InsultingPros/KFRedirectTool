mod file_server;
mod html_templates;

use crate::ServerErrors;
use crate::config::CONFIG;
use file_server::handle_file_download;
use html_templates::HTML_TEMPLATE1;
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt as _, Full};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, Result, StatusCode};
use hyper_util::rt::TokioIo;
use std::net::{IpAddr, SocketAddr};
use tokio::net::TcpListener;
use tokio::signal;

// type BoxBody = http_body_util::combinators::BoxBody<Bytes, ServerErrors>;
type MyBoxBody = Response<BoxBody<Bytes, std::io::Error>>;
const FAVICON: &str = "crates//kfuz2_server//src//server//static//favicon.ico";

/// # Errors
/// _
/// # Panics
/// _
pub async fn run_server() -> std::result::Result<(), ServerErrors> {
    // Define the address to bind to
    let addr: SocketAddr = SocketAddr::new(IpAddr::V4(CONFIG.ip4), CONFIG.port);
    // Create TCP listener
    let listener = TcpListener::bind(addr).await?;
    let redirect_dir = &CONFIG.server.get("test_server").unwrap().redirect_directory;

    // Create a directory for files if it doesn't exist
    if !redirect_dir.exists() {
        std::fs::create_dir(redirect_dir).expect("Failed to create files directory");
    }

    println!("Server running on http://{addr}");
    println!("Press Ctrl+C to stop the server");

    // Create a task to handle the shutdown signal
    let shutdown = shutdown_signal();
    let server = async {
        loop {
            // Accept incoming connections
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            // dbg!(&io);

            // Spawn a task to handle the connection
            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service_fn(handle_request))
                    .await
                {
                    eprintln!("Error serving connection: {err:?}");
                }
            });
        }
        #[allow(unreachable_code)]
        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
    };

    // Run the server until a shutdown signal is received
    tokio::select! {
        result = server => {
            if let Err(e) = result {
                eprintln!("Server error: {e:?}");
            }
        },
        () = shutdown => {
            println!("Shutting down gracefully...");
        }
    }

    println!("Server has been shut down");
    Ok(())
}

/// # Errors
/// _
/// # Panics
/// _
pub async fn handle_request(req: Request<hyper::body::Incoming>) -> Result<MyBoxBody> {
    let method = req.method();
    let uri = req.uri().path();
    dbg!(method, uri);

    match (method, uri) {
        // Serve a simple HTML page with download links
        (&hyper::Method::GET, "/") => Ok(build_response(StatusCode::OK, HTML_TEMPLATE1)),

        // favicon
        (&hyper::Method::GET, "/favicon.ico") => Ok(serve_favicon().await),

        // download attempt!
        (&hyper::Method::GET, path) => Ok(handle_file_download(path).await),

        // Return 404 for other routes
        _ => Ok(build_response(
            StatusCode::METHOD_NOT_ALLOWED,
            "Method not allowed!",
        )),
    }
}

fn build_response(status_code: StatusCode, body: &str) -> MyBoxBody {
    Response::builder()
        .status(status_code)
        .body(
            Full::new(body.to_owned().into())
                .map_err(|e| match e {})
                .boxed(),
        )
        .unwrap()
}

/// # Panics
/// _
pub async fn serve_favicon() -> MyBoxBody {
    (tokio::fs::read(FAVICON).await).map_or_else(
        |_| build_response(StatusCode::NOT_FOUND, "Favicon not found"),
        |content| {
            // println!("found it!!!!!!!!!!!!!!");
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "image/x-icon")
                .header("Cache-Control", "public, max-age=86400") // Cache for 24 hours
                .body(
                    Full::new(Bytes::from(content))
                        .map_err(|e| match e {})
                        .boxed(),
                )
                .unwrap()
        },
    )
}

// Simple signal handler that works cross-platform
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }

    println!("Shutdown signal received, starting graceful shutdown");
}
