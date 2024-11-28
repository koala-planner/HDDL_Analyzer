use std::{borrow::Borrow, hash::Hash};

use crate::TokenPosition;

use super::*;

#[derive(Debug)]
pub struct Task<'a> {
    pub name: &'a str,
    pub name_pos: TokenPosition,
    pub parameters: Vec<Symbol<'a>>
}

impl <'a> Task <'a> {
    pub fn new(name: &'a str, name_pos: TokenPosition, parameters: Vec<Symbol<'a>>) -> Task<'a> {
        Task {
            name,
            name_pos,
            parameters
        }
    }
}

impl <'a> Hash for Task<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl <'a> PartialEq for Task<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl <'a> Eq for Task<'a> {}

impl <'a> Borrow<str> for &Task<'a> {
    fn borrow(&self) -> &'a str {
        &self.name
    }
}

impl <'a> Borrow<&'a str> for &Task<'a> {
    fn borrow(&self) -> &&'a str {
        &self.name
    }
}