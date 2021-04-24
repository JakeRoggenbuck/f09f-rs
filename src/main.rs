use std::env;
use std::fs;
use std::process;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Tokens {
    // Types
    Char,
    Int,
    Prec,
    Bool,
    String,
    True,
    False,
    // Keywords
    Function,
    Return,
    Returns,
    While,
    Do,
    For,
    If,
    Else,
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Carrot,
    Greater,
    Less,
    // Symbols
    Assignment,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Dot,
    Comma,
    Colon,
    Semicolon,
    // Other
    Identifier,
    StringLiteral,
    NumericLiteral,
    SingleQuote,
    DoubleQuote,
    // Ignore
    Space,
    Tab,
    Newline,
    Comment,
    None,
}

#[derive(PartialEq, Debug)]
struct Token {
    part: String,
    token: Tokens,
}

fn is_token_symbol(token: Tokens) -> bool {
    match token {
        Tokens::LeftBracket
        | Tokens::RightBracket
        | Tokens::LeftBrace
        | Tokens::RightBrace
        | Tokens::LeftParen
        | Tokens::RightParen
        | Tokens::Dot
        | Tokens::Comma
        | Tokens::Colon
        | Tokens::Semicolon
        | Tokens::Assignment => true,
        _ => false,
    }
}

fn is_token_operator(token: Tokens) -> bool {
    match token {
        Tokens::Plus
        | Tokens::Minus
        | Tokens::Star
        | Tokens::Slash
        | Tokens::Carrot
        | Tokens::Greater
        | Tokens::Less => true,
        _ => false,
    }
}

fn is_token_whitespace(token: Tokens) -> bool {
    match token {
        Tokens::Tab | Tokens::Space | Tokens::Newline => true,
        _ => false,
    }
}

fn is_char_symbol(ch: char) -> bool {
    match ch {
        '[' | ']' | '{' | '}' | '(' | ')' | '.' | ',' | ':' | ';' | '=' | '\'' | '\"' => true,
        _ => false,
    }
}

fn is_char_operator(ch: char) -> bool {
    match ch {
        '+' | '-' | '*' | '/' | '^' | '>' | '<' => true,
        _ => false,
    }
}

fn is_char_whitespace(ch: char) -> bool {
    match ch {
        '\t' | ' ' | '\n' => true,
        _ => false,
    }
}

fn is_char_numeric(ch: char) -> bool {
    return ch.is_digit(10);
}

fn is_comment(ch: char) -> bool {
    return ch == '~';
}

fn is_double_quote(ch: char) -> bool {
    return ch == '\"';
}

fn is_single_quote(ch: char) -> bool {
    return ch == '\'';
}

fn begins_token(prev: char, cur: char) -> bool {
    if is_char_whitespace(cur) {
        return false;
    }
    if is_char_whitespace(prev) {
        return true;
    }
    if is_char_symbol(cur) {
        return true;
    }
    if is_char_symbol(prev) {
        return true;
    }
    return false;
}

fn ends_token(cur: char, next: char) -> bool {
    if is_char_whitespace(next) {
        return true;
    }
    if is_char_symbol(cur) {
        return true;
    }
    if is_char_symbol(next) {
        return true;
    }
    if is_char_operator(cur) {
        return true;
    }
    if is_char_operator(next) {
        return true;
    }
    if is_char_whitespace(cur) {
        return false;
    }
    return false;
}

fn tokenize(part: &str) -> Token {
    let mut token = match part {
        "{" => Tokens::LeftBrace,
        "}" => Tokens::RightBrace,
        "[" => Tokens::LeftBracket,
        "]" => Tokens::RightBracket,
        "(" => Tokens::LeftParen,
        ")" => Tokens::RightParen,
        "." => Tokens::Dot,
        "," => Tokens::Comma,
        "=" => Tokens::Assignment,
        ";" => Tokens::Semicolon,
        ":" => Tokens::Colon,

        "int" => Tokens::Int,
        "prec" => Tokens::Prec,
        "char" => Tokens::Char,
        "bool" => Tokens::Bool,
        "string" => Tokens::String,
        "true" => Tokens::True,
        "false" => Tokens::False,

        "fun" => Tokens::Function,
        "return" => Tokens::Return,
        "returns" => Tokens::Returns,
        "while" => Tokens::While,
        "do" => Tokens::Do,
        "for" => Tokens::For,
        "if" => Tokens::If,
        "else" => Tokens::Else,

        "+" => Tokens::Plus,
        "-" => Tokens::Minus,
        "*" => Tokens::Star,
        "/" => Tokens::Slash,
        "^" => Tokens::Carrot,
        ">" => Tokens::Greater,
        "<" => Tokens::Less,

        " " => Tokens::Space,
        "\t" => Tokens::Tab,
        "\n" => Tokens::Newline,

        "~" => Tokens::Comment,
        "\'" => Tokens::SingleQuote,
        "\"" => Tokens::DoubleQuote,
        _ => Tokens::Identifier,
    };

    // Find what identifiers are actually numbers
    if token == Tokens::Identifier {
        for c in part.chars() {
            if is_char_numeric(c) {
                // Reassign them to be numbers
                token = Tokens::NumericLiteral;
                break;
            }
        }
    }

    let part = String::from(part);
    return Token { part, token };
}

trait Lex {
    fn next(&mut self, inc_before: bool);
    fn check_comment(&mut self, in_comment: bool) -> bool;
    fn lexer(&mut self);
}

struct Lexer {
    contents: String,
    chars: Vec<char>,
    index: usize,
    previous_char: char,
    current_char: char,
    next_char: char,
    tokens: Vec<Token>,
}

impl Lex for Lexer {
    fn next(&mut self, inc_before: bool) {
        // Shift location in contents
        // shift each character to the next
        if inc_before {
            self.index += 1;
        }
        self.previous_char = self.current_char;
        self.current_char = self.next_char;
        self.next_char = self.chars[self.index];
        if !inc_before {
            self.index += 1;
        }
    }

    fn check_comment(&mut self, mut in_comment: bool) -> bool {
        if is_comment(self.current_char) {
            if !in_comment {
                in_comment = true;
            } else if in_comment {
                in_comment = false;
                self.next(true);
            }
        }
        if in_comment {
            self.next(true);
            return true;
        }
        return false;
    }

    fn lexer(&mut self) {
        // Add after content buffer
        self.chars = self.contents.chars().collect();
        let mut current_part = String::new();

        self.index = 0;
        let chars_len = self.contents.len();

        let in_comment = false;
        while self.index + 1 <= chars_len {
            if self.check_comment(in_comment) {
                continue;
            }

            if !is_char_whitespace(self.current_char) {
                current_part.push(self.current_char);
                if ends_token(self.current_char, self.next_char) {
                    self.tokens.push(tokenize(&current_part));
                    current_part = String::new();
                }
            }
            println!(
                "{:?} {:?} {:?}",
                self.previous_char, self.current_char, self.next_char
            );
            self.next(false);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("Error: Please include a file");
        process::exit(0);
    }
    let filename = &args[1];

    let contents =
        fs::read_to_string(filename).expect("Something went wrong reading the file") + "   ";

    let mut lexer = Lexer {
        contents: contents,
        chars: Vec::new(),
        index: 0,
        previous_char: ' ',
        current_char: ' ',
        next_char: ' ',
        tokens: Vec::new(),
    };

    lexer.lexer();

    // Display tokens (not needed, verbose output)
    for tok in lexer.tokens.iter() {
        println!("{:?}:\t\t{}", tok.token, tok.part);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_token_symbol_test() {
        let array = [
            Tokens::Dot,
            Tokens::LeftParen,
            Tokens::LeftBracket,
            Tokens::Assignment,
        ];
        for i in array.iter() {
            assert!(is_token_symbol(*i));
        }

        let array = [
            Tokens::String,
            Tokens::Int,
            Tokens::Space,
            Tokens::While,
            Tokens::For,
        ];
        for i in array.iter() {
            assert!(!is_token_symbol(*i));
        }
    }

    #[test]
    fn is_token_operator_test() {
        let array = [Tokens::Plus, Tokens::Minus, Tokens::Slash, Tokens::Star];
        for i in array.iter() {
            assert!(is_token_operator(*i));
        }

        let array = [
            Tokens::String,
            Tokens::Int,
            Tokens::Dot,
            Tokens::Comma,
            Tokens::While,
        ];
        for i in array.iter() {
            assert!(!is_token_operator(*i));
        }
    }

    #[test]
    fn is_token_whitespace_test() {
        let array = [Tokens::Space, Tokens::Tab, Tokens::Newline];
        for i in array.iter() {
            assert!(is_token_whitespace(*i));
        }

        let array = [
            Tokens::String,
            Tokens::Int,
            Tokens::Dot,
            Tokens::Comma,
            Tokens::While,
        ];
        for i in array.iter() {
            assert!(!is_token_operator(*i));
        }
    }

    #[test]
    fn is_char_symbol_test() {
        for i in ['[', ']', ')', '(', '.', ';'].iter() {
            assert!(is_char_symbol(*i));
        }
        for i in ['a', 'b', '7', '8'].iter() {
            assert!(!is_char_symbol(*i));
        }
    }

    #[test]
    fn is_char_operator_test() {
        for i in ['+', '-', '*', '^'].iter() {
            assert!(is_char_operator(*i));
        }

        for i in ['a', '(', '7', ']'].iter() {
            assert!(!is_char_operator(*i));
        }
    }

    #[test]
    fn is_char_whitespace_test() {
        for i in [' ', '\t', '\n'].iter() {
            assert!(is_char_whitespace(*i));
        }

        for i in ['a', '(', '7', ']'].iter() {
            assert!(!is_char_whitespace(*i));
        }
    }

    #[test]
    fn is_char_numeric_test() {
        for i in ['1', '3', '5', '9'].iter() {
            assert!(is_char_numeric(*i));
        }

        for i in ['a', '(', ']', '+', 'n'].iter() {
            assert!(!is_char_numeric(*i));
        }
    }

    #[test]
    fn is_comment_test() {
        assert!(is_comment('a') == false);
        assert!(is_comment('~') == true);
    }

    #[test]
    fn is_double_quote_test() {
        assert!(is_double_quote('\'') == false);
        assert!(is_double_quote('\"') == true);
    }

    #[test]
    fn is_single_quote_test() {
        assert!(is_single_quote('\'') == true);
        assert!(is_single_quote('\"') == false);
    }

    #[test]
    fn tokenize_test() {
        assert_eq!(tokenize("for").token, Tokens::For);
        assert_eq!(tokenize("while").token, Tokens::While);
        assert_eq!(tokenize("int").token, Tokens::Int);
        assert_eq!(tokenize("<").token, Tokens::Less);
        assert_eq!(tokenize(">").token, Tokens::Greater);

        assert!(tokenize("forgot").token != Tokens::For);
        assert!(tokenize("whil").token != Tokens::While);
        assert!(tokenize("intent").token != Tokens::Int);
        assert!(tokenize("this<").token != Tokens::Less);
        assert!(tokenize("a>").token != Tokens::Greater);
    }

    fn check_symbol_in_lexer(original_part: &str, new_part: &str, token: &Tokens) {
        let contents = String::from(new_part) + "   ";
        let mut lexer = Lexer {
            contents: contents,
            chars: Vec::new(),
            index: 0,
            previous_char: ' ',
            current_char: ' ',
            next_char: ' ',
            tokens: Vec::new(),
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
            contents: contents,
            chars: Vec::new(),
            index: 0,
            previous_char: ' ',
            current_char: ' ',
            next_char: ' ',
            tokens: Vec::new(),
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
    }
}
