#[allow(unreachable_code)]
mod parser;
mod tokenizer;

fn main() {
    loop {
        print!("> ");
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
            break;
        };
        let mut tokenizer = tokenizer::Tokenizer::new(input_str);
        tokenizer.tokenize();
        let mut parser = parser::Parser::new(tokenizer.tokens);
        // Enable max precision for f32 display
        println!("{:.8}", parser.parse());
    }
}
