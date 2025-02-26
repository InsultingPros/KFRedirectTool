use bytes::Bytes;
use futures_util::TryStreamExt;
use http_body_util::{BodyExt, Full, StreamBody, combinators::BoxBody};
use hyper::body::Frame;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, Result, StatusCode};
use hyper_util::rt::TokioIo;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use tokio::{fs::File, net::TcpListener};
use tokio_util::io::ReaderStream;

use crate::config::ConfigData;

static INDEX: &str = "index.html";
static NOTFOUND: &[u8] = b"Not Found";

pub async fn run(args: &ConfigData) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let address: SocketAddr = SocketAddr::new(IpAddr::V4(args.config.ip4), args.config.port);
    let listener: TcpListener = TcpListener::bind(address).await?;
    println!("Listening on http://{address}");

    let test_path: PathBuf = PathBuf::from(INDEX);
    println!(
        "test_path is: {:#?}, exists: {:#?}",
        test_path,
        test_path.try_exists()
    );
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(response_examples))
                .await
            {
                println!("Failed to serve connection: {err:?}");
            }
        });
    }
}

async fn response_examples(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, std::io::Error>>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/" | "/index.html") => {
            print!("sending index!");
            simple_file_send(&PathBuf::from(INDEX)).await
        }
        (&Method::GET, "/no_file.html") => {
            // Test what happens when file cannot be found
            simple_file_send(&PathBuf::from("this_file_should_not_exist.html")).await
        }
        _ => Ok(not_found()),
    }
}

/// HTTP status code 404
fn not_found() -> Response<BoxBody<Bytes, std::io::Error>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(NOTFOUND.into()).map_err(|e| match e {}).boxed())
        .unwrap()
}

async fn simple_file_send(filename: &PathBuf) -> Result<Response<BoxBody<Bytes, std::io::Error>>> {
    // Open file for reading
    let file = File::open(filename).await;
    if file.is_err() {
        eprintln!("ERROR: Unable to open file.");
        return Ok(not_found());
    }

    let file: File = file.unwrap();

    // Wrap to a tokio_util::io::ReaderStream
    let reader_stream = ReaderStream::new(file);

    // Convert to http_body_util::BoxBody
    let stream_body = StreamBody::new(reader_stream.map_ok(Frame::data));
    let boxed_body = stream_body.boxed();

    // Send response
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(boxed_body)
        .unwrap();

    Ok(response)
}
