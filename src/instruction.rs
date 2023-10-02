use ruint::aliases::U256;

use crate::opcode::Opcode;

#[derive(Debug)]
pub struct Comment {
    pub text: String,
}

#[derive(Debug)]
pub struct DocComment {
    pub text: String,
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
    pub data: Vec<U256>,
    pub ws: Whitespace,
    pub kind: Opcode,
    pub span: Span,
}

#[derive(Debug)]
pub struct HexItem {
    pub value: U256,
    pub whitespace: Whitespace,
}

#[derive(Debug, Default)]
pub struct Whitespace {
    pub whitespaces: Vec<WhitespaceKind>,
    pub span: Span,
}

#[derive(Debug, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Instruction {
    pub fn new(data: Vec<U256>, kind: Opcode) -> Self {
        println!("this");
        Instruction {
            data,
            ws: Whitespace::default(),
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
