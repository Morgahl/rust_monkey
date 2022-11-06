use fallible_iterator::{FallibleIterator, Peekable};

use crate::{lexer::Lexer, token::Token};

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {}

pub trait Expression: Node {}

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}

pub struct LetStatement {
    token: Token,
    name: Identifier,
    value: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }
}

impl Statement for LetStatement {}

struct Identifier {
    token: Token,
    value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal()
    }
}

impl Expression for Identifier {}

pub struct Parser<'a> {
    lexer: Peekable<&'a mut Lexer<'a>>,
}

impl Parser<'_> {
    pub fn new<'a>(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer: lexer.peekable(),
        }
    }
}

impl FallibleIterator for Parser<'_> {
    type Item = Box<dyn Statement>;
    type Error = anyhow::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        Ok(None)
    }
}
