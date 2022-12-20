use crate::assembler::tokens::COMMENT_CLOSE;
use crate::assembler::tokens::{Rune, TextToken};
use core::str::Chars;
use std::iter::Peekable;

pub fn string_until(close_char: char, chars: &mut Peekable<Chars>) -> Result<String, String> {
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
    let mut chars = assembly_text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\n' => {
                handle_token(&mut buffer, &mut text_tokens)?;
                text_tokens.push(TextToken::NewLine);

                // look for indentation after a new line
                let mut num_tabs = 0;
                while let Some(c) = chars.peek() {
                    match c {
                        '\t' => {
                            num_tabs += 1;
                            chars.next();
                        }
                        _ => break,
                    }
                }
                if num_tabs > 0 {
                    text_tokens.push(TextToken::Tab(num_tabs));
                }
            }
            c if c.is_whitespace() => {
                handle_token(&mut buffer, &mut text_tokens)?;

                // if we just added a TextToken::Rune(Rune::OpenComment)
                if matches!(text_tokens.last(), Some(TextToken::Rune(Rune::OpenComment))) {
                    // pop the TextToken::Rune
                    text_tokens.pop();

                    // replace with parsed TextToken::Comment
                    let comment = string_until(COMMENT_CLOSE, &mut chars)?;
                    let comment = comment.trim().to_string();
                    text_tokens.push(TextToken::Comment(comment));
                }
            }
            c => buffer.push(c),
        }
    }

    Ok(text_tokens)
}
