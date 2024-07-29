use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

/// 处理http请求
pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    // 定义监听的端口
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    // 使用 tracing 输出监听端口的日志
    info!("Serveing {:?} on port{},", path, addr);

    // 将启动命令时传入的path交给state
    let state = HttpServeState { path: path.clone() };

    let service = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd(); // 开启几个压缩方式的开关

    let router = Router::new()
        .nest_service("/tower", service)
        .route("/*path", get(file_handler))
        .route("/", get(hello_world_handler))
        .with_state(Arc::new(state)); // 多个线程中，都会对这个内存指定，只有进程结束，或者指针为0时， state才会被释放掉
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn hello_world_handler() -> &'static str {
    "Hello world! This is a Rust http server!!!"
}

// 可以在 handler中，使用match的方式，获取路由中绑定过来的state
async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    // (StatusCode::OK, format!("匹配到用户输入的 {}, 服务器绑定目录 {}", &path, &state.path.to_str().unwrap()))
    let file_path = std::path::Path::new(&state.path).join(path);
    if file_path.exists() && file_path.is_file() {
        match tokio::fs::read_to_string(file_path).await {
            Ok(content) => {
                info!("Reading {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(err) => {
                warn!("Error reading file: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("请求文件失败: {}", err),
                )
            }
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            format!("404 Not Found: {:?}", file_path.display()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });

        let (statue, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        println!("read file: {:?}", content);
        assert_eq!(statue, StatusCode::OK);
        assert!(content.starts_with("[package]"));
        // run this test use nextest show detail
        // cargo nextest run -- test_file_handler
    }
}
