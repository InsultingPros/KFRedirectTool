use http_body_util::{BodyExt as _, Full};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use percent_encoding::percent_decode_str;
use std::convert::Infallible;
use std::fs::File;
use std::io::Read;
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use tokio::net::TcpListener;
use tokio::signal;

use crate::config::Config;

const REDIRECT_DIR: &str = "D://Games//KF Dedicated Server//Redirect";

const HTML_TEMPLATE1: &str = r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>File Download Server</title>
                <link
                    rel="stylesheet"
                    href="https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.min.css"
                >
                </head>
                <body>
                    <h1>Available Files</h1>
                    <ul>
                        <li><a href="/download/example.txt">example.txt</a></li>
                        <li><a href="/download/KF_Invasion.u.uz2">KF_Invasion.u.uz2</a></li>
                    </ul>
                </body>
            </html>
            "#;

const HTML_TEMPLATE2: &str = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta name="color-scheme" content="light dark">
    <link rel="stylesheet" href="css/pico.min.css">
    <title>Hello world!</title>
  </head>
  <body>
    <main class="container">
      <h1>Hello world!</h1>
    </main>
  </body>
</html>"#;

type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

// Helper function to convert a string to BoxBody
fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

/// # Errors
/// _
/// # Panics
/// _
pub async fn run_server(args: &Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Define the address to bind to
    let addr: SocketAddr = SocketAddr::new(IpAddr::V4(args.ip4), args.port);
    // Create TCP listener
    let listener: TcpListener = TcpListener::bind(addr).await?;

    // Create a directory for files if it doesn't exist
    if !Path::new(REDIRECT_DIR).exists() {
        std::fs::create_dir(REDIRECT_DIR).expect("Failed to create files directory");
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

async fn handle_request(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody>, Infallible> {
    let method = req.method();
    let path = req.uri().path();
    dbg!(method, path);

    match (method, path) {
        (&hyper::Method::GET, "/") => {
            // Serve a simple HTML page with download links
            let html = HTML_TEMPLATE1;
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(full(html))
                .unwrap())
        }
        // if path.starts_with("/download/")
        (&hyper::Method::GET, path) => {
            // Extract the filename from the path
            // let filename = &path[10..]; // Skip "/download/"
            let filename = percent_decode_str(&path[10..])
                .decode_utf8()
                .unwrap_or_default()
                .to_string();

            // For security, check that the filename doesn't contain path traversal
            if filename.contains("..") {
                return Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(full("Invalid filename"))
                    .unwrap());
            }

            // Construct the file path (assuming files are in a "files" directory)
            let file_path = format!("{REDIRECT_DIR}/{filename}");

            // Try to open the file
            match File::open(&file_path) {
                Ok(mut file) => {
                    // Read the file contents
                    let mut contents = Vec::new();
                    if file.read_to_end(&mut contents).is_err() {
                        return Ok(Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(full("Failed to read file"))
                            .unwrap());
                    }

                    // Set Content-Disposition header for download
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header(
                            "Content-Disposition",
                            format!("attachment; filename=\"{filename}\""),
                        )
                        .header("Content-Type", "application/octet-stream")
                        .body(full(contents))
                        .unwrap())
                }
                Err(_) => {
                    // File not found
                    Ok(Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(full("File not found"))
                        .unwrap())
                }
            }
        }
        // Return 404 for other routes
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(full("Not Found"))
            .unwrap()),
    }
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
