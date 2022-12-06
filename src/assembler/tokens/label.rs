use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    str::FromStr,
};

use crate::assembler::tokens::validate_string;

use super::{
    ANCHOR_ADDR_ABS, ANCHOR_ADDR_REL, ANCHOR_DEF, EXPORTED_ROUTINE_ADDRESS, EXPORTED_ROUTINE_CALL,
    MACRO_PARAM, MACRO_PARAM_USE_CLOSE, MACRO_PARAM_USE_OPEN, MACRO_USE, ROUTINE_ADDRESS,
    ROUTINE_CALL,
};

#[derive(Clone)]
pub enum Marker {
    CallRoutine,
    CallExportedRoutine,
    RoutineAddress,
    ExportedRoutineAddress,
    Parameter,
    UseMacro,
    DefineAnchor,
    AnchorAddress,
    AnchorRelativeAddress,
}
impl FromStr for Marker {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<_> = s.chars().collect();
        if chars.len() != 1 {
            return Err(format!(
                "Given string '{}' is longer than one character; not a Marker",
                s
            ));
        }

        match chars[0] {
            ROUTINE_CALL => Ok(Self::CallRoutine),
            EXPORTED_ROUTINE_CALL => Ok(Self::CallExportedRoutine),
            ROUTINE_ADDRESS => Ok(Self::RoutineAddress),
            EXPORTED_ROUTINE_ADDRESS => Ok(Self::ExportedRoutineAddress),
            MACRO_PARAM => Ok(Self::Parameter),
            MACRO_USE => Ok(Self::UseMacro),
            ANCHOR_DEF => Ok(Self::DefineAnchor),
            ANCHOR_ADDR_ABS => Ok(Self::AnchorAddress),
            ANCHOR_ADDR_REL => Ok(Self::AnchorRelativeAddress),
            _ => Err(format!("'{}' is not a recognized marker", s)),
        }
    }
}
impl Display for Marker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Marker::CallRoutine => String::from("Routine Call:"),
            Marker::CallExportedRoutine => String::from("Exported Routine Call:"),
            Marker::RoutineAddress => String::from("Routine Address:"),
            Marker::ExportedRoutineAddress => String::from("Exported Routine Address:"),
            Marker::Parameter => String::from("Parameter:"),
            Marker::UseMacro => String::from("Use Macro:"),
            Marker::DefineAnchor => String::from("Anchor:"),
            Marker::AnchorAddress => String::from("Anchor Address:"),
            Marker::AnchorRelativeAddress => String::from("Relative Anchor Address:"),
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone)]
enum Component {
    Literal(String),
    Parameter(String),
}

#[derive(Clone)]
pub struct Label {
    components: Vec<Component>,
}
impl Label {
    pub fn to_string(&self, parameters: &HashMap<String, String>) -> Result<String, String> {
        let mut buffer = String::new();
        for component in &self.components {
            match component {
                Component::Literal(string) => buffer.push_str(&string),
                Component::Parameter(param) => match parameters.get(param) {
                    Some(value) => buffer.push_str(value),
                    None => return Err(format!("Label contains undefined parameter `{}`", param)),
                },
            }
        }
        Ok(buffer)
    }
}
impl FromStr for Label {
    type Err = String;
    fn from_str(s: &str) -> Result<Label, String> {
        fn validate(buffer: &mut String) -> Result<String, String> {
            let string = buffer.clone();
            if let Err(e) = validate_string(&string) {
                return Err(format!("Invalid Label Component: {}", e));
            }
            buffer.clear();
            Ok(string)
        }
        fn push_literal(buffer: &mut String, tokens: &mut Vec<Component>) -> Result<(), String> {
            let string = validate(buffer)?;
            let literal = Component::Literal(string);
            tokens.push(literal);
            Ok(())
        }
        fn push_parameter(buffer: &mut String, tokens: &mut Vec<Component>) -> Result<(), String> {
            let string = validate(buffer)?;
            let literal = Component::Parameter(string);
            tokens.push(literal);
            Ok(())
        }

        let mut chars = s.chars();
        let mut buffer = String::new();
        let mut tokens = Vec::new();
        while let Some(c) = chars.next() {
            match c {
                MACRO_PARAM_USE_OPEN => {
                    // start reading in a parameter key
                    let mut param_buffer = String::new();
                    while let Some(ch) = chars.next() {
                        match ch {
                            MACRO_PARAM_USE_CLOSE => {
                                push_literal(&mut buffer, &mut tokens)?;
                                push_parameter(&mut param_buffer, &mut tokens)?;
                                break;
                            }
                            _ => param_buffer.push(ch),
                        }
                    }
                    if !param_buffer.is_empty() {
                        buffer.push_str(&param_buffer);
                        push_literal(&mut buffer, &mut tokens)?;
                    }
                }
                _ => buffer.push(c),
            }
        }
        if !buffer.is_empty() {
            push_literal(&mut buffer, &mut tokens)?;
        }

        Ok(Label { components: tokens })
    }
}
impl Debug for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.components {
            match token {
                Component::Literal(string) => write!(f, "{}", string)?,
                Component::Parameter(param) => write!(f, "{{{}}}", param)?,
            }
        }
        Ok(())
    }
}
