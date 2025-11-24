use std::collections::HashSet;

pub fn with_context(
    matches: &[(usize, String, bool)],
    total_lines: &[String],
    context: usize,
) -> Vec<(usize, String, bool)> {
    let mut result = Vec::new();
    let mut added = HashSet::new();

    let match_lines: HashSet<usize> = matches.iter()
        .filter(|(_, _, is_match)| *is_match)
        .map(|(num, _, _)| *num)
        .collect();

    for &(line_num, _, is_match) in matches {
        if !is_match {
            continue;
        }

        let start = line_num.saturating_sub(1 + context);
        let end = (line_num + context).min(total_lines.len());

        for i in start..end {
            if added.insert(i + 1) {
                let is_match_line = match_lines.contains(&(i + 1));
                result.push((i + 1, total_lines[i].clone(), is_match_line));
            }
        }
    }

    result.sort_by_key(|(line_num, _, _)| *line_num);
    result
}

