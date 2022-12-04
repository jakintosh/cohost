use crate::assembler::representation::{Component, Macro, Module, Routine, Token};
use std::collections::HashMap;

type Hash = [u8; 32];
type ByteCo = Vec<u8>;
pub struct Library {
    pub macros: HashMap<String, ByteCo>,
    pub routine_names: HashMap<String, Hash>,
    pub routines: HashMap<Hash, ByteCo>,
}
impl Library {
    pub fn new() -> Library {
        Library {
            macros: HashMap::new(),
            routine_names: HashMap::new(),
            routines: HashMap::new(),
        }
    }
}

pub struct Context<'a> {
    macros: HashMap<String, Macro>,
    routines: HashMap<String, Routine>,
    assembled_tokens: HashMap<String, Vec<ByteCoIL>>,
    library: &'a Library,
}
impl<'a> Context<'a> {
    pub fn new(library: &'a Library) -> Context<'a> {
        Context {
            macros: HashMap::new(),
            routines: HashMap::new(),
            assembled_tokens: HashMap::new(),
            library,
        }
    }
    pub fn parse_module(&mut self, module: Module) -> Result<(), String> {
        for c in module.components {
            match c {
                Component::Macro { label, tokens } => {
                    self.register_macro(Macro { label, tokens })?
                }
                Component::Routine {
                    export,
                    label,
                    tokens,
                } => self.register_routine(Routine {
                    export,
                    label,
                    tokens,
                })?,
                _ => {}
            }
        }

        Ok(())
    }
    pub fn assemble(self) -> Result<ByteCo, String> {
        let Context {
            macros,
            routines,
            mut assembled_tokens,
            library,
        } = self;

        for (name, routine) in routines {
            println!("\n==================");
            let mut bytecoil: Vec<ByteCoIL> = Vec::new();
            bytecoil.push(ByteCoIL::RoutineDef(name));
            for token in routine.tokens {
                match Context::pre_assemble_token(token, &mut assembled_tokens, &macros, &library) {
                    Ok(mut il) => bytecoil.append(&mut il),
                    Err(err) => return Err(err),
                }
            }
            bytecoil.push(ByteCoIL::RoutineEnd);
            for il in bytecoil {
                println!("{}", il);
            }
            println!("==================");
        }

        Ok(vec![])
    }
    fn pre_assemble_token(
        token: Token,
        assembled_tokens: &mut HashMap<String, Vec<ByteCoIL>>,
        macros: &HashMap<String, Macro>,
        library: &'a Library,
    ) -> Result<Vec<ByteCoIL>, String> {
        let mut vec = Vec::new();
        match token {
            Token::Comment { string } => vec.push(ByteCoIL::Comment(string)),
            Token::Literal { literal } => vec.push(ByteCoIL::Assembled(literal.into())),
            Token::Instruction { code } => vec.push(ByteCoIL::Assembled(vec![code])),
            Token::RoutineCallLocal { id } => vec.push(ByteCoIL::RoutineCallLocal(id)),
            Token::RoutineCallExported { id } => vec.push(ByteCoIL::RoutineCallExported(id)),
            Token::RoutineAddressLocal { id } => vec.push(ByteCoIL::RoutineAddressLocal(id)),
            Token::RoutineAddressExported { id } => vec.push(ByteCoIL::RoutineAddressExported(id)),
            Token::LabelDef { id } => vec.push(ByteCoIL::LabelDef(id)),
            Token::LabelAddressRelative { id } => vec.push(ByteCoIL::LabelRel(id)),
            Token::LabelAddressAbsolute { id } => vec.push(ByteCoIL::LabelAbs(id)),
            Token::MacroUse { id } => vec.append(&mut Context::pre_assemble_macro(
                &id,
                assembled_tokens,
                macros,
                library,
            )?),
        };

        Ok(vec)
    }
    fn pre_assemble_macro(
        name: &str,
        assembled_tokens: &mut HashMap<String, Vec<ByteCoIL>>,
        macros: &HashMap<String, Macro>,
        library: &'a Library,
    ) -> Result<Vec<ByteCoIL>, String> {
        match assembled_tokens.get(name) {
            // if already assembled, use that
            Some(il) => Ok(il.clone()),

            // if not assembled
            None => match macros.get(name) {
                // first check context defined macros
                Some(mac) => {
                    let mut vec = Vec::new();
                    for token in &mac.tokens {
                        let mut il = Context::pre_assemble_token(
                            token.clone(),
                            assembled_tokens,
                            macros,
                            library,
                        )?;
                        vec.append(&mut il);
                    }
                    // once assembled, cache it
                    assembled_tokens.insert(name.to_string(), vec.clone());
                    Ok(vec)
                }
                // then check library
                None => match library.macros.get(name) {
                    Some(mac) => Ok(vec![ByteCoIL::Assembled(mac.clone())]),
                    None => return Err(format!("Using undefined macro: {}", name)),
                },
            },
        }
    }
    fn register_macro(&mut self, mac: Macro) -> Result<(), String> {
        match self.macros.insert(mac.label.clone(), mac) {
            None => Ok(()),
            Some(prev) => Err(format!("Context Error: Duplicate macro `{}`", prev.label)),
        }
    }
    fn register_routine(&mut self, routine: Routine) -> Result<(), String> {
        match self.routines.insert(routine.label.clone(), routine) {
            None => Ok(()),
            Some(prev) => Err(format!("Context Error: Duplicate routine `{}`", prev.label)),
        }
    }
}

#[derive(Clone)]
enum ByteCoIL {
    Assembled(ByteCo),
    Comment(String),
    RoutineDef(String),
    RoutineCallLocal(String),
    RoutineCallExported(String),
    RoutineAddressLocal(String),
    RoutineAddressExported(String),
    RoutineEnd,
    LabelDef(String),
    LabelRel(String),
    LabelAbs(String),
}
impl ByteCoIL {
    pub fn len(&self) -> usize {
        match self {
            Self::Assembled(byteco) => byteco.len(),
            Self::Comment(string) => string.len() + 1,
            Self::RoutineDef(..) => 0,
            Self::RoutineCallLocal(_) => 4,
            Self::RoutineCallExported(_) => 33,
            Self::RoutineAddressLocal(_) => 3,
            Self::RoutineAddressExported(_) => 33,
            Self::RoutineEnd => 1,
            Self::LabelDef(..) => 0,
            Self::LabelRel(_) => 2,
            Self::LabelAbs(_) => 3,
        }
    }
}
impl std::fmt::Display for ByteCoIL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assembled(byteco) => write!(f, "Assembled({:?})", byteco),
            Self::Comment(string) => write!(f, "Comment({})", string),
            Self::RoutineDef(name) => write!(f, "RoutineDef({})", name),
            Self::RoutineCallLocal(name) => write!(f, "RoutineCallLocal({})", name),
            Self::RoutineCallExported(name) => write!(f, "RoutineCallExported({})", name),
            Self::RoutineAddressLocal(name) => write!(f, "RoutineAddressLocal({})", name),
            Self::RoutineAddressExported(name) => write!(f, "RoutineAddressExported({})", name),
            Self::RoutineEnd => write!(f, "RoutineEnd"),
            Self::LabelDef(name) => write!(f, "LabelDef({})", name),
            Self::LabelRel(name) => write!(f, "LabelRel({})", name),
            Self::LabelAbs(name) => write!(f, "LabelAbs({})", name),
        }
    }
}
