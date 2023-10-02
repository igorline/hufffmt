use ruint::aliases::U256;

use crate::opcode::Opcode;

#[derive(Debug)]
pub struct Comment {
    text: String,
}

#[derive(Debug)]
pub struct DocComment {
    text: String,
}

#[derive(Debug)]
pub enum WhitespaceKind {
    Space,
    Newline,
    Comment(Comment),
    DocComment(DocComment),
}

#[derive(Debug)]
pub struct Instruction {
    data: Vec<U256>,
    ws: Whitespace,
    kind: Opcode,
    span: Span,
}

#[derive(Debug)]
pub struct HexItem {
    value: U256,
    whitespace: Whitespace,
}

#[derive(Debug)]
pub struct Whitespace {
    whitespaces: Vec<WhitespaceKind>,
    span: Span,
}

#[derive(Debug)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Instruction {
    pub fn new(data: Vec<U256>, kind: Opcode) -> Self {
        println!("this");
        Instruction {
            data,
            ws: Whitespace::new(),
            kind,
            span: Span { start: 0, end: 0 },
        }
    }

    pub fn into_span(mut self, start: usize, end: usize) -> Self {
        self.span.start = start;
        self.span.end = end;
        self
    }
}

impl Whitespace {
    pub fn new() -> Self {
        Whitespace {
            whitespaces: vec![],
            span: Span { start: 0, end: 0 },
        }
    }
}
