use crate::assembler::tokens::{Command, Marker, NumberLiteral, Rune, SourceToken, TextToken};
use std::fmt::Display;

pub struct Macro {
    pub name: String,
    pub tokens: Vec<SourceToken>,
}
impl Macro {
    pub fn from_text_tokens(
        text_tokens: &mut dyn Iterator<Item = TextToken>,
    ) -> Result<Macro, String> {
        let Some(TextToken::StringLiteral(name)) = text_tokens.next() else {
            return Err("First token of macro must be string literal".into());
        };

        let mut tokens = Vec::new();
        while let Some(text_token) = text_tokens.next() {
            let source_token = match text_token {
                TextToken::Comment(string) => SourceToken::Comment { string },
                TextToken::Rune(rune) => match rune {
                    Rune::CloseDefinition => break,
                    Rune::OpenParameters => {
                        while let Some(token) = text_tokens.next() {
                            match token {
                                TextToken::StringLiteral(name) => {
                                    tokens.push(SourceToken::ParameterDef { name })
                                }
                                TextToken::Rune(rune) => match rune {
                                    Rune::CloseParameters => break,
                                    _ => {
                                        return Err(format!(
                                            "Invalid rune '{}' in parameter list",
                                            rune
                                        ))
                                    }
                                },
                                _ => {
                                    return Err(format!(
                                        "Invalid token '{}' in parameter list",
                                        token
                                    ))
                                }
                            }
                        }
                        continue;
                    }
                    _ => return Err("Invalid rune inside routine definition".into()),
                },
                TextToken::Label(Command { marker, label }) => match marker {
                    Marker::CallRoutine => SourceToken::RoutineCallLocal { label },
                    Marker::CallExportedRoutine => SourceToken::RoutineCallExported { label },
                    Marker::RoutineAddress => SourceToken::RoutineAddressLocal { label },
                    Marker::ExportedRoutineAddress => SourceToken::RoutineAddressExported { label },
                    Marker::UseMacro => SourceToken::MacroUse { label },
                    Marker::DefineAnchor => SourceToken::AnchorDef { label },
                    Marker::AnchorAddress => SourceToken::AnchorAddressAbsolute { label },
                    Marker::AnchorRelativeAddress => SourceToken::AnchorAddressRelative { label },
                    Marker::Parameter => SourceToken::ParameterUse { label },
                },
                TextToken::Assembly(opcode) => match opcode {
                    opcode if opcode >= 176 && opcode < 180 => {
                        let Some(TextToken::NumberLiteral(number)) = text_tokens.next() else {
                                return Err("Text token after LIT opcode is not Number Literal".into());
                            };
                        let literal = match opcode {
                            176 => NumberLiteral::Byte(number as u8),
                            177 => NumberLiteral::Short(number as u16),
                            178 => NumberLiteral::Int(number as u32),
                            179 => NumberLiteral::Long(number as u64),
                            _ => unreachable!(),
                        };
                        SourceToken::NumberLiteral { literal }
                    }
                    opcode => SourceToken::Instruction { opcode },
                },
                TextToken::StringLiteral(_) => return Err("Dangling string literal".into()),
                TextToken::NumberLiteral(_) => return Err("Dangling number literal".into()),
                TextToken::Import(_) => return Err("Invalid import".into()),
                TextToken::NewLine => continue,
                TextToken::Tab(_) => continue,
            };
            tokens.push(source_token);
        }

        Ok(Macro { name, tokens })
    }
}
impl Display for Macro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Macro: {}\n", self.name)?;
        for token in &self.tokens {
            write!(f, " - {}\n", token)?;
        }
        Ok(())
    }
}
