use super::*;
use crate::TokenPosition;
#[derive(Debug)]
pub struct ProblemAST<'a> {
    pub requirements: Vec<RequirementType>,
    pub init_tn: Option<InitialTaskNetwork<'a>>,
    pub init_state: Vec<Predicate<'a>>,
    pub goal: Option<Formula<'a>>,
    pub objects: Vec<Symbol<'a>>,
}

impl <'a> ProblemAST<'a> {
    pub fn new() -> ProblemAST<'a> {
        ProblemAST {
            requirements: vec![],
            init_tn: None,
            init_state: vec![],
            goal: None,
            objects: vec![]
        }
    }
    pub fn add_object(&mut self, name: &'a str, object_pos: TokenPosition) {
        let object = Symbol::new(name, object_pos, None, None);
        self.objects.push(object);
    }
    pub fn add_typed_object(&mut self, name: &'a str, name_pos: TokenPosition, object_type: &'a str, type_pos: TokenPosition) {
        let object = Symbol::new(name, name_pos, Some(object_type), Some(type_pos));
        self.objects.push(object);
    }
    pub fn add_init_tn(&mut self, tn: InitialTaskNetwork<'a>) {
        self.init_tn = Some(tn);
    }
    pub fn add_init_state(&mut self, state: Vec<Predicate<'a>>) {
        self.init_state = state;
    }
    pub fn add_goal(&mut self, goal: Formula<'a>) {
        self.goal = Some(goal);
    }
    pub fn add_requirement(&mut self, req: RequirementType) {
        self.requirements.push(req);
    }
}