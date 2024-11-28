use std::hash::Hash;
use std::borrow::Borrow;
use crate::lexical_analyzer::TokenPosition;

use super::*;


#[derive(Debug)]
pub struct Action<'a> {
    pub name: &'a str,
    pub name_pos: TokenPosition,
    pub parameters: Vec<Symbol<'a>>,
    pub preconditions: Option<Formula<'a>>,
    pub effects: Option<Formula<'a>>
}

impl <'a> Hash for Action<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl <'a> PartialEq for Action<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name)
    }
}

impl <'a> Eq for Action<'a> {}

impl <'a> Borrow<&'a str> for &Action<'a> {
    fn borrow(&self) -> &&'a str {
        &self.name
    }
}