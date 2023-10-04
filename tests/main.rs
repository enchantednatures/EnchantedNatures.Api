use crate::common::get_or_init_server;
use api::models::PhotoViewModel;
use hyper::{Body, Client, Method, Request, Uri};

mod common;

#[tokio::test]
async fn health_check() {
    let addr = get_or_init_server().await;
    let client = Client::new();
    let uri: Uri = format!("http://{}/health_check", addr).parse().unwrap();

    // Construct JSON payload to create user.
    // Create request.
    let req = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(Body::from(""))
        .expect("request builder");

    let resp = client.request(req).await.expect("request failed");

    // Further assertions based on your API spec, e.g., checking the response JSON.
    assert_eq!(resp.status(), 200);
    assert_eq!(
        resp.headers().get("content-type").unwrap(),
        "application/json"
    );
    // assert the status is ok
    let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    assert_eq!("{\"status\":\"Ok\"}", body);
}

#[tokio::test]
async fn photos() {
    let addr = get_or_init_server().await;
    let client = Client::new();
    let uri: Uri = format!("http://{}/api/v0/photos", addr).parse().unwrap();

    let req = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(Body::from(""))
        .expect("request builder");

    let resp = client.request(req).await.expect("request failed");

    // Ensure status code is 200 OK
    assert_eq!(resp.status(), 200);
    // Ensure content-type is application/json
    assert_eq!(
        resp.headers().get("content-type").unwrap(),
        "application/json"
    );

    // Deserialize the JSON body
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    let photos: Vec<PhotoViewModel> =
        serde_json::from_slice(&body_bytes).expect("Failed to deserialize JSON");

    // Additional assertions based on your API spec and test criteria
    assert!(!photos.is_empty());
}

#[tokio::test]
async fn auth() {
    let addr = get_or_init_server().await;
    let client = Client::new();
    let uri: Uri = format!("http://{}/authorize", addr).parse().unwrap();

    // Construct JSON payload to create user.
    // Create request.
    let req = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(Body::from(""))
        .expect("request builder");

    let resp = client.request(req).await.expect("request failed");

    let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    assert_eq!("", body);
}
