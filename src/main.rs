use walkdir::WalkDir;
use std::fs;
use std::io::Read;
use std::collections::BTreeMap;
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
    let use_color = atty::is(atty::Stream::Stdout);

    let pattern = if args.ignore_case {
        args.pattern.to_lowercase()
    } else {
        args.pattern.clone()
    };

    let walker = WalkDir::new(&args.path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file());

    let results: Vec<(String, Vec<(usize, String)>)> = walker.par_bridge()
        .filter_map(|entry| {
            if is_binary(entry.path()) { return None; }

            let path_str = entry.path().display().to_string();
            let content = fs::read_to_string(entry.path()).ok()?;
            let matches = find_matches(&pattern, &content, args.ignore_case);
            if matches.is_empty() { return None; }

            Some((path_str, matches))
        })
        .collect();

    let mut files_matches = BTreeMap::new();
    for (path, matches) in results {
        files_matches.insert(path, matches);
    }

    for (path, mut matches) in files_matches {
        println!("{}", path.cyan());

        matches.sort_by_key(|(line_num, _)| *line_num);

        for (line_num, line) in matches {
            let highlighted = highlight_line(&line, &pattern, args.ignore_case, use_color);
            if use_color {
                println!("{}: {}", line_num.to_string().yellow(), highlighted);
            } else {
                println!("{}: {}", line_num, line);
            }
        }
        println!();
    }
}

fn is_binary(path: &std::path::Path) -> bool {
    const SAMPLE: usize = 8192;

    let Ok(mut file) = std::fs::File::open(path) else { return false; };

    let mut buf = [0u8; SAMPLE];
    let Ok(n) = file.read(&mut buf) else { return false; };

    buf[..n].contains(&0)
}

fn highlight_line(line: &str, pattern: &str, ignore_case: bool, use_color: bool) -> String {
    if !use_color {
        return line.to_string();
    }

    let mut out = String::with_capacity(line.len() + 16);
    let haystack = if ignore_case { line.to_lowercase() } else { line.to_string() };
    let pattern_len = pattern.len();
    let mut idx = 0;

    while let Some(pos) = haystack[idx..].find(pattern) {
        let abs_pos = idx + pos;
        out.push_str(&line[idx..abs_pos]);
        out.push_str(&line[abs_pos..abs_pos + pattern_len].red().bold().to_string());
        idx = abs_pos + pattern_len;
    }

    out.push_str(&line[idx..]);
    out
}
