use lexer::Token;
use syntax::SyntaxKind;

pub(crate) struct Source<'t, 'input> {
    tokens: &'t [Token<'input>],
    cursor: usize,
}

impl<'t, 'input> Source<'t, 'input> {
    pub(crate) fn new(tokens: &'t [Token<'input>]) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub(crate) fn next_token(&mut self) -> Option<&'t Token<'input>> {
        self.eat_trivia();

        let token = self.tokens.get(self.cursor)?;
        self.cursor += 1;

        Some(token)
    }

    pub(crate) fn peek_kind(&mut self) -> Option<Result<SyntaxKind, ()>> {
        self.eat_trivia();
        self.peek_kind_raw()
    }

    fn eat_trivia(&mut self) {
        while self.at_trivia() {
            self.cursor += 1;
        }
    }

    fn at_trivia(&self) -> bool {
        self.peek_kind_raw()
            .map_or(false, |kind| kind.is_ok_and(|x| SyntaxKind::is_trivia(x)))
    }

    fn peek_kind_raw(&self) -> Option<Result<SyntaxKind, ()>> {
        self.tokens
            .get(self.cursor)
            .map(|Token { kind, .. }| match *kind {
                Ok(kind) => Ok(kind.into()),
                Err(kind) => Err(kind.into()),
            })
    }
}
