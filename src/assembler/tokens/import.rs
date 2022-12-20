use crate::assembler::tokens::{IMPORT_NAME_ASSIGNMENT, MACRO_DEF, ROUTINE_DEF};
use std::{fmt::Display, str::FromStr};

pub enum Import {
    Routine { identifier: String, name: String },
    Macro { identifier: String, name: String },
}
impl FromStr for Import {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_name_id(s: String) -> (String, String) {
            match s.split_once(IMPORT_NAME_ASSIGNMENT) {
                Some((identifier, name)) => (identifier.into(), name.into()),
                None => (s.clone(), s),
            }
        }

        let mut chars = s.chars();
        match chars.next() {
            Some(MACRO_DEF) => {
                let (identifier, name) = parse_name_id(chars.collect());
                Ok(Import::Macro { identifier, name })
            }
            Some(ROUTINE_DEF) => {
                let (identifier, name) = parse_name_id(chars.collect());
                Ok(Import::Routine { identifier, name })
            }
            _ => Err("Couldn't parse".into()),
        }
    }
}
impl Display for Import {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Import::Routine { name, .. } => write!(f, "Routine({})", name),
            Import::Macro { name, .. } => write!(f, "Macro({})", name),
        }
    }
}
