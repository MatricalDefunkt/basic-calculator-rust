use crate::tokenizer::Tokens;

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

    pub fn parse(&mut self) -> f32 {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> f32 {
        let mut result = self.parse_term();

        while self.current_token() == Tokens::Plus || self.current_token() == Tokens::Minus {
            let token = self.current_token();
            self.next_token();

            if token == Tokens::Plus {
                result += self.parse_term();
            } else {
                result -= self.parse_term();
            }
        }

        result
    }

    fn parse_term(&mut self) -> f32 {
        let mut result = self.parse_factor();

        while self.current_token() == Tokens::Multiply || self.current_token() == Tokens::Divide {
            let token = self.current_token();
            self.next_token();

            if token == Tokens::Multiply {
                result *= self.parse_factor();
            } else {
                result /= self.parse_factor();
            }
        }

        result
    }

    fn parse_factor(&mut self) -> f32 {
        let mut result = self.parse_exponent();

        while self.current_token() == Tokens::Power {
            self.next_token();
            result = result.powf(self.parse_exponent() as f32);
        }

        result
    }

    fn parse_exponent(&mut self) -> f32 {
        let mut result = self.parse_atom();

        while self.current_token() == Tokens::Power {
            self.next_token();
            result = result.powf(self.parse_atom() as f32);
        }

        result
    }

    fn parse_atom(&mut self) -> f32 {
        let token = self.current_token();
        self.next_token();

        match token {
            Tokens::Number { value } => value,
            Tokens::LeftParen => {
                let result = self.parse_expression();
                self.next_token();
                result
            }
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
