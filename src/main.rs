use std::env;
use std::fs;
use std::process;

use tokenizer::{ends_token, is_char_whitespace, is_comment, is_double_quote, tokenize};
use tokens::Tokens;
use lexer::{Lexer, Lex};

pub mod tokenizer;
pub mod tokens;
pub mod lexer;

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
            println!("{:?}:\t\t{}", tok.token, tok.part);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_symbol_in_lexer(original_part: &str, new_part: &str, token: &Tokens) {
        let contents = String::from(new_part) + "   ";
        let mut lexer = Lexer {
            contents,
            chars: Vec::new(),
            index: 0,
            previous_char: ' ',
            current_char: ' ',
            next_char: ' ',
            tokens: Vec::new(),
            verbose: false,
        };

        lexer.lexer();
        assert_eq!(
            lexer.tokens,
            vec!(Token {
                part: String::from(original_part),
                token: *token
            })
        );
    }

    fn check_lexer(new_part: &str, tokens: Vec<Token>) {
        let contents = String::from(new_part) + "   ";
        let mut lexer = Lexer {
            contents,
            chars: Vec::new(),
            index: 0,
            previous_char: ' ',
            current_char: ' ',
            next_char: ' ',
            tokens: Vec::new(),
            verbose: false,
        };

        lexer.lexer();
        assert_eq!(lexer.tokens, tokens);
    }

    fn make_symbol_array(part: &str) -> [String; 5] {
        return [
            String::from(part),
            part.to_owned() + &" ".to_owned(),
            " ".to_owned() + &part.to_owned() + &" ".to_owned(),
            "\n".to_owned() + &part.to_owned(),
            " ".to_owned() + &part.to_owned() + &"   ".to_owned(),
        ];
    }

    fn check_symbol(part: &str, token: &Tokens) {
        for i in make_symbol_array(part).iter() {
            check_symbol_in_lexer(part, i, token);
        }
    }

    #[test]
    fn lexer_test() {
        check_symbol("fun", &Tokens::Function);
        check_symbol("for", &Tokens::For);
        check_symbol("while", &Tokens::While);

        check_symbol(".", &Tokens::Dot);
        check_symbol(";", &Tokens::Semicolon);
        check_symbol("{", &Tokens::LeftBrace);
        check_symbol("+", &Tokens::Plus);

        check_lexer(
            "fun factorial",
            vec![
                Token {
                    part: String::from("fun"),
                    token: Tokens::Function,
                },
                Token {
                    part: String::from("factorial"),
                    token: Tokens::Identifier,
                },
            ],
        );

        check_lexer(
            "return 0;",
            vec![
                Token {
                    part: String::from("return"),
                    token: Tokens::Return,
                },
                Token {
                    part: String::from("0"),
                    token: Tokens::NumericLiteral,
                },
                Token {
                    part: String::from(";"),
                    token: Tokens::Semicolon,
                },
            ],
        );

        check_lexer(
            "int fact = 1;",
            vec![
                Token {
                    part: String::from("int"),
                    token: Tokens::Int,
                },
                Token {
                    part: String::from("fact"),
                    token: Tokens::Identifier,
                },
                Token {
                    part: String::from("="),
                    token: Tokens::Assignment,
                },
                Token {
                    part: String::from("1"),
                    token: Tokens::NumericLiteral,
                },
                Token {
                    part: String::from(";"),
                    token: Tokens::Semicolon,
                },
            ],
        );

        check_lexer(
            "int x = 0; ~ Starts at zero ~",
            vec![
                Token {
                    part: String::from("int"),
                    token: Tokens::Int,
                },
                Token {
                    part: String::from("x"),
                    token: Tokens::Identifier,
                },
                Token {
                    part: String::from("="),
                    token: Tokens::Assignment,
                },
                Token {
                    part: String::from("0"),
                    token: Tokens::NumericLiteral,
                },
                Token {
                    part: String::from(";"),
                    token: Tokens::Semicolon,
                },
                Token {
                    part: String::from("~ Starts at zero ~"),
                    token: Tokens::Comment,
                },
            ],
        );

        check_lexer(
            "string name = \"Jake\";",
            vec![
                Token {
                    part: String::from("string"),
                    token: Tokens::String,
                },
                Token {
                    part: String::from("name"),
                    token: Tokens::Identifier,
                },
                Token {
                    part: String::from("="),
                    token: Tokens::Assignment,
                },
                Token {
                    part: String::from("\"Jake\""),
                    token: Tokens::StringLiteral,
                },
                Token {
                    part: String::from(";"),
                    token: Tokens::Semicolon,
                },
            ],
        );
    }
}
