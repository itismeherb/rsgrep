pub fn find_matches(pattern: &str, content: &str, ignore_case: bool) -> Vec<(usize, String)> {
    let mut result = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let cmp = if ignore_case {
            line.to_lowercase()
        } else {
            line.to_string()
        };

        if cmp.contains(pattern) {
            result.push((line_num + 1, line.trim_start().to_string()));
        }
    }

    result
}

