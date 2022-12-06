use crate::assembler::tokens::{Label, Marker};
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Clone)]
pub struct Command {
    pub marker: Marker,
    pub label: Label,
}
impl FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Command, String> {
        let mut chars = s.chars();
        let marker = chars.next().ok_or("label empty")?.to_string().parse()?;
        let label = chars.collect::<String>().parse()?;

        Ok(Command { marker, label })
    }
}
impl Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:?}", self.marker, self.label)
    }
}
