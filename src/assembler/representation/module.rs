use std::fmt::Display;

use crate::assembler::representation::{Macro, Routine};
use crate::assembler::tokens::{Rune, TextToken};

pub struct Module {
    pub macros: Vec<Macro>,
    pub routines: Vec<Routine>,
}
impl Module {
    pub fn from_text_tokens(tokens: Vec<TextToken>) -> Result<Module, String> {
        let mut macros = Vec::new();
        let mut routines = Vec::new();
        let mut tokens = tokens.into_iter();
        while let Some(token) = tokens.next() {
            match token {
                TextToken::Rune(rune) => match rune {
                    Rune::OpenRoutine | Rune::OpenExportedRoutine => {
                        let exported = match rune {
                            Rune::OpenExportedRoutine => true,
                            _ => false,
                        };
                        let routine = Routine::from_text_tokens(exported, &mut tokens)?;
                        routines.push(routine);
                    }
                    Rune::OpenMacro => {
                        let mac = Macro::from_text_tokens(&mut tokens)?;
                        macros.push(mac);
                    }
                    _ => return Err(format!("Invalid rune '{}' in module", rune)),
                },
                TextToken::Comment(_) | TextToken::NewLine | TextToken::Tab(_) => continue,
                _ => return Err(format!("Invalid token '{}' in module", token)),
            }
        }
        Ok(Module { macros, routines })
    }
}
impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for m in &self.macros {
            write!(f, "{}\n", m)?;
        }
        for r in &self.routines {
            write!(f, "{}\n", r)?;
        }
        Ok(())
    }
}
