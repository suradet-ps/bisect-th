mod links;
mod util;
mod verify;

use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str = r#"bisect-th - verification gates for the bisect-th Thai translation

Usage:
  bisect-th verify [options]
      Byte-exact check of the translation against upstream cargo-bisect-rustc:
      code blocks, heading levels, ref-links, and inline link targets.

  bisect-th check-links [options]
      Resolve every anchor link in the built mdbook HTML.

Options:
  -o, --orig <path>    upstream guide/src
                       (default: ../cargo-bisect-rustc/guide/src)
  -t, --trans <path>   translated guide/src (default: guide/src)
  -b, --book <path>    built book directory (default: guide/book)
  -h, --help           print this help
  -V, --version        print the version

Run from the repository root, or pass explicit paths."#;

enum CliError {
    Usage(String),
    Run(String),
}

impl From<String> for CliError {
    fn from(message: String) -> Self {
        CliError::Run(message)
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match dispatch(&args) {
        Ok(code) => code,
        Err(CliError::Usage(message)) => {
            eprintln!("error: {message}");
            eprintln!();
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
        Err(CliError::Run(message)) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn dispatch(args: &[String]) -> Result<ExitCode, CliError> {
    let Some((command, rest)) = args.split_first() else {
        return Err(CliError::Usage("missing command".to_string()));
    };
    if rest.iter().any(|arg| arg == "-h" || arg == "--help") {
        println!("{USAGE}");
        return Ok(ExitCode::SUCCESS);
    }
    match command.as_str() {
        "-h" | "--help" => {
            println!("{USAGE}");
            Ok(ExitCode::SUCCESS)
        }
        "-V" | "--version" => {
            println!("bisect-th {}", env!("CARGO_PKG_VERSION"));
            Ok(ExitCode::SUCCESS)
        }
        "verify" => gate(verify::run(&parse_verify(rest)?)),
        "check-links" => gate(links::run(&parse_links(rest)?)),
        other => Err(CliError::Usage(format!("unknown command: {other}"))),
    }
}

fn gate(result: Result<bool, String>) -> Result<ExitCode, CliError> {
    Ok(if result? {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

fn parse_verify(rest: &[String]) -> Result<verify::Options, CliError> {
    let mut options = verify::Options::default();
    let mut index = 0;
    while index < rest.len() {
        let arg = rest[index].as_str();
        let (flag, inline) = match arg.split_once('=') {
            Some((flag, value)) => (flag, Some(value)),
            None => (arg, None),
        };
        match flag {
            "-o" | "--orig" => {
                options.orig = Some(PathBuf::from(take_value(rest, &mut index, flag, inline)?));
            }
            "-t" | "--trans" => {
                options.trans = PathBuf::from(take_value(rest, &mut index, flag, inline)?);
            }
            other => {
                return Err(CliError::Usage(format!(
                    "unknown option for verify: {other}"
                )));
            }
        }
        index += 1;
    }
    Ok(options)
}

fn parse_links(rest: &[String]) -> Result<links::Options, CliError> {
    let mut options = links::Options::default();
    let mut index = 0;
    while index < rest.len() {
        let arg = rest[index].as_str();
        let (flag, inline) = match arg.split_once('=') {
            Some((flag, value)) => (flag, Some(value)),
            None => (arg, None),
        };
        match flag {
            "-b" | "--book" => {
                options.book = PathBuf::from(take_value(rest, &mut index, flag, inline)?);
            }
            other => {
                return Err(CliError::Usage(format!(
                    "unknown option for check-links: {other}"
                )));
            }
        }
        index += 1;
    }
    Ok(options)
}

fn take_value(
    args: &[String],
    index: &mut usize,
    flag: &str,
    inline: Option<&str>,
) -> Result<String, CliError> {
    if let Some(value) = inline {
        return Ok(value.to_string());
    }
    *index += 1;
    args.get(*index)
        .cloned()
        .ok_or_else(|| CliError::Usage(format!("missing value for {flag}")))
}
