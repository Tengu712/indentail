use std::{collections::HashMap, fs, io, path::PathBuf};

mod parser;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Node {
    pub tag: String,
    pub options: HashMap<String, String>,
    pub modifiers: Vec<(String, String)>,
    pub children: Vec<Node>,
}

#[derive(Debug)]
pub enum IndentailError {
    Syntax {
        line: usize,
        column: usize,
        detail: SyntaxErrorDetail,
    },
    IO(io::Error),
}

#[derive(Debug, PartialEq)]
pub enum SyntaxErrorDetail {
    InvalidToken(String),
    InvalidColon,
    InvalidComma,
    InvalidDot,
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
    FilePath(PathBuf),
}

impl Input {
    fn get(self) -> Result<String, IndentailError> {
        match self {
            Self::Inline(v) => Ok(v),
            Self::FilePath(v) => fs::read_to_string(v).map_err(IndentailError::IO),
        }
    }
}

pub fn parse(option: Options) -> Result<Vec<Node>, IndentailError> {
    parser::parse(option.input.get()?)
}
