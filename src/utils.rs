/// Helper function to join lines while preserving the original trailing newline.
///
/// When using `str::lines()` followed by `join("\n")`, we lose information about
/// whether the original text ended with a newline. This function preserves that.
pub fn join_lines_preserve_trailing_newline(lines: Vec<String>, original_text: &str) -> String {
    let result = lines.join("\n");

    // Check if the original text ended with a newline
    if !original_text.is_empty() && original_text.ends_with('\n') {
        format!("{result}\n")
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preserve_trailing_newline() {
        let text_with_newline = "line1\nline2\n";
        let lines = vec!["line1".to_string(), "line2".to_string()];
        let result = join_lines_preserve_trailing_newline(lines.clone(), text_with_newline);
        assert_eq!(result, "line1\nline2\n");
    }

    #[test]
    fn test_no_trailing_newline() {
        let text_without_newline = "line1\nline2";
        let lines = vec!["line1".to_string(), "line2".to_string()];
        let result = join_lines_preserve_trailing_newline(lines, text_without_newline);
        assert_eq!(result, "line1\nline2");
    }

    #[test]
    fn test_empty_text() {
        let text = "";
        let lines = vec![];
        let result = join_lines_preserve_trailing_newline(lines, text);
        assert_eq!(result, "");
    }

    #[test]
    fn test_single_line_with_newline() {
        let text = "line1\n";
        let lines = vec!["line1".to_string()];
        let result = join_lines_preserve_trailing_newline(lines, text);
        assert_eq!(result, "line1\n");
    }

    #[test]
    fn test_single_line_without_newline() {
        let text = "line1";
        let lines = vec!["line1".to_string()];
        let result = join_lines_preserve_trailing_newline(lines, text);
        assert_eq!(result, "line1");
    }
}
