use crate::markers::{make_begin_marker, make_end_marker};

pub fn wrap(content: &str, id: &str) -> String {
    let content_trimmed = content.trim_end_matches('\n');
    format!(
        "{}\n{}\n{}",
        make_begin_marker(id),
        content_trimmed,
        make_end_marker(id)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_content() {
        let content = "This is the content to wrap";
        let result = wrap(content, "test-id");

        assert_eq!(
            result,
            "<!-- KUGIRI-BEGIN: test-id -->\nThis is the content to wrap\n<!-- KUGIRI-END: test-id -->"
        );
    }

    #[test]
    fn test_wrap_multiline_content() {
        let content = "Line 1\nLine 2\nLine 3";
        let result = wrap(content, "multi");

        assert_eq!(
            result,
            "<!-- KUGIRI-BEGIN: multi -->\nLine 1\nLine 2\nLine 3\n<!-- KUGIRI-END: multi -->"
        );
    }

    #[test]
    fn test_wrap_with_trailing_newline() {
        let content = "Content with trailing newline\n";
        let result = wrap(content, "test");

        // Should trim the trailing newline
        assert_eq!(
            result,
            "<!-- KUGIRI-BEGIN: test -->\nContent with trailing newline\n<!-- KUGIRI-END: test -->"
        );
    }

    #[test]
    fn test_wrap_empty_content() {
        let content = "";
        let result = wrap(content, "empty");

        assert_eq!(
            result,
            "<!-- KUGIRI-BEGIN: empty -->\n\n<!-- KUGIRI-END: empty -->"
        );
    }
}
