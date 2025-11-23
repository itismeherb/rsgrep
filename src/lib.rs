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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_match() {
        let content = "hello\nworld";
        assert_eq!(find_matches("hello", content, false), vec![(1, "hello".to_string())]);
    }

    #[test]
    fn no_match() {
        let content = "hello\nworld";
        assert!(find_matches("xyz", content, false).is_empty());
    }

    #[test]
    fn ignore_case() {
        let content = "HELLO\nworld";
        assert_eq!(find_matches("hello", content, true), vec![(1, "HELLO".to_string())]);
    }
}

