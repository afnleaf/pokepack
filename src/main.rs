/*
* main.rs
*/

use std::fmt;

mod parser;
//mod encoder;

use crate::parser::{Pokemon, Tv};

const NAMES: &str = include_str!("../dex/names.txt");
const ITEMS: &str = include_str!("../dex/items.txt");
const ABILITIES: &str = include_str!("../dex/abilities.txt");
const MOVES: &str = include_str!("../dex/moves.txt");
const NATURES: &str = include_str!("../dex/natures.txt");
const TERAS: &str = include_str!("../dex/teras.txt");

#[derive(Debug, Default)]
struct Table {
    names: Vec<String>,
    items: Vec<String>,
    abilities: Vec<String>,
    moves: Vec<String>,
    natures: Vec<String>,
    teras: Vec<String>,
}

fn parse_tables() -> Table {
    Table {
        names: parse_table(NAMES),
        items: parse_table(ITEMS),
        abilities: parse_table(ABILITIES),
        moves: parse_table(MOVES),
        natures: parse_table(NATURES),
        teras: parse_table(TERAS),
    }
}

fn parse_table(file: &str) -> Vec<String> {
    // standard imperative
    /*
    let mut result = Vec::new();
    for line in file.lines() {
        result.push(line.to_string())
    }
    result
    */
    //idiomatic rust
    file
        .lines()
        .map(String::from)
        .collect()
}


// standard function, returns usize, then cast to the correct size later
fn element_to_binary(table: &Vec<String>, element: String) -> usize {
    let compare = element.to_lowercase();
    for (i, e) in table.iter().enumerate() {
        //println!("i: {}, e: {}", i, e);
        if *e == compare {

            return i;
        }
    }

    0
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

// see if we add this up without bit packing -> 241 bits?
#[derive(Debug, Default, Clone)]
pub struct PokemonBin {
    pub name: u16,
    pub gender: u8,
    pub item: u16,
    pub ability: u16,
    pub level: u8,
    pub shiny: bool,
    pub tera: u8,
    pub evs: TvBin,
    pub nature: u8,
    pub ivs: TvBin,
    pub moves: Vec<u16>, // we will just encode the first 4 for ease
}

impl fmt::Display for PokemonBin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {} {:04X} {} {:04X} {:04X} {:04X} {:04X}",
            self.name,
            self.gender,
            self.item,
            self.ability,
            self.level,
            if self.shiny {1} else {0},
            self.tera,
            self.evs,
            self.nature,
            self.ivs,
            self.moves[0],
            self.moves[1],
            self.moves[2],
            self.moves[3],
        )
    }
}

// training values either 0-31 or 0-255
#[derive(Debug, Default, Clone)]
pub struct TvBin {
    pub hp: u8, 
    pub atk: u8,
    pub def: u8,
    pub spa: u8,
    pub spd: u8,
    pub spe: u8,
}

impl fmt::Display for TvBin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:04X} {:04X} {:04X} {:04X} {:04X} {:04X}",
            self.hp,
            self.atk,
            self.def,
            self.spa,
            self.spd,
            self.spe,
        ) 
    }
}

// group u32
const POKEMON_BITS: u32 = 11;
const GENDER_BITS: u32 = 2;
const ITEM_BITS: u32 = 10;
const ABILITY_BITS: u32 = 9;
// group u8
const LEVEL_BITS: u32 = 7;
const SHINY_BITS: u32 = 1;
// group u128
const TERA_BITS: u32 = 5;
const EV_BITS: u32 = 8;
const IV_BITS: u32 = 5;
const NATURE_BITS: u32 = 5;
const MOVE_BITS: u32 = 10;


impl PokemonBin {
    pub fn pack_to_bytes(&self) -> [u8; 21] {
        let mut group1: u32 = 0;
        let mut group2: u8 = 0;
        let mut group3: u128 = 0;

        // ill be honest idk how to do this part on my own
        // like I get what bit shifting does but I cant express it myself in Rust
        // I just know I need to do it, 
        // and I know I'm pushing bits onto an unsigned int
        group1 |= self.name as u32;
        group1 <<= GENDER_BITS;     group1 |= self.gender as u32;
        group1 <<= ITEM_BITS;       group1 |= self.item as u32;
        group1 <<= ABILITY_BITS;    group1 |= self.ability as u32;

        group2 |= self.level as u8;
        group2 <<= SHINY_BITS;      group2 |= if self.shiny { 1 } else { 0 };

        group3 |= self.tera as u128;
        let evs = [
            self.evs.hp, 
            self.evs.atk, 
            self.evs.def, 
            self.evs.spa, 
            self.evs.spd, 
            self.evs.spe
        ];
        for ev in evs {
            group3 <<= EV_BITS;     group3 |= ev as u128;
        }

        group3 <<= NATURE_BITS;     group3 |= self.nature as u128;

        let ivs = [
            self.ivs.hp, 
            self.ivs.atk, 
            self.ivs.def, 
            self.ivs.spa, 
            self.ivs.spd, 
            self.ivs.spe
        ];
        for iv in ivs {
            group3 <<= IV_BITS;      group3 |= iv as u128;
        }

        for i in 0..4 {
            let move_id = self.moves.get(i).cloned().unwrap_or(0);
            group3 <<= MOVE_BITS;   group3 |= move_id as u128;
        }

        let mut result = [0u8; 21];
        result[0..4].copy_from_slice(&group1.to_be_bytes());
        result[4..5].copy_from_slice(&group2.to_be_bytes());
        result[5..21].copy_from_slice(&group3.to_be_bytes());

        result
    }
}

pub fn unpack_from_bytes(bytes: &[u8; 21]) -> PokemonBin {
    // have to reconstruct the integer groups from the u8 array
    let mut group1_bytes = [0u8; 4];
    group1_bytes.copy_from_slice(&bytes[0..4]);
    let mut group1 = u32::from_be_bytes(group1_bytes);

    // group 2 is just 1 byte
    let mut group2 = u8::from_be_bytes(bytes[4..5].try_into().unwrap());
    
    let mut group3_bytes = [0u8; 16];
    group3_bytes.copy_from_slice(&bytes[5..21]);
    let mut group3 = u128::from_be_bytes(group3_bytes);

    let mut pbin = PokemonBin::default();

    // unpack group 3 u128 in reverse
    let mut moves = Vec::with_capacity(4);
    for _ in 0..4 {
        moves.insert(0, (group3 & ((1 << MOVE_BITS) - 1)) as u16);
        group3 >>= MOVE_BITS;
    }
    pbin.moves = moves;

    pbin.ivs.spe = (group3 & ((1 << IV_BITS) - 1)) as u8; group3 >>= IV_BITS;
    pbin.ivs.spd = (group3 & ((1 << IV_BITS) - 1)) as u8; group3 >>= IV_BITS;
    pbin.ivs.spa = (group3 & ((1 << IV_BITS) - 1)) as u8; group3 >>= IV_BITS;
    pbin.ivs.def = (group3 & ((1 << IV_BITS) - 1)) as u8; group3 >>= IV_BITS;
    pbin.ivs.atk = (group3 & ((1 << IV_BITS) - 1)) as u8; group3 >>= IV_BITS;
    pbin.ivs.hp  = (group3 & ((1 << IV_BITS) - 1)) as u8; group3 >>= IV_BITS;

    pbin.nature = (group3 & ((1 << NATURE_BITS) - 1)) as u8;
    group3 >>= NATURE_BITS;

    pbin.evs.spe = (group3 & ((1 << EV_BITS) - 1)) as u8; group3 >>= EV_BITS;
    pbin.evs.spd = (group3 & ((1 << EV_BITS) - 1)) as u8; group3 >>= EV_BITS;
    pbin.evs.spa = (group3 & ((1 << EV_BITS) - 1)) as u8; group3 >>= EV_BITS;
    pbin.evs.def = (group3 & ((1 << EV_BITS) - 1)) as u8; group3 >>= EV_BITS;
    pbin.evs.atk = (group3 & ((1 << EV_BITS) - 1)) as u8; group3 >>= EV_BITS;
    pbin.evs.hp  = (group3 & ((1 << EV_BITS) - 1)) as u8; group3 >>= EV_BITS;

    pbin.tera = (group3 & ((1 << TERA_BITS) - 1)) as u8;

    // unpack group 2 u8 in reverse
    pbin.shiny = (group2 & 1) == 1;
    group2 >>= SHINY_BITS;
    pbin.level = group2 & ((1 << LEVEL_BITS) - 1);

    // unpack group 1 u32 in reverse
    pbin.ability = (group1 & ((1 << ABILITY_BITS) - 1)) as u16;
    group1 >>= ABILITY_BITS;
    pbin.item = (group1 & ((1 << ITEM_BITS) - 1)) as u16;
    group1 >>= ITEM_BITS;
    pbin.gender = (group1 & ((1 << GENDER_BITS) - 1)) as u8;
    group1 >>= GENDER_BITS;
    pbin.name = (group1 & ((1 << POKEMON_BITS) - 1)) as u16;

    pbin
}

fn pokebin_to_string(tables: &Table, pbin: &PokemonBin) -> Pokemon {
    Pokemon {
        name: binary_to_element(&tables.names, pbin.name.clone().into()),
        gender: binary_to_gender(pbin.gender.clone().into()),
        item: binary_to_element(&tables.items, pbin.item.clone().into()),
        ability: binary_to_element(&tables.abilities, pbin.ability.clone().into()),
        level: pbin.level.clone().to_string(),
        shiny: if pbin.shiny { "Yes".into() } else { "".into() }, // bruh
        tera: binary_to_element(&tables.teras, pbin.tera.clone().into()),
        evs: decode_tvs(pbin.evs.clone().into()),
        nature: binary_to_element(&tables.natures, pbin.nature.clone().into()),
        ivs: decode_tvs(pbin.ivs.clone().into()),
        moves: decode_moves(&tables.moves, pbin.moves.clone().into()),
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
        hp: tvs.hp.to_string(),
        atk: tvs.atk.to_string(),
        def: tvs.def.to_string(),
        spa: tvs.spa.to_string(),
        spd: tvs.spd.to_string(),
        spe: tvs.spe.to_string(),
    }
}

fn encode_tvs(tvs: Tv, ifiv: bool) -> TvBin {
    TvBin {
        hp: small_to_u8(tvs.hp, ifiv),
        atk: small_to_u8(tvs.atk, ifiv),
        def: small_to_u8(tvs.def, ifiv),
        spa: small_to_u8(tvs.spa, ifiv),
        spd: small_to_u8(tvs.spd, ifiv),
        spe: small_to_u8(tvs.spe, ifiv),
    }
}

fn encode_moves(movetable: &Vec<String>, moves: &Vec<String>) -> Vec<u16> {
    moves
        .iter()
        .map(|m| element_to_binary(movetable, m.into()) as u16)
        .collect()
}

fn encoded_pokemon(tables: &Table, pokemon: &Pokemon) -> PokemonBin {
    PokemonBin {
        name: element_to_binary(&tables.names, pokemon.name.clone()) as u16,
        gender: gender_to_binary(pokemon.gender.clone()) as u8,
        item: element_to_binary(&tables.items, pokemon.item.clone()) as u16,
        ability: element_to_binary(&tables.abilities, pokemon.ability.clone()) as u16,
        level: small_to_u8(pokemon.level.clone(), false) as u8,
        shiny: false, // placeholder
        tera: element_to_binary(&tables.teras, pokemon.tera.clone()) as u8,
        evs: encode_tvs(pokemon.evs.clone(), false),
        nature: element_to_binary(&tables.natures, pokemon.nature.clone()) as u8,
        ivs: encode_tvs(pokemon.ivs.clone(), true),
        moves: encode_moves(&tables.moves, &pokemon.moves),
    }
}

fn main() {
    tests();
}

fn tests() {
    let tables = parse_tables();
    //println!("{:?}", tables);

    assert!(element_to_binary(&tables.names, "bulbasaur".into()) == 0);
    assert!(element_to_binary(&tables.items, "eject pack".into()) == 101);
    assert!(element_to_binary(&tables.abilities, "mind's eye".into()) == 303);
    assert!(element_to_binary(&tables.moves, "blood moon".into()) == 901);
    assert!(element_to_binary(&tables.natures, "Modest".into()) == 13);
    assert!(element_to_binary(&tables.teras, "Stellar".into()) == 18);

    assert!(binary_to_element(&tables.names, 1) == String::from("ivysaur"));

    let test: &str = include_str!("../paste.txt");
    
    let vec_pokemon: Vec<Pokemon> = parser::parse_pokepaste(test.into());
    for p in &vec_pokemon {
        println!("{}", p);
    }

    let vec_encoded_pokemon: Vec<PokemonBin> = 
        vec_pokemon
            .iter()
            .map(|p| encoded_pokemon(&tables, p))
            .collect();

    for v in &vec_encoded_pokemon {
        let packed_bytes = v.pack_to_bytes();
        println!("Packed: {:02X?}", packed_bytes);
        let unpacked_bytes = unpack_from_bytes(&packed_bytes);
        println!("Unpacked: {}", unpacked_bytes);
        let poke_string = pokebin_to_string(&tables, &unpacked_bytes);
        println!("String:\n{}", poke_string);

    }
}


    // imperative way?
    /*
    let mut i = 0;
    for t in table.teras {
        if t == tera {
            
        }
    }
    */
