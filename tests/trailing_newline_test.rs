use kugiri::{insert, remove, trim, update, upsert};

#[test]
fn test_update_preserves_trailing_newline() {
    let text_with_newline =
        "Header\n<!-- KUGIRI-BEGIN: test -->\nOld\n<!-- KUGIRI-END: test -->\nFooter\n";
    let text_without_newline =
        "Header\n<!-- KUGIRI-BEGIN: test -->\nOld\n<!-- KUGIRI-END: test -->\nFooter";

    let result_with = update(text_with_newline, "test", "New").unwrap();
    let result_without = update(text_without_newline, "test", "New").unwrap();

    assert!(
        result_with.ends_with('\n'),
        "Should preserve trailing newline"
    );
    assert!(
        !result_without.ends_with('\n'),
        "Should not add trailing newline"
    );
}

#[test]
fn test_insert_preserves_trailing_newline() {
    let text_with_newline =
        "Header\n<!-- KUGIRI-BEGIN: anchor -->\nContent\n<!-- KUGIRI-END: anchor -->\nFooter\n";
    let text_without_newline =
        "Header\n<!-- KUGIRI-BEGIN: anchor -->\nContent\n<!-- KUGIRI-END: anchor -->\nFooter";

    let result_with = insert(
        text_with_newline,
        "new",
        "New content",
        None,
        Some("anchor"),
    )
    .unwrap();
    let result_without = insert(
        text_without_newline,
        "new",
        "New content",
        None,
        Some("anchor"),
    )
    .unwrap();

    assert!(
        result_with.ends_with('\n'),
        "Should preserve trailing newline"
    );
    assert!(
        !result_without.ends_with('\n'),
        "Should not add trailing newline"
    );
}

#[test]
fn test_remove_preserves_trailing_newline() {
    let text_with_newline =
        "Header\n<!-- KUGIRI-BEGIN: test -->\nContent\n<!-- KUGIRI-END: test -->\nFooter\n";
    let text_without_newline =
        "Header\n<!-- KUGIRI-BEGIN: test -->\nContent\n<!-- KUGIRI-END: test -->\nFooter";

    let result_with = remove(text_with_newline, "test").unwrap();
    let result_without = remove(text_without_newline, "test").unwrap();

    assert!(
        result_with.ends_with('\n'),
        "Should preserve trailing newline"
    );
    assert!(
        !result_without.ends_with('\n'),
        "Should not add trailing newline"
    );
}

#[test]
fn test_trim_preserves_trailing_newline() {
    let text_with_newline =
        "Header\n<!-- KUGIRI-BEGIN: test -->\nContent\n<!-- KUGIRI-END: test -->\nFooter\n";
    let text_without_newline =
        "Header\n<!-- KUGIRI-BEGIN: test -->\nContent\n<!-- KUGIRI-END: test -->\nFooter";

    let result_with = trim(text_with_newline);
    let result_without = trim(text_without_newline);

    assert!(
        result_with.ends_with('\n'),
        "Should preserve trailing newline"
    );
    assert!(
        !result_without.ends_with('\n'),
        "Should not add trailing newline"
    );
}

#[test]
fn test_upsert_preserves_trailing_newline() {
    // Test update path
    let text_with_newline =
        "Header\n<!-- KUGIRI-BEGIN: test -->\nOld\n<!-- KUGIRI-END: test -->\nFooter\n";
    let text_without_newline =
        "Header\n<!-- KUGIRI-BEGIN: test -->\nOld\n<!-- KUGIRI-END: test -->\nFooter";

    let result_with = upsert(text_with_newline, "test", "New", None, Some("anchor")).unwrap();
    let result_without = upsert(text_without_newline, "test", "New", None, Some("anchor")).unwrap();

    assert!(
        result_with.ends_with('\n'),
        "Should preserve trailing newline on update"
    );
    assert!(
        !result_without.ends_with('\n'),
        "Should not add trailing newline on update"
    );

    // Test insert path
    let text_with_newline_insert =
        "Header\n<!-- KUGIRI-BEGIN: anchor -->\nContent\n<!-- KUGIRI-END: anchor -->\nFooter\n";
    let text_without_newline_insert =
        "Header\n<!-- KUGIRI-BEGIN: anchor -->\nContent\n<!-- KUGIRI-END: anchor -->\nFooter";

    let result_with_insert = upsert(
        text_with_newline_insert,
        "new",
        "New content",
        None,
        Some("anchor"),
    )
    .unwrap();
    let result_without_insert = upsert(
        text_without_newline_insert,
        "new",
        "New content",
        None,
        Some("anchor"),
    )
    .unwrap();

    assert!(
        result_with_insert.ends_with('\n'),
        "Should preserve trailing newline on insert"
    );
    assert!(
        !result_without_insert.ends_with('\n'),
        "Should not add trailing newline on insert"
    );
}
