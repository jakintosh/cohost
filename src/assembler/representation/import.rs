use crate::assembler::tokens::{self, Path, Rune, TextToken};
use std::fmt::Display;

pub enum SymbolType {
    Macro,
    Routine,
}
pub struct Import {
    pub name: String,
    pub path: Path,
    pub symbol: SymbolType,
}
impl Import {
    pub fn from_text_tokens(
        text_tokens: &mut dyn Iterator<Item = TextToken>,
    ) -> Result<Vec<Import>, String> {
        let Some(TextToken::Path(path)) = text_tokens.next() else {
            return Err("First token of import must be a path".into());
        };

        let mut imports = Vec::new();
        while let Some(token) = text_tokens.next() {
            match token {
                TextToken::Import(import) => match import {
                    tokens::Import::Macro { identifier, name } => {
                        let mut path = path.clone();
                        path.push(identifier);
                        let symbol = SymbolType::Macro;
                        imports.push(Import { name, path, symbol });
                    }
                    tokens::Import::Routine { identifier, name } => {
                        let mut path = path.clone();
                        path.push(identifier);
                        let symbol = SymbolType::Routine;
                        imports.push(Import { name, path, symbol });
                    }
                },
                TextToken::Rune(rune) => match rune {
                    Rune::CloseDefinition => break,
                    _ => return Err("Invalid rune inside import definition".into()),
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
        match &self.symbol {
            SymbolType::Macro => write!(f, " - type: Macro\n")?,
            SymbolType::Routine => write!(f, " - type: Routine\n")?,
        };
        write!(f, " - path: {}\n", self.path)
    }
}
