mod undefined_elements;
mod type_checker;
mod tests;
mod analyzers;
mod tdg;

use crate::syntactic_analyzer::*;
use crate::output::*;
use undefined_elements::*;
use type_checker::*;

extern crate petgraph;

pub use analyzers::*;
pub use tdg::TDG;