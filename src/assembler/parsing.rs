use crate::assembler::tokens::COMMENT_CLOSE;
use crate::assembler::tokens::{Rune, TextToken};
use core::str::Chars;

pub fn string_until(close_char: char, chars: &mut Chars) -> Result<String, String> {
    let mut found = false;
    let mut string = String::new();
    while let Some(c) = chars.next() {
        match c {
            c if c == close_char => {
                found = true;
                break;
            }
            _ => string.push(c),
        }
    }
    match found {
        true => Ok(string),
        false => Err(format!(
            "string_until: Didn't find close_char `{}`",
            close_char
        )),
    }
}
pub fn parse_text(assembly_text: &str) -> Result<Vec<TextToken>, String> {
    fn handle_token(buffer: &mut String, text_tokens: &mut Vec<TextToken>) -> Result<(), String> {
        if !buffer.is_empty() {
            let string = buffer.clone();
            let token = string.parse()?;
            text_tokens.push(token);
            buffer.clear();
        }
        Ok(())
    }

    let mut text_tokens = Vec::new();
    let mut buffer = String::new();
    let mut chars = assembly_text.chars();
    while let Some(c) = chars.next() {
        match c {
            '\n' => {
                handle_token(&mut buffer, &mut text_tokens)?;
                text_tokens.push(TextToken::NewLine);
            }
            '\t' => {
                handle_token(&mut buffer, &mut text_tokens)?;
                let mut num_tabs = 1;
                loop {
                    match chars.next() {
                        Some(c) => match c {
                            '\t' => num_tabs += 1,
                            c => {
                                buffer.push(c);
                                break;
                            }
                        },
                        None => break,
                    }
                }

                text_tokens.push(TextToken::Tab(num_tabs));
            }
            c if c.is_whitespace() => {
                handle_token(&mut buffer, &mut text_tokens)?;
                if let Some(last) = text_tokens.last() {
                    if let TextToken::Rune(rune) = last {
                        match rune {
                            &Rune::OpenComment => {
                                text_tokens.pop();
                                let comment = string_until(COMMENT_CLOSE, &mut chars)?;
                                let comment = comment.trim().to_string();
                                text_tokens.push(TextToken::Comment(comment));
                            }
                            _ => {}
                        }
                    }
                }
            }
            c => buffer.push(c),
        }
    }

    Ok(text_tokens)
}
