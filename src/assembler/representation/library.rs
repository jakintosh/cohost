use std::collections::HashMap;

use super::{ByteCo, Hash};

pub struct Library {
    pub macros: HashMap<String, ByteCo>,
    pub routine_names: HashMap<String, Hash>,
    pub routines: HashMap<Hash, ByteCo>,
}
impl Library {
    pub fn new() -> Library {
        Library {
            macros: HashMap::new(),
            routine_names: HashMap::new(),
            routines: HashMap::new(),
        }
    }
}
