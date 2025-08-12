use super::operator::Operator;

#[derive(Clone, Debug)]
pub struct ExpressionAST {
    pub op: Operator,
    pub left: Option<Box<ExpressionAST>>,
    pub right: Option<Box<ExpressionAST>>,
}

impl ExpressionAST {
    pub fn to_string(&self) -> String {
        match &self.op {
            Operator::VAL(value) => {
                return format!("{:.6}", value);
            }
            Operator::Add | Operator::Subtract | Operator::MUL | Operator::DIV => {
                let left_str = match &self.left {
                    Some(left) => left.to_string(),
                    None => String::from("<?>"),
                };
                let right_str = match &self.right {
                    Some(right) => right.to_string(),
                    None => String::from("<?>"),
                };

                let op_str = match &self.op {
                    Operator::Add => "+",
                    Operator::Subtract => "-",
                    Operator::MUL => "*",
                    Operator::DIV => "/",
                    _ => unreachable!(),
                };

                return format!("({} {} {})", left_str, op_str, right_str);
            }
        }
    }
}
