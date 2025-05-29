use std::fmt::{Debug, Error, Formatter};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tokens {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,
    LeftParen,
    RightParen,
    Number { value: f32 },
    Sin,
    Cos,
    Tan,
    Sqrt,
    Pi, // Added Pi
    E,  // Added E
    Exp, // Added Exp
}

pub struct Tokenizer<'a> {
    input: &'a str,
    pub position: usize,
    pub read_position: usize,
    pub current_char: char,
    pub tokens: Vec<Tokens>,
}

impl Tokenizer<'_> {
    pub fn new<'a>(input: &'a str) -> Tokenizer<'a> {
        let t = Tokenizer {
            input,
            position: 0,
            read_position: 0,
            current_char: 0 as char,
            tokens: Vec::new(),
        };
        t
    }
    pub fn tokenize(&mut self) {
        while self.read_position < self.input.len() {
            self.read_char();
            match self.current_char {
                '+' => self.tokens.push(Tokens::Plus),
                '-' => {
                    if self.position == 0 || self.peek_prev_char() == '(' || self.peek_prev_char().is_alphabetic() || ['+', '-', '*', '/', '^', '('].contains(&self.peek_prev_char()) {
                        let mut number = String::new();
                        number.push(self.current_char);
                        while self.peek_char().is_numeric() {
                            self.read_char();
                            number.push(self.current_char);
                        }
                        self.tokens.push(Tokens::Number {
                            value: number.parse().unwrap(),
                        });
                    } else {
                        self.tokens.push(Tokens::Minus);
                    }
                }
                '*' => self.tokens.push(Tokens::Multiply),
                '/' => self.tokens.push(Tokens::Divide),
                '%' => self.tokens.push(Tokens::Modulo),
                '^' => self.tokens.push(Tokens::Power),
                '(' => self.tokens.push(Tokens::LeftParen),
                ')' => self.tokens.push(Tokens::RightParen),
                ' ' => continue,
                '\r' => continue,
                '\t' => continue,
                '\n' => continue,
                _ => {
                    if self.current_char.is_numeric() || (self.current_char == '-' && (self.position == 0 || self.peek_prev_char() == '(' || self.peek_prev_char().is_alphabetic() || ['+', '-', '*', '/', '^', '('].contains(&self.peek_prev_char()))) {
                        let mut number = String::new();
                        number.push(self.current_char);
                        while self.peek_char().is_numeric() {
                            self.read_char();
                            number.push(self.current_char);
                        }
                        self.tokens.push(Tokens::Number {
                            value: number.parse().unwrap(),
                        });
                    } else if self.current_char == '.' {
                        let current_token = self.tokens.pop().unwrap();
                        match current_token {
                            Tokens::Number { value } => {
                                let mut number = String::new();
                                number.push_str(&value.to_string());
                                number.push(self.current_char);
                                while self.peek_char().is_numeric() {
                                    self.read_char();
                                    number.push(self.current_char);
                                }
                                self.tokens.push(Tokens::Number {
                                    value: number.parse().unwrap(),
                                });
                            }
                            _ => {
                                self.unexpected_char_error(self.position, self.current_char);
                            }
                        }
                    } else if self.current_char.is_alphabetic() {
                        let current_index = self.position;
                        let current_char_val = self.current_char;
                        let mut identifier = String::new();
                        identifier.push(self.current_char);
                        while self.peek_char().is_alphabetic() {
                            self.read_char();
                            identifier.push(self.current_char);
                        }
                        match identifier.to_lowercase().as_str() {
                            "sin" => self.tokens.push(Tokens::Sin),
                            "cos" => self.tokens.push(Tokens::Cos),
                            "tan" => self.tokens.push(Tokens::Tan),
                            "sqrt" => self.tokens.push(Tokens::Sqrt),
                            "pi" => self.tokens.push(Tokens::Pi),
                            "e" => self.tokens.push(Tokens::E),
                            "exp" => self.tokens.push(Tokens::Exp),
                            _ => {
                                self.unexpected_char_error(current_index, current_char_val);
                            }
                        }
                    } else {
                        self.unexpected_char_error(self.position, self.current_char);
                    }
                }
            }
        }
    }

    fn unexpected_char_error(&mut self, position: usize, first_char: char) {
        let mut error_msg = String::new();
        let mut padding = String::new();
        error_msg.push_str(self.input);
        error_msg.push('\n');
        for _ in 0..self.position {
            padding.push(' ');
        }
        error_msg.push_str(padding.as_str());
        error_msg.push_str("^\n");
        for _ in 0..3 {
            error_msg.push_str(padding.as_str());
            error_msg.push_str("|\n");
        }
        println!("{}", position);
        panic!(
            "Unexpected character \"{}\"\n\n{}",
            if position == 0 {
                self.current_char
            } else {
                first_char
            },
            error_msg
        );
    }
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.current_char = 0 as char;
        } else {
            self.current_char = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            0 as char
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    fn peek_prev_char(&self) -> char {
        if self.position == 0 {
            0 as char
        } else {
            self.input.chars().nth(self.position - 1).unwrap()
        }
    }
}

impl Debug for Tokenizer<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:?}", self.tokens)
    }
}
