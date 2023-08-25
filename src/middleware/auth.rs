use axum::http::Request;
use hyper::http;
use serde::{Deserialize, Serialize};
use std::task::{Context, Poll};
use tower::{Layer, Service};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone)]
struct AuthClient;

#[derive(Debug, Clone)]
struct Auth<S> {
    inner: S,
    state: AuthClient,
}

impl<S> Auth<S> {
    fn new(inner: S) -> Self {
        Self {
            inner,
            state: AuthClient,
        }
    }
}

impl<S, B> Service<Request<B>> for Auth<S>
where
    S: Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        // Do something with `self.state`.
        //
        // See `axum::RequestExt` for how to run extractors directly from
        // a `Request`.
        let auth_header = req
            .headers()
            .get(http::header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok());

        self.inner.call(req)
    }
}

#[derive(Debug, Clone)]
struct AuthLayer {
    state: AuthClient,
}

impl<S> Layer<S> for AuthLayer {
    type Service = Auth<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Auth {
            inner,
            state: self.state.clone(),
        }
    }
}
