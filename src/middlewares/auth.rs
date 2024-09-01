use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

use crate::services::auth::JWTClaims;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

fn extract_jwt_from_header(req: &ServiceRequest) -> Option<String> {
    req.headers().get("Authorization").and_then(|header_value| {
        header_value.to_str().ok().and_then(|auth_str| {
            if auth_str.starts_with("Bearer ") {
                Some(auth_str.trim_start_matches("Bearer ").to_string())
            } else {
                None
            }
        })
    })
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path().to_string();
        let jwt_token = extract_jwt_from_header(&req);
        let service_call = self.service.call(req);

        // Apply middleware logic
        if path.starts_with("/auth") {
            // Token is valid; proceed with the original service call
            Box::pin(async move {
                // Your JWT validation logic here
                let res = service_call.await?;
                Ok(res)
            })
        } else {
            Box::pin(async move {
                if let Some(jwt_token) = jwt_token {
                    // Validate JWT token (pseudo-code)
                    println!("JWT token: {:?}", jwt_token.clone());
                    match JWTClaims::decode_token(&jwt_token) {
                        Ok(_) => {
                            // Token is valid; proceed with the original service call
                            let response = service_call.await?;
                            Ok(response)
                        }
                        Err(_) => {
                            // Token is invalid; return a 401 Unauthorized response
                            Err(actix_web::error::ErrorUnauthorized("Invalid token"))
                        }
                    }
                } else {
                    // JWT token is missing; return a 401 Unauthorized response
                    Err(actix_web::error::ErrorUnauthorized(
                        "Authorization header missing or invalid",
                    ))
                }
            })
        }
    }
}
