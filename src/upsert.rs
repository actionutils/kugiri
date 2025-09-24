use crate::insert::insert;
use crate::markers::find_section;
use crate::update::update;
use anyhow::Result;

pub fn upsert(
    text: &str,
    id: &str,
    content: &str,
    before: Option<&str>,
    after: Option<&str>,
) -> Result<String> {
    // Check if section exists
    if find_section(text, id).is_some() {
        // Section exists, update it
        update(text, id, content)
    } else {
        // Section doesn't exist, insert it
        insert(text, id, content, before, after)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upsert_updates_existing() {
        let text = r#"# README

<!-- KUGIRI-BEGIN: existing -->
Old content
<!-- KUGIRI-END: existing -->

<!-- KUGIRI-BEGIN: anchor -->
Anchor content
<!-- KUGIRI-END: anchor -->

Footer"#;

        let result = upsert(text, "existing", "Updated content", None, Some("anchor")).unwrap();

        assert!(result.contains("<!-- KUGIRI-BEGIN: existing -->"));
        assert!(result.contains("Updated content"));
        assert!(!result.contains("Old content"));
    }

    #[test]
    fn test_upsert_inserts_new() {
        let text = r#"# README

<!-- KUGIRI-BEGIN: existing -->
Existing content
<!-- KUGIRI-END: existing -->

Footer"#;

        let result = upsert(text, "new-section", "New content", None, Some("existing")).unwrap();

        assert!(result.contains("<!-- KUGIRI-BEGIN: new-section -->"));
        assert!(result.contains("New content"));
        assert!(result.contains("<!-- KUGIRI-END: new-section -->"));

        // Check order: existing should come before new-section
        let existing_pos = result.find("<!-- KUGIRI-END: existing -->").unwrap();
        let new_pos = result.find("<!-- KUGIRI-BEGIN: new-section -->").unwrap();
        assert!(existing_pos < new_pos);
    }

    #[test]
    fn test_upsert_requires_position_for_new() {
        let text = "Some text";

        let result = upsert(text, "new-section", "New content", None, None);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Must specify"));
    }
}
