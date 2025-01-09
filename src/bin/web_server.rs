use axum::response::IntoResponse;
use axum::{response, routing::get, Json, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 创建路由
    let app = Router::new().route("/", get(root));

    // 设置监听地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);

    // 运行服务
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 异步处理函数，返回一个简单的 HTML 响应
async fn root() -> impl IntoResponse {
    response::Response::builder()
        .header("Content-Type", "application/json")
        .body(axum::body::Full::from(r#"{"message":"Hello, world!"}"#))
        .unwrap()
}
