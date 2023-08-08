use std::fs;
use std::error::Error;
use bytemuck::{bytes_of, from_bytes_mut};

use sav_tools::gen2::*;
use sav_tools::search_bytes;
use crystal_patch::Arguments;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Arguments::parse()?;
    let sav_file = &args.sav_file;

    let mut sav_data = fs::read(sav_file)?;
    println!("Read in {} bytes from '{}'", sav_data.len(), sav_file);

    let player = Player {
        name: [0xcb, 0x3b, 0xb7, 0x50, 0x00, 0x00],
        id: 25916,
    };

//    let team = find_team_data(&bytes, &player);
    

    println!("Searching for first party pokemon data");
    let offsets = search_bytes(
        &sav_data, 
        &pokemon_header(), 
        &vec![1, 2, 3, 4, 5],
    );
    println!("Found {} matches at offsets {:x?}", offsets.len(), offsets);

    let offset = &offsets[1];

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
        92, // species index number
        00, // held item index number - skip
        00, 00, 00, 00, // move ids - skip
        u16::to_le_bytes(trainer_id)[1],
        u16::to_le_bytes(trainer_id)[0],
    ];

    my_pokemon
}

fn find_team_data() {
    /*
    
    there may be many results if just searching for player's name
    but it may be possible to do an incremental search, peeking at the following bytes to filter. for example, team data has the player name followed by a terminator, then a 1 or 0 for custom moves, then depending on that byte, a number of pokemon structures followed by a terminator 0xFF. 

    this should be possible to 絞り込む with a more advanced search function!

    https://bulbapedia.bulbagarden.net/wiki/Trainer_data_structure_(Generation_II)

    */

}

/// Registers a new pokemon to the party
fn add_pokemon_to_party() {
    /*
        needs to patch (at least) the trainer data, party data, and pokedex completion data
    */
}
