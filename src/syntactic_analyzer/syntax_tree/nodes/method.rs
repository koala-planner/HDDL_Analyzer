use crate::TokenPosition;

use super::*;


#[derive(Debug)]
pub struct Method<'a> {
    pub name: Symbol<'a>,
    pub task: Symbol<'a>, 
    pub task_terms: Vec<Symbol<'a>>,
    pub params: Vec<Symbol<'a>>,
    pub precondition: Option<Formula<'a>>,
    pub tn: HTN<'a>
}