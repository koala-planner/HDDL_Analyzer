mod cli_args;

use clap::Parser;
use hddl_analyzer::HDDLAnalyzer;
use std::fs;

use cli_args::{CLIArgs, Commands};

pub fn main() {
    // ANSI escape color codes
    let yellow = "\x1b[33m";
    let green = "\x1b[32m";
    let red = "\x1b[31m";
    // ANSI escape code to reset text color
    let reset = "\x1b[0m";

    let args = CLIArgs::parse();
    match args.command {
        Commands::Metadata(info) => {
            let domain = fs::read(info.domain_path);
            match domain {
                Ok(domain_content) => match HDDLAnalyzer::get_metadata(&domain_content, None) {
                    Ok(result) => {
                        print!("{}", result)
                    }
                    Err(error) => {
                        eprintln!("{}[Error]{} {}", red, reset, error)
                    }
                },
                Err(read_error) => {
                    eprintln!("{}[Error]{} {}", red, reset, read_error)
                }
            }
        }
        Commands::Verify(input) => {
            let domain = fs::read(input.domain_path);
            match domain {
                Ok(domain_content) => match input.problem_path {
                    Some(problem_path) => {
                        let problem = fs::read(problem_path);
                        match problem {
                            Ok(problem_content) => {
                                let output = HDDLAnalyzer::verify(&domain_content, Some(&problem_content));
                                match output {
                                    Ok(warnings) => {
                                        for warning in warnings {
                                            println!("{}[Warning]{} {}", yellow, reset, warning);
                                        }
                                        println!("{}[Ok]{}", green, reset);
                                    }
                                    Err(parsing_error) => {
                                        eprintln!("{}[Error]{} {}", red, reset, parsing_error)
                                    }
                                }
                            }
                            Err(read_error) => {
                                eprintln!("{}[Error]{} {}", red, reset, read_error)
                            }
                        }
                    }
                    None => {
                        let output = HDDLAnalyzer::verify(&domain_content, None);
                        match output {
                            Ok(warnings) => {
                                for warning in warnings {
                                    println!("{}[Warning]{} {}", yellow, reset, warning);
                                }
                                println!("{}[Ok]{}", green, reset);
                            }
                            Err(parsing_error) => {
                                eprintln!("{}[Error]{} {}", red, reset, parsing_error)
                            }
                        }
                    }
                },
                Err(read_error) => {
                    eprintln!("{}[Error]{} {}", red, reset, read_error)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recursion_type_test_integration() {
        let domain = fs::read("domain.hddl");
        match domain {
            Ok(domain_content) => match HDDLAnalyzer::verify(&domain_content, None) {
                Ok(warning) => {

                }
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}
