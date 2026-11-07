use super::*;
use std::mem;

mod state;
#[cfg(test)]
mod test;

use state::State;

pub(crate) fn parse(code: String) -> Result<Vec<Node>, IndentailError> {
    let mut itr = code.chars().peekable();

    let mut line_count = 1;
    let mut char_count = 1;
    let mut buffer_start_line_count = 1;
    let mut buffer_start_char_count = 1;
    let mut buffer = String::default();

    let mut state = State::default();
    let mut states = vec![];
    let mut childrens = vec![vec![]];

    macro_rules! flush {
        () => {
            if !buffer.is_empty() {
                let (n, s) = state.build();
                if let Some(node) = n {
                    childrens.last_mut().unwrap().push(node);
                }
                state = s.append(
                    buffer_start_line_count,
                    buffer_start_char_count,
                    mem::take(&mut buffer),
                )?;
            }
        };
    }

    while let Some(c) = itr.next() {
        match c {
            '/' if itr.peek() == Some(&'/') => {
                flush!();
                itr.find(|&c| c == '\n');
                line_count += 1;
                char_count = 1;
            }

            ' ' | '\t' => {
                flush!();
                char_count += 1;
            }

            '\r' | '\n' => {
                flush!();
                line_count += 1;
                char_count = 1;
            }

            '(' => {
                flush!();
                state = state.open_paren(line_count, char_count)?;
                char_count += 1;
            }

            ')' => {
                flush!();
                state = state.close_paren(line_count, char_count)?;
                char_count += 1;
            }

            ':' => {
                flush!();
                state = state.separate_option(line_count, char_count)?;
                char_count += 1;
            }

            ',' => {
                flush!();
                state = state.advance_option(line_count, char_count)?;
                char_count += 1;
            }

            '{' => {
                flush!();

                if !matches!(state, State::Tagged { .. } | State::Optioned { .. }) {
                    return Err(IndentailError::Syntax {
                        line: line_count,
                        column: char_count,
                        detail: SyntaxErrorDetail::InvalidLeftBrace,
                    });
                }

                states.push(mem::take(&mut state));
                childrens.push(vec![]);
                char_count += 1;
            }

            '}' => {
                flush!();

                let (n, s) = state.build();
                if let Some(node) = n {
                    childrens.last_mut().unwrap().push(node);
                }

                let (State::Empty, Some(s), Some(c)) = (s, states.pop(), childrens.pop()) else {
                    return Err(IndentailError::Syntax {
                        line: line_count,
                        column: char_count,
                        detail: SyntaxErrorDetail::InvalidRightBrace,
                    });
                };

                state = s.set_children(c)?;
                char_count += 1;
            }

            '\'' => {
                flush!();
                buffer_start_line_count = line_count;
                buffer_start_char_count = char_count;
                char_count += 1;

                loop {
                    let Some(c) = itr.next() else {
                        return Err(IndentailError::Syntax {
                            line: buffer_start_line_count,
                            column: buffer_start_char_count,
                            detail: SyntaxErrorDetail::UnclosedQuote,
                        });
                    };
                    match c {
                        '\'' => {
                            char_count += 1;
                            break;
                        }
                        '\\' => {
                            let c = match itr.next() {
                                Some('\\') => '\\',
                                Some('t') => '\t',
                                Some('r') => '\r',
                                Some('n') => '\n',
                                Some('\'') => '\'',
                                Some(c) => {
                                    return Err(IndentailError::Syntax {
                                        line: line_count,
                                        column: char_count,
                                        detail: SyntaxErrorDetail::InvalidEscape(c),
                                    });
                                }
                                None => continue,
                            };
                            buffer.push(c);
                            char_count += 2;
                        }
                        '\r' | '\n' => {
                            buffer.push(c);
                            line_count += 1;
                            char_count = 1;
                        }
                        _ => {
                            buffer.push(c);
                            char_count += 1;
                        }
                    }
                }

                flush!();
            }

            _ => {
                if buffer.is_empty() {
                    buffer_start_line_count = line_count;
                    buffer_start_char_count = char_count;
                }
                buffer.push(c);
                char_count += 1;
            }
        }
    }

    flush!();

    let (n, s) = state.build();
    if let Some(node) = n {
        childrens.last_mut().unwrap().push(node);
    }

    if !matches!(s, State::Empty) {
        return Err(IndentailError::Syntax {
            line: line_count,
            column: char_count,
            detail: SyntaxErrorDetail::UnclosedParen,
        });
    }
    if !states.is_empty() || childrens.len() != 1 {
        return Err(IndentailError::Syntax {
            line: line_count,
            column: char_count,
            detail: SyntaxErrorDetail::UnclosedBrace,
        });
    }

    Ok(childrens.pop().unwrap())
}
