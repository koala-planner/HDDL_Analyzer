use super::*;

#[derive(Clone)]
pub struct TypeChecker<'a> {
    pub type_hierarchy: GraphMap<&'a str, (), Directed>,
}

impl<'a> TypeChecker<'a> {
    pub fn new(types: &Option<Vec<Symbol<'a>>>) -> TypeChecker<'a> {
        match &types {
            None => TypeChecker {
                type_hierarchy: GraphMap::new(),
            },
            Some(type_deps) => {
                let mut type_graph: GraphMap<&str, (), Directed> =
                    GraphMap::<_, (), Directed>::new();
                for delcared_type in type_deps {
                    if !type_graph.contains_node(delcared_type.name) {
                        type_graph.add_node(delcared_type.name);
                    }
                    match &delcared_type.symbol_type {
                        None => {}
                        Some(parent) => {
                            if !type_graph.contains_node(parent) {
                                type_graph.add_node(parent);
                            }
                            type_graph.add_edge(delcared_type.name, parent, ());
                        }
                    }
                }
                return TypeChecker {
                    type_hierarchy: type_graph,
                };
            }
        }
    }

    pub fn verify_type_hierarchy(&self) -> Result<(), SemanticErrorType> {
        match toposort(&self.type_hierarchy, None) {
            Ok(_) => Ok(()),
            Err(_) => {
                return Err(SemanticErrorType::CyclicTypeDeclaration);
            }
        }
    }

    pub fn check_type_declarations(
        &self,
        parameters: &Vec<Symbol<'a>>,
    ) -> Option<SemanticErrorType> {
        for parameter in parameters.iter() {
            if let Some(t) = parameter.symbol_type {
                if !self.type_hierarchy.contains_node(t) {
                    return Some(SemanticErrorType::UndefinedType(UndefinedSymbolError {
                        symbol: parameter.symbol_type.unwrap().to_string(),
                        position: parameter.type_pos.unwrap(),
                    }));
                }
            }
        }
        None
    }

    pub fn is_var_type_consistent(
        &self,
        found: Option<&'a str>,
        expected: Option<&'a str>,
    ) -> bool {
        match (found, expected) {
            (Some(found_typing), Some(defined_typing)) => {
                // type matches exactly
                if found_typing == defined_typing {
                    return true;
                }
                // search whether there is a path from current type to a super type
                if !has_path_connecting(&self.type_hierarchy, found_typing, defined_typing, None) {
                    return false;
                } else {
                    return true;
                }
            }
            (None, None) => {
                return true;
            }
            (None, Some(_)) => return false,
            (Some(_), None) => {
                return false;
            }
        }
    }
}
