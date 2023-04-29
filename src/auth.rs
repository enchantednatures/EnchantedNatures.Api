
// Add the necessary imports at the top of your main.rs or another file
use actix_web::{HttpRequest, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

// Middleware to validate JWT tokens
async fn jwt_validator(
    req: HttpRequest,
    credentials: BearerAuth,
) -> Result<HttpRequest, Error> {
    let secret = "your_jwt_secret";
    let validation = Validation {
        algorithms: vec![Algorithm::HS256],
        ..Default::default()
    };

    match decode::<Claims>(
        &credentials.token(),
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    ) {
        Ok(token_data) => {
            // Store user information in the request's extensions
            req.extensions_mut().insert(token_data.claims);
            Ok(req)
        }
        Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid token")),
    }
}

// Example of a protected route
#[api_v2_operation]
async fn protected_route(req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims = req
        .extensions()
        .get::<Claims>()
        .expect("Claims should be available in the request extensions");

    // Check user's role to determine if they have the required permissions
    if claims.role == "admin" {
        // Perform the protected action
        Ok(HttpResponse::Ok().body("Protected content"))
    } else {
        Err(actix_web::error::ErrorForbidden("Insufficient permissions"))
    }
}

