use colored::*;

pub fn highlight_line(line: &str, pattern: &str, ignore_case: bool, use_color: bool) -> String {
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
        out.push_str(&line[abs_pos..abs_pos + pattern_len].yellow().to_string());
        idx = abs_pos + pattern_len;
    }

    out.push_str(&line[idx..]);
    out
}

