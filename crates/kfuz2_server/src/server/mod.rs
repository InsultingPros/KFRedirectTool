mod html_templates;

use crate::config::CONFIG;
use futures_util::TryStreamExt;
use html_templates::HTML_TEMPLATE1;
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Full, StreamBody};
use hyper::body::{Bytes, Frame};
use hyper::{Request, Response, Result, StatusCode};
use kfuz2_lib::helper::try_to_compress;
use kfuz2_lib::types::{InputArguments, LogLevel};
use percent_encoding::percent_decode_str;
use std::path::{Path, PathBuf};
use tokio_util::io::ReaderStream;

// type BoxBody = http_body_util::combinators::BoxBody<Bytes, ServerErrors>;
type MyBoxBody = Response<BoxBody<Bytes, std::io::Error>>;
const FAVICON: &str = "crates//kfuz2_server//src//server//static//favicon.ico";

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

        // Return 405 for other routes
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

/// Fix Percent-encoded strings, KF1 love to do that...
fn normalize_path(input: &str) -> String {
    // Skip "/download/"
    percent_decode_str(input)
        .decode_utf8()
        .unwrap_or_default()
        .to_string()
}

pub async fn handle_file_download(path: &str) -> MyBoxBody {
    // For security, check that the filename doesn't contain path traversal
    if path.contains("..") {
        return build_response(StatusCode::BAD_REQUEST, "Invalid filename");
    }

    let nomalized_path = normalize_path(path);
    let requested_file_path: &Path = Path::new(&nomalized_path);

    let Some(requested_file_name) = requested_file_path.file_name() else {
        return build_response(StatusCode::NOT_FOUND, "File not found");
    };

    // TODO add file check and compress if required

    let destination_file = CONFIG
        .server
        .get("test_server")
        .unwrap()
        .redirect_directory
        .join(requested_file_name);

    dbg!(&destination_file);

    // Try to open the file
    let file = match tokio::fs::File::open(&destination_file).await {
        // return build_response(StatusCode::NOT_FOUND, "File not found");
        Ok(result) => result,
        Err(_) => {
            // not found, let's check the directory
            let mut x = InputArguments {
                input_path: PathBuf::from(r"D:\Games\KF Dedicated Server\System\BitCore.u"),
                output_path: PathBuf::from(r"D:\Games\KF Dedicated Server\Redirect"),
                log_level: LogLevel::Minimal,
                ignore_kf_files: true,
            };
            try_to_compress(&mut x).unwrap();
            tokio::fs::File::open(x.output_path).await.unwrap()
        }
    };

    // Get file metadata to determine file size
    let Ok(metadata) = file.metadata().await else {
        return build_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to read file metadata",
        );
    };

    let file_size = metadata.len();

    // Wrap to a tokio_util::io::ReaderStream with a larger buffer
    let reader_stream = ReaderStream::with_capacity(file, 64 * 1024); // Increase buffer size to 64KB

    // Convert to http_body_util::BoxBody
    let stream_body = StreamBody::new(reader_stream.map_ok(Frame::data));
    let boxed_body = stream_body.boxed();

    // Send response with proper headers
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Length", file_size.to_string())
        .header(
            "Content-Disposition",
            format!(
                "attachment; filename=\"{}\"",
                requested_file_name.to_string_lossy()
            ),
        )
        .header("Content-Type", "application/octet-stream")
        .body(boxed_body)
        .unwrap()
}
