// use bytemuck::{bytes_of, from_bytes_mut};
use std::error::Error;
use std::fs;

use std::mem::size_of;

use crystal_patch::Arguments;
use sav_tools::gen2::*;
use sav_tools::search_bytes;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Arguments::parse()?;
    let sav_file = &args.sav_file;

    let mut sav_data = fs::read(sav_file)?;
    println!("Read in {} bytes from '{}'", sav_data.len(), sav_file);

    let player = Player {
        name: [0xcb, 0x3b, 0xb7, 0x50, 0x50, 0xff],
        id: 25916,
    };

    let palette = find_player_color(&mut sav_data, 18207);
    println!("Previous Color: {:?}", palette);
    *palette = PlayerColor::Gray;
    println!("New Color: {:?}", palette);

    /* let team = */
    let team: &mut PartyPokemonList = find_team_data(&mut sav_data, &player);
    for idx in 0..team.pokemon.len() {
        let pokemon = &mut team.pokemon[idx];
        println!("{}", pokemon);
        if pokemon.species == PokemonSpecies::Eevee as u8 {
            println!("Current Friendship: {}", pokemon.friendship);
            pokemon.friendship = 255;
            println!("New Friendship: {}", pokemon.friendship);
        }
    }

    // 備忘録のために残し
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

    fs::write(sav_file, sav_data)?;

    Ok(())
}

#[allow(unused)]
struct Player {
    name: [u8; 6],
    id: u16,
}

/// https://bulbapedia.bulbagarden.net/wiki/Save_data_structure_(Generation_II)#Player_palette
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum PlayerColor {
    Red = 0,
    Blue = 1,
    Green = 2,
    Brown = 3,
    Orange = 4,
    Gray = 5,
    DarkGreen = 6,
    DarkRed = 7,
}

/// note: money is a 3-byte unsigned value (u24) thus must be under 1<<24
fn find_player_color<'a>(sav_data: &'a mut [u8], money: u32) -> &'a mut PlayerColor {
    assert!(money < 1 << 24); // money is a 3-byte value
    let money = money.to_be_bytes()[1..].to_owned();
    let money_offset = {
        search_bytes(&sav_data, &money, &[])
            .pop()
            .expect("money not found")
    };
    // https://archives.glitchcity.info/forums/board-76/thread-1342/page-0.html
    let money_to_palette_rel_offset = 0xd84e - 0xd4dc;
    let palette = &mut sav_data[money_offset - money_to_palette_rel_offset];
    unsafe { &mut *(palette as *mut u8 as *mut PlayerColor) }
}

fn find_team_data<'a>(sav_data: &'a mut [u8], player: &Player) -> &'a mut PartyPokemonList {
    /*
        https://bulbapedia.bulbagarden.net/wiki/Save_data_structure_(Generation_II)#Pok.C3.A9mon_lists
    let count = 6;
    let capacity = 6;
    let entry_size = 48;
    */
    // const SIZE: usize = 6 * (48 + 13) + 2;
    const SIZE: usize = size_of::<PartyPokemonList>();

    // make a Team struct, default it, then config the values u can, and ignore the ranges udk
    let mut team = PartyPokemonList::default();
    team.count = 6;
    team.species[6] = 0xff;
    // team.ot_names = [player.name; 6];
    let mut sample_pokemon = PartyPokemonData::default();
    sample_pokemon.trainer_id = [
        u16::to_le_bytes(player.id)[1],
        u16::to_le_bytes(player.id)[0],
    ];
    team.pokemon = [sample_pokemon; 6];
    // println!("{:?}", team);

    let team_bytes: [u8; SIZE] = unsafe { std::mem::transmute(team) };
    let mut matches = sav_tools::search_bytes(&sav_data, &team_bytes, &{
        let mut ignore = Vec::new();
        for i in 0..team_bytes.len() {
            if team_bytes[i] == 0 {
                ignore.push(i);
            }
        }
        ignore
    });

    // println!("{:#x?}", matches);

    let team_offset = matches.pop().expect("team data not found");
    let team = &mut sav_data[team_offset..team_offset + SIZE];
    unsafe { &mut *(team.as_mut_ptr() as *mut PartyPokemonList) }
}

// /// Registers a new pokemon to the party
// fn add_pokemon_to_party() {
//     /*
//         needs to patch (at least) the trainer data, party data, and pokedex completion data
//     */
// }
