use std::{fmt::Display, str::FromStr};

use super::{MACRO_DEF, ROUTINE_DEF};

pub enum Import {
    Routine { name: String },
    Macro { name: String },
}
impl FromStr for Import {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        match chars.next() {
            Some(MACRO_DEF) => Ok(Import::Macro {
                name: chars.collect(),
            }),
            Some(ROUTINE_DEF) => Ok(Import::Routine {
                name: chars.collect(),
            }),
            _ => Err("Couldn't parse".into()),
        }
    }
}
impl Display for Import {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Import::Routine { name } => write!(f, "Routine({})", name),
            Import::Macro { name } => write!(f, "Macro({})", name),
        }
    }
}
