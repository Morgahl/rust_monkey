use std::iter::{Enumerate, Peekable};
use std::str::Chars;

use crate::token::Token;

pub trait Lexable<'a> {
    fn lex(self) -> Lexer<'a>;
}

impl<'a, I> Lexable<'a> for I
where
    I: Into<&'a str>,
{
    #[inline(always)]
    fn lex(self) -> Lexer<'a> {
        Lexer::new(self.into())
    }
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Lexer<'a> {
    scratch: String,
    chars: Option<Peekable<Enumerate<Chars<'a>>>>,
}

impl<'a> Lexer<'a> {
    #[inline(always)]
    fn new(input: &'a str) -> Lexer<'a> {
        // 52 is the max length of an underscore delimited 128 signed negative integer literal so we chose the next
        // highest power of 2 (64) to avoid reallocations.
        let scratch = String::with_capacity(64);
        let chars = Some(input.chars().enumerate().peekable());
        Lexer { scratch, chars }
    }

    #[inline(always)]
    fn read_then_else(&mut self, c: char, then: Token, els: Token) -> Token {
        self.chars
            .as_mut()
            .expect("read_then_else should always be called with a Some(chars)")
            .next();
        match self
            .chars
            .as_mut()
            .expect("read_then_else should always be called with a Some(chars)")
            .next_if(|(_, next)| *next == c)
        {
            Some(_) => then,
            None => els,
        }
    }

    #[inline(always)]
    fn read_keyword_or_identifier(&mut self) -> Token {
        self.scratch.clear();
        while let Some((_, c)) = self
            .chars
            .as_mut()
            .expect("read_keyword_or_identifier should always be called with a Some(chars)")
            .next_if(|(_, c)| c.is_ascii_alphanumeric())
        {
            self.scratch.push(c);
        }
        Token::keyword_or_identifier(&self.scratch)
    }

    #[inline(always)]
    fn read_integer(&mut self) -> Token {
        self.scratch.clear();
        while let Some((_, c)) = self
            .chars
            .as_mut()
            .expect("read_integer should always be called with a Some(chars)")
            .next_if(|(_, c)| matches!(*c, '0'..='9' | '_'))
        {
            self.scratch.push(c);
        }
        Token::Int(self.scratch.clone())
    }

    #[inline(always)]
    fn read_token(&mut self, token: Token) -> Token {
        self.chars.as_mut().expect("read_token should always be called with a Some(chars)").next();
        token
    }

    #[inline(always)]
    fn skip_whitespace(&mut self) {
        while let Some(_) = self
            .chars
            .as_mut()
            .expect("skip_whitespace should always be called with a Some(chars)")
            .next_if(|(_, c)| c.is_whitespace())
        {}
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.chars.is_none() {
            return None;
        }

        self.skip_whitespace();
        let token = match self.chars.as_mut().unwrap().peek() {
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
            Some((n, c)) => Token::Invalid(*c, *n),
            None => Token::EOF,
        };
        if matches!(token, Token::Invalid(_, _) | Token::EOF) {
            self.chars = None;
        }
        Some(token)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
