use std::{collections::HashMap, fs, path::PathBuf};

pub mod error;
mod format;
mod parser;
pub mod xml;

use error::*;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Node {
    pub tag: String,
    pub options: HashMap<String, String>,
    pub modifiers: Vec<(String, String)>,
    pub children: Vec<Node>,
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
