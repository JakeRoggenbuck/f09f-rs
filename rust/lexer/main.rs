use std::fs;
use std::env;
use std::process;

#[derive(Debug)]
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
	Literal,
    // Ignore
    Space,
	Tab,
	Newline,
	Comment,
	None,
}

struct Token {
    part: String,
    token: Tokens
}

fn is_token_symbol(token: Tokens) -> bool {
    match token {
        Tokens::LeftBracket | Tokens::RightBracket  |
        Tokens::LeftBrace   | Tokens::RightBrace    |
        Tokens::LeftParen   | Tokens::RightParen    |
        Tokens::Dot         | Tokens::Comma         |
        Tokens::Colon       | Tokens::Semicolon     |
        Tokens::Assignment => true,
        _ => false,
    }
}

fn is_token_operator(token: Tokens) -> bool {
    match token {
        Tokens::Plus    | Tokens::Minus     |
        Tokens::Star    | Tokens::Slash     |
        Tokens::Carrot  | Tokens::Greater   |
        Tokens::Less => true,
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
        '[' | ']' | '{' | '}' |
        '(' | ')' | '.' | ',' |
        ':' | ';' | '=' => true,
        _ => false,
    }
}


fn is_char_operator(ch: char) -> bool {
    match ch {
        '+' | '-' | '*' | '/' |
        '^' | '>' | '<' => true,
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


fn begins_token(prev: char, cur: char) -> bool {
    if is_char_whitespace(prev) { return true; }
    if is_char_whitespace(cur) { return false; }
    return false;
}

fn ends_token(cur: char, next: char) -> bool {
    if is_char_whitespace(cur) { return false; }
    if is_char_whitespace(next) { return true; }
    return false;
}

fn tokenize(part: &str) -> Token {
    let token = match part {
        "{" => Tokens::LeftBrace,
        "}" => Tokens::RightBrace,
        "[" => Tokens::LeftBracket,
        "]" => Tokens::RightBracket,
        "(" => Tokens::LeftParen,
        ")" => Tokens::RightParen,
        "." => Tokens::Dot,
        "," => Tokens::Comma,
        "=" => Tokens::Assignment,

        "int" => Tokens::Int,
        "char" => Tokens::Char,
        "bool" => Tokens::Bool,
        "string" => Tokens::String,
        "true" => Tokens::True,
        "false" => Tokens::False,

        "fun" => Tokens::Function,
        "return" => Tokens::Return,
        "while" => Tokens::While,
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
        _ => Tokens::Identifier,
    };

    let part = String::from(part);
    return Token {part, token}
}

fn lexer(contents: String) {
    for ch in contents.chars() {
        if is_char_symbol(ch) || is_char_operator(ch) {
            let tok = tokenize(&ch.to_string());
            println!("{:?} {}", tok.token, tok.part);
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

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    lexer(contents);
}
