use super::{Label, NumberLiteral};
use std::fmt::Display;

#[derive(Clone)]
pub enum SourceToken {
    Comment { string: String },
    Instruction { opcode: u8 },
    NumberLiteral { literal: NumberLiteral },
    ParameterDef { name: String },
    ParameterUse { label: Label },
    RoutineCallLocal { label: Label },
    RoutineCallExported { label: Label },
    RoutineAddressLocal { label: Label },
    RoutineAddressExported { label: Label },
    MacroUse { label: Label },
    AnchorDef { label: Label },
    AnchorAddressRelative { label: Label },
    AnchorAddressAbsolute { label: Label },
}
impl Display for SourceToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SourceToken::Comment { string } => format!("Comment( {} )", string),
            SourceToken::Instruction { opcode } => {
                format!("Instruction( {} )", crate::core::opcode_to_str(*opcode))
            }
            SourceToken::NumberLiteral { literal } => format!("Literal( {} )", literal),
            SourceToken::ParameterDef { name } => format!("Define Parameter( {} )", name),
            SourceToken::ParameterUse { label } => format!("Pass Parameter( {:?} )", label),
            SourceToken::RoutineCallLocal { label } => format!("Local Call `{:?}`", label),
            SourceToken::RoutineCallExported { label } => format!("Exported Call `{:?}`", label),
            SourceToken::RoutineAddressLocal { label } => {
                format!("Local Routine Address `{:?}`", label)
            }
            SourceToken::RoutineAddressExported { label } => {
                format!("Exported Routine Address `{:?}`", label)
            }
            SourceToken::MacroUse { label } => format!("Insert Macro `{:?}`", label),
            SourceToken::AnchorDef { label } => format!("Define Anchor `{:?}`", label),
            SourceToken::AnchorAddressRelative { label } => {
                format!("Anchor Address Relative `{:?}`", label)
            }
            SourceToken::AnchorAddressAbsolute { label } => {
                format!("Anchor Address Absolute `{:?}`", label)
            }
        };
        write!(f, "{}", s)
    }
}
