use std::fmt::Display;

use crate::core::Instruction;

pub enum Component {
    Routine {
        export: bool,
        label: String,
        tokens: Vec<String>,
    },
    Macro {
        label: String,
        tokens: Vec<String>,
    },
    Comment {
        tokens: Vec<String>,
    },
}
impl Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Component::Routine {
                export,
                label,
                tokens,
            } => {
                let exported: String = match export {
                    true => "Exported ".into(),
                    false => "".into(),
                };
                format!(
                    "{}Routine: label(\"{}\") tokens{:?}",
                    exported, label, tokens
                )
            }
            Component::Macro { label, tokens } => {
                format!("Macro: label(\"{}\") tokens{:?}", label, tokens)
            }
            Component::Comment { tokens } => format!("Comment: {:?}", tokens),
        };
        write!(f, "{}", s)
    }
}

pub struct Module {
    pub components: Vec<Component>,
}
impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        for c in &self.components {
            buffer.push_str(&format!("{}\n", c));
        }
        write!(f, "{}", buffer)
    }
}
