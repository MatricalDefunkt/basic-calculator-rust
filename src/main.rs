mod parser;
mod tokenizer;

fn main() {
    loop {
        print!("> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input_str = input.trim();
        if input_str == "exit" {
            break;
        };

        let mut tokenizer = tokenizer::Tokenizer::new(input_str);
        tokenizer.tokenize();
        let mut parser = parser::Parser::new(tokenizer.tokens);
        println!("{}", parser.parse());
    }
}
