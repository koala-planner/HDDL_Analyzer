use petgraph::prelude::GraphMap;
use petgraph::algo::toposort;
use petgraph::Directed;

use crate::TokenPosition;
use super::*;

#[derive(Debug)]
pub struct InitialTaskNetwork<'a> {
    pub parameters: Option<Vec<Symbol<'a>>>,
    pub tn: HTN<'a>
}

#[derive(Debug, Clone)]
pub struct HTN<'a> {
    pub subtasks: Vec<Subtask<'a>>,
    pub ordering_pos: Option<TokenPosition>,
    pub orderings: TaskOrdering<'a>,
    pub constraints: Option<Vec<Constraint<'a>>>, 
}

#[derive(Debug, Clone)]
pub struct Subtask<'a> {
    pub id: Option<Symbol<'a>>,
    pub task: Symbol<'a>,
    pub terms: Vec<Symbol<'a>>
}


#[derive(Debug, Clone)]
pub enum Constraint<'a> {
    Equal(&'a str, &'a str),
    NotEqual(&'a str, &'a str)
}

#[derive(Debug, Clone)]
pub enum TaskOrdering<'a> {
    Total,
    Partial(Vec<(&'a str, &'a str)>)
}

impl <'a> TaskOrdering<'a> {
    pub fn is_acyclic(&self) -> bool {
        match &self {
            TaskOrdering::Total => { true }
            TaskOrdering::Partial(orderings) => {
                let ordering_graph = GraphMap::<_, (), Directed>::from_edges(orderings);
                match toposort(&ordering_graph, None) {
                    Ok(_) => { true }
                    Err(_) => {
                        return false;
                    }
                }
            }
        }
    }
}