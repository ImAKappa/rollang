use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(
    Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Logos, FromPrimitive, ToPrimitive, Hash,
)]
pub enum SyntaxKind {
    #[regex(" +")]
    Whitespace,

    #[token("d")]
    D,

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

    #[token("end")]
    End,

    Error,
    Root,
    BinaryExpr,
    DiceExpr,
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

impl<'a> Iterator for Lexer<'a> {
    type Item = (Result<SyntaxKind, ()>, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();

        Some((kind, text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: SyntaxKind) {
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some((Ok(kind), input)));
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
    fn lex_diceop() {
        check("d", SyntaxKind::D);
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
    fn lex_valid_dice() {
        let command = "200d12";
        let mut lexer = SyntaxKind::lexer(command);

        assert_eq!(lexer.next(), Some(Ok(SyntaxKind::Number)));
        assert_eq!(lexer.slice(), "200");

        assert_eq!(lexer.next(), Some(Ok(SyntaxKind::D)));
        assert_eq!(lexer.slice(), "d");

        assert_eq!(lexer.next(), Some(Ok(SyntaxKind::Number)));
        assert_eq!(lexer.slice(), "12");
    }

    #[test]
    fn lex_invalid_qty_of_dice() {
        let command = "ad20";
        let mut lexer = SyntaxKind::lexer(command);
        assert_eq!(lexer.next(), Some(Err(())));
        assert_eq!(lexer.slice(), "a");

        let command = "02d8";
        let mut lexer = SyntaxKind::lexer(command);
        assert_eq!(lexer.next(), Some(Err(())));
        assert_eq!(lexer.slice(), "0");
    }
}
