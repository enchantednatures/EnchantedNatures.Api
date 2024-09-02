mod helpers;
use api::models::PhotoViewModel;

use axum::http::StatusCode;
use hyper::Request;

use axum::{
    body::Body,
    http::{self},
};
use tower::ServiceExt;

use crate::helpers::test_app;

#[tokio::test]
async fn default() {
    let app = test_app().await;

    let request = Request::builder()
        .uri("/health_check")
        .method(http::Method::GET)
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(request).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn get_all_photos() {
    let app = test_app().await;

    let request = Request::builder()
        .uri("/api/v0/photos")
        .method(http::Method::GET)
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(request).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    let body: Vec<PhotoViewModel> = serde_json::from_slice(&body).unwrap();

    assert!(!body.is_empty());
    // let body: = serde_json::from_str(resp.body()().await.unwrap().as_str())
}

#[tokio::test]
async fn not_found() {
    let app = test_app().await;
    let request = Request::builder()
        .uri("/not-a-valid-path")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert!(body.is_empty());
}
