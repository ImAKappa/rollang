use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(
    Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Logos, FromPrimitive, ToPrimitive, Hash,
)]
pub enum SyntaxKind {
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

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("end")]
    End,

    #[regex("#.*")]
    Comment,

    Error,
    Root,
    BinaryExpr,
    DiceExpr,
}

impl SyntaxKind {
    pub(crate) fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
}

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, SyntaxKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: SyntaxKind::lexer(input),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Lexeme<'a> {
    pub(crate) kind: Result<SyntaxKind, ()>,
    pub(crate) text: &'a str,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Lexeme<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();

        Some(Self::Item { kind, text })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: SyntaxKind) {
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next(),
            Some(Lexeme {
                kind: Ok(kind),
                text: input
            })
        );
    }

    #[test]
    fn lex_whitespace() {
        check("   ", SyntaxKind::Whitespace);
    }

    #[test]
    fn lex_number() {
        check("200", SyntaxKind::Number);
        check("24518672", SyntaxKind::Number);
    }

    #[test]
    fn lex_dice() {
        check("d20", SyntaxKind::Dice);
        check("1d20", SyntaxKind::Dice);
        check("100d24", SyntaxKind::Dice);
    }

    #[test]
    fn lex_alphabetic_identifier() {
        check("abcd", SyntaxKind::Ident);
    }

    #[test]
    fn lex_alphanumeric_identifier() {
        check("ab123cde456", SyntaxKind::Ident);
    }

    #[test]
    fn lex_mixed_case_identifier() {
        check("ABCdef", SyntaxKind::Ident);
    }

    #[test]
    fn lex_single_char_identifier() {
        check("x", SyntaxKind::Ident);
    }

    #[test]
    fn lex_left_parenthesis() {
        check("(", SyntaxKind::LParen);
    }

    #[test]
    fn lex_right_parenthesis() {
        check(")", SyntaxKind::RParen);
    }

    #[test]
    fn lex_end() {
        check("end", SyntaxKind::End);
    }

    #[test]
    fn lex_plus() {
        check("+", SyntaxKind::Plus);
    }

    #[test]
    fn lex_minus() {
        check("-", SyntaxKind::Minus);
    }

    #[test]
    fn lex_comment() {
        check("# foo", SyntaxKind::Comment);
    }

    #[test]
    fn lex_newlines() {
        check(" \n ", SyntaxKind::Whitespace);
    }
}
