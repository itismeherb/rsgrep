use std::collections::HashSet;

pub fn with_context(
    matches: &[(usize, String, bool)],
    total_lines: &[String],
    context: usize,
) -> Vec<(usize, String, bool)> {
    let mut result = Vec::new();
    let mut added = HashSet::new();

    // Only real matches trigger context
    let match_lines: HashSet<usize> = matches
        .iter()
        .filter(|(_, _, is_match)| *is_match)
        .map(|(num, _, _)| *num)
        .collect();

    for &(line_num, _, is_match) in matches {
        if !is_match {
            continue; // Context derives ONLY from match lines, not context lines
        }

        let start = line_num.saturating_sub(1 + context);
        let end = (line_num + context).min(total_lines.len());

        for i in start..end {
            let line_no = i + 1;

            // Avoid duplicates
            if added.insert(line_no) {
                let is_match_line = match_lines.contains(&line_no);
                result.push((line_no, total_lines[i].clone(), is_match_line));
            }
        }
    }

    // Sort by line number
    result.sort_by_key(|(line_num, _, _)| *line_num);

    result
}

