use walkdir::WalkDir;
use std::fs;
use std::io::Read;
use clap::Parser;
use colored::*;
use atty;
use rayon::prelude::*;

use rsgrep::find_matches;

#[derive(Parser)]
struct Args {
    pattern: String,
    path: String,
    #[arg(short, long)]
    ignore_case: bool,
}

fn main() {
    let args = Args::parse();

    // Auto-detect if output is a TTY
    let use_color = atty::is(atty::Stream::Stdout);

    let pattern = if args.ignore_case {
        args.pattern.to_lowercase()
    } else {
        args.pattern.clone()
    };

    let walker = WalkDir::new(&args.path)
        .into_iter()
        .filter_map(|e| e.ok());

    // PARALLEL processing here
    walker.par_bridge().for_each(|entry| {
        if !entry.file_type().is_file() {
            return;
        }

        // SKIP BINARY FILES
        if is_binary(entry.path()) {
            return;
        }

        if let Ok(content) = fs::read_to_string(entry.path()) {
            let matches = find_matches(&pattern, &content, args.ignore_case);

            for (line_num, line) in matches {
                let highlighted = highlight_line(line, &pattern, args.ignore_case, use_color);

                if use_color {
                    println!(
                        "{}:{}: {}",
                        entry.path().display().to_string().cyan(),
                        line_num.to_string().yellow(),
                        highlighted
                    );
                } else {
                    println!("{}:{}: {}", entry.path().display(), line_num, line);
                }
            }
        }
    });
}

fn is_binary(path: &std::path::Path) -> bool {
    const SAMPLE: usize = 8192;

    let Ok(mut file) = std::fs::File::open(path) else {
        return false;
    };

    let mut buf = [0u8; SAMPLE];
    let Ok(n) = file.read(&mut buf) else {
        return false;
    };

    buf[..n].contains(&0)
}

fn highlight_line(line: &str, pattern: &str, ignore_case: bool, use_color: bool) -> String {
    if !use_color {
        return line.to_string();
    }

    let mut out = String::new();
    let mut idx = 0;

    let haystack = if ignore_case {
        line.to_lowercase()
    } else {
        line.to_string()
    };

    let pattern_len = pattern.len();

    while let Some(pos) = haystack[idx..].find(pattern) {
        let abs_pos = idx + pos;

        out.push_str(&line[idx..abs_pos]);
        out.push_str(&line[abs_pos..abs_pos + pattern_len].red().bold().to_string());

        idx = abs_pos + pattern_len;
    }

    out.push_str(&line[idx..]);
    out
}

