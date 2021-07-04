use super::{Token, Tokens};

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

pub fn is_char_whitespace(ch: char) -> bool {
    match ch {
        '\t' | ' ' | '\n' => true,
        _ => false,
    }
}

fn is_char_numeric(ch: char) -> bool {
    return ch.is_digit(10);
}

pub fn is_comment(ch: char) -> bool {
    return ch == '~';
}

pub fn is_double_quote(ch: char) -> bool {
    return ch == '\"';
}

pub fn ends_token(cur: char, next: char) -> bool {
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

pub fn tokenize(part: &str) -> Token {
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
        "#" => Tokens::Tag,
        "&" => Tokens::Reference,
        "?" => Tokens::Question,

        "int" => Tokens::Int,
        "prec" => Tokens::Prec,
        "char" => Tokens::Char,
        "bool" => Tokens::Bool,
        "string" => Tokens::String,
        "true" => Tokens::True,
        "false" => Tokens::False,
        "byte" => Tokens::Byte,
        "class" => Tokens::Class,
        "static" => Tokens::Static,

        "assert" => Tokens::Assert,
        "print" => Tokens::Print,
        "input" => Tokens::Input,

        "fun" => Tokens::Function,
        "return" => Tokens::Return,
        "returns" => Tokens::Returns,
        "while" => Tokens::While,
        "do" => Tokens::Do,
        "for" => Tokens::For,
        "in" => Tokens::In,
        "if" => Tokens::If,
        "else" => Tokens::Else,
        "include" => Tokens::Include,
        "break" => Tokens::Break,
        "continue" => Tokens::Continue,

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
