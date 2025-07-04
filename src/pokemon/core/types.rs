use scylla::DeserializeRow;
use serde::Serialize;

#[derive(Serialize)]
pub struct PokemonResponse {
    pub id: String,
    pub name: String,
    pub types: Vec<String>,
}

pub struct Pokemon {
    pub id: String,
    pub nombre: String,
    pub types: Vec<String>,
}

#[derive(DeserializeRow)]
pub struct PokemonDbDto {
    pub id: String,
    pub name: String,
    pub types: Vec<String>,
}
