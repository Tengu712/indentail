use std::{collections::HashMap, fs, path::Path};

pub mod error;
mod format;
mod parser;

pub use format::xml;
pub use parser::parse;

use error::*;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Node {
    pub tag: String,
    pub options: HashMap<String, String>,
    pub modifiers: Vec<(String, String)>,
    pub children: Vec<Node>,
}

pub fn parse_from_file(path: &Path) -> Result<Vec<Node>, TailortailError> {
    parse(fs::read_to_string(path).map_err(TailortailError::IO)?)
}
