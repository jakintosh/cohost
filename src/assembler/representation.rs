use std::fmt::Display;

pub enum RawToken {
    Token { string: String },
    Comment { string: String },
}

#[derive(Clone)]
pub enum Literal {
    Byte(u8),
    Short(u16),
    Int(u32),
    Long(u64),
}

#[derive(Clone)]
pub enum Token {
    Comment { string: String },
    Literal { literal: Literal },
    Instruction { code: u8 },
    RoutineCallLocal { id: String },
    RoutineCallExported { id: String },
    RoutineAddressLocal { id: String },
    RoutineAddressExported { id: String },
    MacroUse { id: String },
    LabelDef { id: String },
    LabelAddressRelative { id: String },
    LabelAddressAbsolute { id: String },
}

pub enum Component {
    Routine {
        export: bool,
        label: String,
        tokens: Vec<Token>,
    },
    Macro {
        label: String,
        tokens: Vec<Token>,
    },
    Comment {
        string: String,
    },
}

pub struct Module {
    pub components: Vec<Component>,
}

pub struct Macro {
    pub label: String,
    pub tokens: Vec<Token>,
}
pub struct Routine {
    pub label: String,
    pub export: bool,
    pub tokens: Vec<Token>,
}

impl From<Literal> for Vec<u8> {
    fn from(lit: Literal) -> Self {
        match lit {
            Literal::Byte(num) => vec![176, num],
            Literal::Short(num) => {
                let mut vec = vec![177];
                for byte in num.to_le_bytes() {
                    vec.push(byte);
                }
                vec
            }
            Literal::Int(num) => {
                let mut vec = vec![178];
                for byte in num.to_le_bytes() {
                    vec.push(byte);
                }
                vec
            }
            Literal::Long(num) => {
                let mut vec = vec![179];
                for byte in num.to_le_bytes() {
                    vec.push(byte);
                }
                vec
            }
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Byte(num) => write!(f, "{} u8", num),
            Literal::Short(num) => write!(f, "{} u16", num),
            Literal::Int(num) => write!(f, "{} u32", num),
            Literal::Long(num) => write!(f, "{} u64", num),
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Token::Comment { string } => format!("Comment ( {} )", string),
            Token::Literal { literal } => format!("Literal( {} )", literal),
            Token::Instruction { code } => {
                format!("Instruction( {} )", crate::core::opcode_to_str(*code))
            }
            Token::RoutineCallLocal { id } => format!("Local Call `{}`", id),
            Token::RoutineCallExported { id } => format!("Exported Call `{}`", id),
            Token::RoutineAddressLocal { id } => format!("Local Routine Address `{}`", id),
            Token::RoutineAddressExported { id } => format!("Exported Routine Address `{}`", id),
            Token::MacroUse { id } => format!("Insert Macro `{}`", id),
            Token::LabelDef { id } => format!("Define Label `{}`", id),
            Token::LabelAddressRelative { id } => format!("Label Address Relative `{}`", id),
            Token::LabelAddressAbsolute { id } => format!("Label Address Absolute `{}`", id),
        };
        write!(f, "{}", s)
    }
}
impl Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let toks;
        match self {
            Component::Routine {
                export,
                label,
                tokens,
            } => {
                let exported: String = match export {
                    true => "Exported ".into(),
                    false => "".into(),
                };
                write!(f, "{}Routine: \"{}\"", exported, label)?;
                toks = tokens;
            }
            Component::Macro { label, tokens } => {
                write!(f, "Macro: \"{}\"", label)?;
                toks = tokens;
            }
            Component::Comment { string } => {
                return write!(f, "Comment ( {} )", string);
            }
        };

        for token in toks {
            write!(f, "\n - {}", token)?;
        }

        Ok(())
    }
}
impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        for c in &self.components {
            buffer.push_str(&format!("{}\n\n", c));
        }

        write!(f, "{}", buffer.trim_end())
    }
}
