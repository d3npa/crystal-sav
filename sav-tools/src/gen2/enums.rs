#[derive(Debug)]
pub enum PokemonSpecies {
    FiveQuestionMarks,
    Bulbasaur,
    Chikorita,
}

impl PokemonSpecies {
    pub fn by_id(id: u8) -> Self {
        use PokemonSpecies::*;
        match id {
            1 => Bulbasaur,
            152 => Chikorita,
            _ => FiveQuestionMarks,
        }
    }
}

