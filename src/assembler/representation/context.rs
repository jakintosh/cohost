use crate::assembler::representation::{ByteCo, ByteCoIL, Library, Macro, Module, Routine};
use crate::assembler::tokens::SourceToken;
use std::collections::HashMap;

pub struct Context<'a> {
    macros: HashMap<String, Macro>,
    routines: HashMap<String, Routine>,
    assembled_tokens: HashMap<String, Vec<ByteCoIL>>,
    library: &'a Library,
}
impl<'a> Context<'a> {
    pub fn new(library: &'a Library, module: Module) -> Result<Context<'a>, String> {
        let mut context = Context {
            macros: HashMap::new(),
            routines: HashMap::new(),
            assembled_tokens: HashMap::new(),
            library,
        };

        for m in module.macros {
            context.register_macro(m)?;
        }
        for r in module.routines {
            context.register_routine(r)?;
        }

        Ok(context)
    }
    pub fn export(self) -> Result<ByteCo, String> {
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
        token: SourceToken,
        assembled_tokens: &mut HashMap<String, Vec<ByteCoIL>>,
        macros: &HashMap<String, Macro>,
        library: &'a Library,
    ) -> Result<Vec<ByteCoIL>, String> {
        let mut vec = Vec::new();
        // match token {
        //     SourceToken::Comment { string } => vec.push(ByteCoIL::Comment(string)),
        //     SourceToken::NumberLiteral { literal } => vec.push(ByteCoIL::Assembled(literal.into())),
        //     SourceToken::Instruction { opcode } => vec.push(ByteCoIL::Assembled(vec![opcode])),
        //     SourceToken::ParameterDef { name } =>
        //     SourceToken::RoutineCallLocal { label } => vec.push(ByteCoIL::RoutineCallLocal(label)),
        //     SourceToken::RoutineCallExported { label } => {
        //         vec.push(ByteCoIL::RoutineCallExported(label))
        //     }
        //     SourceToken::RoutineAddressLocal { label } => {
        //         vec.push(ByteCoIL::RoutineAddressLocal(label))
        //     }
        //     SourceToken::RoutineAddressExported { label } => {
        //         vec.push(ByteCoIL::RoutineAddressExported(label))
        //     }
        //     SourceToken::AnchorDef { label } => vec.push(ByteCoIL::AnchorDef(label)),
        //     SourceToken::AnchorAddressRelative { label } => vec.push(ByteCoIL::AnchorRel(label)),
        //     SourceToken::AnchorAddressAbsolute { label } => vec.push(ByteCoIL::AnchorAbs(label)),
        //     SourceToken::MacroUse { label } => vec.append(&mut Context::pre_assemble_macro(
        //         label,
        //         assembled_tokens,
        //         macros,
        //         library,
        //     )?),
        // };

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
        match self.macros.insert(mac.name.clone(), mac) {
            None => Ok(()),
            Some(prev) => Err(format!("Context Error: Duplicate macro `{}`", prev.name)),
        }
    }
    fn register_routine(&mut self, routine: Routine) -> Result<(), String> {
        match self.routines.insert(routine.name.clone(), routine) {
            None => Ok(()),
            Some(prev) => Err(format!("Context Error: Duplicate routine `{}`", prev.name)),
        }
    }
}
