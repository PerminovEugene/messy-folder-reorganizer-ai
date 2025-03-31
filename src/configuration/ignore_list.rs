use regex::Regex;

use crate::errors::app_error::AppError;

pub fn parse_ignore_list(ignore_list: &Option<Vec<String>>) -> Result<Vec<Regex>, AppError> {
    let result = ignore_list
        .as_ref()
        .unwrap_or(&vec![])
        .iter()
        .map(|pattern| {
            Regex::new(pattern).map_err(|_| {
                AppError::Configuration(format!("Invalid ignore pattern: {}", pattern))
            })
        })
        .collect::<Result<Vec<_>, _>>();

    result
}

pub fn is_ignored(str: &str, ignore_patterns: &[Regex]) -> bool {
    ignore_patterns.iter().any(|pattern| pattern.is_match(str))
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    // parse_ignore_list

    #[test]
    fn test_parse_ignore_list_some_valid() {
        let input = Some(vec![
            String::from(r"^\.git$"),
            String::from(r"node_modules"),
        ]);

        let result = parse_ignore_list(&input).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result[0].is_match(".git"));
        assert!(result[1].is_match("node_modules/react"));
    }

    #[test]
    fn test_parse_ignore_list_none() {
        let input: Option<Vec<String>> = None;
        let result = parse_ignore_list(&input).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_ignore_list_invalid_pattern_returns_error() {
        let input = Some(vec![String::from("*invalid[")]);

        let result = parse_ignore_list(&input);
        assert!(
            matches!(result, Err(AppError::Configuration(msg)) if msg.contains("Invalid ignore pattern"))
        );
    }

    // is_ignored

    fn compile_patterns(patterns: &[&str]) -> Vec<Regex> {
        patterns
            .iter()
            .map(|s| Regex::new(s).expect("Invalid regex"))
            .collect()
    }

    #[test]
    fn test_is_ignored_patterns_match() {
        let patterns = compile_patterns(&[
            r"^\.[^/]+$",
            r"^temp_.*\.log$",
            r"node_modules",
            r"Projects",
        ]);

        assert!(is_ignored(".env", &patterns));
        assert!(is_ignored(".git", &patterns));
        assert!(is_ignored("temp_error.log", &patterns));
        assert!(is_ignored("node_modules", &patterns));
        assert!(is_ignored("Projects", &patterns));
        assert!(is_ignored("MyProjects", &patterns));
        assert!(is_ignored("node_modules/react/index.js", &patterns));
    }

    #[test]
    fn test_is_ignored_patterns_no_match() {
        let patterns = compile_patterns(&[
            r"^\.[^/]+$",
            r"^temp_.*\.log$",
            r"node_modules",
            r"Projects",
        ]);

        assert!(!is_ignored("src/main.rs", &patterns));
        assert!(!is_ignored("notes.txt", &patterns));
        assert!(!is_ignored("tempfile.log", &patterns));
        assert!(!is_ignored("hidden.hidden/file.txt", &patterns));
        assert!(!is_ignored("Apps", &patterns));
    }

    #[test]
    fn test_is_ignored_empty_patterns() {
        let patterns: Vec<Regex> = vec![];
        assert!(!is_ignored("any.file", &patterns));
    }
}
