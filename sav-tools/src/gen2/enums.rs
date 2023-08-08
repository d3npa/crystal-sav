#[macro_export]
macro_rules! create_id_list {
    ( $enum_name:ident, $default:ident = $dvalue:expr, $($name:ident = $value:expr,) *) => {
        #[derive(Debug)]
        pub enum $enum_name {
            $default,
            $($name),*
        }

        impl $enum_name {
            pub fn id(&self) -> u8 {
                match self {
                    $($enum_name::$name => $value), *,
                    $enum_name::$default => $dvalue,
                }
            }

            pub fn from_id(id: u8) -> $enum_name {
                match id {
                    $($value => $enum_name::$name), *,
                    _ => $enum_name::$default,
                }
            }
        }
    }
}

mod species;
pub use species::PokemonSpecies;

mod items;
pub use items::Items;

mod moves;
pub use moves::Moves;
