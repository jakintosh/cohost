use crate::assembler::representation::{Component, Literal, Module, RawToken, Token};
use core::str::Chars;

const COMMENT_OPEN: char = '(';
const COMMENT_CLOSE: char = ')';
const ROUTINE_DEF: char = ':';
const EXPORTED_ROUTINE_DEF: char = '^';
const ROUTINE_CLOSE: char = ';';
const ROUTINE_CALL: char = '>';
const EXPORTED_ROUTINE_CALL: char = '<';
const ROUTINE_ADDRESS: char = '$';
const EXPORTED_ROUTINE_ADDRESS: char = '@';
const MACRO_DEF: char = '%';
const MACRO_PARAM_OPEN: char = '[';
const MACRO_PARAM_CLOSE: char = ']';
const MACRO_PARAM_USE_OPEN: char = '{';
const MACRO_PARAM_USE_CLOSE: char = '}';
const MACRO_PARAM_SEPARATE: char = ',';
const MACRO_USE: char = '~';
const LABEL_DEF: char = '#';
const LABEL_ADDR_ABS: char = '*';
const LABEL_ADDR_REL: char = '&';
// const unused: char = '|';
// const unused: char = '\\';
// const unused: char = '/';
// const unused: char = '=';
// const unused: char = '+';
// const unused: char = '-';

fn string_until(close_char: char, chars: &mut Chars) -> Result<String, String> {
    let mut string = String::new();
    while let Some(c) = chars.next() {
        match c {
            c if c == close_char => break,
            _ => string.push(c),
        }
    }
    Ok(string)
}
fn raw_tokens_until(close_char: char, chars: &mut Chars) -> Result<Vec<RawToken>, String> {
    fn close_token(buffer: &mut String, strings: &mut Vec<RawToken>) {
        if !buffer.is_empty() {
            let string = buffer.clone();
            strings.push(RawToken::Token { string });
            buffer.clear();
        }
    }

    let mut buffer = String::new();
    let mut raw_tokens: Vec<RawToken> = Vec::new();
    while let Some(c) = chars.next() {
        match c {
            c if c == close_char => break,
            c if c == COMMENT_OPEN => {
                close_token(&mut buffer, &mut raw_tokens);
                let string = string_until(COMMENT_CLOSE, chars)?.trim().into();
                raw_tokens.push(RawToken::Comment { string });
            }
            c if c.is_whitespace() => {
                close_token(&mut buffer, &mut raw_tokens);
            }
            _ => buffer.push(c),
        }
    }
    Ok(raw_tokens)
}
fn parse_token(raw_token: RawToken) -> Result<Token, String> {
    let string = match raw_token {
        RawToken::Token { string } => string,
        RawToken::Comment { string } => return Ok(Token::Comment { string }),
    };

    let mut chars = string.chars();
    match chars.next() {
        Some(c) => {
            let token = match c {
                ROUTINE_CALL => Token::RoutineCallLocal {
                    id: chars.collect(),
                },
                EXPORTED_ROUTINE_CALL => Token::RoutineCallExported {
                    id: chars.collect(),
                },
                ROUTINE_ADDRESS => Token::RoutineAddressLocal {
                    id: chars.collect(),
                },
                EXPORTED_ROUTINE_ADDRESS => Token::RoutineAddressExported {
                    id: chars.collect(),
                },
                MACRO_USE => Token::MacroUse {
                    id: chars.collect(),
                },
                LABEL_DEF => Token::LabelDef {
                    id: chars.collect(),
                },
                LABEL_ADDR_REL => Token::LabelAddressRelative {
                    id: chars.collect(),
                },
                LABEL_ADDR_ABS => Token::LabelAddressAbsolute {
                    id: chars.collect(),
                },
                c => {
                    let mut string = String::from(c);
                    string.push_str(&chars.collect::<String>());
                    match crate::core::str_to_opcode(&string) {
                        Some(code) => Token::Instruction { code },
                        None => Token::Comment { string },
                    }
                }
            };
            Ok(token)
        }
        None => Err("Empty token".into()),
    }
}

pub fn from_text(assembly_text: &str) -> Result<Module, String> {
    let mut components = Vec::new();
    let mut chars = assembly_text.chars();
    while let Some(c) = chars.next() {
        match c {
            COMMENT_OPEN => {
                let string = string_until(COMMENT_CLOSE, &mut chars)?.trim().into();
                components.push(Component::Comment { string });
            }
            MACRO_DEF | EXPORTED_ROUTINE_DEF | ROUTINE_DEF => {
                let mut raw_tokens = raw_tokens_until(ROUTINE_CLOSE, &mut chars)?.into_iter();
                let Some(RawToken::Token{ string: label}) = raw_tokens.next() else {
                    let name = if c == MACRO_DEF { "Macro" } else { "Routine" };
                    return Err(format!("Missing {} Label", name));
                };
                let mut tokens = Vec::new();
                while let Some(raw_token) = raw_tokens.next() {
                    let token = match parse_token(raw_token)? {
                        // if lit token
                        Token::Instruction { code } if code >= 176 && code < 180 => {
                            // get next non-comment token
                            let next_token_string = loop {
                                match raw_tokens.next() {
                                    Some(RawToken::Comment { string }) => {
                                        tokens.push(Token::Comment { string })
                                    }
                                    Some(RawToken::Token { string }) => break string,
                                    None => return Err("LIT not followed by another token".into()),
                                }
                            };

                            // parse num
                            let Some(num) = parse_number(&next_token_string) else {
                                return Err("Couldn't parse number from token after LIT".into());
                            };
                            let literal = match code {
                                176 => Literal::Byte(num as u8),
                                177 => Literal::Short(num as u16),
                                178 => Literal::Int(num as u32),
                                179 => Literal::Long(num),
                                _ => unreachable!(),
                            };
                            Token::Literal { literal }
                        }
                        token => token,
                    };
                    tokens.push(token);
                }
                match c {
                    MACRO_DEF => components.push(Component::Macro { label, tokens }),
                    EXPORTED_ROUTINE_DEF => components.push(Component::Routine {
                        export: true,
                        label,
                        tokens,
                    }),
                    ROUTINE_DEF => components.push(Component::Routine {
                        export: false,
                        label,
                        tokens,
                    }),
                    _ => unreachable!(),
                }
            }
            c if c.is_whitespace() => continue,
            c => println!("found character {} outside component definition", c),
        }
    }

    Ok(Module { components })
}

fn parse_number(string: &str) -> Option<u64> {
    let (token, radix) = match string.strip_prefix("0x") {
        Some(hex) => (hex, 16),
        None => (string, 10),
    };
    match u64::from_str_radix(token, radix) {
        Ok(lit) => Some(lit),
        Err(_) => None,
    }
}
