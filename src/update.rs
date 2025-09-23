use crate::markers::find_section;
use anyhow::Result;

pub fn update(text: &str, id: &str, content: &str) -> Result<String> {
    let section = find_section(text, id)
        .ok_or_else(|| anyhow::anyhow!("Section with id '{}' not found", id))?;

    let lines: Vec<&str> = text.lines().collect();
    let mut result: Vec<String> = Vec::new();

    // Trim trailing newline from content
    let content_trimmed = content.trim_end_matches('\n');

    // Add indent to each line of new content
    let indented_content = content_trimmed
        .lines()
        .map(|line| {
            if line.is_empty() {
                line.to_string()
            } else {
                format!("{}{}", section.indent, line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    for (idx, &line) in lines.iter().enumerate() {
        if idx == section.start_line {
            // Keep the begin marker
            result.push(line.to_string());
            // Add new content with proper indentation
            result.push(indented_content.clone());
            // Skip to the end marker
        } else if idx > section.start_line && idx < section.end_line {
            // Skip old content
            continue;
        } else {
            result.push(line.to_string());
        }
    }

    Ok(result.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_existing_section() {
        let text = r#"# README

<!-- KUGIRI-BEGIN: test-section -->
Old content here
that should be replaced
<!-- KUGIRI-END: test-section -->

Footer text"#;

        let result = update(
            text,
            "test-section",
            "New updated content"
        ).unwrap();

        assert!(result.contains("<!-- KUGIRI-BEGIN: test-section -->"));
        assert!(result.contains("New updated content"));
        assert!(result.contains("<!-- KUGIRI-END: test-section -->"));
        assert!(!result.contains("Old content here"));
        assert!(result.contains("Footer text"));
    }

    #[test]
    fn test_update_section_not_found() {
        let text = "Some text without markers";

        let result = update(
            text,
            "non-existent",
            "New content"
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_update_nested_section() {
        let text = r#"<!-- KUGIRI-BEGIN: outer -->
Outer content start

  <!-- KUGIRI-BEGIN: inner -->
  Old inner content
  <!-- KUGIRI-END: inner -->

Outer content end
<!-- KUGIRI-END: outer -->"#;

        let result = update(
            text,
            "inner",
            "New inner content"
        ).unwrap();

        assert!(result.contains("Outer content start"));
        assert!(result.contains("New inner content"));
        assert!(!result.contains("Old inner content"));
        assert!(result.contains("Outer content end"));

        // Check that inner content has proper indentation
        assert!(result.contains("  New inner content"));
    }

    #[test]
    fn test_update_preserves_surrounding_content() {
        let text = r#"Header content

<!-- KUGIRI-BEGIN: section1 -->
Section 1 content
<!-- KUGIRI-END: section1 -->

Middle content

<!-- KUGIRI-BEGIN: section2 -->
Section 2 old content
<!-- KUGIRI-END: section2 -->

Footer content"#;

        let result = update(
            text,
            "section2",
            "Section 2 new content"
        ).unwrap();

        assert!(result.contains("Header content"));
        assert!(result.contains("Section 1 content"));
        assert!(result.contains("Middle content"));
        assert!(result.contains("Section 2 new content"));
        assert!(!result.contains("Section 2 old content"));
        assert!(result.contains("Footer content"));
    }
}
