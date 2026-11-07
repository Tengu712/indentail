use bpaf::Bpaf;
use indentail::{Input, Options, error::*, parse};
use std::{fs, path::PathBuf, process::ExitCode};

#[derive(Bpaf)]
#[bpaf(options)]
struct Args {
    #[bpaf(long, argument("N"), fallback(100), display_fallback)]
    max_width: usize,
    #[bpaf(long, argument("N"), fallback(4), display_fallback)]
    indent_spaces: usize,
    #[bpaf(short, long, argument("PATH"))]
    out_path: Option<PathBuf>,
    #[bpaf(positional("FILE"), some("expected at least one FILE"))]
    paths: Vec<PathBuf>,
}

fn main() -> ExitCode {
    let args = args().run();
    let mut out = String::new();

    for path in args.paths {
        let options = Options {
            input: Input::FilePath(path.clone()),
        };
        match parse(options) {
            Ok(v) => {
                for node in v {
                    out.push_str(&node.format(args.max_width, args.indent_spaces));
                    out.push('\n');
                }
            }
            Err(e) => {
                let path = path.display();
                match e {
                    IndentailError::Syntax { .. } => eprintln!("{path}:{e}"),
                    IndentailError::IO(_) => eprintln!("{path}: {e}"),
                }
                return ExitCode::from(1);
            }
        }
    }

    if let Some(path) = args.out_path {
        if let Err(e) = fs::write(&path, out) {
            eprintln!("{}: {e}", path.display());
            return ExitCode::from(1);
        }
    } else {
        print!("{out}");
    }

    ExitCode::from(0)
}
