use super::*;

pub(crate) enum OptionState {
    Empty,
    Keyed(String),
    Valuing(String),
    Valued((String, String)),
}

#[derive(Default)]
pub(crate) enum State {
    #[default]
    Empty,
    Tagged {
        tag: String,
    },
    Optioning {
        tag: String,
        options: HashMap<String, String>,
        state: OptionState,
    },
    Optioned {
        tag: String,
        options: HashMap<String, String>,
    },
    Modifying {
        tag: String,
        options: HashMap<String, String>,
        modifiers: Vec<(String, String)>,
        children: Vec<Node>,
    },
}

impl State {
    pub(crate) fn build(self) -> (Option<Node>, Self) {
        match self {
            Self::Tagged { tag } => (
                Some(Node {
                    tag,
                    ..Default::default()
                }),
                Self::Empty,
            ),
            Self::Optioned { tag, options } => (
                Some(Node {
                    tag,
                    options,
                    ..Default::default()
                }),
                Self::Empty,
            ),
            Self::Modifying {
                tag,
                options,
                modifiers,
                children,
            } => (
                Some(Node {
                    tag,
                    options,
                    modifiers,
                    children,
                }),
                Self::Empty,
            ),
            _ => (None, self),
        }
    }

    pub(crate) fn append(
        self,
        line_count: usize,
        char_count: usize,
        buffer: String,
    ) -> Result<Self, IndentailError> {
        match self {
            Self::Empty => Ok(Self::Tagged { tag: buffer }),
            Self::Optioning {
                tag,
                options,
                state: OptionState::Empty,
            } => Ok(Self::Optioning {
                tag,
                options,
                state: OptionState::Keyed(buffer),
            }),
            Self::Optioning {
                tag,
                options,
                state: OptionState::Valuing(key),
            } => Ok(Self::Optioning {
                tag,
                options,
                state: OptionState::Valued((key, buffer)),
            }),
            _ => Err(IndentailError::Syntax {
                line: line_count,
                column: char_count,
                detail: SyntaxErrorDetail::InvalidToken(buffer),
            }),
        }
    }

    pub(crate) fn open_paren(
        self,
        line_count: usize,
        char_count: usize,
    ) -> Result<Self, IndentailError> {
        match self {
            Self::Tagged { tag } => Ok(Self::Optioning {
                tag,
                options: Default::default(),
                state: OptionState::Empty,
            }),
            _ => Err(IndentailError::Syntax {
                line: line_count,
                column: char_count,
                detail: SyntaxErrorDetail::InvalidLeftParen,
            }),
        }
    }

    pub(crate) fn close_paren(
        self,
        line_count: usize,
        char_count: usize,
    ) -> Result<Self, IndentailError> {
        match self {
            Self::Optioning {
                tag,
                options,
                state: OptionState::Empty,
            } => Ok(Self::Optioned { tag, options }),
            Self::Optioning {
                tag,
                mut options,
                state: OptionState::Valued((key, value)),
            } => {
                options.insert(key, value);
                Ok(Self::Optioned { tag, options })
            }
            _ => Err(IndentailError::Syntax {
                line: line_count,
                column: char_count,
                detail: SyntaxErrorDetail::InvalidRightParen,
            }),
        }
    }

    pub(crate) fn separate_option(
        self,
        line_count: usize,
        char_count: usize,
    ) -> Result<Self, IndentailError> {
        match self {
            Self::Optioning {
                tag,
                options,
                state: OptionState::Keyed(key),
            } => Ok(Self::Optioning {
                tag,
                options,
                state: OptionState::Valuing(key),
            }),
            _ => Err(IndentailError::Syntax {
                line: line_count,
                column: char_count,
                detail: SyntaxErrorDetail::InvalidColon,
            }),
        }
    }

    pub(crate) fn advance_option(
        self,
        line_count: usize,
        char_count: usize,
    ) -> Result<Self, IndentailError> {
        match self {
            Self::Optioning {
                tag,
                mut options,
                state: OptionState::Valued((key, value)),
            } => {
                options.insert(key, value);
                Ok(Self::Optioning {
                    tag,
                    options,
                    state: OptionState::Empty,
                })
            }
            _ => Err(IndentailError::Syntax {
                line: line_count,
                column: char_count,
                detail: SyntaxErrorDetail::InvalidComma,
            }),
        }
    }

    pub(crate) fn set_children(self, children: Vec<Node>) -> Result<Self, IndentailError> {
        match self {
            Self::Tagged { tag } => Ok(Self::Modifying {
                tag,
                options: Default::default(),
                modifiers: Default::default(),
                children,
            }),
            Self::Optioned { tag, options } => Ok(Self::Modifying {
                tag,
                options,
                modifiers: Default::default(),
                children,
            }),
            _ => unreachable!(),
        }
    }
}
