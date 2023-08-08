macro_rules! create_pokedex {
    ( $default:ident = $dvalue:expr, $($name:ident = $value:expr,) *) => {
        #[derive(Debug)]
        pub enum PokemonSpecies {
            $default,
            $($name),*
        }

        impl PokemonSpecies {
            pub fn id(&self) -> u8 {
                match self {
                    $(PokemonSpecies::$name => $value), *,
                    PokemonSpecies::$default => $dvalue,
                }
            }

            pub fn from_id(id: u8) -> PokemonSpecies {
                match id {
                    $($value => PokemonSpecies::$name), *,
                    _ => PokemonSpecies::$default,
                }
            }
        }
    }
}

create_pokedex! {
    FiveQuestionMarks = 0,
    Bulbasaur = 1,
    Sandshrew = 27,
    Poliwag = 60,
    Gastly = 92,
    Eevee = 133,
    Chikorita = 152,
    Bayleef = 153,
    Spinarak = 167,
    Togepi = 175,
    Snubbull = 209,
}
