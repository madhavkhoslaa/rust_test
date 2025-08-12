use super::operator::Operator;

pub struct ExpressionAST {
    OP: Operator,
    left: Option<Box<ExpressionAST>>,
    right: Option<Box<ExpressionAST>>,
}
