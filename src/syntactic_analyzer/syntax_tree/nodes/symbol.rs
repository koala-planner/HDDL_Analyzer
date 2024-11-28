use std::{borrow::Borrow, hash::Hash};

use crate::TokenPosition;

#[derive(Clone, Debug)]
pub struct Symbol<'a> {
    pub name: &'a str,
    pub name_pos: TokenPosition,
    pub symbol_type: Option<&'a str>,
    pub type_pos: Option<TokenPosition>
}

impl <'a> Symbol<'a> {
    pub fn new(name: &'a str, name_pos: TokenPosition, symbol_type: Option<&'a str>, type_pos: Option<TokenPosition>) -> Symbol<'a> {
        Symbol {
            name,
            name_pos,
            symbol_type,
            type_pos
        }
    }
}

impl <'a> Eq for Symbol<'a> {}

impl <'a> PartialEq for Symbol<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name)
    }
}

impl <'a> Hash for Symbol<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl <'a> Borrow<&'a str> for &Symbol<'a> {
    fn borrow(&self) -> &&'a str {
        &self.name
    }
}