/*
* codec.rs
*
* logic for converting between paste and binary structs
*/

use std::collections::HashMap;

use crate::{
    dex::{Tables, Maps},
    parser::{Pokemon, Tv},
    binary::{PokemonBin, TvBin},
};

// standard function, returns usize
// cast to the correct size in PokemonBin
fn element_to_binary(map: &HashMap<String, usize>, element: String) -> usize {
    match map.get(&element) {
        Some(i) => *i,
        None => 0,
    }

}

// our chill n(1) lookup?
fn binary_to_element(table: &Vec<String>, index: usize) -> String {
    table[index].clone()
}

fn gender_to_binary(gender: String) -> u8 {
    // make sure it is lowercase for comparison
    // male, female or genderless
    match gender.to_lowercase().as_str() {
        "m" => 0,
        "f" => 1,
        _ => 2,
    }
}

fn binary_to_gender(gender: u8) -> String {
    match gender {
        0 => "m".into(),
        1 => "f".into(),
        _ => "".into(),
    }
}

// need to check if IV so that it defaults to 31 "perfect"
// which is the intended behavior
fn small_to_u8(s: String, ifiv: bool) -> u8 {
    //println!("test level -{}-", level);
    if s == "" {
        if ifiv {
            31 
        } else {
            0
        }
    } else {
        s.trim().parse::<u8>().unwrap()  
    }
}


pub fn pokebin_to_string(tables: &Tables, pbin: &PokemonBin) -> Pokemon {
    Pokemon {
        name:       binary_to_element(&tables.names, pbin.name.clone().into()),
        gender:     binary_to_gender(pbin.gender.clone().into()),
        item:       binary_to_element(&tables.items, pbin.item.clone().into()),
        ability:    binary_to_element(&tables.abilities, pbin.ability.clone().into()),
        level:      pbin.level.clone().to_string(),
        shiny:      if pbin.shiny { "Yes".into() } else { "".into() }, // bruh
        tera:       binary_to_element(&tables.teras, pbin.tera.clone().into()),
        evs:        decode_tvs(pbin.evs.clone().into()),
        nature:     binary_to_element(&tables.natures, pbin.nature.clone().into()),
        ivs:        decode_tvs(pbin.ivs.clone().into()),
        moves:      decode_moves(&tables.moves, pbin.moves.clone().into()),
    }
}

fn decode_moves(table: &Vec<String>, moves_bin: Vec<u16>) -> Vec<String> {
    let mut moves: Vec<String> = Vec::new();
    for m in moves_bin {
        moves.push(binary_to_element(table, m.into()));
    }
    moves
}

fn decode_tvs(tvs: TvBin) -> Tv {
    Tv {
        hp:     tvs.hp.to_string(),
        atk:    tvs.atk.to_string(),
        def:    tvs.def.to_string(),
        spa:    tvs.spa.to_string(),
        spd:    tvs.spd.to_string(),
        spe:    tvs.spe.to_string(),
    }
}

fn encode_tvs(tvs: Tv, ifiv: bool) -> TvBin {
    TvBin {
        hp:     small_to_u8(tvs.hp, ifiv),
        atk:    small_to_u8(tvs.atk, ifiv),
        def:    small_to_u8(tvs.def, ifiv),
        spa:    small_to_u8(tvs.spa, ifiv),
        spd:    small_to_u8(tvs.spd, ifiv),
        spe:    small_to_u8(tvs.spe, ifiv),
    }
}

//fn encode_moves(movetable: &Vec<String>, moves: &Vec<String>) -> Vec<u16> {
fn encode_moves(moves_map: &HashMap<String, usize>, moves: &Vec<String>) -> Vec<u16> {
    moves
        .iter()
        .map(|m| element_to_binary(moves_map, m.into()) as u16)
        .collect()
}

//fn encoded_pokemon(tables: &Tables, pokemon: &Pokemon) -> PokemonBin {
pub fn encoded_pokemon(maps: &Maps, pokemon: &Pokemon) -> PokemonBin {
    PokemonBin {
        name:       element_to_binary(&maps.names, pokemon.name.clone()) as u16,
        gender:     gender_to_binary(pokemon.gender.clone()) as u8,
        item:       element_to_binary(&maps.items, pokemon.item.clone()) as u16,
        ability:    element_to_binary(&maps.abilities, pokemon.ability.clone()) as u16,
        level:      small_to_u8(pokemon.level.clone(), false) as u8,
        shiny:      false, // placeholder
        tera:       element_to_binary(&maps.teras, pokemon.tera.clone()) as u8,
        evs:        encode_tvs(pokemon.evs.clone(), false),
        nature:     element_to_binary(&maps.natures, pokemon.nature.clone()) as u8,
        ivs:        encode_tvs(pokemon.ivs.clone(), true),
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


