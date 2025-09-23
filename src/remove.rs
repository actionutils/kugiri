use crate::markers::find_section;
use anyhow::Result;

pub fn remove(text: &str, id: &str) -> Result<String> {
    let section = find_section(text, id)
        .ok_or_else(|| anyhow::anyhow!("Section with id '{id}' not found"))?;

    let lines: Vec<&str> = text.lines().collect();
    let mut result: Vec<&str> = Vec::new();

    for (idx, &line) in lines.iter().enumerate() {
        if idx >= section.start_line && idx <= section.end_line {
            // Skip the entire section including markers
            continue;
        }
        result.push(line);
    }

    Ok(result.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_section() {
        let text = r#"Header

<!-- KUGIRI-BEGIN: to-remove -->
Content to remove
<!-- KUGIRI-END: to-remove -->

Footer"#;

        let result = remove(text, "to-remove").unwrap();

        assert!(result.contains("Header"));
        assert!(result.contains("Footer"));
        assert!(!result.contains("KUGIRI-BEGIN: to-remove"));
        assert!(!result.contains("Content to remove"));
        assert!(!result.contains("KUGIRI-END: to-remove"));
    }

    #[test]
    fn test_remove_section_not_found() {
        let text = "Some text";
        let result = remove(text, "non-existent");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }
}
