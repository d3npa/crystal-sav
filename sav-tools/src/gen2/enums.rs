#[derive(Debug)]
pub enum PokemonSpecies {
    FiveQuestionMarks,
    Bulbasaur,
    Sandshrew,
    Poliwag,
    Gastly,
    Chikorita,
    Bayleef,
    Spinarak,
    Togepi,
}

impl PokemonSpecies {
    pub fn by_id(id: u8) -> Self {
        use PokemonSpecies::*;
        match id {
            1 => Bulbasaur,
            27 => Sandshrew,
            60 => Poliwag,
            92 => Gastly,
            152 => Chikorita,
            153 => Bayleef,
            167 => Spinarak,
            175 => Togepi,
            _ => FiveQuestionMarks,
        }
    }
}

