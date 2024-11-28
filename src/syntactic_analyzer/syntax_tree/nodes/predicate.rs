use std::{fmt::format, hash::Hash};

use crate::TokenPosition;

use super::*;

#[derive(Clone, Debug)]
pub struct Predicate<'a> {
    pub name: &'a str,
    pub name_pos: TokenPosition,
    pub variables: Vec<Symbol<'a>>
}

impl <'a> Predicate<'a> {
    pub fn new(name: &'a str, name_pos: TokenPosition, variables: Vec<Symbol<'a>>) -> Predicate<'a> {
        Predicate {
            name,
            name_pos,
            variables
        }
    }
    pub fn new_dummy(name: &'a str) -> Predicate {
        Predicate {
            name,
            name_pos: TokenPosition { line: 0 },
            variables: vec![]
        }
    }
}

impl <'a> PartialEq for Predicate<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name)
    }
}

impl <'a> Eq for Predicate<'a> {}

impl <'a> Hash for Predicate<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl <'a> ToString for Predicate<'a> {
    fn to_string(&self) -> String {
        let mut s = String::from(self.name);
        s.push('(');
        for var in self.variables.iter() {
            s.push_str(var.name);
            s.push(',');
        }
        s.push(')');
        s
    }
}