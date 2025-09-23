use crate::markers::find_section;
use anyhow::Result;

pub fn extract(text: &str, id: &str) -> Result<String> {
    let section = find_section(text, id)
        .ok_or_else(|| anyhow::anyhow!("Section with id '{}' not found", id))?;

    Ok(section.content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_section() {
        let text = r#"Header

<!-- KUGIRI-BEGIN: test-section -->
This is the content
to extract
<!-- KUGIRI-END: test-section -->

Footer"#;

        let result = extract(text, "test-section").unwrap();
        assert_eq!(result, "This is the content\nto extract");
    }

    #[test]
    fn test_extract_section_not_found() {
        let text = "Some text";
        let result = extract(text, "non-existent");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }
}
