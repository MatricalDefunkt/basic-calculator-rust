# Basic Calculator (Rust)

A simple command-line calculator written in Rust.

## Features

*   Parses and evaluates mathematical expressions.
*   Supports basic arithmetic operations: addition, subtraction, multiplication, and division.
*   Supports exponentiation (`^`).
*   Supports trigonometric functions: `sin()`, `cos()`, `tan()`.
*   Supports square root: `sqrt()`.
*   Supports the mathematical constants `pi` and `e`.
*   Supports the exponential function `exp(x)` (e raised to the power of x).
*   Builds an Abstract Syntax Tree (AST) to represent the expression before evaluation.
*   Follows order of operations (PEMDAS/BODMAS).

## Prerequisites

*   Rust programming language (latest stable version recommended)

## Getting Started

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/MatricalDefunkt/basic-calculator-rust
    cd basic-calculator-rust
    ```
2.  **Build the project:**
    ```bash
    cargo build
    ```
3.  **Run the calculator:**
    The calculator runs in an interactive loop.
    ```bash
    cargo run
    ```
    You will then be prompted to enter expressions:
    ```
    Calculate > 2 + 2 * 3
    = 8.00000000
    Calculate > 10 / (2 + 3) - 1
    = 1.00000000
    Calculate > sin(1) + cos(1)
    = 1.38177329
    Calculate > pi * 2
    = 6.28318548
    Calculate > exp(1)
    = 2.71828175
    Calculate > exit
    ```
    Type `exit` to quit the calculator.

## Project Structure

*   `src/main.rs`: Entry point of the application. Handles command-line input and orchestrates the tokenization, parsing, and evaluation.
*   `src/tokenizer.rs`: Responsible for breaking down the input expression string into a sequence of tokens (numbers, operators, functions, constants, parentheses).
*   `src/parser.rs`: Takes the token stream from the tokenizer and builds an Abstract Syntax Tree (AST).
*   `src/ast.rs`: Defines the structure of the Abstract Syntax Tree (AST) nodes and includes the logic for evaluating the AST.
*   `Cargo.toml`: The manifest file for the Rust project, defining project metadata, dependencies, and build configurations.

## How it Works

1.  **Tokenization (`tokenizer.rs`):** The input expression string is scanned and converted into a list of tokens. For example, the expression `"2 + exp(1) * pi"` would be tokenized into `[Number(2), Plus, Exp, LeftParen, Number(1), RightParen, Multiply, Pi]`.
2.  **Parsing (`parser.rs`):** The sequence of tokens is then processed using a recursive descent parser. This method involves a set of mutually recursive functions that correspond to the non-terminals in the grammar of the mathematical expressions. It constructs an Abstract Syntax Tree (AST) representing the structure of the expression.
3.  **AST Definition and Evaluation (`ast.rs`):** The `ast.rs` file defines the `AstNode` enum (e.g., `Number`, `BinaryOp`, `UnaryOp`) that makes up the tree. It also contains the `eval()` method which recursively traverses the AST to compute the final numerical result, correctly handling operator precedence and function calls.
4.  **Main Application (`main.rs`):**
    *   Enters an interactive loop, prompting the user for input.
    *   Reads the mathematical expression entered by the user.
    *   Automatically balances parentheses if needed.
    *   Passes the expression to the tokenizer.
    *   Passes the resulting tokens to the parser to construct the AST.
    *   Evaluates the AST to compute the result.
    *   Prints the final result to the console.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
