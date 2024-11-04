use cstree::{
    build::{Checkpoint, GreenNodeBuilder, NodeCache},
    interning::Interner,
    text::TextRange,
};

use crate::syntax::*;

mod grammar;

#[cfg(test)]
mod tests;

const ASSIGNMENT_OPS: &[SyntaxKind] = &[
    SyntaxKind::Equals,
    SyntaxKind::PlusEquals,
    SyntaxKind::MinusEquals,
    SyntaxKind::StarEquals,
    SyntaxKind::SlashEquals,
    SyntaxKind::PercentEquals,
];

const TRIVIA_TOKENS: &[SyntaxKind] = &[
    SyntaxKind::Comment,
    SyntaxKind::Newlines,
    SyntaxKind::Whitespace,
];

#[derive(Debug)]
pub struct ParseError {
    pub expected: Vec<SyntaxKind>,
    pub found: SyntaxKind,
    pub range: TextRange,
}

pub fn parse(
    input: &str,
    node_cache: &mut NodeCache<'_, impl Interner>,
) -> (SyntaxNode, Vec<ParseError>) {
    let mut parser = Parser::new(input, node_cache);
    grammar::file(&mut parser);
    parser.finish()
}

struct Parser<'input, 'cache, 'interner, I> {
    input: &'input str,
    tokens: Vec<SyntaxKind>,
    ranges: Vec<TextRange>,
    pos: usize,
    errors: Vec<ParseError>,
    builder: GreenNodeBuilder<'cache, 'interner, SyntaxKind, I>,
}

impl<'input, 'cache, 'interner, I: Interner> Parser<'input, 'cache, 'interner, I> {
    fn new(input: &'input str, node_cache: &'cache mut NodeCache<'interner, I>) -> Self {
        let (tokens, ranges) = SyntaxKind::lexer(input).unzip();

        Parser {
            input,
            tokens,
            ranges,
            pos: 0,
            errors: Vec::new(),
            builder: GreenNodeBuilder::with_cache(node_cache),
        }
    }

    fn finish(self) -> (SyntaxNode, Vec<ParseError>) {
        (SyntaxNode::new_root(self.builder.finish().0), self.errors)
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(kind);
    }

    fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.builder.start_node_at(checkpoint, kind);
    }

    fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    fn checkpoint(&self) -> Checkpoint {
        self.builder.checkpoint()
    }

    fn skip_trivia(&mut self) {
        while let Some(kind) = self.tokens.get(self.pos).copied() {
            if !TRIVIA_TOKENS.contains(&kind) {
                break;
            }

            let text = &self.input[self.ranges[self.pos]];
            self.builder.token(kind, text);
            self.pos += 1;
        }
    }

    fn next(&mut self) -> SyntaxKind {
        self.skip_trivia();

        if let Some(kind) = self.tokens.get(self.pos).copied() {
            let span = self.ranges[self.pos];
            let text = &self.input[span];
            self.builder.token(kind, text);
            self.pos += 1;
            kind
        } else {
            SyntaxKind::Eof
        }
    }

    fn peek(&self) -> SyntaxKind {
        self.tokens
            .get(self.pos..)
            .and_then(|tokens| tokens.iter().find(|token| !TRIVIA_TOKENS.contains(*token)))
            .copied()
            .unwrap_or(SyntaxKind::Eof)
    }

    fn at(&self, kind: SyntaxKind) -> bool {
        self.peek() == kind
    }

    fn at_any(&self, kinds: &[SyntaxKind]) -> bool {
        kinds.contains(&self.peek())
    }

    fn eof(&self) -> bool {
        self.at(SyntaxKind::Eof)
    }

    fn consume(&mut self, kind: SyntaxKind) -> bool {
        let consumed = self.at(kind);
        if consumed {
            self.next();
        }
        consumed
    }

    fn expect(&mut self, expected: SyntaxKind) -> bool {
        let consumed = self.consume(expected);
        if !consumed {
            self.next_error(&[expected]);
        }
        consumed
    }

    fn next_error(&mut self, expected: &[SyntaxKind]) {
        self.start_node(SyntaxKind::ErrorTree);
        let found = self.next();
        self.errors.push(ParseError {
            expected: expected.to_vec(),
            found,
            range: self.ranges.get(self.pos - 1).copied().unwrap_or_default(),
        });
        self.finish_node();
    }

    fn recover_to(&mut self, recovery: &[SyntaxKind]) {
        self.start_node(SyntaxKind::ErrorTree);
        while !self.at_any(recovery) && !self.eof() {
            self.next();
        }
        self.finish_node();
    }
}
