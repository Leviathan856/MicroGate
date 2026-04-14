use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::response::Response;

pub async fn serve_file(base_dir: impl AsRef<Path>, request_path: &str) -> Response {
    let mut path = PathBuf::from(base_dir.as_ref());

    // Minimal path traversal prevention
    let safe_path = request_path.trim_start_matches('/');
    if safe_path.contains("..") {
        return Response::bad_request();
    }
    path.push(safe_path);

    if path.is_dir() {
        path.push("index.html");
    }

    match tokio::fs::read(&path).await {
        Ok(contents) => {
            // Very basic mime guessing
            let ext = path
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();
            let mime = match ext {
                "html" => "text/html",
                "css" => "text/css",
                "js" => "application/javascript",
                "json" => "application/json",
                "png" => "image/png",
                "jpg" | "jpeg" => "image/jpeg",
                _ => "application/octet-stream",
            };

            Response::new()
                .with_header("Content-Type", mime)
                .with_body(contents)
        }
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                Response::not_found()
            } else {
                Response::internal_server_error()
            }
        }
    }
}
