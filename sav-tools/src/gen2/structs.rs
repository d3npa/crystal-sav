use bytemuck::{Pod, Zeroable};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct PartyPokemonData {
    pub species: u8,
    pub held_item: u8,
    pub moves: [u8; 4],
    pub trainer_id: [u8; 2],
    pub experience: [u8; 3],
    pub hp_ev: [u8; 2],
    pub attack_ev: [u8; 2],
    pub defense_ev: [u8; 2],
    pub speed_ev: [u8; 2],
    pub special_ev: [u8; 2],
    pub iv_data: [u8; 2],
    pub move_pps: [u8; 4],
    pub friendship: u8,
    pub pokerus: u8,
    pub caught_data: [u8; 2],
    pub level: u8,
    pub status_condition: u8,
    pub unused: u8,
    pub current_hp: [u8; 2],
    pub max_hp: [u8; 2],
    pub attack: [u8; 2],
    pub defense: [u8; 2],
    pub speed: [u8; 2],
    pub special_attack: [u8; 2],
    pub special_defense: [u8; 2],
}

unsafe impl Zeroable for PartyPokemonData {}
unsafe impl Pod for PartyPokemonData {}
