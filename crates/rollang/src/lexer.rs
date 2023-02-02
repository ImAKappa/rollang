use logos::Logos;

#[derive(Debug, Copy, Clone, PartialEq, Logos)]
pub(crate) enum SyntaxKind {
    // Data
    Dice,
    Number,
    String,

    // Keywords
    FnKw,
    RollKw,

    // Other
    #[regex(" +")]
    Whitespace,

    #[error]
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_spaces() {
        let mut lexer = SyntaxKind::lexer("  ");

        assert_eq!(lexer.next(), Some(SyntaxKind::Whitespace));
        assert_eq!(lexer.slice(), "  ");
    }
}