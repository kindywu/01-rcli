use std::{error::Error, net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::{Path, State},
    http::{header, HeaderName, StatusCode},
    // response::{Html, IntoResponse},
    routing::get,
    Router,
};

use tower_http::services::ServeDir;

use tracing::info;

pub async fn process_http_serve(path: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!(
        "http server listen on {}, serve static path is {:?}",
        addr, path
    );

    let dir_service = ServeDir::new(path.clone())
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_deflate()
        .precompressed_br()
        .precompressed_zstd();

    let state = HttpServeState { path };
    // let addr = format!("0.0.0.0:{0}", port);

    // let app = Router::new().route("/", get(|| async { "Index" }));
    let app = Router::new()
        .route("/", get(file_handler))
        .route("/*path", get(file_handler))
        .nest_service("/tower", dir_service)
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

// async fn file_handler(
//     State(state): State<Arc<HttpServeState>>,
//     path: Option<Path<String>>,
// ) -> impl IntoResponse {
//     let mut full_path = PathBuf::from(&state.path);
//     if let Some(Path(path)) = path {
//         full_path = full_path.join(path);
//     }

//     if !full_path.exists() {
//         (
//             StatusCode::NOT_FOUND,
//             Html(format!("File {} note found", full_path.display())),
//         )
//     } else if full_path.is_dir() {
//         match read_dir_content(full_path.as_path(), Option::None).await {
//             Ok(content) => {
//                 let mut lis = String::new();

//                 for item in content {
//                     let li = item;
//                     lis.push_str(format!("<li><a href={}>{}</a></li>", li, li).as_str())
//                 }
//                 (
//                     StatusCode::OK,
//                     Html(format!("<html><body><ul>{}</ul></body></html>", lis)),
//                 )
//             }
//             Err(e) => (
//                 StatusCode::NOT_FOUND,
//                 Html(format!("<html><body><ul><li>{}</li></ul></body></html>", e)),
//             ),
//         }
//     } else {
//         match tokio::fs::read_to_string(full_path).await {
//             Ok(content) => (StatusCode::OK, Html(content)),
//             Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Html(e.to_string())),
//         }
//     }
// }

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    path: Option<Path<String>>,
) -> (StatusCode, [(HeaderName, &'static str); 1], String) {
    let mut full_path = PathBuf::from(&state.path);
    if let Some(Path(path)) = path {
        full_path = full_path.join(path);
    }

    let mut result = (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html")],
        String::new(),
    );

    if !full_path.exists() {
        result.0 = StatusCode::NOT_FOUND;
        result.2 = format!("File {} note found", full_path.display());
        result
    } else if full_path.is_dir() {
        match read_dir_content(&state.path, full_path.as_path()).await {
            Ok(content) => {
                let mut lis = String::new();

                for item in content {
                    let li = item;
                    lis.push_str(format!("<li><a href={}>{}</a></li>", li, li).as_str())
                }

                result.2 = format!("<html><body><ul>{}</ul></body></html>", lis);
                result
            }
            Err(e) => {
                result.0 = StatusCode::INTERNAL_SERVER_ERROR;
                result.2 = e.to_string();
                result
            }
        }
    } else {
        match tokio::fs::read_to_string(full_path).await {
            Ok(content) => {
                result.2 = content;
                result
            }
            Err(e) => {
                result.0 = StatusCode::INTERNAL_SERVER_ERROR;
                result.2 = e.to_string();
                result
            }
        }
    }
}

async fn read_dir_content(
    root: &std::path::Path,
    dir_path: &std::path::Path,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut list = Vec::new();
    let mut read_dir = tokio::fs::read_dir(dir_path).await?;
    let path_root_prefix = root.to_str().unwrap();
    while let Some(entry) = read_dir.next_entry().await? {
        let path = entry.path();
        let path = path.to_str().unwrap();
        let path = path.strip_prefix(path_root_prefix).unwrap();
        list.push(path.to_owned());
    }
    Ok(list)
}

// http://localhost:8080/
// http://localhost:8080/b64.txt
// http://localhost:8080/sub_dir

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler_exist_file() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("fixtures"),
        });
        let (code, _, body) =
            file_handler(State(state), Some(Path("blake3_plain.txt".to_string()))).await;
        assert_eq!(code, StatusCode::OK);
        assert!(body.starts_with("你好"));
    }

    #[tokio::test]
    async fn test_file_handler_exist_dir() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("fixtures"),
        });
        let (code, _, body) = file_handler(State(state), None).await;

        assert_eq!(code, StatusCode::OK);
        assert!(body.contains("sub_dir"));
    }

    #[tokio::test]
    async fn test_file_handler_not_exist_dir() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("fixtures"),
        });
        let (code, _, _) = file_handler(State(state), Some(Path("abc".to_string()))).await;
        assert_eq!(code, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_file_handler_sub_dir_dir() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("fixtures"),
        });
        let (code, _, body) = file_handler(State(state), Some(Path("sub_dir".to_string()))).await;
        assert_eq!(code, StatusCode::OK);
        assert!(body.contains("Cargo"));
    }
}
