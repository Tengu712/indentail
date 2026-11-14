use std::{error, fmt, io};

#[derive(Debug)]
pub enum TailortailError {
    Syntax {
        line: usize,
        column: usize,
        detail: SyntaxErrorDetail,
    },
    IO(io::Error),
    DuplicateAttribute(String),
    MultipleTopLevelNodes,
}

impl fmt::Display for TailortailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Syntax {
                line,
                column,
                detail,
            } => write!(f, "{line}:{column}: {detail}"),
            Self::IO(e) => write!(f, "{e}"),
            Self::DuplicateAttribute(v) => write!(f, "duplicate attribute '{v}' found"),
            Self::MultipleTopLevelNodes => write!(f, "multiple top-level nodes found"),
        }
    }
}

impl error::Error for TailortailError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::IO(e) => Some(e),
            _ => None,
        }
    }
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
    DuplicateOption(String),
}

impl fmt::Display for SyntaxErrorDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidToken(v) => write!(f, "unexpected token '{v}' found"),
            Self::InvalidColon => write!(f, "unexpected ':' found"),
            Self::InvalidComma => write!(f, "unexpected ',' found"),
            Self::InvalidDot => write!(f, "unexpected '.' found"),
            Self::InvalidLeftParen => write!(f, "unexpected '(' found"),
            Self::InvalidRightParen => write!(f, "unexpected ')' found"),
            Self::UnclosedParen => write!(f, "unclosed '(' found"),
            Self::InvalidLeftBrace => write!(f, "unexpected '{{' found"),
            Self::InvalidRightBrace => write!(f, "unexpected '}}' found"),
            Self::UnclosedBrace => write!(f, "unclosed '{{' found"),
            Self::InvalidEscape(c) => write!(f, "invalid escape '\\{c}' found"),
            Self::UnclosedQuote => write!(f, "unclosed quote found"),
            Self::DuplicateOption(v) => write!(f, "duplicate option '{v}' found"),
        }
    }
}
