/*
* dex.rs
*
* we parse and build our "ground truth" pokedex data structures here
* first we parse the .txt files from the dex folders into Vec<String>
* then we turn those in HashMaps
* this allows an o(1) lookup for both encoding and decoding the binary
*/

use std::collections::HashMap;

const NAMES: &str = include_str!("../dex/names.txt");
const ITEMS: &str = include_str!("../dex/items.txt");
const ABILITIES: &str = include_str!("../dex/abilities.txt");
const MOVES: &str = include_str!("../dex/moves.txt");
const NATURES: &str = include_str!("../dex/natures.txt");
const TERAS: &str = include_str!("../dex/teras.txt");

#[derive(Debug, Default)]
pub struct Tables {
    pub names:      Vec<String>,
    pub items:      Vec<String>,
    pub abilities:  Vec<String>,
    pub moves:      Vec<String>,
    pub natures:    Vec<String>,
    pub teras:      Vec<String>,
}

pub struct Maps {
    pub names:      HashMap<String, usize>,
    pub items:      HashMap<String, usize>,
    pub abilities:  HashMap<String, usize>,
    pub moves:      HashMap<String, usize>,
    pub natures:    HashMap<String, usize>,
    pub teras:      HashMap<String, usize>,
}

pub struct Dex {
    pub tables: Tables,
    pub maps: Maps,
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
            
fn build_map(table: &Vec<String>) -> HashMap<String, usize> {
    let mut map: HashMap<String, usize>= HashMap::new();
    for (i, t) in table.iter().enumerate() {
        map.insert(t.clone(), i);
    }
    map
}

pub fn build_dex() -> Dex {
    let tables = parse_tables();
    let maps = build_maps(&tables);
    Dex {
        tables,
        maps,
    }
}

