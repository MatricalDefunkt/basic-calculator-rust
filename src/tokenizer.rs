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
    // Accomodate for negative numbers at beginning
    pub fn tokenize(&mut self) {
        self.read_char();
        while self.read_position < self.input.len() {
            self.read_char();
            match self.current_char {
                '+' => self.tokens.push(Tokens::Plus),
                '-' => {
                    if self.read_position == 1 || self.peek_prev_char() == '(' {
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
                    if self.current_char.is_numeric() {
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
                        // Panic if we encounter an unknown character and point to the position

                        panic!(
                            "Unknown character \"{}\" at position {}",
                            self.current_char as i32, self.position
                        );
                    }
                }
            }
        }
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
