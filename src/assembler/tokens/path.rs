use super::IMPORT_PATH_SEPARATOR;
use std::{fmt::Display, str::FromStr};

#[derive(Clone)]
pub struct Path {
    names: Vec<String>,
}
impl Path {
    pub fn push(&mut self, s: String) {
        self.names.push(s)
    }
}
impl FromStr for Path {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.strip_prefix(IMPORT_PATH_SEPARATOR) {
            Some(path) => {
                let names = path
                    .split(IMPORT_PATH_SEPARATOR)
                    .map(|s| s.into())
                    .collect();
                let path = Path { names };
                Ok(path)
            }
            None => Err("Not a path".into()),
        }
    }
}
impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, name) in self.names.iter().enumerate() {
            if i != 0 {
                write!(f, " .")?;
            }
            write!(f, " {}", name)?;
        }
        Ok(())
    }
}
