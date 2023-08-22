use std::net::SocketAddr;

use axum::{Router, Server};
use tower_http::trace::TraceLayer;

pub async fn serve(app: Router, addr: SocketAddr) {
    Server::bind(&addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();
}
