use std::fmt::Display;

use crate::assembler::tokens::{self, Rune, TextToken};

pub enum X {
    Routine(String),
    Macro(String),
}

pub struct Import {
    pub name: String,
    pub x: X,
}
impl Import {
    pub fn from_text_tokens(
        text_tokens: &mut dyn Iterator<Item = TextToken>,
    ) -> Result<Vec<Import>, String> {
        let Some(TextToken::StringLiteral(root)) = text_tokens.next() else {
            return Err("First token of routine must be string literal".into());
        };

        let mut imports = Vec::new();

        while let Some(token) = text_tokens.next() {
            match token {
                TextToken::Rune(rune) => match rune {
                    Rune::CloseDefinition => break,
                    _ => return Err("Invalid rune inside import definition".into()),
                },
                TextToken::Import(import) => match import {
                    tokens::Import::Routine { name } => {
                        imports.push(Import {
                            x: X::Routine(format!("{}.{}", root, name.clone())),
                            name,
                        });
                    }
                    tokens::Import::Macro { name } => {
                        imports.push(Import {
                            x: X::Macro(format!("{}.{}", root, name.clone())),
                            name,
                        });
                    }
                },
                TextToken::NewLine | TextToken::Tab(_) | TextToken::Comment(_) => continue,
                _ => return Err("Invalid token inside import definition".into()),
            }
        }

        Ok(imports)
    }
}
impl Display for Import {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Import: {}\n", self.name)?;
        match &self.x {
            X::Routine(path) => write!(f, " - type: Routine\n - path: {}\n", path),
            X::Macro(path) => write!(f, " - type: Macro\n - path: {}\n", path),
        }
    }
}
