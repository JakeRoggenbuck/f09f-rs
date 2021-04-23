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

fn lexer(mut contents: String) -> Vec<Token> {
    // Add after content buffer
    contents = contents + "  ";
    let chars: Vec<_> = contents.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();

    let mut current_part = String::new();

    let mut index = 0;
    let chars_len = contents.len();

    // These will be the chars passed into
    // the begins_token and ends_token
    let (mut previous_char, mut current_char, mut next_char) = (' ', ' ', ' ');

    let mut in_comment = false;
    while index + 1 <= chars_len {
        if is_comment(current_char) {
            if !in_comment {
                in_comment = true;
            } else if in_comment {
                in_comment = false;
                // TODO: Make function to add to index,
                // and reset value of previous_char,
                // current_char, next_char
                index += 1;
                previous_char = current_char;
                current_char = next_char;
                next_char = chars[index];
            }
        }
        if in_comment {
            index += 1;
            previous_char = current_char;
            current_char = next_char;
            next_char = chars[index];
            continue;
        }
        if !is_char_whitespace(current_char) {
            current_part.push(current_char);
            if ends_token(current_char, next_char) {
                tokens.push(tokenize(&current_part));
                current_part = String::new();
            }
        }

        println!("{:?} {:?} {:?}", previous_char, current_char, next_char);

        // Shift location in contents
        // shift each character to the next
        previous_char = current_char;
        current_char = next_char;
        next_char = chars[index];

        index += 1;
    }
    return tokens;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("Error: Please include a file");
        process::exit(0);
    }
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let tokens: Vec<Token> = lexer(contents);
    for tok in tokens.iter() {
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

    fn check_lexer(part: &str, token: &Tokens) {
        assert_eq!(
            lexer(String::from(part)),
            vec!(Token {
                part: String::from(part),
                token: *token
            })
        );
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
            check_lexer(i, token);
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
    }
}
