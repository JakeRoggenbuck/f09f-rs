use std::env;
use std::fs;
use std::process;

use lexer::{Lex, Lexer};
use tokenizer::{ends_token, is_char_whitespace, is_comment, is_double_quote, tokenize};
use tokens::Tokens;
use utils::horizontal_space;

pub mod lexer;
pub mod tokenizer;
pub mod tokens;
pub mod utils;

#[derive(PartialEq, Debug)]
pub struct Token {
    part: String,
    token: Tokens,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("Error: Please include a file");
        process::exit(0);
    }

    let filename = &args[1];
    let mut verbose = false;
    if args.len() >= 2 {
        for arg in 2..args.len() {
            if args[arg] == "-v" {
                verbose = true;
            }
        }
    }

    let contents =
        fs::read_to_string(filename).expect("Something went wrong reading the file") + "   ";

    let mut lexer = Lexer {
        contents,
        chars: Vec::new(),
        index: 0,
        previous_char: ' ',
        current_char: ' ',
        next_char: ' ',
        tokens: Vec::new(),
        verbose,
    };

    lexer.lexer();

    if verbose {
        for tok in lexer.tokens.iter() {
            let token_length: usize = format!("{:?}", tok.token).len();
            println!("{:?}:{}{}", tok.token, horizontal_space(20 - token_length, ' '), tok.part);
        }
    }
}
