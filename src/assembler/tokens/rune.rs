use super::{
    COMMENT_CLOSE, COMMENT_OPEN, DEFINITION_CLOSE, EXPORTED_ROUTINE_DEF, IMPORT_DEF, MACRO_DEF,
    MACRO_PARAM_CLOSE, MACRO_PARAM_OPEN, ROUTINE_DEF,
};
use std::{fmt::Display, str::FromStr};

pub enum Rune {
    OpenComment,
    CloseComment,
    OpenImport,
    OpenRoutine,
    OpenExportedRoutine,
    OpenMacro,
    CloseDefinition,
    OpenParameters,
    CloseParameters,
}
impl FromStr for Rune {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<_> = s.chars().collect();
        if chars.len() != 1 {
            return Err(format!(
                "Given string '{}' is longer than one character; not a Rune",
                s
            ));
        }

        match chars[0] {
            COMMENT_OPEN => Ok(Self::OpenComment),
            COMMENT_CLOSE => Ok(Self::CloseComment),
            IMPORT_DEF => Ok(Self::OpenImport),
            ROUTINE_DEF => Ok(Self::OpenRoutine),
            EXPORTED_ROUTINE_DEF => Ok(Self::OpenExportedRoutine),
            MACRO_DEF => Ok(Self::OpenMacro),
            DEFINITION_CLOSE => Ok(Self::CloseDefinition),
            MACRO_PARAM_OPEN => Ok(Self::OpenParameters),
            MACRO_PARAM_CLOSE => Ok(Self::CloseParameters),
            _ => Err(format!("'{}' is not a recognized rune", s)),
        }
    }
}
impl Display for Rune {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rune::OpenComment => write!(f, "OpenComment"),
            Rune::CloseComment => write!(f, "CloseComment"),
            Rune::OpenImport => write!(f, "OpenImport"),
            Rune::OpenRoutine => write!(f, "OpenRoutine"),
            Rune::OpenExportedRoutine => write!(f, "OpenExportedRoutine"),
            Rune::OpenMacro => write!(f, "OpenMacro"),
            Rune::CloseDefinition => write!(f, "CloseRoutine"),
            Rune::OpenParameters => write!(f, "OpenParameters"),
            Rune::CloseParameters => write!(f, "CloseParameters"),
        }
    }
}
