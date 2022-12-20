use crate::assembler::tokens::{validate_string, Command, Import, Path, Rune};
use crate::core::{opcode_to_str, str_to_opcode};
use std::{fmt::Display, str::FromStr};

pub enum TextToken {
    Comment(String),
    Rune(Rune),
    Label(Command),
    Import(Import),
    Path(Path),
    NumberLiteral(u64),
    Assembly(u8),
    StringLiteral(String),
    NewLine,
    Tab(u8),
}
impl FromStr for TextToken {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // try parse rune
        if let Ok(rune) = s.parse() {
            return Ok(Self::Rune(rune));
        }

        // try parse label
        if let Ok(label) = s.parse() {
            return Ok(Self::Label(label));
        }

        // try parse number
        if let Some(number) = parse_number(s) {
            return Ok(Self::NumberLiteral(number));
        }

        // try parse instruction
        if let Some(opcode) = str_to_opcode(s) {
            return Ok(Self::Assembly(opcode));
        }

        // try parse path
        if let Ok(path) = s.parse() {
            return Ok(Self::Path(path));
        }

        // try parse import
        if let Ok(import) = s.parse() {
            return Ok(Self::Import(import));
        }

        // validate remaining string
        if let Ok(_) = validate_string(s) {
            let s = String::from(s);
            return Ok(Self::StringLiteral(s));
        }

        Err(format!("Couldn't parse token {}", s))
    }
}
impl Display for TextToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Comment(string) => write!(f, "Comment({})", string),
            Self::Rune(rune) => write!(f, "Rune({})", rune),
            Self::Label(label) => write!(f, "Label({:?})", label),
            Self::Import(import) => write!(f, "Import({})", import),
            Self::Path(path) => write!(f, "Path({})", path),
            Self::NumberLiteral(number) => write!(f, "Number({})", number),
            Self::Assembly(opcode) => write!(f, "Assembly({})", opcode_to_str(*opcode)),
            Self::StringLiteral(string) => write!(f, "String Literal({})", string),
            Self::NewLine => write!(f, "New Line"),
            Self::Tab(count) => write!(f, "Tab({})", count),
        }
    }
}

fn parse_number(string: &str) -> Option<u64> {
    let (token, radix) = match string.strip_prefix("0x") {
        Some(hex) => (hex, 16),
        None => (string, 10),
    };
    match u64::from_str_radix(token, radix) {
        Ok(lit) => Some(lit),
        Err(_) => None,
    }
}
