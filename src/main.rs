mod parser;
mod tokenizer;
use std::{
    io::{self, Write},
    process::exit,
};

fn main() {
    loop {
        print!("Calculate > ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let l_parenthesis_count = input.matches("(").count();
        let r_parenthesis_count = input.matches(")").count();
        if l_parenthesis_count < r_parenthesis_count {
            for _ in 0..(r_parenthesis_count - l_parenthesis_count) {
                input.insert_str(0, "(");
            }
        } else if l_parenthesis_count > r_parenthesis_count {
            for _ in 0..(l_parenthesis_count - r_parenthesis_count) {
                input.push_str(")");
            }
        }
        let input_str = input.trim();
        if input_str == "exit" {
            exit(0);
        };
        let mut tokenizer = tokenizer::Tokenizer::new(input_str);
        tokenizer.tokenize();
        // println!("{:?}", tokenizer.tokens);
        let mut parser = parser::Parser::new(tokenizer.tokens);
        println!("= {:.8}", parser.parse());
    }
}
