#[derive(Debug, PartialEq, Eq)]
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

    // end of file
    EOF,

    // Invalid token
    Invalid(char, usize), // Invalid token (character, position)
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
    pub fn keyword_or_identifier(kw: &String) -> Self {
        match kw.as_str() {
            Self::KW_FN => Self::Fn,
            Self::KW_LET => Self::Let,
            Self::KW_TRUE => Self::True,
            Self::KW_FALSE => Self::False,
            Self::KW_IF => Self::If,
            Self::KW_ELSE => Self::Else,
            Self::KW_RETURN => Self::Return,
            _ => Self::Identifier(kw.clone()),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EOF => write!(f, "EOF"),
            Self::Invalid(c, pos) => write!(f, "Invalid char '{}' at position {}", c, pos),
            Self::Assign => write!(f, "{}", Self::SYM_ASSIGN),
            Self::Plus => write!(f, "{}", Self::SYM_PLUS),
            Self::Minus => write!(f, "{}", Self::SYM_MINUS),
            Self::Bang => write!(f, "{}", Self::SYM_BANG),
            Self::Asterisk => write!(f, "{}", Self::SYM_ASTERISK),
            Self::Slash => write!(f, "{}", Self::SYM_SLASH),
            Self::LThan => write!(f, "{}", Self::SYM_LT),
            Self::GThan => write!(f, "{}", Self::SYM_GT),
            Self::Comma => write!(f, "{}", Self::SYM_COMMA),
            Self::Semicolon => write!(f, "{}", Self::SYM_SEMICOLON),
            Self::LParen => write!(f, "{}", Self::SYM_LPAREN),
            Self::RParen => write!(f, "{}", Self::SYM_RPAREN),
            Self::LBrace => write!(f, "{}", Self::SYM_LBRACE),
            Self::RBrace => write!(f, "{}", Self::SYM_RBRACE),
            Self::Eq => write!(f, "{}", Self::SYM_EQ),
            Self::NotEq => write!(f, "{}", Self::SYM_NOT_EQ),
            Self::Fn => write!(f, "{}", Self::KW_FN),
            Self::Let => write!(f, "{}", Self::KW_LET),
            Self::True => write!(f, "{}", Self::KW_TRUE),
            Self::False => write!(f, "{}", Self::KW_FALSE),
            Self::If => write!(f, "{}", Self::KW_IF),
            Self::Else => write!(f, "{}", Self::KW_ELSE),
            Self::Return => write!(f, "{}", Self::KW_RETURN),
            Self::Identifier(s) => write!(f, "{}", s),
            Self::Int(s) => write!(f, "{}", s),
        }
    }
}
