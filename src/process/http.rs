use std::{error::Error, net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tracing::info;

pub async fn process_http_serve(path: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!(
        "http server listen on {}, serve static path is {:?}",
        addr, path
    );

    let state = HttpServeState { path };
    // let addr = format!("0.0.0.0:{0}", port);

    // let app = Router::new().route("/", get(|| async { "Index" }));
    let app = Router::new()
        .route("/", get(file_handler))
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    path: Option<Path<String>>,
) -> impl IntoResponse {
    let mut full_path = PathBuf::from(&state.path);
    if let Some(Path(path)) = path {
        full_path = full_path.join(path);
    }

    if !full_path.exists() {
        (
            StatusCode::NOT_FOUND,
            Html(format!("File {} note found", full_path.display())),
        )
    } else if full_path.is_dir() {
        match read_dir_content(full_path.as_path(), Option::None).await {
            Ok(content) => {
                let mut lis = String::new();

                for item in content {
                    let li = item;
                    lis.push_str(format!("<li><a href={}>{}</a></li>", li, li).as_str())
                }
                (
                    StatusCode::OK,
                    Html(format!("<html><body><ul>{}</ul></body></html>", lis)),
                )
            }
            Err(e) => (
                StatusCode::NOT_FOUND,
                Html(format!("<html><body><ul><li>{}</li></ul></body></html>", e)),
            ),
        }
    } else {
        match tokio::fs::read_to_string(full_path).await {
            Ok(content) => (StatusCode::OK, Html(content)),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Html(e.to_string())),
        }
    }
}

async fn read_dir_content(
    root: &std::path::Path,
    path: Option<&std::path::Path>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut dir = root.to_path_buf();
    if let Some(path) = path {
        dir = dir.join(path);
    }
    let mut list = Vec::new();
    let mut read_dir = tokio::fs::read_dir(&dir).await?;
    while let Some(entry) = read_dir.next_entry().await? {
        let path = entry.path();
        let path = path.to_str().unwrap();
        let path = path.strip_prefix(root.to_str().unwrap()).unwrap();
        list.push(path.to_owned());
    }
    Ok(list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler_exist_file() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("fixtures"),
        });
        let ir = file_handler(State(state), Some(Path("b64_plain.txt".to_string()))).await;
        assert_eq!(ir.into_response().status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_file_handler_exist_dir() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("fixtures"),
        });
        let ir = file_handler(State(state), None).await;

        assert_eq!(ir.into_response().status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_file_handler_not_exist_dir() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("fixtures"),
        });
        let ir = file_handler(State(state), Some(Path("abc".to_string()))).await;
        assert_eq!(ir.into_response().status(), StatusCode::NOT_FOUND);
    }
}
