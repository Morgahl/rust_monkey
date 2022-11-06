use std::iter::{Enumerate, Peekable};
use std::str::Chars;

use anyhow::{bail, Result};
use fallible_iterator::FallibleIterator;

use crate::token::Token;

pub struct Lexer<'a> {
    chars: Peekable<Enumerate<Chars<'a>>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            chars: input.chars().enumerate().peekable(),
        }
    }

    fn read_then_else(&mut self, c: char, then: Token, els: Token) -> Token {
        self.chars.next();
        self.chars
            .next_if(|(_, next)| *next == c)
            .map(|_| then)
            .unwrap_or(els)
    }

    fn read_keyword_or_identifier(&mut self) -> Token {
        let mut ident = String::new();
        while let Some((_, c)) = self.chars.next_if(|(_, c)| c.is_alphanumeric()) {
            ident.push(c);
        }
        Token::keyword(&ident).unwrap_or_else(|| Token::Identifier(ident))
    }

    fn read_integer(&mut self) -> Token {
        let mut int = String::new();
        while let Some((_, c)) = self.chars.next_if(|(_, c)| c.is_numeric()) {
            int.push(c);
        }
        Token::Int(int)
    }

    fn read_token(&mut self, token: Token) -> Token {
        self.chars.next();
        token
    }

    fn skip_whitespace(&mut self) {
        while let Some(_) = self.chars.next_if(|(_, c)| c.is_whitespace()) {}
    }

    fn next_token(&mut self) -> Result<Option<Token>> {
        self.skip_whitespace();
        let token = match self.chars.peek() {
            Some((_, Token::SYMBOL_ASSIGN)) => self.read_then_else('=', Token::Eq, Token::Assign),
            Some((_, Token::SYMBOL_PLUS)) => self.read_token(Token::Plus),
            Some((_, Token::SYMBOL_MINUS)) => self.read_token(Token::Minus),
            Some((_, Token::SYMBOL_BANG)) => self.read_then_else('=', Token::NotEq, Token::Bang),
            Some((_, Token::SYMBOL_ASTERISK)) => self.read_token(Token::Asterisk),
            Some((_, Token::SYMBOL_SLASH)) => self.read_token(Token::Slash),
            Some((_, Token::SYMBOL_LT)) => self.read_token(Token::LThan),
            Some((_, Token::SYMBOL_GT)) => self.read_token(Token::GThan),
            Some((_, Token::SYMBOL_COMMA)) => self.read_token(Token::Comma),
            Some((_, Token::SYMBOL_SEMICOLON)) => self.read_token(Token::Semicolon),
            Some((_, Token::SYMBOL_LPAREN)) => self.read_token(Token::LParen),
            Some((_, Token::SYMBOL_RPAREN)) => self.read_token(Token::RParen),
            Some((_, Token::SYMBOL_LBRACE)) => self.read_token(Token::LBrace),
            Some((_, Token::SYMBOL_RBRACE)) => self.read_token(Token::RBrace),
            Some((_, c)) if c.is_alphabetic() => self.read_keyword_or_identifier(),
            Some((_, c)) if c.is_numeric() => self.read_integer(),
            Some((n, c)) => bail!("unexpected character: {} at {}", c, n),
            None => return Ok(None),
        };
        Ok(Some(token))
    }
}

impl<'a> FallibleIterator for Lexer<'a> {
    type Item = Token;
    type Error = anyhow::Error;

    fn next(&mut self) -> Result<Option<Self::Item>> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_iterator() {
        let input = "let five = 5;\nlet test = 10;\nlet add = fn(x, y) {\n  x + y;\n};\nlet result = add(five, ten);\n!-/*5;\n5 < 10 > 5;\nif (5 < 10) {\n  return true;\n} else {\n  return false;\n}\n10 == 10;\n10 != 9;\n";
        let expected = vec![
            Token::Let,
            Token::Identifier("five".to_string()),
            Token::Assign,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("test".to_string()),
            Token::Assign,
            Token::Int("10".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("add".to_string()),
            Token::Assign,
            Token::Fn,
            Token::LParen,
            Token::Identifier("x".to_string()),
            Token::Comma,
            Token::Identifier("y".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Identifier("x".to_string()),
            Token::Plus,
            Token::Identifier("y".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Identifier("result".to_string()),
            Token::Assign,
            Token::Identifier("add".to_string()),
            Token::LParen,
            Token::Identifier("five".to_string()),
            Token::Comma,
            Token::Identifier("ten".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::Int("5".to_string()),
            Token::LThan,
            Token::Int("10".to_string()),
            Token::GThan,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int("5".to_string()),
            Token::LThan,
            Token::Int("10".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Int("10".to_string()),
            Token::Eq,
            Token::Int("10".to_string()),
            Token::Semicolon,
            Token::Int("10".to_string()),
            Token::NotEq,
            Token::Int("9".to_string()),
            Token::Semicolon,
        ];
        let mut lexer = Lexer::new(input);

        for expected_token in expected {
            assert_eq!(lexer.next().unwrap().unwrap(), expected_token);
        }

        assert_eq!(lexer.next().unwrap(), None);
        assert_eq!(lexer.next().unwrap(), None);
    }
}
