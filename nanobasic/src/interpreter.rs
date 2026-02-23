use crate::parser::Node;
use crate::parser::{
    Line,
    expressions::{BinaryOperator, Expression, UnaryOperator},
    statements::{Statement, let_statment::LetStatement},
};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("Write to undeclared variable: '{0}")]
    UndeclaredVariable(String),

    #[error("Cannot GOTO line: '{0}")]
    InvalidGoto(isize),

    #[error("RETURN without GOSUB")]
    ReturnWithoutGosub,
}

pub type Result<T> = std::result::Result<T, InterpreterError>;

pub struct Interpreter {
    program: Vec<Line>,
    variables: HashMap<String, isize>,
    statement_index: usize,
    subroutine_stack: Vec<usize>,
}

impl Interpreter {
    pub fn new(program: Vec<Line>) -> Self {
        Interpreter {
            program,
            variables: HashMap::new(),
            statement_index: 0,
            subroutine_stack: Vec::new(),
        }
    }

    fn calculate_expression(&self, expression: &Expression) -> Result<isize> {
        use Expression::*;
        let value = match expression {
            BinaryOperation(binary_op) => {
                let left = self.calculate_expression(&binary_op.left.content)?;
                let right = self.calculate_expression(&binary_op.left.content)?;
                match binary_op.operator {
                    BinaryOperator::Devide => left / right,
                    BinaryOperator::Multiply => left * right,
                    BinaryOperator::Plus => left + right,
                    BinaryOperator::Minus => left - right,
                }
            }
            UnaryOperation {
                expression,
                operator,
            } => match operator {
                UnaryOperator::Minus => (-1) * self.calculate_expression(&expression.content)?,
            },
            NumberLiteral(n) => *n,
            VarRetrieve(x) => *self
                .variables
                .get(x)
                .ok_or(InterpreterError::UndeclaredVariable(x.clone()))?,
        };
        Ok(value)
    }

    fn interpret(&mut self) -> Result<()> {
        let Line {
            statement: Node { content, .. },
            ..
        } = &self.program[self.statement_index];
        match content {
            Statement::Let(let_stmt) => {
                let LetStatement { name, expression } = &**let_stmt;
                let value = self.calculate_expression(&expression.content)?;
                self.variables.insert(name.clone(), value);
                self.statement_index += 1;
            }
            Statement::GoTo(expression) | Statement::GoSub(expression) => {
                let line_id = self.calculate_expression(expression)?;
                if line_id < 0 {
                    return Err(InterpreterError::InvalidGoto(line_id));
                }
                let new_index = self
                    .program
                    .iter()
                    .position(|line| line.line_id == line_id as usize)
                    .ok_or(InterpreterError::InvalidGoto(line_id))?;

                if let Statement::GoSub { .. } = content {
                    self.subroutine_stack.push(self.statement_index);
                };
                self.statement_index = new_index;
            }
            Statement::Return => {
                let index = self
                    .subroutine_stack
                    .pop()
                    .ok_or(InterpreterError::ReturnWithoutGosub)?;

                self.statement_index = index;
            }
            Statement::If(..) => {}
            Statement::Print(..) => {}
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        while self.statement_index < self.program.len() {
            self.interpret()?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Interpreter;

    #[test]
    pub fn test_interpreter() {
        let lines = crate::parser::parse_file("Examples/factorial.bas").unwrap();
        let _interpreter: Interpreter = Interpreter::new(lines);
        println!("ende")
    }
}
