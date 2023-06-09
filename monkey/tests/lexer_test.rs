use assert_matches::assert_matches;
use fallible_iterator::FallibleIterator;

use monkey::{lexer::Lexable, token::Token};

const INPUT: &str = "\
    let five = 5;\n\
    let test = 10;\n\
    let add = fn(x, y) {\n\
        x + y;\n\
    };\n\
    let result = add(five, ten);\n\
    !-/*5;\n\
    5 < 10 > 5;\n\
    if (5 < 10) {\n\
        return true;\n\
    } else {\n\
        return false;\n\
    }\n\
    10 == 10;\n\
    10 != 9;\n\
    ";

#[test]
fn test_lexer_iterator() {
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

    let mut lexer = INPUT.lex();

    for expected_token in expected {
        assert_matches!(lexer.next(), Ok(Some(token)) if token == expected_token);
    }

    assert_matches!(lexer.next(), Ok(None));
    // assert second call to next() after EOF returns None
    assert_matches!(lexer.next(), Ok(None));
}
