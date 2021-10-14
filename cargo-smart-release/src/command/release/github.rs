#![allow(dead_code)]
use crate::utils::Program;

pub struct Support {
    gh: Program,
}

impl Default for Support {
    fn default() -> Self {
        Self::new()
    }
}

impl Support {
    pub fn new() -> Self {
        Support { gh: Program::new("gh") }
    }
}
