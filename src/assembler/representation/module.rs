use crate::assembler::representation::{Import, Macro, Routine};
use crate::assembler::tokens::{Rune, TextToken};
use std::fmt::Display;

pub struct Module {
    pub imports: Vec<Import>,
    pub macros: Vec<Macro>,
    pub routines: Vec<Routine>,
}
impl Module {
    pub fn from_text_tokens(tokens: Vec<TextToken>) -> Result<Module, String> {
        let mut imports = Vec::new();
        let mut macros = Vec::new();
        let mut routines = Vec::new();
        let mut text_tokens = tokens.into_iter();
        while let Some(token) = text_tokens.next() {
            match token {
                TextToken::Rune(rune) => match rune {
                    Rune::OpenInclude => {
                        imports.append(&mut Import::from_text_tokens(&mut text_tokens)?);
                    }
                    Rune::OpenRoutine => {
                        routines.push(Routine::from_text_tokens(false, &mut text_tokens)?);
                    }
                    Rune::OpenExportedRoutine => {
                        routines.push(Routine::from_text_tokens(true, &mut text_tokens)?)
                    }
                    Rune::OpenMacro => {
                        macros.push(Macro::from_text_tokens(&mut text_tokens)?);
                    }
                    Rune::OpenComment => {
                        while let Some(token) = text_tokens.next() {
                            match token {
                                TextToken::Rune(rune) => match rune {
                                    Rune::CloseComment => break,
                                    _ => continue,
                                },
                                _ => continue,
                            }
                        }
                    }
                    _ => return Err(format!("Invalid rune '{}' in module", rune)),
                },
                TextToken::Comment(_) | TextToken::NewLine | TextToken::Tab(_) => continue,
                _ => return Err(format!("Invalid token '{}' in module", token)),
            }
        }
        Ok(Module {
            imports,
            macros,
            routines,
        })
    }
}
impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.imports {
            write!(f, "{}\n", i)?;
        }
        for m in &self.macros {
            write!(f, "{}\n", m)?;
        }
        for r in &self.routines {
            write!(f, "{}\n", r)?;
        }
        Ok(())
    }
}
