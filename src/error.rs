use std::{
    //error::Error,
    fmt,
};
use regex::Error as RegexError;


#[derive(Debug)]
pub enum ParseError {
    // input string is only whitespace
    EmptyInput,
    // no information in a block
    EmptyBlock,
    // pokemon block is missing the name (required)
    MissingName { block: String },
    // EV or IV string is not in "VALUE STAT" format
    MalformedTvString { line: String },
    // when a line is whack
    MalformedLine { line: String },
    // for regex creation
    Regex(RegexError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyInput => {
                write!(f, "Input Pokepaste was empty.")
            },
            ParseError::EmptyBlock => {
                write!(f, "Found an empty Pokémon block.")
            },
            ParseError::MissingName { block } => {
                write!(
                    f, 
                    "Could not find a Pokemon name in this block: \n{}\n", 
                    block
                )
            },
            ParseError::MalformedTvString { line } => {
                write!(f, "Malofrmed EV/IV string {}", line)
            },
            ParseError::MalformedLine { line } => {
                write!(f, "Unrecognized or malformed line: '{}'", line)
            }
            ParseError::Regex(err) => {
                write!(f, "Regex compilation failed: {}", err)
            },
        }
    }
}

impl From<RegexError> for ParseError {
    fn from(err: RegexError) -> Self {
        ParseError::Regex(err)
    }
}

//impl Error for ParseError {}
