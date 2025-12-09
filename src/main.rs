use lambda_http::{run, service_fn, Body, Error, Request, Response};

use crate::types::FenInput;

mod handlers;
mod bot;
mod utils;
mod types;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

async fn handler(req: Request) -> Result<Response<Body>, Error> {
    match (req.method().as_str(), req.uri().path()) {
        ("GET", "/health_check") => {
            let result = handlers::health_check::health_check().await;
            Ok(Response::builder()
                .status(200)
                .body(serde_json::to_string(&result)?.into())
                .unwrap())
        }

        ("POST", "/best_move") => {
            let fen_input: FenInput = match serde_json::from_slice(req.body()) {
                Ok(fi) => fi,
                Err(e) => {
                    let error = format!("Invalid request body: {}", e);
                    return Ok(Response::builder()
                        .status(400)
                        .body(serde_json::to_string(&error)?.into())
                        .unwrap())
                }
            };

            let result = handlers::best_move::best_move(fen_input.fen).await;
            match result {
                Ok(best_move_response) => {
                    Ok(Response::builder()
                        .status(200)
                        .body(serde_json::to_string(&best_move_response)?.into())
                        .unwrap())
                }
                Err(response_error) => {
                    Ok(Response::builder()
                        .status(400)
                        .body(serde_json::to_string(&response_error)?.into())
                        .unwrap())
                }
            }
        }

        ("POST", "/legal_moves") => {
            let fen_input: FenInput = match serde_json::from_slice(req.body()) {
                Ok(fi) => fi,
                Err(e) => {
                    let error = format!("Invalid request body: {}", e);
                    return Ok(Response::builder()
                        .status(400)
                        .body(serde_json::to_string(&error)?.into())
                        .unwrap())
                }
            };

            let result = handlers::legal_moves::legal_moves(fen_input.fen).await;
            match result {
                Ok(legal_moves) => {
                    Ok(Response::builder()
                        .status(200)
                        .body(serde_json::to_string(&legal_moves)?.into())
                        .unwrap())
                }
                Err(response_error) => {
                    Ok(Response::builder()
                        .status(400)
                        .body(serde_json::to_string(&response_error)?.into())
                        .unwrap())
                }
            }
        }

        ("POST", "/validate_fen") => {
            let fen_input: FenInput = match serde_json::from_slice(req.body()) {
                Ok(fi) => fi,
                Err(e) => {
                    let error = format!("Invalid request body: {}", e);
                    return Ok(Response::builder()
                        .status(400)
                        .body(serde_json::to_string(&error)?.into())
                        .unwrap())
                }
            };

            let result = handlers::validate_fen::validate_fen(fen_input.fen).await;
            match result {
                Ok(validate_fen_response) => {
                    Ok(Response::builder()
                        .status(200)
                        .body(serde_json::to_string(&validate_fen_response)?.into())
                        .unwrap())
                    }
                Err(response_error) => {
                    Ok(Response::builder()
                        .status(400)
                        .body(serde_json::to_string(&response_error)?.into())
                        .unwrap())
                    }
            }
        }

        _ => Ok(Response::builder()
                .status(404)
                .body(Body::from("Not Found"))
                .unwrap())
    }
}