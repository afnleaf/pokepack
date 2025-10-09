/*
* parser.rs
*
* logic for parsing the pokepaste text format
* example found in paste.txt
*/

use std::{
    //fmt::{self, Write},
    fmt,
    sync::OnceLock,
};
use regex::Regex;
use regex::Error as RegexError;

use crate::error::ParseError;

// data struct logic ----------------------------------------------------------

// our pokemon information struct 
// makes it easier to convert to the intermediate binary format
// easier to print out
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

// not sure how to fix not displaying an info if it is empty,
// without making this function wonkier than it already is
// yeaa.. were gonna have to do it the wonky wayy..
impl fmt::Display for Pokemon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // NAME
        write!(f, "{}", self.name)?;
        // GENDER
        if !self.gender.is_empty() {
            write!(f, " ({})", self.gender.to_uppercase())?;
        }
        // ITEM
        if !self.item.is_empty() {
            write!(f, ": {}", self.item)?;
        } else {
            write!(f, "\n")?;
        }
        // ABILITY
        if !self.ability.is_empty() {
            writeln!(f, "{}", self.ability)?;
        }
        // LEVEL
        if !self.level.is_empty() {
            writeln!(f, "Level: {}", self.level)?;
        }
        // SHINY
        if self.shiny.to_lowercase() == "yes" {
            writeln!(f, "Shiny: Yes")?;
        }
        // TERA
        if !self.tera.is_empty() {
            writeln!(f, "Tera Type: {}", self.tera)?;
        }
        // EVS
        write!(f, "{}", self.evs)?;
        // NATURE
        if !self.nature.is_empty() {
            writeln!(f, "{} Nature", self.nature)?;
        }
        // IVS
        write!(f, "{}", self.ivs)?;
        // MOVES
        if !self.moves.iter().all(|m| m.is_empty()) {
            for m in &self.moves {
                if !m.is_empty() {
                    write!(f, "- {m}\n")?;
                }
            }
        }
        Ok(())
    }
}

// training values
// needs ifiv
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
        // we don't want to display IVs that are 31 and EVs that are 0
        if self.ifiv {
            printtvs(f, self, "31")?
        } else {
            printtvs(f, self, "0")?
        }
        Ok(())
    }
}


// yeah idk this is just the way im gonna try it
//fn printtvs(ivs: &Tv, cmp: &str) -> String {
fn printtvs(f: &mut fmt::Formatter, ivs: &Tv, cmp: &str) -> fmt::Result {
    //let mut text = String::new();
    let mut v: Vec<String> = Vec::new();

    if ivs.hp   != cmp { v.push(format!("HP {}", ivs.hp)); }
    if ivs.atk  != cmp { v.push(format!("Atk {}", ivs.atk)); }
    if ivs.def  != cmp { v.push(format!("Def {}", ivs.def)); }
    if ivs.spa  != cmp { v.push(format!("SpA {}", ivs.spa)); }
    if ivs.spd  != cmp { v.push(format!("SpD {}", ivs.spd)); }
    if ivs.spe  != cmp { v.push(format!("Spe {}", ivs.spe)); }

    if v.is_empty() {
        return Ok(());
    }

    let label = match cmp {
        "31" => "IVs",
        "0" => "EVs",
        _ => "error",
    };

    write!(f, "{}: {}", label, v.join(" / "))?;
    writeln!(f)?;

    Ok(())
 
    /*
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
    */
}

// parsing logic --------------------------------------------------------------

// this is the main function being called from this module
pub fn parse_pokepaste(paste: String) -> Result<Vec<Pokemon>, ParseError>{
    //let text = paste.trim().to_lowercase();
    let text = paste.trim();
    if text.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    split_into_blocks(text)
        .into_iter()
        .map(parse_pokemon)
        .collect()

    /*
    //println!("{}", text);
    let blocks = split_into_blocks(&text);
    let mut vec_pokemon: Vec<Pokemon> = Vec::new();
    //println!("{:?}", blocks);
    for b in blocks {
        //println!("{:?}\n", b);
        vec_pokemon.push(parse_pokemon(b).expect("ERROR: "));
        //println!("");
    }
    // remove bad blocks?
    vec_pokemon
    */
}

// by convention there are two new lines between each pokemon block
// but do we wanna make this more robust?
// just gotta watch out for those carriage returns cause of windows
fn split_into_blocks(text: &str) -> Vec<String> {
    if text.contains("\r") {
        text.split("\r\n\r\n")
        //text.split("\r\n")
            .map(|t| t.into())
            .collect()

    } else { 
        text.split("\n\n")
        //text.split("\n")
            .map(|t| t.into())
            .collect()
    }
}

static GENDER_REGEX: OnceLock<Result<Regex, RegexError>> = OnceLock::new();
static NICKNAME_REGEX: OnceLock<Result<Regex, RegexError>> = OnceLock::new();

// get_or_init returns an  &Result<Regex, regex::Error>
// as_ref to Result<&Regex, &regex::Error>
// map_err to operate on Result and wrap with ParseError
fn get_gender_regex() -> Result<&'static Regex, ParseError> {
    GENDER_REGEX
        .get_or_init(|| { Regex::new(r"\(([mf])\)") })
        .as_ref()
        .map_err(|err| ParseError::Regex(err.clone()))
}

fn get_nickname_regex() -> Result<&'static Regex, ParseError> {
    NICKNAME_REGEX
        .get_or_init(|| { Regex::new(r"\(([^)]+)\)") })
        .as_ref()
        .map_err(|err| ParseError::Regex(err.clone()))
}

// this will parse one pokemon at a time
fn parse_pokemon(text: String) -> Result<Pokemon, ParseError> {
    let gender_regex = get_gender_regex()?;
    let nickname_regex = get_nickname_regex()?;
    let mut pokemon = Pokemon::default();

    // we only want to consider non empty lines from a block
    let lines: Vec<&str> = 
        text
            .lines()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

    if lines.is_empty() {
        return Err(ParseError::EmptyBlock);
    }

    // first line of a block needs to be a pokemon name
    // this is in line with PS behavior
    let header = lines[0];
    if header.contains(':') || header.starts_with('-') {
        return Err(ParseError::MissingName { block: text });
    }

    // we set default name as full field
    // this handles NAME (GENDER) @ ITEM fully
    let mut name: String = header.to_string();
    let mut gender = String::new();
    let mut item = String::new();
    
    // split on item
    if name.contains("@") {
        let l: Vec<&str> = name.split('@').collect();
        let name_part = l[0].trim().to_string();
        let item_part = l[1].trim().to_string();
        name = name_part;
        item = item_part;
    }
    
    // check for (f) or (m) in the name
    if let Some(captures) = gender_regex.captures(&name) {
        if let Some(gender_match) = captures.get(1) {
            gender = gender_match
                        .as_str()
                        .to_string();
            name = gender_regex
                    .replace_all(&name, "")
                    .trim()
                    .to_string();
        }
    }

    // check if there is a nickname 
    if name.contains("(") && name.contains(")") {
        if let Some(captures) = nickname_regex.captures(&name) {
            if let Some(name_match) = captures.get(1) {
                // extract species name from parentheses
                name = name_match
                        .as_str()
                        .to_string();
                // lowkey really funny to go as &str to String
            }
        }
    }
    
    // assign header info to pokemon struct
    pokemon.name = name.to_lowercase();
    pokemon.item = item.to_lowercase();
    pokemon.gender = gender;

    // now we can parse over the rest of the block
    for line in &lines[1..] {
        //println!("{}", line);
        // split line via : into pairs
        let parts: Vec<&str> = line.split(": ").collect();
        // converting to lowercase makes the parsing easier
        let lower = parts[0].to_lowercase();
        //println!("{:?}", parts);
        if parts.len() >= 2 {
            //println!(" 2");
            let value = parts[1].trim().to_string();
            match lower.as_str() {
                "ability"   => pokemon.ability = value,
                "level"     => pokemon.level = value,
                "tera type" => pokemon.tera = value,
                "shiny"     => pokemon.shiny = value,
                "evs"       => pokemon.evs = parse_tvs(value, false)?,
                "ivs"       => pokemon.ivs = parse_tvs(value, true)?,
                // should just ignore anything not defined
                _ => {},
            }
        } else if lower.contains("nature") {
            let nature: Vec<&str> = lower.split(" nature").collect();
            pokemon.nature = nature[0].trim().into();
        } else if parts[0].starts_with("-") {
            if parts[0].len() > 1 {
                pokemon.moves.push(parts[0][1..].trim().into());
            } else {
                return Err(ParseError::MalformedLine {
                    line: line.to_string()
                });
            }

        } else {

        }
    }
    //println!("\n\n\n{}", pokemon);
    if pokemon.name.is_empty() {
        return Err(ParseError::MissingName { block: text });
    }
    Ok(pokemon)
}

fn parse_tvs(text: String, ifiv: bool) -> Result<Tv, ParseError> {
    let mut tv = Tv::default();
    let parts: Vec<&str> = text.split(" / ").collect();
    for p in parts {
        //println!("p: -{}-", p);
        let c: Vec<&str> = p.trim().split(" ").collect();

        // check for "VALUE STAT" format
        if c.len() != 2 {
            return Err(ParseError::MalformedTvString { line: p.to_string() });
        }

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
    Ok(tv)
}

