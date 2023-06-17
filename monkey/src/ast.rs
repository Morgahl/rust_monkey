use std::iter::Peekable;

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
        match self.statements.first() {
            Some(stmt) => stmt.token_literal(),
            None => "".to_string(),
        }
    }
}

pub struct LetStatement<Expr>
where
    Expr: Expression,
{
    token: Token,
    name: Identifier,
    value: Expr,
}

impl<Expr> Node for LetStatement<Expr>
where
    Expr: Expression,
{
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl<Expr> Statement for LetStatement<Expr> where Expr: Expression {}

struct Identifier {
    token: Token,
    value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Expression for Identifier {}

pub struct Parser<'a> {
    lexer: Peekable<&'a mut Lexer<'a>>,
}

impl Parser<'_> {
    pub fn new<'a>(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        Parser { lexer: lexer.peekable() }
    }
}
