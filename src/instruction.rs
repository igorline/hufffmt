use std::fmt::Display;

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

#[derive(Debug)]
pub struct SourceUnit(pub Vec<Instruction>);

impl Display for SourceUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for instruction in &self.0 {
            writeln!(f, "{}", instruction)?;
        }
        Ok(())
    }
}

impl Instruction {
    pub fn new(data: Vec<U256>, kind: Opcode) -> Self {
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

fn get_bytes_slice_trimmed_zeros(bytes: &[u8]) -> &[u8] {
    let first_non_zero_index = bytes.iter().position(|&x| x != 0).unwrap_or(15);
    &bytes[first_non_zero_index..]
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for hex in &self.data {
            let bytes: [u8; 32] = hex.to_be_bytes();
            write!(
                f,
                "0x{} ",
                hex::encode(get_bytes_slice_trimmed_zeros(&bytes))
            )?;
        }
        write!(f, "{}", self.kind.as_static_str())?;
        Ok(())
    }
}
