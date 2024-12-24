use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use tracing::info;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(dir: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", dir, addr);

    let state = HttpServeState { path: dir.clone() };
    let app = Router::new()
        .nest_service("/tower", ServeDir::new(dir))
        .route("/*dir", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn file_handler(
    Path(path): Path<String>,
    State(state): State<Arc<HttpServeState>>,
) -> (StatusCode, String) {
    let path = std::path::Path::new(&state.path).join(path);
    if !path.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File {} note found", path.display()),
        )
    } else {
        match tokio::fs::read_to_string(path).await {
            Ok(content) => (StatusCode::OK, content),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        }
    }
}
