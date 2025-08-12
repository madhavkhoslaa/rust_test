use std::error::Error;
use std::io;

use super::expression_ast::ExpressionAST;
use super::operator::Operator;
pub struct ExpressionContext {}

impl ExpressionContext {
    pub fn new_constant_expression(val: f64) -> ExpressionAST {
        return ExpressionAST {
            op: Operator::VAL(val),
            left: None,
            right: None,
        };
    }
    pub fn new() -> Self {
        return ExpressionContext {};
    }
    pub fn new_binary_expression(
        operator: Operator,
        left_expression: ExpressionAST,
        right_expression: ExpressionAST,
    ) -> ExpressionAST {
        ExpressionAST {
            op: operator,
            left: Some(Box::new(left_expression)),
            right: Some(Box::new(right_expression)),
        }
    }

    fn eval_sync(&self, ast: &ExpressionAST) -> Result<f64, Box<dyn Error>> {
        match ast.op {
            Operator::VAL(value) => Ok(value),
            Operator::Add | Operator::Subtract | Operator::MUL | Operator::DIV => {
                let left_ast = ast.left.as_ref().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Left operand missing")
                })?;
                let right_ast = ast.right.as_ref().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Right operand missing")
                })?;

                let left_value = self.eval_sync(left_ast)?;
                let right_value = self.eval_sync(right_ast)?;

                let result = match ast.op {
                    Operator::Add => left_value + right_value,
                    Operator::Subtract => left_value - right_value,
                    Operator::MUL => left_value * right_value,
                    Operator::DIV => {
                        if right_value == 0.0 {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidInput,
                                "Division by zero",
                            )
                            .into());
                        }
                        left_value / right_value
                    }
                    _ => unreachable!(),
                };

                Ok(result)
            }
        }
    }

    pub async fn eval(&self, ast: &ExpressionAST) -> Result<f64, Box<dyn Error>> {
        // Reason why I have do is to avoid Pinning ast because recursive AST requires pinning of the type
        // to avoid race conditions while async recursive tasks are executed.
        self.eval_sync(ast)
    }
}
