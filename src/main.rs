/*
* main.rs
*
* just some testing of the library
*/

use pokepack::{
    dex::{self, Dex, Tables, Maps},
    parser::{self, Pokemon},
    binary::{self, PokemonBin},
    codec::{self},
};

fn main() {
    tests();
}

fn tests() {
    //let tables = parse_tables();
    //println!("{:?}", tables);
    let dex: Dex = dex::build_dex();
    let tables: Tables = dex.tables;
    let maps: Maps = dex.maps;

    let test: &str = include_str!("../paste.txt");
    
    let vec_pokemon: Vec<Pokemon> = parser::parse_pokepaste(test.into());
    for p in &vec_pokemon {
        println!("{}", p);
    }

    let vec_encoded_pokemon: Vec<PokemonBin> = 
        codec::encode_all_pokemon(&maps, vec_pokemon);

    for v in &vec_encoded_pokemon {
        let packed_bytes = v.pack_to_bytes();
        println!("Packed: {:02X?}", packed_bytes);
        let unpacked_bytes = binary::unpack_from_bytes(&packed_bytes);
        println!("Unpacked: {}", unpacked_bytes);
        let poke_string = codec::pokebin_to_string(&tables, &unpacked_bytes);
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
    //assert!(element_to_binary(&tables.names, "bulbasaur".into()) == 0);
    //assert!(element_to_binary(&tables.items, "eject pack".into()) == 101);
    //assert!(element_to_binary(&tables.abilities, "mind's eye".into()) == 303);
    //assert!(element_to_binary(&tables.moves, "blood moon".into()) == 901);
    //assert!(element_to_binary(&tables.natures, "Modest".into()) == 13);
    //assert!(element_to_binary(&tables.teras, "Stellar".into()) == 18);
    //assert!(binary_to_element(&tables.names, 1) == String::from("ivysaur"));
/*
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
*/
