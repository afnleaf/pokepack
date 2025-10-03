/*
* lib.rs
*/

pub mod dex;
pub mod parser;
pub mod binary;
pub mod codec;

use crate::dex::Dex;
use crate::parser::Pokemon;
use crate::binary::PokemonBin;

use base64::prelude::*;
use std::fmt::Write;
use std::sync::OnceLock;

// we only need one instances of the Dex 
// 
static POKEDEX: OnceLock<Dex> = OnceLock::new();

fn get_dex() -> &'static Dex {
    // return ref or init closure once
    POKEDEX.get_or_init(|| {
        println!("Building Pokédex for the first time...");
        dex::Dex::build()
    })

}

// have to figure out best way for the library to be used
// what is the most desired output?

// pokepaste to output --------------------------------------------------------
pub fn pokepaste_to_byte_array(pokepaste: String) -> Vec<[u8; 21]> {
    let dex = get_dex();
    // parse pokepaste into pokemon string struct
    let pokemon_strings: Vec<Pokemon> = parser::parse_pokepaste(pokepaste);
    // convert string to unpacked binary struct
    let pokemon_bin: Vec<PokemonBin> = 
        codec::encode_all_pokemon(&dex.maps, pokemon_strings);
    let mut r: Vec<[u8; 21]> = Vec::new();
    for v in &pokemon_bin {
        r.push(v.pack_to_bytes());
    }
    r
}

pub fn pokepaste_to_hex(pokepaste: String) -> String {
    let packed_pokemon = pokepaste_to_byte_array(pokepaste);
    let mut text = String::new();
    for p in packed_pokemon {
        for b in p {
            write!(&mut text, "{:02X?}", b).unwrap();
        }
        write!(&mut text, "\n").unwrap();
    }
    text
}

pub fn pokepaste_to_base64(pokepaste: String) -> String {
    let packed_pokemon = pokepaste_to_byte_array(pokepaste);
    let mut text = String::new();
    for p in packed_pokemon {
        let b64 = BASE64_STANDARD.encode(p);
        write!(&mut text, "{}\n", b64).unwrap();
    }
    text
}

// byte formats to pokepaste --------------------------------------------------
pub fn byte_array_to_pokepaste(vec_bytearr: Vec<[u8; 21]>) -> String {
    let dex = get_dex();

    let mut text = String::new();
    for arr in vec_bytearr {
        let pbin = binary::unpack_from_bytes(&arr);
        let s = codec::pokebin_to_string(&dex.tables, &pbin);
        write!(&mut text, "{}\n", s).unwrap();
    }
    text
}

pub fn hex_to_pokepaste(_hex: String) -> String {
    // so each will be one pokemon on each line
    //let vec_hex = hex.lines().map(String::from()).collect();
    // we go hex -> u8;21 -> string
    todo!(); 
}

pub fn base64_to_pokepaste(b64: String) -> String {
    let vec_b64: Vec<String> = b64.lines().map(String::from).collect();
    let mut vec_bytes: Vec<[u8; 21]> = Vec::new();
    for v in vec_b64 {
        let mut pack: [u8; 21] = [0; 21];
        let decoded: Vec<u8> = BASE64_STANDARD.decode(v).unwrap();
        for (i, byte) in decoded.into_iter().enumerate() {
            pack[i] = byte;
        }
        vec_bytes.push(pack);
    }
    byte_array_to_pokepaste(vec_bytes)
}

