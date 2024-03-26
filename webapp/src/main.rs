use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
