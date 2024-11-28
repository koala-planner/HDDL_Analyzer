use super::*;
use std::collections::HashSet;
use petgraph::prelude::GraphMap;
use petgraph::Directed;

#[derive(Debug)]
pub struct SymbolTable<'a> {
    pub warnings: Vec<WarningType>,
    pub constants: HashSet<&'a Symbol<'a>>,
    pub predicates: HashSet<&'a Predicate<'a>>,
    pub tasks: HashSet<&'a Task<'a>>,
    pub actions: HashSet<&'a Action<'a>>,
    pub type_hierarchy: GraphMap<&'a str, (), Directed>,
}