use super:: *;

#[test]
pub fn primitive_refinement_test () {
    let program = String::from(
        "(define (domain bal)
            (:predicates 
                (at ?l)
            )
            (:action p_1
            :parameters(?l1)
            :precondition (at ?l1)
            )
            (:action p_2
            :parameters(?l1)
            :precondition (at ?l1)
            )
            (:task abs_1 :parameters(?a))
            (:task abs_2 :parameters(?a))
            (:task abs_3 :parameters(?a))

            (:method m_1
                :parameters (?p1) 
                :task (abs_1 ?p1)
                :ordered-subtasks (and
                    (t4 (p_1 ?p1))
                )
            )
            (:method m_2
                :parameters (?p1) 
                :task (abs_2 ?p1)
                :ordered-subtasks ()
            )
        ) ",
    )
    .into_bytes();
    let lexer = LexicalAnalyzer::new(&program);
    let parser = Parser::new(lexer);
    let ast = parser.parse().unwrap();
    match ast {
        AbstractSyntaxTree::Domain(d) => {
            let semantic_analyzer = DomainSemanticAnalyzer::new(&d);
            match semantic_analyzer.verify_domain() {
                Ok(sym_table) => {
                    assert_eq!(sym_table.warnings.len(), 1);
                    match &sym_table.warnings[0] {
                        WarningType::NoPrimitiveRefinement(info) => {
                            assert_eq!(info.symbol, "abs_3");
                            assert_eq!(info.position.line, 15);
                        }
                        _ => panic!()
                    }
                }
                _ => panic!()
            }
        }
        _ => panic!()
    }
}

#[test]
pub fn unsat_action_prec_test () {
    let program = String::from(
        "(define (domain bal)
            (:predicates 
                (at ?l)
            )
            (:action p_1
            :parameters(?l1)
            :precondition (at ?l1)
            )
            (:action p_2
            :parameters(?l1)
            :precondition (and
                    (at ?l1)
                    (not (at ?l1))
                )
            )
            (:task abs_1 :parameters(?a))
            (:task abs_2 :parameters(?a))

            (:method m_1
                :parameters (?p1) 
                :task (abs_1 ?p1)
                :ordered-subtasks (and
                    (t4 (p_1 ?p1))
                )
            )
            (:method m_2
                :parameters (?p1) 
                :task (abs_2 ?p1)
                :ordered-subtasks ()
            )
        ) ",
    )
    .into_bytes();
    let lexer = LexicalAnalyzer::new(&program);
    let parser = Parser::new(lexer);
    let ast = parser.parse().unwrap();
    match ast {
        AbstractSyntaxTree::Domain(d) => {
            let semantic_analyzer = DomainSemanticAnalyzer::new(&d);
            match semantic_analyzer.verify_domain() {
                Ok(sym_table) => {
                    assert_eq!(sym_table.warnings.len(), 1);
                    match &sym_table.warnings[0] {
                        WarningType::UnsatisfiableActionPrecondition(info) => {
                            assert_eq!(info.symbol, "p_2");
                            assert_eq!(info.position.line, 9);
                        }
                        _ => panic!()
                    }
                }
                _ => panic!()
            }
        }
        _ => panic!()
    }
}


#[test]
pub fn unsat_method_prec_test () {
    let program = String::from(
        "(define (domain bal)
            (:predicates 
                (at ?l)
            )
            (:action p_1
            :parameters(?l1)
            :precondition (at ?l1)
            )
            (:action p_2
            :parameters(?l1)
            :precondition ()
            )
            (:task abs_1 :parameters(?a))
            (:task abs_2 :parameters(?a))

            (:method m_1
                :parameters (?p1) 
                :task (abs_1 ?p1)
                :precondition (and
                    (at ?p1)
                    (not (at ?p1))
                )
                :ordered-subtasks (and
                    (t4 (p_1 ?p1))
                )
            )
            (:method m_2
                :parameters (?p1) 
                :task (abs_2 ?p1)
                :ordered-subtasks ()
            )
        ) ",
    )
    .into_bytes();
    let lexer = LexicalAnalyzer::new(&program);
    let parser = Parser::new(lexer);
    let ast = parser.parse().unwrap();
    match ast {
        AbstractSyntaxTree::Domain(d) => {
            let semantic_analyzer = DomainSemanticAnalyzer::new(&d);
            match semantic_analyzer.verify_domain() {
                Ok(sym_table) => {
                    assert_eq!(sym_table.warnings.len(), 1);
                    match &sym_table.warnings[0] {
                        WarningType::UnsatisfiableMethodPrecondition(info) => {
                            assert_eq!(info.symbol, "m_1");
                            assert_eq!(info.position.line, 16);
                        }
                        _ => panic!()
                    }
                }
                token => panic!("{:?}", token)
            }
        }
        _ => panic!()
    }
}