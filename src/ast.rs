#[derive(Debug, PartialEq, Clone)]
pub enum AstNode {
    Number(f32),
    UnaryOp {
        op: UnaryOperator,
        operand: Box<AstNode>,
    },
    BinaryOp {
        op: BinaryOperator,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnaryOperator {
    Sin,
    Cos,
    Tan,
    Sqrt,
    Exp, // Added Exp
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

impl AstNode {
    pub fn eval(&self) -> f32 {
        match self {
            AstNode::Number(n) => *n,
            AstNode::UnaryOp { op, operand } => {
                let val = operand.eval();
                match op {
                    UnaryOperator::Sin => val.sin(),
                    UnaryOperator::Cos => val.cos(),
                    UnaryOperator::Tan => val.tan(),
                    UnaryOperator::Sqrt => val.sqrt(),
                    UnaryOperator::Exp => val.exp(), // Evaluate e^operand
                }
            }
            AstNode::BinaryOp { op, left, right } => {
                let left_val = left.eval();
                let right_val = right.eval();
                match op {
                    BinaryOperator::Add => left_val + right_val,
                    BinaryOperator::Subtract => left_val - right_val,
                    BinaryOperator::Multiply => left_val * right_val,
                    BinaryOperator::Divide => left_val / right_val,
                    BinaryOperator::Power => left_val.powf(right_val),
                }
            }
        }
    }
}
