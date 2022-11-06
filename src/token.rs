#[derive(Debug, PartialEq)]
pub enum Token {
    // Single-character tokens
    Assign,    // =
    Plus,      // +
    Minus,     // -
    Bang,      // !
    Asterisk,  // *
    Slash,     // /
    LThan,     // <
    GThan,     // >
    Comma,     // ,
    Semicolon, // ;
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }

    // Double-character tokens
    Eq,    // ==
    NotEq, // !=

    // keyword tokens
    Fn,     // fn
    Let,    // let
    True,   // true
    False,  // false
    If,     // if
    Else,   // else
    Return, // return

    // identifier tokens
    Identifier(String),

    // literal tokens
    Int(String),
}

impl Token {
    // Single character tokens
    pub const SYMBOL_ASSIGN: char = '=';
    pub const SYMBOL_PLUS: char = '+';
    pub const SYMBOL_MINUS: char = '-';
    pub const SYMBOL_BANG: char = '!';
    pub const SYMBOL_ASTERISK: char = '*';
    pub const SYMBOL_SLASH: char = '/';
    pub const SYMBOL_LT: char = '<';
    pub const SYMBOL_GT: char = '>';
    pub const SYMBOL_COMMA: char = ',';
    pub const SYMBOL_SEMICOLON: char = ';';
    pub const SYMBOL_LPAREN: char = '(';
    pub const SYMBOL_RPAREN: char = ')';
    pub const SYMBOL_LBRACE: char = '{';
    pub const SYMBOL_RBRACE: char = '}';

    // Double character tokens
    pub const SYMBOL_EQ: &str = "==";
    pub const SYMBOL_NOT_EQ: &str = "!=";

    // Keywords
    pub const KEYWORD_FN: &str = "fn";
    pub const KEYWORD_LET: &str = "let";
    pub const KEYWORD_TRUE: &str = "true";
    pub const KEYWORD_FALSE: &str = "false";
    pub const KEYWORD_IF: &str = "if";
    pub const KEYWORD_ELSE: &str = "else";
    pub const KEYWORD_RETURN: &str = "return";

    // Returns a keyword token if the given string is a keyword, otherwise returns None
    pub fn keyword(str: &String) -> Option<Self> {
        match str.as_str() {
            Self::KEYWORD_FN => Some(Self::Fn),
            Self::KEYWORD_LET => Some(Self::Let),
            Self::KEYWORD_TRUE => Some(Self::True),
            Self::KEYWORD_FALSE => Some(Self::False),
            Self::KEYWORD_IF => Some(Self::If),
            Self::KEYWORD_ELSE => Some(Self::Else),
            Self::KEYWORD_RETURN => Some(Self::Return),
            _ => None,
        }
    }

    // Returns a String literal of the token
    pub fn literal(&self) -> String {
        match self {
            Self::Assign => Self::SYMBOL_ASSIGN.to_string(),
            Self::Plus => Self::SYMBOL_PLUS.to_string(),
            Self::Minus => Self::SYMBOL_MINUS.to_string(),
            Self::Bang => Self::SYMBOL_BANG.to_string(),
            Self::Asterisk => Self::SYMBOL_ASTERISK.to_string(),
            Self::Slash => Self::SYMBOL_SLASH.to_string(),
            Self::LThan => Self::SYMBOL_LT.to_string(),
            Self::GThan => Self::SYMBOL_GT.to_string(),
            Self::Comma => Self::SYMBOL_COMMA.to_string(),
            Self::Semicolon => Self::SYMBOL_SEMICOLON.to_string(),
            Self::LParen => Self::SYMBOL_LPAREN.to_string(),
            Self::RParen => Self::SYMBOL_RPAREN.to_string(),
            Self::LBrace => Self::SYMBOL_LBRACE.to_string(),
            Self::RBrace => Self::SYMBOL_RBRACE.to_string(),
            Self::Eq => Self::SYMBOL_EQ.to_string(),
            Self::NotEq => Self::SYMBOL_NOT_EQ.to_string(),
            Self::Fn => Self::KEYWORD_FN.to_string(),
            Self::Let => Self::KEYWORD_LET.to_string(),
            Self::True => Self::KEYWORD_TRUE.to_string(),
            Self::False => Self::KEYWORD_FALSE.to_string(),
            Self::If => Self::KEYWORD_IF.to_string(),
            Self::Else => Self::KEYWORD_ELSE.to_string(),
            Self::Return => Self::KEYWORD_RETURN.to_string(),
            Self::Identifier(s) => s.to_string(),
            Self::Int(s) => s.to_string(),
        }
    }
}
