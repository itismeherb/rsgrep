use walkdir::WalkDir;
use std::collections::BTreeMap;
use clap::{Parser, ArgAction};
use atty;
use rayon::prelude::*;
use colored::*;
use std::fs;

use rsgrep::search::find_matches;
use rsgrep::highlight::highlight_line;
use rsgrep::fs_utils::is_binary;

#[derive(Parser)]
struct Args {
    pattern: String,
    path: String,

    #[arg(
        short = 'l', 
        long,
    )]
    only_filenames: bool,

    #[arg(
        short = 'c',
        long
    )]
    count: bool,

    #[arg(short, long)]
    ignore_case: bool,

    #[arg(
        short = 'n', 
        long,
        default_value_t = false,
        action = ArgAction::SetTrue
    )]
    no_line_numbers: bool,
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

        if args.only_filenames {
            continue;
        } 

        if args.count {
            println!(": {}", matches.len());
            continue;
        }

        matches.sort_by_key(|(line_num, _)| *line_num);

        for (line_num, line) in matches {
            let highlighted = highlight_line(&line, &pattern, args.ignore_case, use_color);
            if use_color {
                if !args.no_line_numbers {
                    println!("{}: {}", line_num.to_string().yellow(), highlighted);
                } else {
                    println!("{}", highlighted);
                }
            } else {
                if !args.no_line_numbers {
                    println!("{}: {}", line_num, line);
                } else {
                    println!("{}", line);
                }
            }
        }
        println!();
    }
}

