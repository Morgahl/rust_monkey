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
        while let Some((_, c)) = self.chars.next_if(|(_, c)| c.is_ascii_alphanumeric()) {
            ident.push(c);
        }
        Token::keyword_or_identifier(ident)
    }

    fn read_integer(&mut self) -> Token {
        let mut int = String::new();
        while let Some((_, c)) = self.chars.next_if(|(_, c)| c.is_ascii_digit()) {
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
            Some((_, Token::SYM_ASSIGN)) => self.read_then_else('=', Token::Eq, Token::Assign),
            Some((_, Token::SYM_PLUS)) => self.read_token(Token::Plus),
            Some((_, Token::SYM_MINUS)) => self.read_token(Token::Minus),
            Some((_, Token::SYM_BANG)) => self.read_then_else('=', Token::NotEq, Token::Bang),
            Some((_, Token::SYM_ASTERISK)) => self.read_token(Token::Asterisk),
            Some((_, Token::SYM_SLASH)) => self.read_token(Token::Slash),
            Some((_, Token::SYM_LT)) => self.read_token(Token::LThan),
            Some((_, Token::SYM_GT)) => self.read_token(Token::GThan),
            Some((_, Token::SYM_COMMA)) => self.read_token(Token::Comma),
            Some((_, Token::SYM_SEMICOLON)) => self.read_token(Token::Semicolon),
            Some((_, Token::SYM_LPAREN)) => self.read_token(Token::LParen),
            Some((_, Token::SYM_RPAREN)) => self.read_token(Token::RParen),
            Some((_, Token::SYM_LBRACE)) => self.read_token(Token::LBrace),
            Some((_, Token::SYM_RBRACE)) => self.read_token(Token::RBrace),
            Some((_, c)) if c.is_ascii_alphabetic() => self.read_keyword_or_identifier(),
            Some((_, c)) if c.is_ascii_digit() => self.read_integer(),
            Some((n, c)) => bail!("unexpected character: {:?} at {}", c, n),
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
