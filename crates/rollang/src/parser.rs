mod event;
mod expr;
mod marker;
mod sink;
mod source;

use crate::syntax::SyntaxNode;
use crate::syntax::{SyntaxKind, SyntaxNode};
use event::Event;
use expr::expr;
use lexer::{Lexer, Token};
use marker::Marker;
use rowan::GreenNode;
use sink::Sink;
use source::Source;

struct Parser<'t, 'input> {
    source: Source<'t, 'input>,
    events: Vec<Event>,
}

impl<'t, 'input> Parser<'t, 'input> {
    pub fn new(tokens: &'t [Token<'input>]) -> Self {
        Self {
            source: Source::new(tokens),
            events: Vec::new(),
        }
    }

    fn parse(mut self) -> Vec<Event> {
        let m = self.start();
        expr(&mut self);
        m.complete(&mut self, SyntaxKind::Root);

        self.events
    }

    fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);
        Marker::new(pos)
    }

    fn at(&mut self, kind: SyntaxKind) -> bool {
        self.peek() == Some(Ok(kind))
    }

    // fn start_node(&mut self, kind: SyntaxKind) {
    //     self.events.push(Event::StartNode {
    //         kind,
    //         forward_parent: None,
    //     });
    // }

    // fn start_node_at(&mut self, checkpoint: usize, kind: SyntaxKind) {
    //     self.events.push(Event::StartNodeAt { kind, checkpoint });
    // }

    // fn finish_node(&mut self) {
    //     self.events.push(Event::FinishNode);
    // }

    fn bump(&mut self) {
        self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
    }

    // fn checkpoint(&self) -> usize {
    //     self.events.len()
    // }

    fn peek(&mut self) -> Option<Result<SyntaxKind, ()>> {
        self.source.peek_kind()
    }
}

pub struct Parse {
    green_node: GreenNode,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        // We cut off the last byte because formatting the SyntaxNode adds a newline at the end.
        formatted[0..formatted.len() - 1].to_string()
    }
}

pub fn parse(input: &str) -> Parse {
    let tokens: Vec<_> = Lexer::new(input).collect();
    let parser = Parser::new(&tokens);
    let events = parser.parse();
    let sink = Sink::new(&tokens, events);

    Parse {
        green_node: sink.finish(),
    }
}

#[cfg(test)]
fn check(input: &str, expected_tree: expect_test::Expect) {
    let parse = parse(input);
    expected_tree.assert_eq(&parse.debug_tree());
}

#[cfg(test)]
mod tests {
    use super::*;
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
