use std::fmt::{Display, Formatter, Error};

#[derive(PartialEq, Eq, Debug)]
pub enum RecursionType {
    NonRecursive,
    Recursive(Vec<(String, String)>), // (task_name, method_name) 
    EmptyRecursion(Vec<(String, String)>), // (task_name, method_name) 
    GrowingEmptyPrefixRecursion(Vec<(String, String)>), // (task_name, method_name) 
    GrowAndShrinkRecursion(Vec<(String, String)>), // (task_name, method_name) 
}

impl Display for RecursionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RecursionType::NonRecursive => write!(f, "Non-recursive"),
            RecursionType::Recursive(pairs) => {
                writeln!(f, "Recursive")?;
                write!(f, "\tCycle: ")?;
                format_task_pairs(pairs, f)
            }
            RecursionType::EmptyRecursion(pairs) => {
                writeln!(f, "Empty recursion")?;
                write!(f, "\tCycle: ")?;
                format_task_pairs(pairs, f)
            }
            RecursionType::GrowingEmptyPrefixRecursion(pairs) => {
                writeln!(f, "Growing empty prefix recursion")?;
                write!(f, "\tCycle: ")?;
                format_task_pairs(pairs, f)
            }
            RecursionType::GrowAndShrinkRecursion(pairs) => {
                writeln!(f, "Grow and shrink recursion")?;
                write!(f, "\tCycle: ")?;
                format_task_pairs(pairs, f)
            }
        }
    }
}

// Helper function to format the vector of task and method pairs
fn format_task_pairs(pairs: &[(String, String)], f: &mut Formatter<'_>) -> std::fmt::Result {
    for (i, (task, method)) in pairs.iter().enumerate() {
        if i != pairs.len() - 1 {
            write!(f, "[{}]-({})->", task, method)?;
        } else {
            write!(f, "[{}]", task)?;
        }
    }
    Ok(())
}


pub struct MetaData {
    pub recursion: RecursionType,
    pub nullables: Vec<String>,
    pub domain_name: String,
    pub n_actions: u32,
    pub n_tasks: u32,
    pub n_methods: u32
}

impl Display for MetaData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "Description")?;
        writeln!(f, "\tHierarchy type: {}", self.recursion)?;
        if self.nullables.len() == 0 {
            writeln!(f, "\tNullable Tasks: None")?;
        } else {
            writeln!(f, "\tNullable Tasks:")?;
            for nullable in self.nullables.iter() {
                writeln!(f, "\t\t{}", nullable)?
            }
        }
        writeln!(f, "\tNumber of actions: {}", self.n_actions)?;
        writeln!(f, "\tNumber of abstract tasks: {}", self.n_tasks)?;
        writeln!(f, "\tNumber of methods: {}", self.n_methods)?;
        Ok(())
    }
}