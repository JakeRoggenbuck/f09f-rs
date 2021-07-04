use super::{ends_token, is_char_whitespace, is_comment, is_double_quote, tokenize};
use super::{Token, Tokens};

pub trait Lex {
    fn sync(&mut self);
    fn next(&mut self, inc_before: bool);
    fn check_comment(&mut self) -> String;
    fn check_string(&mut self) -> String;
    fn lexer(&mut self);
}

pub struct Lexer {
    pub contents: String,
    pub chars: Vec<char>,
    pub index: usize,
    pub previous_char: char,
    pub current_char: char,
    pub next_char: char,
    pub tokens: Vec<Token>,
    pub verbose: bool,
}

impl Lex for Lexer {
    fn sync(&mut self) {
        // Shift location in contents
        // shift each character to the next
        self.previous_char = self.current_char;
        self.current_char = self.next_char;
        self.next_char = self.chars[self.index];
    }

    fn next(&mut self, inc_before: bool) {
        if inc_before {
            self.index += 1;
        }
        self.sync();
        if !inc_before {
            self.index += 1;
        }
    }

    fn check_comment(&mut self) -> String {
        self.sync();
        let mut comment = String::new();
        while !is_comment(self.current_char) {
            comment.push(self.current_char);
            self.next(true);
        }
        self.next(true);
        comment = String::from("~") + &comment + "~";
        return comment;
    }

    fn check_string(&mut self) -> String {
        self.sync();
        let mut string = String::new();
        while !is_double_quote(self.current_char) {
            string.push(self.current_char);
            self.next(true);
        }
        self.next(true);
        string = String::from("\"") + &string + "\"";
        return string;
    }

    fn lexer(&mut self) {
        // Add after content buffer
        self.chars = self.contents.chars().collect();
        let mut current_part = String::new();

        self.index = 0;
        let chars_len = self.contents.len();

        while self.index + 1 <= chars_len {
            // Skip over comments, then add a comment token
            if is_comment(self.current_char) {
                let comment = self.check_comment();
                self.tokens.push(Token {
                    part: comment,
                    token: Tokens::Comment,
                });
            }

            // Skip over strings, then add a string token
            if is_double_quote(self.current_char) {
                let string = self.check_string();
                self.tokens.push(Token {
                    part: string,
                    token: Tokens::StringLiteral,
                });
            }

            if !is_char_whitespace(self.current_char) {
                current_part.push(self.current_char);
                if ends_token(self.current_char, self.next_char) {
                    self.tokens.push(tokenize(&current_part));
                    current_part = String::new();
                }
            }
            if self.verbose {
                println!(
                    "{:?} {:?} {:?}",
                    self.previous_char, self.current_char, self.next_char
                );
            }
            self.next(false);
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
