use std::net::SocketAddr;

use axum::{Router, Server};

pub async fn serve(app: Router, addr: SocketAddr) {
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
