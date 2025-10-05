/*
* binary.rs
*
* custom binary file
*/

use std::fmt;

// see if we add this up without bit packing -> 241 bits?
#[derive(Debug, Default, Clone)]
pub struct PokemonBin {
    pub name:       u16,
    pub gender:     u8,
    pub item:       u16,
    pub ability:    u16,
    pub level:      u8,
    pub shiny:      bool,
    pub tera:       u8,
    pub evs:        TvBin,
    pub nature:     u8,
    pub ivs:        TvBin,
    pub moves:      Vec<u16>, // we will just encode the first 4 for ease
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
    pub hp:     u8, 
    pub atk:    u8,
    pub def:    u8,
    pub spa:    u8,
    pub spd:    u8,
    pub spe:    u8,
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


// this is just pack but in reverse
// start from group 3 and go backwards
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
