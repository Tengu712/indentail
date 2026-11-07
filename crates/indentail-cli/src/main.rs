use indentail::{Input, Options, error::*, parse};
use std::{env, path::PathBuf, process::ExitCode};

const HELP: &str = "\
usage: indentail <file-path>+\
";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("{HELP}");
        return ExitCode::from(1);
    }

    for path in args {
        let options = Options {
            input: Input::FilePath(PathBuf::from(&path)),
        };
        match parse(options) {
            Ok(v) => println!("{v:?}"),
            Err(e) => {
                match e {
                    IndentailError::Syntax { .. } => eprintln!("{path}:{e}"),
                    IndentailError::IO(_) => eprintln!("{path}: {e}"),
                }
                return ExitCode::from(1);
            }
        }
    }

    ExitCode::from(0)
}
