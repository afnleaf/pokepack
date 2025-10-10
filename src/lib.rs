/*
* lib.rs
*/

pub mod dex;
pub mod parser;
pub mod binary;
pub mod codec;
pub mod error;

use crate::dex::Dex;
use crate::parser::Pokemon;
use crate::binary::PokemonBin;
use crate::error::ParseError;

use std::fmt::Write;
use std::sync::OnceLock;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use base64::prelude::*;
use hex;


// error bridges
impl From<ParseError> for JsValue {
    fn from(error: ParseError) -> Self {
        JsValue::from_str(&error.to_string())
    }
}

// we only need one instance of the Dex 
static POKEDEX: OnceLock<Dex> = OnceLock::new();

pub fn get_dex() -> &'static Dex {
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
pub fn pokepaste_to_pokepack(
    pokepaste: String
) -> Result<Vec<[u8; 21]>, ParseError> {
    let dex = get_dex();
    // parse pokepaste into pokemon string struct
    let pokemon_strings: Vec<Pokemon> = parser::parse_pokepaste(pokepaste)?;
    // convert string to unpacked binary struct
    let pokemon_bin: Vec<PokemonBin> = 
        codec::encode_all_pokemon(&dex.maps, pokemon_strings);
    
    let packed_bytes = pokemon_bin
        .iter()
        .map(|p| p.pack_to_bytes())
        .collect();

    Ok(packed_bytes)
}

// flat byte array
#[wasm_bindgen]
pub fn pokepaste_to_bytes(pokepaste: String) -> Result<Vec<u8>, JsValue> {
    let packed_pokemon: Vec<[u8; 21]> = pokepaste_to_pokepack(pokepaste)?;
    Ok(packed_pokemon.into_iter().flatten().collect())
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
        let arr: [u8; 21] = chunk
            .try_into()
            .map_err(|e| {
                JsValue::from_str(
                    &format!("Internal error: failed to convert slice: {}", e))
            })?;
        let pbin = binary::unpack_from_bytes(&arr);
        let s = codec::pokebin_to_string(&dex.tables, &pbin);
        writeln!(&mut text, "{}", s)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
    }

    Ok(text.trim().to_string())
}


// base64
#[wasm_bindgen]
pub fn pokepaste_to_base64(pokepaste: String) -> Result<String, JsValue> {
    let packed_pokemon = pokepaste_to_pokepack(pokepaste)?;
    /*
    let mut text = String::new();
    for p in packed_pokemon {
        let b64 = BASE64_STANDARD.encode(p);
        writeln!(&mut text, "{}", b64)?;
    }
    Ok(text)
    */
    let lines: Vec<String> = packed_pokemon
        .iter()
        .map(|p| BASE64_STANDARD.encode(p))
        .collect();

    Ok(lines.join("\n"))
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
pub fn pokepaste_to_hex(pokepaste: String) -> Result<String, JsValue> {
    let packed_pokemon = pokepaste_to_pokepack(pokepaste)?;
    /*
    let mut text = String::new();
    for p in packed_pokemon {
        for b in p {
            write!(&mut text, "{:02X?}", b).unwrap();
        }
        write!(&mut text, "\n").unwrap();
    }
    Ok(text)
    */
    let lines: Vec<String> = packed_pokemon
        .iter()
        .map(|p| hex::encode(p))
        .collect();

    Ok(lines.join("\n"))
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



// the tests here are fairly simple
// the input == output
// assert: check for semantic equality, not just string equality.
// The output format might have minor whitespace differences, but the
// parsed data structures should be identical.
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_PASTE: &str = r#"
Miraidon @ Life Orb  
Ability: Hadron Engine  
Level: 50  
Tera Type: Electric  
EVs: 28 HP / 28 Def / 196 SpA / 4 SpD / 252 Spe  
Modest Nature  
- Electro Drift  
- Volt Switch  
- Draco Meteor  
- Protect  

Lunala @ Electric Seed  
Ability: Shadow Shield  
Level: 50  
Tera Type: Fairy  
EVs: 220 HP / 36 Def / 236 SpA / 12 SpD  
Modest Nature  
IVs: 0 Atk / 29 Spe  
- Moongeist Beam  
- Moonblast  
- Wide Guard  
- Trick Room  

Iron Hands @ Assault Vest  
Ability: Quark Drive  
Level: 50  
Shiny: Yes  
Tera Type: Bug  
EVs: 212 HP / 156 Atk / 4 Def / 124 SpD / 12 Spe  
Adamant Nature  
- Low Kick  
- Wild Charge  
- Heavy Slam  
- Fake Out  

Volcarona (M) @ Leftovers  
Ability: Flame Body  
Level: 50  
Shiny: Yes  
Tera Type: Water  
EVs: 252 HP / 252 Def / 4 SpA  
Bold Nature  
IVs: 0 Atk  
- Fiery Dance  
- Rage Powder  
- Struggle Bug  
- Tailwind  

Urshifu-Rapid-Strike @ Focus Sash  
Ability: Unseen Fist  
Level: 50  
Tera Type: Ghost  
EVs: 4 HP / 252 Atk / 252 Spe  
Adamant Nature  
- Surging Strikes  
- Close Combat  
- Taunt  
- Protect  

Ogerpon-Cornerstone (F) @ Cornerstone Mask  
Ability: Sturdy  
Level: 50  
Tera Type: Rock  
EVs: 4 HP / 252 Atk / 252 Spe  
Jolly Nature  
- Ivy Cudgel  
- Stomping Tantrum  
- Follow Me  
- Spiky Shield  
"#;
    #[test]
    fn test_bytes_conversion_roundtrip() {
        let paste = SAMPLE_PASTE.trim().to_string();
        
        let bytes_encoded = pokepaste_to_bytes(paste.clone()).unwrap();
        let decoded_paste = bytes_to_pokepaste(bytes_encoded).unwrap();

        let original_structs = parser::parse_pokepaste(paste).unwrap();
        let decoded_structs = parser::parse_pokepaste(decoded_paste).unwrap();
        assert_eq!(original_structs, decoded_structs);
    }

    #[test]
    fn test_base64_conversion_roundtrip() {
        let paste = SAMPLE_PASTE.trim().to_string();

        let base64_encoded = pokepaste_to_base64(paste.clone()).unwrap();
        let decoded_paste = base64_to_pokepaste(base64_encoded).unwrap();
        
        let original_structs = parser::parse_pokepaste(paste).unwrap();
        let decoded_structs = parser::parse_pokepaste(decoded_paste).unwrap();
        assert_eq!(original_structs, decoded_structs);
    }

    #[test]
    fn test_hex_conversion_roundtrip() {
        let paste = SAMPLE_PASTE.trim().to_string();

        let hex_encoded = pokepaste_to_hex(paste.clone()).unwrap();
        let decoded_paste = hex_to_pokepaste(hex_encoded).unwrap();

        let original_structs = parser::parse_pokepaste(paste).unwrap();
        let decoded_structs = parser::parse_pokepaste(decoded_paste).unwrap();
        assert_eq!(original_structs, decoded_structs);
    }
}

