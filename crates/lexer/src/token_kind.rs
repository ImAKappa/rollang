use logos::Logos;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Logos, Hash)]
pub enum TokenKind {
    #[regex("[ \n]+")]
    Whitespace,

    #[regex("([1-9][0-9]*)?d[1-9][0-9]*")]
    Dice,

    #[regex("[A-Za-z][A-Za-z0-9]*")]
    Ident,

    #[regex("[1-9][0-9]*")]
    Number,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("=")]
    Equals,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[regex("#.*")]
    Comment,

    Error,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Lexer, Token};

    fn check(input: &str, kind: TokenKind) {
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next(),
            Some(Token {
                kind: Ok(kind),
                text: input
            })
        );
    }

    #[test]
    fn lex_whitespace() {
        check("   ", TokenKind::Whitespace);
    }

    #[test]
    fn lex_number() {
        check("200", TokenKind::Number);
        check("24518672", TokenKind::Number);
    }

    #[test]
    fn lex_dice() {
        check("d20", TokenKind::Dice);
        check("1d20", TokenKind::Dice);
        check("100d24", TokenKind::Dice);
    }

    #[test]
    fn lex_alphabetic_identifier() {
        check("abcd", TokenKind::Ident);
    }

    #[test]
    fn lex_alphanumeric_identifier() {
        check("ab123cde456", TokenKind::Ident);
    }

    #[test]
    fn lex_mixed_case_identifier() {
        check("ABCdef", TokenKind::Ident);
    }

    #[test]
    fn lex_single_char_identifier() {
        check("x", TokenKind::Ident);
    }

    #[test]
    fn lex_left_parenthesis() {
        check("(", TokenKind::LParen);
    }

    #[test]
    fn lex_right_parenthesis() {
        check(")", TokenKind::RParen);
    }

    #[test]
    fn lex_plus() {
        check("+", TokenKind::Plus);
    }

    #[test]
    fn lex_minus() {
        check("-", TokenKind::Minus);
    }

    #[test]
    fn lex_comment() {
        check("# foo", TokenKind::Comment);
    }

    #[test]
    fn lex_newlines() {
        check(" \n ", TokenKind::Whitespace);
    }
}
