use super::{Tokens, Token};
use super::{is_comment, is_double_quote, is_char_whitespace, ends_token, tokenize};

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
