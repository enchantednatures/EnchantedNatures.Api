use crate::common::*;
use api::models::PhotoViewModel;

mod common;

#[test]
fn health_check() {
    use_app(async {
        let response = reqwest::get("http://localhost:6969/health_check")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        assert_eq!(&response[..], "{\"status\":\"Ok\"}");
    })
}

#[test]
fn photos() {
    use_app(async {
        let response = reqwest::get("http://localhost:6969/api/v0/photos")
            .await
            .unwrap()
            .json::<Vec<PhotoViewModel>>()
            .await
            .unwrap();

        assert!(!response.is_empty())
    })
}

#[test]
fn auth() {
    use_app(async {
        let response = reqwest::get("http://localhost:6969/authorize")
            .await
            .unwrap();

        assert!(response
            .url()
            .to_string()
            .contains("auth.enchantednatures.com"))
        // assert!(response.len() > 0)
    })
}
