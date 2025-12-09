use lambda_http::{run, service_fn, Body, Error, Request, Response};

mod handlers;
mod error;
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
            let body = req.body();
            let result = handlers::best_move::best_move(body.clone()).await;
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
                        .body(serde_json::to_string(&response_error.error)?.into())
                        .unwrap())
                }
            }
        }

        ("POST", "/legal_moves") => {
            let body = req.body();
            let result = handlers::legal_moves::legal_moves(body.clone()).await;
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
                        .body(serde_json::to_string(&response_error.error)?.into())
                        .unwrap())
                }
            }
        }

        ("POST", "/validate_fen") => {
            let body = req.body();
            let result = handlers::validate_fen::validate_fen(body.clone()).await;
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
                        .body(serde_json::to_string(&response_error.error)?.into())
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