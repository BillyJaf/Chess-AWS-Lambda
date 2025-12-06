use lambda_http::{run, service_fn, Body, Error, Request, Response};

mod handlers;
mod error;
mod bot;
mod utils;
mod types;

// #[tokio::main]
// async fn main() {
//     let app = routes::app_routes();

//     let port = std::env::var("PORT").unwrap_or(String::from("8080"));
//     let address = format!("0.0.0.0:{port}");

//     let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

//     axum::serve(listener, app).await.unwrap();
// }

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
            Ok(Response::builder()
                .status(200)
                .body(serde_json::to_string(&result)?.into())
                .unwrap())
        }

        ("POST", "/legal_moves") => {
            let body = req.body();
            let result = handlers::legal_moves::legal_moves(body.clone()).await;
            Ok(Response::builder()
                .status(200)
                .body(serde_json::to_string(&result)?.into())
                .unwrap())
        }

        ("POST", "/validate_fen") => {
            let body = req.body();
            let result = handlers::validate_fen::validate_fen(body.clone()).await;
            Ok(Response::builder()
                .status(200)
                .body(serde_json::to_string(&result)?.into())
                .unwrap())
        }

        _ => Ok(Response::builder()
                .status(404)
                .body(Body::from("Not Found"))
                .unwrap())
    }
}