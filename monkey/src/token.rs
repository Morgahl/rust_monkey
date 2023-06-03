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
    pub const SYM_ASSIGN: char = '=';
    pub const SYM_PLUS: char = '+';
    pub const SYM_MINUS: char = '-';
    pub const SYM_BANG: char = '!';
    pub const SYM_ASTERISK: char = '*';
    pub const SYM_SLASH: char = '/';
    pub const SYM_LT: char = '<';
    pub const SYM_GT: char = '>';
    pub const SYM_COMMA: char = ',';
    pub const SYM_SEMICOLON: char = ';';
    pub const SYM_LPAREN: char = '(';
    pub const SYM_RPAREN: char = ')';
    pub const SYM_LBRACE: char = '{';
    pub const SYM_RBRACE: char = '}';

    // Double character tokens
    pub const SYM_EQ: &'static str = "==";
    pub const SYM_NOT_EQ: &'static str = "!=";

    // Keywords
    pub const KW_FN: &'static str = "fn";
    pub const KW_LET: &'static str = "let";
    pub const KW_TRUE: &'static str = "true";
    pub const KW_FALSE: &'static str = "false";
    pub const KW_IF: &'static str = "if";
    pub const KW_ELSE: &'static str = "else";
    pub const KW_RETURN: &'static str = "return";

    // Returns a keyword token if the given string is a keyword, otherwise returns None
    pub fn keyword_or_identifier(kw_str: String) -> Self {
        match kw_str.as_str() {
            Self::KW_FN => Self::Fn,
            Self::KW_LET => Self::Let,
            Self::KW_TRUE => Self::True,
            Self::KW_FALSE => Self::False,
            Self::KW_IF => Self::If,
            Self::KW_ELSE => Self::Else,
            Self::KW_RETURN => Self::Return,
            _ => Self::Identifier(kw_str),
        }
    }

    // Returns a String literal of the token
    pub fn literal(&self) -> String {
        match self {
            Self::Assign => Self::SYM_ASSIGN.to_string(),
            Self::Plus => Self::SYM_PLUS.to_string(),
            Self::Minus => Self::SYM_MINUS.to_string(),
            Self::Bang => Self::SYM_BANG.to_string(),
            Self::Asterisk => Self::SYM_ASTERISK.to_string(),
            Self::Slash => Self::SYM_SLASH.to_string(),
            Self::LThan => Self::SYM_LT.to_string(),
            Self::GThan => Self::SYM_GT.to_string(),
            Self::Comma => Self::SYM_COMMA.to_string(),
            Self::Semicolon => Self::SYM_SEMICOLON.to_string(),
            Self::LParen => Self::SYM_LPAREN.to_string(),
            Self::RParen => Self::SYM_RPAREN.to_string(),
            Self::LBrace => Self::SYM_LBRACE.to_string(),
            Self::RBrace => Self::SYM_RBRACE.to_string(),
            Self::Eq => Self::SYM_EQ.to_string(),
            Self::NotEq => Self::SYM_NOT_EQ.to_string(),
            Self::Fn => Self::KW_FN.to_string(),
            Self::Let => Self::KW_LET.to_string(),
            Self::True => Self::KW_TRUE.to_string(),
            Self::False => Self::KW_FALSE.to_string(),
            Self::If => Self::KW_IF.to_string(),
            Self::Else => Self::KW_ELSE.to_string(),
            Self::Return => Self::KW_RETURN.to_string(),
            Self::Identifier(s) => s.to_string(),
            Self::Int(s) => s.to_string(),
        }
    }
}
