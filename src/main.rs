use crate::pokemon::shell::handlers::AppState;
use crate::pokemon::shell::handlers::get_pokemon_handler;
use crate::pokemon::shell::repository::OpenScyllaConnectionError;
use crate::pokemon::shell::repository::create_scylla_session;
use axum::{Router, routing::get};
use std::net::SocketAddr;
use std::sync::Arc;
mod pokemon;

async fn ping() -> &'static str {
    "Pong"
}

#[tokio::main]
async fn main() -> Result<(), OpenScyllaConnectionError> {
    tracing_subscriber::fmt::init();

    let app_state = AppState {
        session: Arc::new(create_scylla_session().await?),
    };

    let app = Router::new()
        .route("/", get(ping))
        .route("/pokemon/:id", get(get_pokemon_handler))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Servidor escuchando en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
