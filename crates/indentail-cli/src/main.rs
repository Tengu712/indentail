use bpaf::Bpaf;
use indentail::{error::*, parse_from_file, xml};
use std::{fs, path::PathBuf, process::ExitCode};

#[derive(Bpaf)]
#[bpaf(options)]
struct Args {
    #[bpaf(long, argument("N"), optional, help("[default: 100]"))]
    max_width: Option<usize>,
    #[bpaf(long, argument("N"), optional, help("[default: 4]"))]
    indent_spaces: Option<usize>,
    #[bpaf(
        long,
        argument::<String>("MODE"),
        parse(xml_mode),
        optional,
        help("[possible values: structural, aggressive, semantic]")
    )]
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
        match parse_from_file(&path) {
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
        Some(mode) => {
            if args.max_width.is_some() {
                eprintln!("warning: --max-width is ignored with --xml");
            }
            if args.indent_spaces.is_some() {
                eprintln!("warning: --indent-spaces is ignored with --xml");
            }
            match xml::format(&nodes, mode) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{e}");
                    return ExitCode::from(1);
                }
            }
        }
        None => {
            let max_width = args.max_width.unwrap_or(100);
            let indent_spaces = args.indent_spaces.unwrap_or(4);
            nodes
                .iter()
                .map(|v| v.format(max_width, indent_spaces) + "\n")
                .collect()
        }
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
