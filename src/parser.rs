/*
* parser.rs
*
* logic for parsing the pokepaste text format
* example found in paste.txt
*/

use std::fmt;

#[derive(Debug, Default, Clone)]
pub struct Pokemon {
    pub name: String,
    pub gender: String,
    pub item: String,
    pub ability: String,
    pub level: String,
    pub shiny: String,
    pub tera: String,
    pub evs: Tv,
    pub nature: String,
    pub ivs: Tv,
    pub moves: Vec<String>,
}

impl fmt::Display for Pokemon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{} @ {}\nAbility: {}\nLevel: {}\n",
            self.name,
            self.gender,
            self.item,
            self.ability,
            self.level,
        )?;
        if self.shiny.to_lowercase() == "yes" {
            writeln!(f, "Shiny: Yes")?;
        }
        write!(
            f,
            "Tera Type: {}\n{}{} Nature\n{}Moves:\n",
            self.tera,
            self.evs,
            self.nature,
            self.ivs,
        )?;
        for m in &self.moves {
            write!(f, "- {m}\n")?;
        }
        Ok(())
    }
}

// training values
#[derive(Debug, Default, Clone)]
pub struct Tv {
    pub ifiv:   bool,
    pub hp:     String,
    pub atk:    String,
    pub def:    String,
    pub spa:    String,
    pub spd:    String,
    pub spe:    String,
}

impl fmt::Display for Tv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.ifiv {
            write!(f, "{}", printtvs(self, "31"))
        } else {
            write!(f, "{}", printtvs(self, "0"))
        }
        /*
        write!(
            f,
            "{} HP / {} Atk / {} Def / {} SpA / {} SpD / {} Spe",
            self.hp,
            self.atk,
            self.def,
            self.spa,
            self.spd,
            self.spe,
        )
        */
    }
}

use std::fmt::Write;

// yeah idk this is just the way im gonna try it
fn printtvs(ivs: &Tv, cmp: &str) -> String {
    let mut text = String::new();
    let mut v: Vec<String> = Vec::new();

    if ivs.hp != cmp {
        v.push(format!("HP {}", ivs.hp));
    }

    if ivs.atk != cmp {
        v.push(format!("Atk {}", ivs.atk));
    }
    
    if ivs.def != cmp {
        v.push(format!("Def {}", ivs.def));
    }
    
    if ivs.spa != cmp {
        v.push(format!("SpA {}", ivs.spa));
    }
    
    if ivs.spd != cmp {
        v.push(format!("SpD {}", ivs.spd));
    }

    if ivs.spe != cmp {
        v.push(format!("Spe {}", ivs.spe));
    }

    for (i, j) in v.iter().enumerate() {
        write!(&mut text, "{}", j).unwrap();
        if i != v.len() - 1 && v.len() != 1 {
            write!(&mut text, " / ").unwrap();
        }
    }
    //println!("{}", &text);
    if text.is_empty() {
        text
    } else {
        let value = match cmp {
            "31" => "IVs",
            "0" => "EVs",
            _ => "error",
        };
        let mut full = String::new();
        writeln!(&mut full, "{}: {}", value, text).unwrap();
        full
    }
}

//fn printevs(evs: &Tv, f: &mut fmt::Formatter) -> fmt::Result {
//
//}

// just gotta watch out for the carriage return
fn split_into_blocks(text: &str) -> Vec<String> {
    if text.contains("\r") {
        text.split("\r\n\r\n")
            .map(|t| t.into())
            .collect()

    } else { 
        text.split("\n\n")
            .map(|t| t.into())
            .collect()
    }
}

fn parse_tvs(text: String, ifiv: bool) -> Tv {
    let mut tv = Tv::default();
    let parts: Vec<&str> = text.split(" / ").collect();
    for p in parts {
        //println!("p: -{}-", p);
        let c: Vec<&str> = p.trim().split(" ").collect();
        match c[1].to_lowercase().as_str() {
            "hp" => {tv.hp = c[0].into()},
            "atk" => {tv.atk = c[0].into()},
            "def" => {tv.def = c[0].into()},
            "spa" => {tv.spa = c[0].into()},
            "spd" => {tv.spd = c[0].into()},
            "spe" => {tv.spe = c[0].into()},
            _ => {},
        }
    }
    tv.ifiv = ifiv;
    tv
}

// how to deal with case sensitivity?
fn parse_pokemon(text: String) -> Pokemon {
    let mut pokemon = Pokemon::default();

    for line in text.lines() {
        //println!("{}", line);
        // split line via : into pairs
        let parts: Vec<&str> = line.split(": ").collect();
        let lower = parts[0].to_lowercase();
        //println!("{}", &lower);
        //println!("{:?}", parts);
        if parts.len() >= 2 {
            //println!(" 2");
            match lower.as_str() {
                "ability" => pokemon.ability = parts[1].trim().into(),
                "level" => pokemon.level = parts[1].trim().into(),
                "tera type" => pokemon.tera = parts[1].trim().into(),
                "shiny" => pokemon.shiny = parts[1].trim().into(),
                "evs" => pokemon.evs = parse_tvs(parts[1].trim().into(), false),
                "ivs" => pokemon.ivs = parse_tvs(parts[1].trim().into(), true),
                _ => todo!(),
            }
        } else if lower.contains("@") {
            //println!("@ found");
            let l: Vec<&str> = parts[0].split("@").collect();
            pokemon.name = l[0].trim().into();
            pokemon.item = l[1].trim().into();
        } else if lower.contains("nature") {
            let nature: Vec<&str> = lower.split(" nature").collect();
            //println!("nature: {}", nature[0]);
            pokemon.nature = capitalize_first_letter(nature[0].trim());
        } else if parts[0].starts_with("-") {
            //println!("move");
            pokemon.moves.push(parts[0][1..].trim().into());
        } else {
            //println!("uuhhh");
            // how to make pokemon with no item work?
        }
    }
    //println!("\n\n\n{}", pokemon);
    pokemon
}

pub fn parse_pokepaste(paste: String) -> Vec<Pokemon>{
    //let text = paste.trim().to_lowercase();
    let text = paste.trim();
    //println!("{}", text);
    let blocks = split_into_blocks(&text);
    let mut vec_pokemon: Vec<Pokemon> = Vec::new();
    //println!("{:?}", blocks);
    for b in blocks {
        //println!("{:?}\n", b);
        vec_pokemon.push(parse_pokemon(b));
        //println!("");
    }
    // remove bad blocks?

    vec_pokemon
}

// stupid helper
fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(), // Handle empty string
        Some(first_char) => {
            first_char.to_uppercase().collect::<String>() + chars.as_str()
        }
    }
}


