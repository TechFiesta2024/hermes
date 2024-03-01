use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::env;

pub async fn verify_key(req: Request<Body>, next: Next) -> Result<Response, Response> {
    let (parts, body) = req.into_parts();
    let api_key = env::var("API_KEY").unwrap();
    let headers = parts.headers.get("x-api-key");

    match headers {
        Some(key) => {
            if key.to_str().unwrap() != api_key {
                return Ok(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::empty())
                    .unwrap());
            }
        }
        None => {
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::empty())
                .unwrap());
        }
    }

    let new_req = Request::from_parts(parts, body);
    Ok(next.run(new_req).await)
}
