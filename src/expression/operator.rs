#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Subtract,
    MUL,
    DIV,
    VAL(f64),
}
