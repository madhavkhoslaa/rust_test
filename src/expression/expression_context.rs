use super::expression_ast::ExpressionAST;
use super::operator::Operator;
pub struct ExpressionContext {
    expressionAST: Option<ExpressionAST>,
}

impl ExpressionContext {
    pub fn new_constant_expression(val: f64) -> Operator {
        return Operator::VAL(val);
    }
    pub fn new() -> Self {
        return ExpressionContext {
            expressionAST: None,
        };
    }
    pub fn new_binary_expression() {}

    pub fn eval(&self) -> f64 {}

    pub fn to_string(&self) -> String {}
}
