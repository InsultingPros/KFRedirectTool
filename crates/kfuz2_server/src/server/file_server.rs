use super::MyBoxBody;
use crate::config::CONFIG;
use crate::server::build_response;
use futures_util::TryStreamExt;
use http_body_util::{BodyExt, StreamBody};
use hyper::{Response, StatusCode, body::Frame};
use percent_encoding::percent_decode_str;
use std::path::Path;
use tokio_util::io::ReaderStream;

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
    let Ok(file) = tokio::fs::File::open(&destination_file).await else {
        return build_response(StatusCode::NOT_FOUND, "File not found");
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
