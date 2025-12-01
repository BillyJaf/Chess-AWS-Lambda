mod routes;
mod handlers;
mod error;
mod bot;

#[tokio::main]
async fn main() {
    let app = routes::app_routes();

    let port = std::env::var("PORT").unwrap_or(String::from("8080"));
    let address = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}