use std::collections::BTreeMap;
use colored::*;

pub fn summarize(
    files_matches: &BTreeMap<String, Vec<(usize, String, bool)>>,
    use_color: bool,
) {
    let files_with_matches = files_matches.len();
    let total_matches: usize = files_matches
        .values()
        .map(|lines| lines.iter().filter(|(_, _, is_match)| *is_match).count())
        .sum();

    let separator = if use_color {
        "─".repeat(40).bright_black().to_string()
    } else {
        "─".repeat(40)
    };

    println!("\n{}", separator);
    if use_color {
        println!("{}", "Search Summary".bold().cyan());
        println!("{}", separator);
        println!(
            "{} {}",
            "Files with matches:".bold(),
            files_with_matches.to_string().green().bold()
        );
        println!(
            "{} {}",
            "Matching lines:".bold(),
            total_matches.to_string().yellow().bold()
        );
    } else {
        println!("Search Summary");
        println!("{}", separator);
        println!("Files with matches: {}", files_with_matches);
        println!("Matching lines: {}", total_matches);
    }
    println!("{}", separator);
}

