use super::repository::get_pokemon_by_id;
use crate::pokemon::core::mappers::{to_pokemon, to_response};
use axum::{
    extract::Path, extract::State, http::StatusCode, response::IntoResponse, response::Json,
};
use scylla::client::session::Session;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub session: Arc<Session>,
}

pub async fn get_pokemon_handler(
    Path(pokemon_id): Path<String>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    get_pokemon_by_id(&app_state.session, pokemon_id)
        .await
        .map(to_pokemon)
        .map(to_response)
        .map(Json)
        .map(IntoResponse::into_response)
        .unwrap_or_else(|e| {
            eprintln!("DB error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error").into_response()
        })
}
