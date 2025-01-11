mod eval;
mod lispies;
mod parser;
use crate::lispies::Lispies;

#[derive(Debug, PartialEq, Clone)]
pub struct Lisper {
    pub list: Vec<Lispies>,
}

impl Lisper {
    pub fn new() -> Self {
        Self { list: vec![] }
    }
}

impl Default for Lisper {
    fn default() -> Self {
        Self::new()
    }
}
