use crate::tokenizer::Tokens;
use crate::ast::{AstNode, BinaryOperator, UnaryOperator};

pub struct Parser {
    tokens: Vec<Tokens>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Tokens>) -> Parser {
        Parser {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> AstNode {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> AstNode {
        let mut node = self.parse_term();

        while self.current_token() == Tokens::Plus || self.current_token() == Tokens::Minus {
            let token = self.current_token();
            self.next_token();

            let right_node = self.parse_term();
            let op = if token == Tokens::Plus {
                BinaryOperator::Add
            } else {
                BinaryOperator::Subtract
            };
            node = AstNode::BinaryOp {
                op,
                left: Box::new(node),
                right: Box::new(right_node),
            };
        }

        node
    }

    fn parse_term(&mut self) -> AstNode {
        let mut node = self.parse_factor();

        while self.current_token() == Tokens::Multiply || self.current_token() == Tokens::Divide {
            let token = self.current_token();
            self.next_token();

            let right_node = self.parse_factor();
            let op = if token == Tokens::Multiply {
                BinaryOperator::Multiply
            } else {
                BinaryOperator::Divide
            };
            node = AstNode::BinaryOp {
                op,
                left: Box::new(node),
                right: Box::new(right_node),
            };
        }

        node
    }

    fn parse_factor(&mut self) -> AstNode {
        let mut node = self.parse_exponent();

        while self.current_token() == Tokens::Power {
            self.next_token();
            let right_node = self.parse_exponent();
            node = AstNode::BinaryOp {
                op: BinaryOperator::Power,
                left: Box::new(node),
                right: Box::new(right_node),
            };
        }

        node
    }

    fn parse_exponent(&mut self) -> AstNode {
        self.parse_atom()
    }

    fn parse_atom(&mut self) -> AstNode {
        let token = self.current_token();
        self.next_token();

        match token {
            Tokens::Number { value } => AstNode::Number(value),
            Tokens::Pi => AstNode::Number(std::f32::consts::PI),
            Tokens::E => AstNode::Number(std::f32::consts::E),
            Tokens::LeftParen => {
                let node = self.parse_expression();
                if self.current_token() == Tokens::RightParen {
                    self.next_token();
                } else {
                    panic!("Expected RightParen");
                }
                node
            }
            Tokens::Cos => AstNode::UnaryOp {
                op: UnaryOperator::Cos,
                operand: Box::new(self.parse_atom()),
            },
            Tokens::Sin => AstNode::UnaryOp {
                op: UnaryOperator::Sin,
                operand: Box::new(self.parse_atom()),
            },
            Tokens::Tan => AstNode::UnaryOp {
                op: UnaryOperator::Tan,
                operand: Box::new(self.parse_atom()),
            },
            Tokens::Sqrt => AstNode::UnaryOp {
                op: UnaryOperator::Sqrt,
                operand: Box::new(self.parse_atom()),
            },
            Tokens::Exp => AstNode::UnaryOp {
                op: UnaryOperator::Exp,
                operand: Box::new(self.parse_atom()),
            },
            _ => panic!("Unexpected token {:?}", token),
        }
    }

    fn current_token(&self) -> Tokens {
        self.tokens[self.position]
    }

    fn next_token(&mut self) {
        if self.tokens.len() > self.position + 1 {
            self.position += 1;
        }
    }
}
