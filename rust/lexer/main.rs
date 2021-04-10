enum Tokens {
    // Types
    Char,
	Int,
	Prec,
	Bool,
    String,
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

fn is_symbol(token: Tokens) -> bool {
    match token {
        Tokens::LeftBracket | Tokens::RightBracket  |
        Tokens::LeftBrace   | Tokens::RightBrace    |
        Tokens::LeftParen   | Tokens::RightParen    |
        Tokens::Dot         | Tokens::Comma         |
        Tokens::Colon       | Tokens::Semicolon => true,
        _ => false,
    }
}

fn is_operator(token: Tokens) -> bool {
    match token {
        Tokens::Plus    | Tokens::Minus     |
        Tokens::Star    | Tokens::Slash     |
        Tokens::Carrot  | Tokens::Greater   |
        Tokens::Less => true,
        _ => false,
    }
}

struct Lexer;
impl Lexer {
    fn lex(&self) {
        println!();
    }
}

fn main() {
    let lexer = Lexer;
    lexer.lex();
}
