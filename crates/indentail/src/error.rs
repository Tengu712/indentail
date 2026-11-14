use std::{error, fmt, io};

#[derive(Debug)]
pub enum IndentailError {
    Syntax {
        line: usize,
        column: usize,
        detail: SyntaxErrorDetail,
    },
    IO(io::Error),
    DuplicateAttribute(String),
}

impl fmt::Display for IndentailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Syntax {
                line,
                column,
                detail,
            } => write!(f, "{line}:{column}: {detail}"),
            Self::IO(e) => write!(f, "{e}"),
            Self::DuplicateAttribute(v) => write!(f, "duplicate attribute '{v}' found"),
        }
    }
}

impl error::Error for IndentailError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Syntax { .. } | Self::DuplicateAttribute(_) => None,
            Self::IO(e) => Some(e),
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
