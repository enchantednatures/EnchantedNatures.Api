use std::task::{Context, Poll};
use tower::{Service, Layer};

use crate::domain::AppState;

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
