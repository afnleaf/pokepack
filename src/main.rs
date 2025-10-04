/*
* main.rs
*
* just some testing of the library
*/

fn main() {
    tests();
}

fn tests() {
    let testpaste: &str = include_str!("../paste2.txt");
    println!("{}", &testpaste);

    let output_bytes = pokepack::pokepaste_to_pokepack(testpaste.into());
    println!("Raw Bytes:");
    for a in output_bytes {
        println!("{:?}", a);
    }
    println!();

    let output_hex = pokepack::pokepaste_to_hex(testpaste.into());
    println!("Hex:\n{}", &output_hex);
    let h = pokepack::hex_to_pokepaste(output_hex).unwrap();
    println!("Hex Conversion:\n{}", &h);

    let output_b64 = pokepack::pokepaste_to_base64(testpaste.into());
    println!("Base64:\n{}", &output_b64);
    let s = pokepack::base64_to_pokepaste(output_b64).unwrap();
    println!("Base64 Conversion:\n{}", &s);
}

//use pokepack::{
//    dex::{self, Dex, Tables, Maps},
//    parser::{self, Pokemon},
//    binary::{self, PokemonBin},
//    codec::{self},
//};
    //for v in &vec_encoded_pokemon {
    //    let packed_bytes = v.pack_to_bytes();
    //    println!("Packed: {:02X?}", packed_bytes);
    //    let unpacked_bytes = binary::unpack_from_bytes(&packed_bytes);
    //    println!("Unpacked: {}", unpacked_bytes);
    //    let poke_string = codec::pokebin_to_string(&tables, &unpacked_bytes);
    //    println!("String:\n{}", poke_string);
    //}
    //for p in &vec_pokemon {
    //    println!("{}", p);
    //}

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
