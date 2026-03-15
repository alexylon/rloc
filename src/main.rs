use std::process::ExitCode;

mod count;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

const RESET: &str = "\x1b[m";
const BOLD: &str = "\x1b[1m";
const RED: &str = "\x1b[31m";

fn main() -> ExitCode {
    let arg = std::env::args().nth(1);
    let root = std::path::PathBuf::from(arg.as_deref().unwrap_or("."));

    match count::count_rust_lines(&root) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{RED}error{RESET}: {error}");
            ExitCode::FAILURE
        }
    }
}
