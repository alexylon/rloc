use std::{ffi::OsStr, path::Path};

use crate::{BOLD, RED, RESET, Result};

const SKIPPED_ATTR_SUFFIXES: [&str; 7] = [
    "allow(",
    "warn(",
    "deny(",
    "expect(",
    "#[cfg_attr(any(windows, target_os = \"wasi\"), expect(",
    "cfg_attr(test,",
    "cfg_attr(fuzzing,",
];

fn count_code_lines(source: &str) -> usize {
    source
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !trimmed.starts_with("//")
        })
        .count()
}

fn count_lines_in_file(file: &Path) -> Result<(usize, usize)> {
    if file.extension() != Some(OsStr::new("rs")) {
        Err(format!("{} is not a Rust file", file.display()))?;
    }

    let source = std::fs::read_to_string(file)?;
    let all = count_code_lines(&source);
    let pure = count_code_lines(&strip_noise(&source));

    Ok((pure, all))
}

pub fn count_rust_lines(root: &Path) -> Result<()> {
    if !root.is_dir() {
        Err(format!("{}: not a directory", root.display()))?;
    }

    let mut file_counts: Vec<(String, usize, usize)> = Vec::new();
    let mut pending_dirs = vec![root.to_path_buf()];
    while let Some(dir) = pending_dirs.pop() {
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if entry.file_type()?.is_dir() {
                let name = entry.file_name();
                if name != "target" && name != ".git" {
                    pending_dirs.push(path);
                }
                continue;
            }
            if path.extension() != Some(OsStr::new("rs")) {
                continue;
            }

            let (pure, all) = count_lines_in_file(&path)?;
            let relative_path = path.strip_prefix(root)?.display().to_string();
            file_counts.push((relative_path, pure, all));
        }
    }
    file_counts.sort();

    print_report(&file_counts)
}

/// Remove lint attributes and everything after `#[cfg(test)]`.
fn strip_noise(source: &str) -> String {
    let kept_lines = source
        .lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            trimmed
                .strip_prefix("#![")
                .or_else(|| trimmed.strip_prefix("#["))
                .is_none_or(|after_hash| {
                    !SKIPPED_ATTR_SUFFIXES
                        .iter()
                        .any(|suffix| after_hash.starts_with(suffix))
                })
        })
        .take_while(|line| !line.contains("#[cfg(test)]"));
    kept_lines.collect::<Vec<_>>().join("\n")
}

fn print_report(file_counts: &[(String, usize, usize)]) -> Result<()> {
    let pure_total: usize = file_counts.iter().map(|(_, pure, _)| pure).sum();
    let all_total: usize = file_counts.iter().map(|(_, _, all)| all).sum();
    let column_width = file_counts
        .iter()
        .map(|(path, _, _)| path.len())
        .max()
        .unwrap_or_default()
        .max(5);

    for (path, pure, _) in file_counts {
        println!("{path:column_width$} {pure:4}");
    }
    let pure_label = "Pure Rust code";
    println!("{BOLD}{pure_label:column_width$} {pure_total:4}{RESET}");
    let all_label = "All Rust code";
    println!("{BOLD}{all_label:column_width$} {all_total:4}{RESET}");

    if all_total == 0 {
        eprintln!("{RED}warning{RESET}: no Rust source files found");
    }

    Ok(())
}
