use bpaf::Bpaf;
use indentail::{Input, Options, error::*, parse, xml};
use std::{fs, path::PathBuf, process::ExitCode};

#[derive(Bpaf)]
#[bpaf(options)]
struct Args {
    #[bpaf(long, argument("N"), fallback(100), display_fallback)]
    max_width: usize,
    #[bpaf(long, argument("N"), fallback(4), display_fallback)]
    indent_spaces: usize,
    #[bpaf(long, argument::<String>("MODE"), parse(xml_mode), optional)]
    xml: Option<xml::Mode>,
    #[bpaf(short, long, argument("PATH"))]
    out_path: Option<PathBuf>,
    #[bpaf(positional("FILE"), some("expected at least one FILE"))]
    paths: Vec<PathBuf>,
}

fn xml_mode(v: String) -> Result<xml::Mode, String> {
    match v.as_str() {
        "structural" => Ok(xml::Mode::Structural),
        "aggressive" => Ok(xml::Mode::Aggressive),
        "semantic" => Ok(xml::Mode::Semantic),
        _ => Err(format!(
            "expected structural, aggressive or semantic, found '{v}'"
        )),
    }
}

fn main() -> ExitCode {
    let args = args().run();
    let mut nodes = vec![];

    for path in args.paths {
        let options = Options {
            input: Input::FilePath(path.clone()),
        };
        match parse(options) {
            Ok(v) => nodes.extend(v),
            Err(e) => {
                let path = path.display();
                match e {
                    IndentailError::Syntax { .. } => eprintln!("{path}:{e}"),
                    _ => eprintln!("{path}: {e}"),
                }
                return ExitCode::from(1);
            }
        }
    }

    let out = match args.xml {
        Some(mode) => match xml::format(&nodes, mode) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{e}");
                return ExitCode::from(1);
            }
        },
        None => nodes
            .iter()
            .map(|v| v.format(args.max_width, args.indent_spaces) + "\n")
            .collect(),
    };

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
