mod command;
mod import;
mod label;
mod number_literal;
mod rune;
mod source_token;
mod text_tokens;

pub use command::Command;
pub use import::Import;
pub use label::Label;
pub use label::Marker;
pub use number_literal::NumberLiteral;
pub use rune::Rune;
pub use source_token::SourceToken;
pub use text_tokens::TextToken;

pub const COMMENT_OPEN: char = '(';
pub const COMMENT_CLOSE: char = ')';
pub const IMPORT_DEF: char = '+';
pub const ROUTINE_DEF: char = ':';
pub const EXPORTED_ROUTINE_DEF: char = '^';
pub const DEFINITION_CLOSE: char = ';';
pub const IMPORT_PATH_SEPARATOR: char = '.';
pub const IMPORT_NAME_ASSIGNMENT: char = '=';
pub const ROUTINE_CALL: char = '>';
pub const EXPORTED_ROUTINE_CALL: char = '<';
pub const ROUTINE_ADDRESS: char = '$';
pub const EXPORTED_ROUTINE_ADDRESS: char = '@';
pub const MACRO_DEF: char = '%';
pub const MACRO_PARAM_OPEN: char = '[';
pub const MACRO_PARAM_CLOSE: char = ']';
pub const MACRO_PARAM_USE_OPEN: char = '{';
pub const MACRO_PARAM_USE_CLOSE: char = '}';
pub const MACRO_PARAM: char = '\'';
pub const MACRO_USE: char = '~';
pub const ANCHOR_DEF: char = '#';
pub const ANCHOR_ADDR_ABS: char = '*';
pub const ANCHOR_ADDR_REL: char = '&';

pub fn validate_string(s: &str) -> Result<(), String> {
    const RESERVED_CHARS: [char; 22] = [
        COMMENT_OPEN,
        COMMENT_CLOSE,
        IMPORT_DEF,
        ROUTINE_DEF,
        EXPORTED_ROUTINE_DEF,
        DEFINITION_CLOSE,
        IMPORT_PATH_SEPARATOR,
        IMPORT_NAME_ASSIGNMENT,
        ROUTINE_CALL,
        EXPORTED_ROUTINE_CALL,
        ROUTINE_ADDRESS,
        EXPORTED_ROUTINE_ADDRESS,
        MACRO_DEF,
        MACRO_PARAM_OPEN,
        MACRO_PARAM_CLOSE,
        MACRO_PARAM_USE_OPEN,
        MACRO_PARAM_USE_CLOSE,
        MACRO_PARAM,
        MACRO_USE,
        ANCHOR_DEF,
        ANCHOR_ADDR_ABS,
        ANCHOR_ADDR_REL,
    ];
    for c in RESERVED_CHARS {
        if s.contains(c) {
            return Err(format!("'{}' contains reserved character '{}'", s, c));
        }
    }
    Ok(())
}
