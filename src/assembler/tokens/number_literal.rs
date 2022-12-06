use std::fmt::Display;

#[derive(Clone)]
pub enum NumberLiteral {
    Byte(u8),
    Short(u16),
    Int(u32),
    Long(u64),
}
impl From<NumberLiteral> for Vec<u8> {
    fn from(lit: NumberLiteral) -> Self {
        match lit {
            NumberLiteral::Byte(num) => vec![176, num],
            NumberLiteral::Short(num) => {
                let mut vec = vec![177];
                for byte in num.to_le_bytes() {
                    vec.push(byte);
                }
                vec
            }
            NumberLiteral::Int(num) => {
                let mut vec = vec![178];
                for byte in num.to_le_bytes() {
                    vec.push(byte);
                }
                vec
            }
            NumberLiteral::Long(num) => {
                let mut vec = vec![179];
                for byte in num.to_le_bytes() {
                    vec.push(byte);
                }
                vec
            }
        }
    }
}
impl Display for NumberLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberLiteral::Byte(num) => write!(f, "{} u8", num),
            NumberLiteral::Short(num) => write!(f, "{} u16", num),
            NumberLiteral::Int(num) => write!(f, "{} u32", num),
            NumberLiteral::Long(num) => write!(f, "{} u64", num),
        }
    }
}
