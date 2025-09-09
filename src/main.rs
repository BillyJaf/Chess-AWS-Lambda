mod routes;
mod handlers;
mod error;
mod bot;

#[tokio::main]
async fn main() {
    let app = routes::app_routes();

    let address = "0.0.0.0:3600";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}