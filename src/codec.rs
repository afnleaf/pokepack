/*
* codec.rs
*
* logic for converting between paste and binary structs
* see binary.rs and parser.rs for the struct definitions
*/

use std::collections::HashMap;

use crate::{
    dex::{Tables, Maps},
    parser::{Pokemon, Tv},
    binary::{PokemonBin, TvBin},
};

// standard function, returns usize
// cast to the correct u-int size in PokemonBin
fn element_to_binary(map: &HashMap<String, usize>, element: &str) -> usize {
    // we convert to lowercase because that is how we built our hashmap
    match map.get(&element.to_lowercase()) {
        Some(i) => *i,
        None    => 0,
    }
}

// our chill o(1) lookup?
// clone is fine
fn binary_to_element(table: &Vec<String>, index: usize) -> String {
    table[index].clone()
}

fn gender_to_binary(gender: &str) -> u8 {
    // make sure it is lowercase for comparison
    // male, female or genderless
    match gender.to_lowercase().as_str() {
        "m" => 0,
        "f" => 1,
        _   => 2,
    }
}

fn binary_to_gender(gender: u8) -> String {
    match gender {
        0 => "m".to_string(),
        1 => "f".to_string(),
        _ => "".to_string(),
    }
}

// need to check if IV so that it defaults to 31 "perfect"
// which is the intended behavior
// need to add conditions for when number is greater than 255
fn small_to_u8(s: &str, ifiv: bool) -> u8 {
    //println!("test level -{}-", level);
    if s == "" {
        if ifiv {
            31 
        } else {
            0
        }
    } else {
        // not sure if this is the error behavior we want
        s.trim().parse::<u8>().unwrap_or(0)  
    }
}

// we use the .into() to convert to usize
pub fn pokebin_to_string(tables: &Tables, pbin: &PokemonBin) -> Pokemon {
    Pokemon {
        name:       binary_to_element(&tables.names, pbin.name.into()),
        gender:     binary_to_gender(pbin.gender.into()),
        item:       binary_to_element(&tables.items, pbin.item.into()),
        ability:    binary_to_element(&tables.abilities, pbin.ability.into()),
        level:      if pbin.level == 0 {"".into()} else {pbin.level.to_string()},
        shiny:      if pbin.shiny { "Yes".to_string() } else { "".to_string() },
        tera:       binary_to_element(&tables.teras, pbin.tera.into()),
        evs:        decode_tvs(&pbin.evs, false),
        nature:     binary_to_element(&tables.natures, pbin.nature.into()),
        ivs:        decode_tvs(&pbin.ivs, true),
        moves:      decode_moves(&tables.moves, &pbin.moves),
    }
}

fn decode_moves(table: &Vec<String>, moves_bin: &Vec<u16>) -> Vec<String> {
    let mut moves: Vec<String> = Vec::new();
    for m in moves_bin {
        moves.push(binary_to_element(table, (*m).into()));
    }
    moves
}

fn decode_tvs(tvs: &TvBin, ifiv: bool) -> Tv {
    Tv {
        ifiv,
        hp:     tvs.hp.to_string(),
        atk:    tvs.atk.to_string(),
        def:    tvs.def.to_string(),
        spa:    tvs.spa.to_string(),
        spd:    tvs.spd.to_string(),
        spe:    tvs.spe.to_string(),
    }
} 

fn encode_tvs(tvs: &Tv, ifiv: bool) -> TvBin {
    TvBin {
        hp:     small_to_u8(&tvs.hp, ifiv),
        atk:    small_to_u8(&tvs.atk, ifiv),
        def:    small_to_u8(&tvs.def, ifiv),
        spa:    small_to_u8(&tvs.spa, ifiv),
        spd:    small_to_u8(&tvs.spd, ifiv),
        spe:    small_to_u8(&tvs.spe, ifiv),
    }
}

fn encode_moves(
    moves_map: &HashMap<String, usize>, 
    moves: &Vec<String>
) -> Vec<u16> {
    moves
        .iter()
        .map(|m| element_to_binary(moves_map, m) as u16)
        .collect()
}

pub fn encoded_pokemon(maps: &Maps, pokemon: &Pokemon) -> PokemonBin {
    PokemonBin {
        name:       element_to_binary(&maps.names, &pokemon.name) as u16,
        gender:     gender_to_binary(&pokemon.gender) as u8,
        item:       element_to_binary(&maps.items, &pokemon.item) as u16,
        ability:    element_to_binary(&maps.abilities, &pokemon.ability) as u16,
        level:      small_to_u8(&pokemon.level, false) as u8,
        shiny:      pokemon.shiny.to_lowercase() == "yes",
        tera:       element_to_binary(&maps.teras, &pokemon.tera) as u8,
        evs:        encode_tvs(&pokemon.evs, false),
        nature:     element_to_binary(&maps.natures, &pokemon.nature) as u8,
        ivs:        encode_tvs(&pokemon.ivs, true),
        moves:      encode_moves(&maps.moves, &pokemon.moves),
    }
}

pub fn encode_all_pokemon(
    maps: &Maps, 
    pokemons: Vec<Pokemon>
) -> Vec<PokemonBin> {
        pokemons
            .iter()
            .map(|p| encoded_pokemon(&maps, p))
            .collect()
}


// gonna treat these kind of like unit tests
// maybe I should combine each pair
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_to_binary() {
        let dex = crate::get_dex();
        assert_eq!(element_to_binary(&dex.maps.names, "bulbasaur"), 0);
    }

    #[test]
    fn test_binary_to_element() {
        let dex = crate::get_dex();
        assert_eq!(binary_to_element(&dex.tables.names, 0), "Bulbasaur");
    }

    #[test]
    fn test_gender_to_binary() {
        assert_eq!(gender_to_binary("m"), 0);
        assert_eq!(gender_to_binary("f"), 1);
        assert_eq!(gender_to_binary("a"), 2);
    }

    #[test]
    fn test_binary_to_gender() {
        assert_eq!(binary_to_gender(0), "m");
        assert_eq!(binary_to_gender(1), "f");
        assert_eq!(binary_to_gender(2), "");
    }
    
    #[test]
    fn test_small_to_u8() {
        assert_eq!(small_to_u8("252", false), 252);
        assert_eq!(small_to_u8("", false), 0);
        assert_eq!(small_to_u8("", true), 31);
    }

/*
    pub fn pokebin_to_string(tables: &Tables, pbin: &PokemonBin) -> Pokemon {
    fn decode_moves(table: &Vec<String>, moves_bin: &Vec<u16>) -> Vec<String> {
    fn decode_tvs(tvs: &TvBin, ifiv: bool) -> Tv {
    fn encode_tvs(tvs: &Tv, ifiv: bool) -> TvBin {
    fn encode_moves(
        moves_map: &HashMap<String, usize>, 
        moves: &Vec<String>
    ) -> Vec<u16> {
    pub fn encoded_pokemon(maps: &Maps, pokemon: &Pokemon) -> PokemonBin {
    pub fn encode_all_pokemon(
        maps: &Maps, 
        pokemons: Vec<Pokemon>
    ) -> Vec<PokemonBin> {
*/
}
