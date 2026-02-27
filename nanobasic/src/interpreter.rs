use crate::parser::Node;
use crate::parser::statements::if_statement::{BooleanExpression, IfStatement, RelationalOperator};
use crate::parser::statements::print_statment::Printable;
use crate::parser::{
    Line,
    expressions::{BinaryOperator, Expression, UnaryOperator},
    statements::{Statement, let_statment::LetStatement},
};
use std::collections::HashMap;
use std::io::Write;
use thiserror::Error;
use std::io::{self};

#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("Write to undeclared variable: '{0}")]
    UndeclaredVariable(String),

    #[error("Cannot GOTO line: '{0}")]
    InvalidGoto(isize),

    #[error("RETURN without GOSUB")]
    ReturnWithoutGosub,

    #[error("Write to output failed")]
    OutputError(#[from] io::Error)
}

pub type Result<T> = std::result::Result<T, InterpreterError>;

pub struct Interpreter<'a> {
    program: Vec<Line>,
    variables: HashMap<String, isize>,
    statement_index: usize,
    subroutine_stack: Vec<usize>,
    output: &'a mut dyn Write,
}

impl<'a>  Interpreter<'a>  {
    pub fn new(program: Vec<Line>, output: &'a mut dyn Write) -> Self {
        
        Interpreter {
            program,
            variables: HashMap::new(),
            statement_index: 0,
            subroutine_stack: Vec::new(),
            output
        }
    }

    fn calculate_boolean_expression(&self, expression: &BooleanExpression) -> Result<bool> {
        let left = self.calculate_expression(&expression.left_expr.content)?;
        let right = self.calculate_expression(&expression.right_expr.content)?;

        use RelationalOperator::*;

        let result = match expression.operator {
            Equal => left == right,
            Greater => left > right,
            GreaterEqual => left >= right,
            Less => left < right,
            LessEqual => left <= right,
            NotEqual => left != right,
        };
        Ok(result)
    }

    fn calculate_expression(&self, expression: &Expression) -> Result<isize> {
        use Expression::*;
        let value = match expression {
            BinaryOperation(binary_op) => {
                let left = self.calculate_expression(&binary_op.left.content)?;
                let right = self.calculate_expression(&binary_op.right.content)?;
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

    fn interpret_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
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

                if let Statement::GoSub { .. } = statement {
                    self.subroutine_stack.push(self.statement_index + 1);
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
            Statement::Print(node_printable) => {
                let printables = &**node_printable;
                let mut output = Vec::new();
                for Node { content, .. } in printables {
                    match content {
                        Printable::String(s) => {
                            output.push(s.clone());
                        }
                        Printable::ExpressionNode(expression) => {
                            let v: isize = self.calculate_expression(expression)?;
                            output.push(v.to_string());
                        }
                    }
                }
                let out_str = output.join("\t");
                writeln!(self.output,"{out_str}")?;
                self.output.flush()?;
                self.statement_index += 1;
            }
            Statement::If(if_statement) => {
                let IfStatement {
                    boolean_expr,
                    then_statement,
                } = &**if_statement;
                let condition = self.calculate_boolean_expression(&boolean_expr.content)?;
                if condition == true {
                    self.interpret_statement(&then_statement.content)?;
                } else {
                    self.statement_index += 1;
                }
            }
        }
        Ok(())
    }

    fn interpret(&mut self) -> Result<()> {
        let Line { statement, line_id } = &self.program[self.statement_index];
        log::debug!("Intrpreting line: {line_id}");

        let content = &statement.clone().content;
        self.interpret_statement(content)
    }

    pub fn run(&mut self) -> Result<()> {
        while self.statement_index < self.program.len() {
            self.interpret()?
        }
        Ok(())
    }
}
