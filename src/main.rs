use std::fs;
use std::error::Error;
use bytemuck::{bytes_of, from_bytes_mut};

use crystal_sav::*;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Arguments::parse()?;
    let sav_file = &args.sav_file;

    let mut sav_data = fs::read(sav_file)?;
    println!("Read in {} bytes from '{}'", sav_data.len(), sav_file);

    // hacky - find chikorita
    let trainer_id: u16 = 25916;
    let experience: u32 = 2408;
    let my_chikorita: Vec<u8> = vec![
        0x98, // species index number
        00, // held item index number - skip
        00, 00, 00, 00, // move ids - skip
        u16::to_le_bytes(trainer_id)[1],
        u16::to_le_bytes(trainer_id)[0],
        u32::to_le_bytes(experience)[2],
        u32::to_le_bytes(experience)[1],
        u32::to_le_bytes(experience)[0],
    ];

    println!("my_chikorita: {:x?}", my_chikorita);
    let offsets = search_bytes(&sav_data, &my_chikorita, &vec![1, 2, 3, 4, 5]);

    println!("Found {} matches at offsets {:x?}", offsets.len(), offsets);

    for offset in offsets {
        let mut bytes = [0; 48];
        for i in 0..48 {
            bytes[i] = sav_data[offset+i];
        }

        let mut chikorita = from_bytes_mut::<PartyPokemonData>(&mut bytes);
        chikorita.held_item = 6;
        chikorita.iv_data[0] &= 31; // & 0b00011111 i.e. set attack IV to 1 
        println!("[{:x}] {:x?}", offset, chikorita);

        let bytes = bytes_of::<PartyPokemonData>(&chikorita);
        for i in 0..48 {
            sav_data[offset+i] = bytes[i];
        }
    }

    fs::write(sav_file, sav_data)?;

    Ok(())
}
