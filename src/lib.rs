use std::env;
use bytemuck::{Pod, Zeroable};

pub mod errors;

use errors::*;

pub struct Arguments {
    pub sav_file: String
}

impl Arguments {
    pub fn parse() -> Result<Arguments, ParserError> {
        let mut args: Vec<String> = env::args().collect();

        if args.len() != 2 {
            return Err(ParserError::InvalidArguments);
        }

        let sav_file = args.remove(1);
        Ok(Arguments { sav_file })
    }
}

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

/// find all offsets in `data` that match `target`
/// indexes in `target` can be skipped to account for gaps
/// note: index 0 should not be skipped! it will result in skiping.. actually skiping like this always happens - fix the code so when it stops a match chain it rechecks for a new chain with the first byte before continuing
pub fn search_bytes(data: &[u8], target: &[u8], skip: &[usize]) -> Vec<usize> {
    let mut matched_index = 0;
    let mut matched_offsets = vec![];

    for offset in 0..data.len() {        
        if matched_index == target.len() {
            matched_offsets.push(offset - target.len());
            matched_index = 0;
        }

        if skip.contains(&matched_index) {
            matched_index += 1;
        } else if data[offset] == target[matched_index] {
            matched_index += 1;
        } else if data[offset] == target[0] { 
            // restart search at this offset before continuing to next offset
            matched_index = 1;
        } else {
            matched_index = 0;
        }
    }

    matched_offsets
}