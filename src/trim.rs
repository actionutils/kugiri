use crate::utils::join_lines_preserve_trailing_newline;

pub fn trim(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut result: Vec<String> = Vec::new();

    for &line in lines.iter() {
        if line.starts_with("<!-- KUGIRI-BEGIN:")
            || line.starts_with("<!-- KUGIRI-END:")
            || line.starts_with("<!-- KUGIRI-INSERT:")
        {
            continue;
        }
        result.push(line.to_string());
    }

    join_lines_preserve_trailing_newline(result, text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_removes_all_markers() {
        let text = r#"Header

<!-- KUGIRI-BEGIN: section1 -->
Content 1
<!-- KUGIRI-END: section1 -->

Middle

<!-- KUGIRI-INSERT: anchor -->

<!-- KUGIRI-BEGIN: section2 -->
Content 2
<!-- KUGIRI-END: section2 -->

Footer"#;

        let result = trim(text);

        assert!(result.contains("Header"));
        assert!(result.contains("Content 1"));
        assert!(result.contains("Middle"));
        assert!(result.contains("Content 2"));
        assert!(result.contains("Footer"));

        assert!(!result.contains("KUGIRI-BEGIN"));
        assert!(!result.contains("KUGIRI-END"));
        assert!(!result.contains("KUGIRI-INSERT"));
    }

    #[test]
    fn test_trim_empty_file() {
        assert_eq!(trim(""), "");
    }

    #[test]
    fn test_trim_no_markers() {
        let text = "Just some text\nwithout any markers";
        assert_eq!(trim(text), text);
    }
}
