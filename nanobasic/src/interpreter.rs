use std::collections::HashMap;

use crate::parser::Line;


#[allow(dead_code)]
pub struct Interpreter{
    program: Vec<Line>,
    variables: HashMap<String,isize>,
    statement_index: usize,
}

impl Interpreter {
    pub fn new(program: Vec<Line>)  -> Self {
        Interpreter {
            program,
            variables: HashMap::new(),
            statement_index: 0
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Interpreter;

    #[test]
    pub fn test_interpreter() {
        let lines = crate::parser::parse_file("nanobasic/Examples/factorial.bas").unwrap();
        let _interpreter: Interpreter = Interpreter::new(lines);
        println!("ende")
    }
}