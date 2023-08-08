use bytemuck::{bytes_of, from_bytes_mut};
use std::error::Error;
use std::fs;

use crystal_patch::Arguments;
use sav_tools::gen2::*;
use sav_tools::search_bytes;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Arguments::parse()?;
    let sav_file = &args.sav_file;

    let mut sav_data = fs::read(sav_file)?;
    println!("Read in {} bytes from '{}'", sav_data.len(), sav_file);

    let player = Player {
        name: [0xcb, 0x3b, 0xb7, 0x50, 0x00, 0x00],
        id: 25916,
    };

    let team = find_team_data(&sav_data, &player);

    // println!("Searching for first party pokemon data");
    // let offsets = search_bytes(
    //     &sav_data,
    //     &pokemon_header(),
    //     &vec![1, 2, 3, 4, 5],
    // );
    // println!("Found {} matches at offsets {:x?}", offsets.len(), offsets);

    // // for offset in offsets {
    //     let mut bytes = [0; 48];
    //     for i in 0..48 {
    //         bytes[i] = sav_data[offset+j*48+i];
    //     }

    //     let mut chikorita = from_bytes_mut::<PartyPokemonData>(&mut bytes);
    //     // chikorita.held_item = 6;
    //     // chikorita.iv_data[0] &= 31; // & 0b00011111 i.e. set attack IV to 1
    //     println!("[{:x}] {} {:?}\n", offset+j*48, chikorita, chikorita);

    //     // let bytes = bytes_of::<PartyPokemonData>(&chikorita);
    //     // for i in 0..48 {
    //     //     sav_data[offset+i] = bytes[i];
    //     // }
    // }

    // fs::write(sav_file, sav_data)?;

    Ok(())
}

struct Player {
    name: [u8; 6],
    id: u16,
}

fn pokemon_header() -> Vec<u8> {
    // hacky - find my pokemon
    let trainer_id: u16 = 25916;
    let my_pokemon: Vec<u8> = vec![
        PokemonSpecies::Eevee.id(), // species index number
        00,                         // held item index number - skip
        00,                         // move ids - skip
        00,
        00,
        00,
        u16::to_le_bytes(trainer_id)[1],
        u16::to_le_bytes(trainer_id)[0],
    ];

    my_pokemon
}

fn find_team_data(sav_data: &[u8], player: &Player) {
    /*
    there may be many results if just searching for player's name but it may
    be possible to do an incremental search, peeking at the following bytes
    to filter.

    for example, team data has the player name followed by a terminator, then
    a 1 or 0 for custom moves, then depending on that byte, a number of
    pokemon structures followed by a terminator 0xFF.

    this should be possible to 絞り込む with a more advanced search function!

    https://bulbapedia.bulbagarden.net/wiki/Save_data_structure_(Generation_II)#Pok.C3.A9mon_lists

    */
    let count = 6;
    let capacity = 6;
    let entry_size = 48;
    const total_size: usize = 6 * (48 + 13) + 2;
    let ot_name = [0xcb, 0x3b, 0xb7, 0x50, 0x50, 0xff]; // all hibiki's pokemon

    // make a Team struct, default it, then config the values u can, and ignore the ranges udk
    let mut team = PartyPokemonList::default();
    team.count = 6;
    // team.ot_names = [ot_name; 6];
    team.species[6] = 0xff;
    let mut sample_pokemon = PartyPokemonData::default();
    sample_pokemon.trainer_id = [
        u16::to_le_bytes(player.id)[1],
        u16::to_le_bytes(player.id)[0],
    ];
    team.pokemon = [sample_pokemon; 6];
    // println!("{:?}", team);
    let team_bytes: [u8; total_size] = unsafe { std::mem::transmute(team) };
    let matches = sav_tools::search_bytes(&sav_data, &team_bytes, &{
        let mut ignore = Vec::new();
        for i in 0..team_bytes.len() {
            if team_bytes[i] == 0 {
                ignore.push(i);
            }
        }
        ignore
    });

    println!("{:#x?}", matches);

    for offset in matches {
        let mut team_bytes = [0u8; total_size];
        for i in 0..total_size {
            team_bytes[i] = sav_data[offset + i];
        }
        let team: PartyPokemonList = unsafe { std::mem::transmute(team_bytes) };
        // println!("{:?}", team);

        for p in team.pokemon {
            println!("{}", p);
        }
    }
}

/// Registers a new pokemon to the party
fn add_pokemon_to_party() {
    /*
        needs to patch (at least) the trainer data, party data, and pokedex completion data
    */
}
