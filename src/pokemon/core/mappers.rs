use super::types::Pokemon;
use super::types::PokemonDbDto;
use super::types::PokemonResponse;

pub fn to_pokemon(dto: PokemonDbDto) -> Pokemon {
    Pokemon {
        id: dto.id,
        nombre: dto.name,
        types: dto.types,
    }
}

pub fn to_response(pkm: Pokemon) -> PokemonResponse {
    PokemonResponse {
        id: pkm.id,
        name: pkm.nombre,
        types: pkm.types,
    }
}
