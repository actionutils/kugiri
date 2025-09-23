#[derive(Debug, Clone, PartialEq)]
pub struct Section {
    pub id: String,
    pub start_line: usize,
    pub end_line: usize,
    pub content: String,
    pub indent: String,
}

pub fn find_section(text: &str, id: &str) -> Option<Section> {
    let begin_marker = make_begin_marker(id);
    let end_marker = make_end_marker(id);

    let lines: Vec<&str> = text.lines().collect();

    let mut begin_line = None;
    let mut end_line = None;
    let mut indent = String::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim_start();
        let line_indent = &line[..line.len() - trimmed.len()];

        if trimmed == begin_marker {
            begin_line = Some(idx);
            indent = line_indent.to_string();
        } else if trimmed == end_marker && begin_line.is_some() {
            end_line = Some(idx);
            break;
        }
    }

    match (begin_line, end_line) {
        (Some(start), Some(end)) if start < end => {
            let content_lines = &lines[start + 1..end];

            // Remove the common indent from content lines
            let content = content_lines
                .iter()
                .map(|line| {
                    if line.starts_with(&indent) {
                        &line[indent.len()..]
                    } else {
                        line
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");

            Some(Section {
                id: id.to_string(),
                start_line: start,
                end_line: end,
                content,
                indent,
            })
        }
        _ => None,
    }
}

pub fn make_begin_marker(id: &str) -> String {
    format!("<!-- KUGIRI-BEGIN: {} -->", id)
}

pub fn make_end_marker(id: &str) -> String {
    format!("<!-- KUGIRI-END: {} -->", id)
}

pub fn make_insert_marker(id: &str) -> String {
    format!("<!-- KUGIRI-INSERT: {} -->", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_section_exists() {
        let text = r#"# README

<!-- KUGIRI-BEGIN: test-section -->
This is the content
of the test section
<!-- KUGIRI-END: test-section -->

More content after"#;

        let section = find_section(text, "test-section").expect("Section should be found");
        assert_eq!(section.id, "test-section");
        assert_eq!(section.content, "This is the content\nof the test section");
        assert_eq!(section.start_line, 2);
        assert_eq!(section.end_line, 5);
        assert_eq!(section.indent, "");
    }

    #[test]
    fn test_find_section_with_indent() {
        let text = r#"# README

  <!-- KUGIRI-BEGIN: indented -->
  This is indented
  content here
  <!-- KUGIRI-END: indented -->

More content"#;

        let section = find_section(text, "indented").expect("Section should be found");
        assert_eq!(section.id, "indented");
        assert_eq!(section.content, "This is indented\ncontent here");
        assert_eq!(section.indent, "  ");
    }

    #[test]
    fn test_find_section_not_exists() {
        let text = "Some text without markers";
        assert!(find_section(text, "non-existent").is_none());
    }

    #[test]
    fn test_markers() {
        assert_eq!(make_begin_marker("test"), "<!-- KUGIRI-BEGIN: test -->");
        assert_eq!(make_end_marker("test"), "<!-- KUGIRI-END: test -->");
        assert_eq!(make_insert_marker("test"), "<!-- KUGIRI-INSERT: test -->");
    }
}
