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

use std::fmt::Write;
use std::sync::OnceLock;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use base64::prelude::*;
use hex;

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

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"Loading dex into memory".into());
    get_dex();
}

// have to figure out best way for the library to be used
// what is the most desired output?

// helper
pub fn pokepaste_to_pokepack(pokepaste: String) -> Vec<[u8; 21]> {
    let dex = get_dex();
    // parse pokepaste into pokemon string struct
    let pokemon_strings: Vec<Pokemon> = parser::parse_pokepaste(pokepaste);
    // convert string to unpacked binary struct
    let pokemon_bin: Vec<PokemonBin> = 
        codec::encode_all_pokemon(&dex.maps, pokemon_strings);
    pokemon_bin
        .iter()
        .map(|p| p.pack_to_bytes())
        .collect()
}

// flat byte array
#[wasm_bindgen]
pub fn pokepaste_to_bytes(pokepaste: String) -> Vec<u8> {
    let packed_pokemon: Vec<[u8; 21]> = pokepaste_to_pokepack(pokepaste);
    packed_pokemon.into_iter().flatten().collect()
}

#[wasm_bindgen]
pub fn bytes_to_pokepaste(flat_byte_arr: Vec<u8>) -> Result<String, JsValue> {
    // must be multiple of 21 
    if flat_byte_arr.len() % 21 != 0 {
        // can expand on error message
        return Err(JsValue::from_str("Invalid input length."));
    }

    let dex = get_dex();
    let mut text = String::new();

    for chunk in flat_byte_arr.chunks_exact(21) {
        let arr: [u8; 21] = chunk.try_into().unwrap();
        let pbin = binary::unpack_from_bytes(&arr);
        let s = codec::pokebin_to_string(&dex.tables, &pbin);
        writeln!(&mut text, "{}\n", s).unwrap();
    }

    Ok(text.trim().into())
}


// base64
#[wasm_bindgen]
pub fn pokepaste_to_base64(pokepaste: String) -> String {
    let packed_pokemon = pokepaste_to_pokepack(pokepaste);
    let mut text = String::new();
    for p in packed_pokemon {
        let b64 = BASE64_STANDARD.encode(p);
        writeln!(&mut text, "{}", b64).unwrap();
    }
    text
}

#[wasm_bindgen]
pub fn base64_to_pokepaste(b64: String) -> Result<String, JsValue> {
    let mut flat_bytes: Vec<u8> = Vec::new();
    for line in b64.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }
        let decoded_chunk = BASE64_STANDARD.decode(trimmed_line)
            .map_err(|e| JsValue::from_str(
                &format!("Base64 decode error: {}", e)))?;
        flat_bytes.extend_from_slice(&decoded_chunk);

    }
    bytes_to_pokepaste(flat_bytes)
}

// hex
#[wasm_bindgen]
pub fn pokepaste_to_hex(pokepaste: String) -> String {
    let packed_pokemon = pokepaste_to_pokepack(pokepaste);
    let mut text = String::new();
    for p in packed_pokemon {
        for b in p {
            write!(&mut text, "{:02X?}", b).unwrap();
        }
        write!(&mut text, "\n").unwrap();
    }
    text
}

#[wasm_bindgen]
pub fn hex_to_pokepaste(hex: String) -> Result<String, JsValue> {
    let mut flat_bytes: Vec<u8> = Vec::new();
    for line in hex.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }
        let decoded_chunk = hex::decode(trimmed_line)
            .map_err(|e| JsValue::from_str(
                &format!("Hex decode error: {}", e)))?;
        flat_bytes.extend_from_slice(&decoded_chunk);
    }
    bytes_to_pokepaste(flat_bytes)
}










/*
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

pub fn hex_to_pokepaste(_hex: String) -> String {
    // so each will be one pokemon on each line
    //let vec_hex = hex.lines().map(String::from()).collect();
    // we go hex -> u8;21 -> string
    todo!(); 
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
*/

