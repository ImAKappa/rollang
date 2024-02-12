use super::event::Event;
use crate::lexer::{Lexeme, SyntaxKind};
use crate::syntax::RollangLanguage;
use rowan::{GreenNode, GreenNodeBuilder, Language};
use smol_str::SmolStr;

pub(super) struct Sink<'l, 'input> {
    builder: GreenNodeBuilder<'static>,
    lexemes: &'l [Lexeme<'input>],
    cursor: usize,
    events: Vec<Event>,
}

impl<'l, 'input> Sink<'l, 'input> {
    pub fn new(lexemes: &'l [Lexeme<'input>], events: Vec<Event>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            lexemes,
            cursor: 0,
            events,
        }
    }

    pub fn finish(mut self) -> GreenNode {
        let mut reordered_events = self.events.clone();

        for (idx, event) in self.events.iter().enumerate() {
            if let Event::StartNodeAt { kind, checkpoint } = event {
                reordered_events.remove(idx);
                reordered_events.insert(*checkpoint, Event::StartNode { kind: *kind });
            }
        }

        for event in reordered_events {
            match event {
                Event::StartNode { kind } => {
                    self.builder.start_node(RollangLanguage::kind_to_raw(kind))
                }
                Event::StartNodeAt {
                    kind: _,
                    checkpoint: _,
                } => unreachable!(),
                Event::AddToken { kind, text } => self.token(Ok(kind), text),
                Event::FinishNode => self.builder.finish_node(),
            }

            self.eat_whitespace();
        }

        self.builder.finish()
    }

    fn token(&mut self, kind: Result<SyntaxKind, ()>, text: SmolStr) {
        let kind = kind.unwrap_or(SyntaxKind::Error);
        self.builder
            .token(RollangLanguage::kind_to_raw(kind), text.as_str());
        self.cursor += 1;
    }

    fn eat_whitespace(&mut self) {
        while let Some(lexeme) = self.lexemes.get(self.cursor) {
            if lexeme.kind != Ok(SyntaxKind::Whitespace) {
                break;
            }

            self.token(lexeme.kind, lexeme.text.into())
        }
    }
}
