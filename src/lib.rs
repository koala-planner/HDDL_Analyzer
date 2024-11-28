mod lexical_analyzer;
mod syntactic_analyzer;
mod semantic_analyzer;
mod output;

use crate::lexical_analyzer::TokenPosition;
use lexical_analyzer::LexicalAnalyzer;
use output::{MetaData, ParsingError};
use semantic_analyzer::*;
use syntactic_analyzer::AbstractSyntaxTree;

pub struct HDDLAnalyzer {}

impl HDDLAnalyzer {
    pub fn verify(domain: &Vec<u8>, problem: Option<&Vec<u8>>) -> Result<Vec<output::WarningType>, output::ParsingError> {
        let lexer = LexicalAnalyzer::new(&domain);
        let domain_parser = syntactic_analyzer::Parser::new(lexer);
        let domain_ast = domain_parser.parse()?;
        if let AbstractSyntaxTree::Domain(d) = domain_ast {
            let domain_semantic_verifier = DomainSemanticAnalyzer::new(&d);
            let symbol_table = domain_semantic_verifier.verify_domain()?;
            match problem {
                Some(p) => {
                    let lexer = LexicalAnalyzer::new(p);
                    let problem_parser = syntactic_analyzer::Parser::new(lexer);
                    let problem_ast = problem_parser.parse()?;
                    match problem_ast {
                        AbstractSyntaxTree::Problem(p_ast) => {
                            let problem_semantic_verifier = ProblemSemanticAnalyzer::new(
                                &p_ast,
                                symbol_table
                            );
                            let warnings= problem_semantic_verifier.verify_problem()?;
                            Ok(warnings)

                        }
                        _ => {
                            panic!("expected problem, found domain")
                        }
                    }
                },
                None => Ok(
                    symbol_table.warnings
                )
            }
        } else {
            panic!("expected domain, found problem")
        }
    }


    pub fn get_metadata(domain: &Vec<u8>, problem: Option<&Vec<u8>>) -> Result<MetaData, ParsingError> {
        let lexer = LexicalAnalyzer::new(&domain);
        let domain_parser = syntactic_analyzer::Parser::new(lexer);
        let domain_ast = domain_parser.parse()?;
        match domain_ast {
            AbstractSyntaxTree::Domain(d) => {
                let tdg = TDG::new(&d);
                let nullables = tdg.compute_nullables();
                let recursion_type= tdg.get_recursion_type(&nullables);
                Ok(MetaData {
                    recursion: recursion_type,
                    nullables: nullables.iter().map(|x| x.to_string()).collect(),
                    domain_name: String::new(),
                    n_actions: d.actions.len() as u32,
                    n_tasks: d.compound_tasks.len() as u32,
                    n_methods: d.methods.len() as u32
                })
            }
            _ => panic!("expected domain, found problem")
        }
    }
}
