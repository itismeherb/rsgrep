use walkdir::WalkDir;
use std::collections::BTreeMap;
use clap::{Parser, ArgAction};
use atty;
use rayon::prelude::*;
use colored::*;
use std::fs;

use rsgrep::search::find_matches;
use rsgrep::context::with_context;
use rsgrep::highlight::highlight_line;
use rsgrep::fs_utils::is_binary;

#[derive(Parser)]
struct Args {
    pattern: String,
    path: String,

    #[arg(
        short = 'l',
        long,
        help = "Only display filenames that contain matches"
    )]
    only_filenames: bool,

    #[arg(short = 'c',
        long,
        help = "Only display the number of matches per file"
    )]
    count: bool,

    #[arg(
        short,
        long,
        help = "Perform a case-insensitive search"
    )]
    ignore_case: bool,

    #[arg(
        short = 'n',
        long,
        help = "Do not display line numbers",
        default_value_t = false,
        action = ArgAction::SetTrue
    )]
    no_line_numbers: bool,

    #[arg(
        short = 'C',
        long,
        help = "Show N lines of context around each match",
        default_value_t = 0
    )]
    context: usize,

    #[arg(
        short = 'r',
        long,
        help = "Print file paths relative to the search path",
        default_value_t = false,
        action = ArgAction::SetTrue
    )]
    relative_paths: bool,
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

    let results: Vec<(String, Vec<(usize, String, bool)>)> = walker
        .par_bridge()
        .filter_map(|entry| {
            if is_binary(entry.path()) {
                return None;
            }

            let path_str = if args.relative_paths {
                entry.path()
                    .strip_prefix(&args.path)
                    .unwrap_or(entry.path())
                    .display()
                    .to_string()
            } else {
                entry.path().display().to_string()
            };

            let content_lines: Vec<String> = fs::read_to_string(entry.path())
                .ok()?
                .lines()
                .map(|l| l.to_string())
                .collect();

            let matches = find_matches(&pattern, &content_lines.join("\n"), args.ignore_case);
            if matches.is_empty() {
                return None;
            }

            let matches_with_context = if args.context > 0 {
                with_context(&matches, &content_lines, args.context)
            } else {
                matches.into_iter().map(|(ln, l)| (ln, l, true)).collect()
            };

            Some((path_str, matches_with_context))
        })
        .collect();

    let mut files_matches = BTreeMap::new();
    for (path, matches) in results {
        files_matches.insert(path, matches);
    }

    for (path, matches) in files_matches {
        println!("{}", path.cyan());

        if args.only_filenames {
            continue;
        }

        if args.count {
            let count = matches.iter().filter(|(_, _, is_match)| *is_match).count();
            println!(": {}", count);
            continue;
        }

        for (line_num, line, is_match) in matches {
            let highlighted = if is_match {
                highlight_line(&line, &pattern, args.ignore_case, use_color)
            } else {
                line.clone()
            };

            let prefix = if is_match { ":" } else { "-" };

            if use_color {
                let line_color = if is_match {
                    highlighted
                } else {
                    highlighted.bright_black().to_string()
                };
                let num_color = if is_match {
                    line_num.to_string().yellow()
                } else {
                    line_num.to_string().bright_black()
                };

                if args.no_line_numbers {
                    println!("{} {}", prefix, line_color);
                } else {
                    println!("{}{} {}", num_color, prefix, line_color);
                }
            } else {
                if args.no_line_numbers {
                    println!("{} {}", prefix, highlighted);
                } else {
                    println!("{}{} {}", line_num, prefix, highlighted);
                }
            }
        }

        println!();
    }
}

