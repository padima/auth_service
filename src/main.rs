use axum::{ Router, routing::{ get, post } };

pub mod constants;
pub mod endpoint;
pub mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let host = dotenv::var("SERVER").unwrap_or_else(|_| "127.0.0.1:5080".to_string());
    let key = dotenv
        ::var("KEY")
        .map(|value| value.to_string())
        .unwrap_or_else(|_| "my_secret_key".to_string());

    let app_state = model::AppState { key: key };

    let app: Router = Router::new()
        .route("/generate", post(endpoint::generate_post))
        .route(
            "/health",
            get(|| async { "OK" })
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(host).await.expect("bind failed");

    axum::serve(listener, app).await.expect("server failed");

    Ok(())
}
