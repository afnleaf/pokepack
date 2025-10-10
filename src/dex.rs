/*
* dex.rs
*
* we parse and build our "ground truth" pokedex data structures here
* first we parse the .txt files from the dex folders into Vec<String>
* then we turn those in HashMaps
* this allows an o(1) lookup for both encoding and decoding the binary
*/

use std::collections::HashMap;

// our ground truth for building the dex
// all elements on a newline, simple to parser
const NAMES:        &str = include_str!("../dex/names.txt");
const ITEMS:        &str = include_str!("../dex/items.txt");
const ABILITIES:    &str = include_str!("../dex/abilities.txt");
const MOVES:        &str = include_str!("../dex/moves.txt");
const NATURES:      &str = include_str!("../dex/natures.txt");
const TERAS:        &str = include_str!("../dex/teras.txt");

/*
our dex struct contains two data structures
Tables: containing array/vector types for o(1) decoding
Maps: containing hashmaps for o(1) encoding
using the vec for encoding would be o(n) on lookups

illustration:

table:
[Bulbasaur, Ivysaur, Venusaur, etc]
    0         1         2       3
simple index access

map:
(key, value)
[(bulbasaur, 0), (ivysaur, 1), (venusaur, 2)]
the value is what we want to encode in our binary pack

we make the keys lowercase so that they are case insensitive
we only need the value which is the index of the pokemon
we keep the Vec<String> the original case

further consideration can be done on nomenclature of Table
-> Array? Vec? idk
cause HashMap and HashTable are the same thing, it could be confusing
*/

#[derive(Debug, Default)]
pub struct Dex {
    pub tables: Tables,
    pub maps: Maps,
}

#[derive(Debug, Default)]
pub struct Tables {
    pub names:      Vec<String>,
    pub items:      Vec<String>,
    pub abilities:  Vec<String>,
    pub moves:      Vec<String>,
    pub natures:    Vec<String>,
    pub teras:      Vec<String>,
}

#[derive(Debug, Default)]
pub struct Maps {
    pub names:      HashMap<String, usize>,
    pub items:      HashMap<String, usize>,
    pub abilities:  HashMap<String, usize>,
    pub moves:      HashMap<String, usize>,
    pub natures:    HashMap<String, usize>,
    pub teras:      HashMap<String, usize>,
}

// the rest of the functions in this module feel self explanatory
impl Dex {
    pub fn build() -> Self {
        let tables = parse_tables();
        let maps = build_maps(&tables);
        Dex {
            tables,
            maps,
        }
    }
}

fn parse_tables() -> Tables {
    Tables {
        names:      parse_table(NAMES),
        items:      parse_table(ITEMS),
        abilities:  parse_table(ABILITIES),
        moves:      parse_table(MOVES),
        natures:    parse_table(NATURES),
        teras:      parse_table(TERAS),
    }
}

fn parse_table(file: &str) -> Vec<String> {
    file
        .lines()
        .map(String::from)
        .collect()
}

fn build_maps(tables: &Tables) -> Maps {
    Maps {
        names:      build_map(&tables.names),
        items:      build_map(&tables.items),
        abilities:  build_map(&tables.abilities),
        moves:      build_map(&tables.moves), 
        natures:    build_map(&tables.natures), 
        teras:      build_map(&tables.teras), 
    }
}

// convert to lowercase to make the input text able to be case insensitive
fn build_map(table: &Vec<String>) -> HashMap<String, usize> {
    table
        .iter()
        .enumerate()
        .map(|(i, t)| (t.to_lowercase(), i))
        .collect::<HashMap<String, usize>>()
}

