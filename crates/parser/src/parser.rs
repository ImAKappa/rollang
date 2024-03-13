pub(crate) mod marker;

use crate::event::Event;
use crate::grammar;
use crate::source::Source;
use marker::Marker;
use syntax::SyntaxKind;

const RECOVERY_SET: [SyntaxKind; 1] = [SyntaxKind::LetKw];

pub(crate) struct Parser<'t, 'input> {
    source: Source<'t, 'input>,
    events: Vec<Event>,
}

impl<'t, 'input> Parser<'t, 'input> {
    pub(crate) fn new(source: Source<'t, 'input>) -> Self {
        Self {
            source,
            events: Vec::new(),
        }
    }

    pub(crate) fn parse(mut self) -> Vec<Event> {
        grammar::root(&mut self);
        self.events
    }

    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);
        Marker::new(pos)
    }

    pub(crate) fn bump(&mut self) {
        self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
    }

    pub(crate) fn at(&mut self, kind: Result<SyntaxKind, ()>) -> bool {
        self.peek() == Some(kind)
    }

    pub(crate) fn peek(&mut self) -> Option<Result<SyntaxKind, ()>> {
        self.source.peek_kind()
    }

    pub(crate) fn expect(&mut self, kind: Result<SyntaxKind, ()>) {
        if self.at(kind) {
            self.bump();
        } else {
            self.error();
        }
    }

    pub(crate) fn error(&mut self) {
        if !self.at_set(&RECOVERY_SET) && !self.at_end() {
            self.bump();
        }
    }

    fn at_set(&mut self, set: &[SyntaxKind]) -> bool {
        self.peek().map_or(false, |k| match k {
            Ok(k) => RECOVERY_SET.contains(&k),
            Err(_) => false,
        })
    }

    pub(crate) fn at_end(&mut self) -> bool {
        self.peek().is_none()
    }
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }

    #[test]
    fn parse_whitespace() {
        check(
            "   ",
            expect![[r#"
Root@0..3
  Whitespace@0..3 "   ""#]],
        );
    }

    #[test]
    fn parse_comment() {
        check(
            "# hello!",
            expect![[r##"
Root@0..8
  Comment@0..8 "# hello!""##]],
        );
    }

    #[test]
    fn parse_binary_expression_interspersed_with_comments() {
        check(
            "
1
  + 1 # Add one
  + 10 # Add ten",
            expect![[r##"
Root@0..35
  Whitespace@0..1 "\n"
  InfixExpr@1..35
    InfixExpr@1..21
      Number@1..2 "1"
      Whitespace@2..5 "\n  "
      Plus@5..6 "+"
      Whitespace@6..7 " "
      Number@7..8 "1"
      Whitespace@8..9 " "
      Comment@9..18 "# Add one"
      Whitespace@18..21 "\n  "
    Plus@21..22 "+"
    Whitespace@22..23 " "
    Number@23..25 "10"
    Whitespace@25..26 " "
    Comment@26..35 "# Add ten""##]],
        );
    }
}
