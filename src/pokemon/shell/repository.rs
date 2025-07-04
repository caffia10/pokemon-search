use crate::pokemon::core::types::PokemonDbDto;
use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use scylla::errors::ExecutionError;
use scylla::errors::FirstRowError;
use scylla::errors::IntoRowsResultError;
use scylla::errors::NewSessionError;
use scylla::errors::SingleRowError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenScyllaConnectionError {
    #[error("Session error: {0}")]
    NewSession(#[from] NewSessionError),

    #[error("Execution error: {0}")]
    Execution(#[from] ExecutionError),
}

#[derive(Error, Debug)]
pub enum ScyllaExecutionError {
    #[error("Execution error: {0}")]
    Execution(#[from] ExecutionError),

    #[error("IntoRowsResult error: {0}")]
    IntoRowsResult(#[from] IntoRowsResultError),

    #[error("FirstRowError error: {0}")]
    FirstRow(#[from] FirstRowError),

    #[error("SingleRowError error: {0}")]
    SingleRow(#[from] SingleRowError),
}

pub async fn create_scylla_session() -> Result<Session, OpenScyllaConnectionError> {
    let uri = std::env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0:9042".to_string());

    let session = SessionBuilder::new().known_node(uri).build().await?;

    session.query_unpaged("CREATE KEYSPACE IF NOT EXISTS examples_ks WITH REPLICATION = {'class' : 'NetworkTopologyStrategy', 'replication_factor' : 1}", &[]).await?;

    Ok(session)
}

pub async fn get_pokemon_by_id(
    session: &Session,
    id: String,
) -> Result<PokemonDbDto, ScyllaExecutionError> {
    const QUERY: &str = "SELECT * FROM pokemon WHERE id = ?";
    session
        .query_unpaged(QUERY, (id,))
        .await?
        .into_rows_result()?
        .single_row::<PokemonDbDto>()
        .map_err(Into::into)
}
