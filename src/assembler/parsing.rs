use crate::assembler::representation::Component;
use crate::assembler::representation::Module;

const COMMENT_OPEN: char = '(';
const COMMENT_CLOSE: char = ')';
const ROUTINE_DEF: char = ':';
const EXPORTED_ROUTINE_DEF: char = '^';
const MACRO_DEF: char = '%';
const ROUTINE_CLOSE: char = ';';
const ROUTINE_CALL: char = '>';
const EXPORTED_ROUTINE_CALL: char = '<';
const MACRO_USE: char = '~';
const ROUTINE_ADDRESS: char = '$';
const EXPORTED_ROUTINE_ADDRESS: char = '@';
const LABEL_DEF: char = '#';
const LABEL_ADDR_ABS: char = '*';
const LABEL_ADDR_REL: char = '&';
// const unused: char = '|';
// const unused: char = '\\';
// const unused: char = '/';
// const unused: char = '=';
// const unused: char = '+';
// const unused: char = '-';
// const unused: char = '[';
// const unused: char = ']';

pub fn from_text(assembly_text: &str) -> Module {
    fn tokens_until(close: char, chars: &mut core::str::Chars) -> Vec<String> {
        let mut buffer = String::new();
        let mut tokens: Vec<String> = Vec::new();
        while let Some(c) = chars.next() {
            match c {
                c if c == close => break,
                c if c.is_whitespace() => {
                    if !buffer.is_empty() {
                        tokens.push(buffer.clone());
                        buffer.clear();
                    }
                }
                _ => buffer.push(c),
            }
        }
        tokens
    }

    let mut components = Vec::new();
    let mut chars = assembly_text.chars();
    while let Some(c) = chars.next() {
        match c {
            COMMENT_OPEN => {
                let tokens = tokens_until(COMMENT_CLOSE, &mut chars);
                components.push(Component::Comment { tokens });
            }
            MACRO_DEF => {
                let tokens = tokens_until(ROUTINE_CLOSE, &mut chars);
                let (label, tokens) = (tokens[0].clone(), Vec::from(&tokens[1..]));
                components.push(Component::Macro { label, tokens });
            }
            ROUTINE_DEF => {
                let tokens = tokens_until(ROUTINE_CLOSE, &mut chars);
                let (label, tokens) = (tokens[0].clone(), Vec::from(&tokens[1..]));
                components.push(Component::Routine {
                    export: false,
                    label,
                    tokens,
                });
            }
            EXPORTED_ROUTINE_DEF => {
                let tokens = tokens_until(ROUTINE_CLOSE, &mut chars);
                let (label, tokens) = (tokens[0].clone(), Vec::from(&tokens[1..]));
                components.push(Component::Routine {
                    export: true,
                    label,
                    tokens,
                });
            }
            c if c.is_whitespace() => {}
            _ => {}
        }
    }

    Module { components }
}

pub fn binary_from_text(assembly_text: &str) -> Vec<u8> {
    let tokens: Vec<_> = assembly_text.split_whitespace().collect();
    let mut binary: Vec<u8> = Vec::with_capacity(tokens.len());

    let mut i = 0;
    while i < tokens.len() {
        let token = tokens[i];
        println!("reading token: {}", token);
        let Some(opcode) = crate::core::str_to_opcode(token) else {
            panic!("received invalid token {}", token);
        };
        binary.push(opcode);

        // uh, check for literals i guess?
        if opcode >= 176 && opcode < 180 {
            i += 1;
            let lit_token = tokens[i];
            println!("reading token: {}", lit_token);
            match opcode {
                176 => {
                    let Some(lit) = parse_lit8(lit_token) else {
                        panic!("couldn't parse lit: {}", token)
                    };
                    binary.push(lit);
                }
                177 => {
                    let Some(lit) = parse_lit16(lit_token) else {
                        panic!("couldn't parse lit: {}", token)
                    };
                    binary.extend_from_slice(&lit.to_le_bytes());
                }
                178 => {
                    let Some(lit) = parse_lit32(lit_token) else {
                        panic!("couldn't parse lit: {}", token)
                    };
                    binary.extend_from_slice(&lit.to_le_bytes());
                }
                179 => {
                    let Some(lit) = parse_lit64(lit_token) else {
                        panic!("couldn't parse lit: {}", token)
                    };
                    binary.extend_from_slice(&lit.to_le_bytes());
                }
                _ => {} // do nothing
            }
        }

        i += 1;
    }

    binary
}

pub fn parse_lit8(token: &str) -> Option<u8> {
    let (token, radix) = match token.strip_prefix("0x") {
        Some(hex) => (hex, 16),
        None => (token, 10),
    };
    match u8::from_str_radix(token, radix) {
        Ok(lit) => Some(lit),
        Err(_) => None,
    }
}
pub fn parse_lit16(token: &str) -> Option<u16> {
    let (token, radix) = match token.strip_prefix("0x") {
        Some(hex) => (hex, 16),
        None => (token, 10),
    };
    match u16::from_str_radix(token, radix) {
        Ok(lit) => Some(lit),
        Err(_) => None,
    }
}
pub fn parse_lit32(token: &str) -> Option<u32> {
    let (token, radix) = match token.strip_prefix("0x") {
        Some(hex) => (hex, 16),
        None => (token, 10),
    };
    match u32::from_str_radix(token, radix) {
        Ok(lit) => Some(lit),
        Err(_) => None,
    }
}
pub fn parse_lit64(token: &str) -> Option<u64> {
    let (token, radix) = match token.strip_prefix("0x") {
        Some(hex) => (hex, 16),
        None => (token, 10),
    };
    match u64::from_str_radix(token, radix) {
        Ok(lit) => Some(lit),
        Err(_) => None,
    }
}
