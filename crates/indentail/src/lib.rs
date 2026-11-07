use std::collections::HashMap;

mod parser;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Node {
    pub tag: String,
    pub options: HashMap<String, String>,
    pub modifiers: Vec<(String, String)>,
    pub children: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub enum IndentailError {
    Syntax {
        line: usize,
        column: usize,
        detail: SyntaxErrorDetail,
    },
}

#[derive(Debug, PartialEq)]
pub enum SyntaxErrorDetail {
    InvalidToken(String),
    InvalidColon,
    InvalidComma,
    InvalidLeftParen,
    InvalidRightParen,
    UnclosedParen,
    InvalidLeftBrace,
    InvalidRightBrace,
    UnclosedBrace,
    InvalidEscape(char),
    UnclosedQuote,
}

#[derive(Debug, Clone)]
pub struct Options {
    pub input: Input,
}

#[derive(Debug, Clone)]
pub enum Input {
    Inline(String),
    FilePath(String),
}

impl Input {
    fn get(self) -> Result<String, IndentailError> {
        match self {
            Self::Inline(v) => Ok(v),
            Self::FilePath(_) => unimplemented!(),
        }
    }
}

pub fn parse(option: Options) -> Result<Vec<Node>, IndentailError> {
    parser::parse(option.input.get()?)
}
