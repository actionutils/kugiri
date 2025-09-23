use crate::markers::{find_marker_for_anchor, make_begin_marker, make_end_marker};
use anyhow::{bail, Result};

pub fn insert(text: &str, id: &str, content: &str, before: Option<&str>, after: Option<&str>) -> Result<String> {
    // Validate parameters
    match (before, after) {
        (Some(_), Some(_)) => bail!("Specify only one of --before or --after"),
        (None, None) => bail!("Must specify --before or --after for insert command"),
        _ => {}
    }

    // Find the marker to insert relative to
    let marker_id = before.or(after).unwrap();
    let marker_section = find_marker_for_anchor(text, marker_id)
        .ok_or_else(|| anyhow::anyhow!("Marker with id '{marker_id}' not found"))?;

    // Trim trailing newline from content
    let content_trimmed = content.trim_end_matches('\n');

    // Get the indent from the marker we're inserting relative to
    let marker_indent = &marker_section.indent;

    // Add indent to each line of new content
    let indented_content = content_trimmed
        .lines()
        .map(|line| {
            if line.is_empty() {
                line.to_string()
            } else {
                format!("{marker_indent}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Build the new section with proper indentation
    let new_section = format!(
        "{}{}\n{}\n{}{}",
        marker_indent,
        make_begin_marker(id),
        indented_content,
        marker_indent,
        make_end_marker(id)
    );

    let lines: Vec<&str> = text.lines().collect();
    let mut result = Vec::new();

    if before.is_some() {
        // Insert before the marker
        for (idx, line) in lines.iter().enumerate() {
            if idx == marker_section.start_line {
                result.push(new_section.as_str());
                result.push("");
            }
            result.push(line);
        }
    } else {
        // Insert after the marker
        for (idx, line) in lines.iter().enumerate() {
            result.push(line);
            if idx == marker_section.end_line {
                result.push("");
                result.push(&new_section);
            }
        }
    }

    Ok(result.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_after_marker() {
        let text = r#"# README

<!-- KUGIRI-BEGIN: existing -->
Existing content
<!-- KUGIRI-END: existing -->

Footer"#;

        let result = insert(
            text,
            "new-section",
            "New content here",
            None,
            Some("existing")
        ).unwrap();

        assert!(result.contains("<!-- KUGIRI-BEGIN: new-section -->"));
        assert!(result.contains("New content here"));
        assert!(result.contains("<!-- KUGIRI-END: new-section -->"));

        // Check order: existing should come before new-section
        let existing_pos = result.find("<!-- KUGIRI-END: existing -->").unwrap();
        let new_pos = result.find("<!-- KUGIRI-BEGIN: new-section -->").unwrap();
        assert!(existing_pos < new_pos);
    }

    #[test]
    fn test_insert_before_marker() {
        let text = r#"# README

<!-- KUGIRI-BEGIN: existing -->
Existing content
<!-- KUGIRI-END: existing -->

Footer"#;

        let result = insert(
            text,
            "new-section",
            "New content here",
            Some("existing"),
            None
        ).unwrap();

        assert!(result.contains("<!-- KUGIRI-BEGIN: new-section -->"));
        assert!(result.contains("New content here"));
        assert!(result.contains("<!-- KUGIRI-END: new-section -->"));

        // Check order: new-section should come before existing
        let new_end_pos = result.find("<!-- KUGIRI-END: new-section -->").unwrap();
        let existing_pos = result.find("<!-- KUGIRI-BEGIN: existing -->").unwrap();
        assert!(new_end_pos < existing_pos);
    }

    #[test]
    fn test_insert_marker_not_found() {
        let text = "Some text";

        let result = insert(
            text,
            "new-section",
            "New content",
            None,
            Some("non-existent")
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_insert_both_before_and_after_error() {
        let text = "Some text";

        let result = insert(
            text,
            "new-section",
            "New content",
            Some("marker1"),
            Some("marker2")
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Specify only one"));
    }

    #[test]
    fn test_insert_with_nested_markers() {
        let text = r#"<!-- KUGIRI-BEGIN: outer -->
Outer content start

  <!-- KUGIRI-BEGIN: inner -->
  Inner content
  <!-- KUGIRI-END: inner -->

Outer content end
<!-- KUGIRI-END: outer -->"#;

        // Insert after inner section (which is nested)
        let result = insert(
            text,
            "new-section",
            "New section content",
            None,
            Some("inner")
        ).unwrap();

        assert!(result.contains("Inner content"));
        assert!(result.contains("<!-- KUGIRI-BEGIN: new-section -->"));
        assert!(result.contains("New section content"));

        // Check that new section has same indentation as inner
        let lines: Vec<&str> = result.lines().collect();
        let new_section_line = lines.iter()
            .find(|l| l.contains("<!-- KUGIRI-BEGIN: new-section -->"))
            .unwrap();
        assert!(new_section_line.starts_with("  "));
    }

    #[test]
    fn test_insert_neither_before_nor_after_error() {
        let text = "Some text";

        let result = insert(
            text,
            "new-section",
            "New content",
            None,
            None
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Must specify"));
    }
}
