# Design Document: **kugiri** — Marker-based Block Editing CLI

## 1. Overview
**kugiri** is a standalone, static CLI (Rust) for **idempotent edits** to text files using **marker lines**. It supports:
- **Insert**: add a new section before or after a specified marker point
- **Update**: replace the contents of an existing section identified by an `id`
- **Remove/Delete**: remove a section and its markers
- **Extract**: print the inner contents of a section (without markers)
- **Trim**: output the entire file with marker lines removed

Primary use cases:
- Maintaining CHANGELOG sections per release version.
- Injecting generated help/benchmark outputs into README files.
- Updating code/doc snippets fenced by comments in any language.

The name "kugiri" (区切り) reflects **boundaries** (BEGIN/END markers) and **sections**.

---

## 2. Goals & Non-Goals
**Goals**
- Idempotent, deterministic edits suitable for CI.
- Single static binary; zero runtime deps.
- Cross-platform (Linux/macOS/Windows), preserving original newline style.
- Safe writes (atomic replace) for in-place edits.
- Flexible output modes (in-place or stdout).

**Non-Goals (v0)**
- Streaming edits for multi-GB files (in-memory processing).
- Encoding conversions (assume UTF-8; binary files unsupported).
- Syntax-aware parsing (pure text matching).

---

## 3. Terminology
- **Markers**: Fixed format markers: `<!-- KUGIRI-BEGIN: {id} -->` and `<!-- KUGIRI-END: {id} -->`
- **Insert marker**: `<!-- KUGIRI-INSERT: {id} -->` indicates where new sections can be inserted
- **Section id**: unique identifier (e.g., `v1.4.0`) embedded in markers.
- **Position**: for insert commands, use `--before {id}` or `--after {id}` to specify insertion point relative to existing marker sections

---

## 4. CLI UX

```
kugiri <COMMAND> <FILE> [flags]

Commands:
insert   Insert a new section before or after a marker
update   Update an existing section identified by --id
remove   Remove a section and its markers (alias: delete)
extract  Print inner content of a section identified by --id
trim     Output the file with all marker lines removed

Common flags:
--id <STRING>
-w, --write           Write changes in-place (default: output to stdout)

Insert/Update flags:
--body-file <PATH|->  (default: "-": read from stdin)
--before <ID>         Insert before section with this ID
--after <ID>          Insert after section with this ID

```

**Examples**
```bash
# Insert new CHANGELOG entry after a marker
kugiri insert CHANGELOG.md \
  --id v1.4.0 \
  --after CHANGELOG-TOP \
  --body-file notes.md \
  -w

# Update existing README help section
generate-help | kugiri update README.md \
  --id CLI-HELP \
  -w

# Extract an existing section's inner text
kugiri extract README.md \
  --id CLI-HELP > help.txt

# Remove a section
kugiri remove README.md --id DEPRECATED-SECTION -w

# Output file with markers stripped
kugiri trim README.md > clean.md
```

---

## 5. Functional Requirements

1. **Markers**
   * Fixed format: `<!-- KUGIRI-BEGIN: {id} -->` and `<!-- KUGIRI-END: {id} -->`
   * Markers matched as **full lines**, whitespace-sensitive
   * Auto-detect comment style based on file extension (future enhancement)

2. **Insert**
   * Add new section with markers
   * Use `--before {id}` or `--after {id}` to specify position relative to existing sections
   * If marker ID not found → error
   * Special insertion marker: `<!-- KUGIRI-INSERT: {id} -->` for explicit insertion points

3. **Update**
   * Replace inner content of existing section
   * Preserve marker lines
   * If section not found → error

4. **Remove/Delete**
   * Remove entire section including markers
   * If section not found → error

5. **Extract**
   * Print inner content (no markers)
   * If not found → error

6. **Trim**
   * Output entire file with all KUGIRI marker lines removed

7. **Positioning**
   * Exact string match for section IDs (not regex)
   * `--before` and `--after` are mutually exclusive

---

## 6. Non-Functional Requirements

* **Atomic writes** (tempfile + rename) for in-place edits.
* **Output modes**: stdout (default) or in-place edit with `-w/--write` flag.
* **EOL preservation**: detect CRLF vs LF; preserve on write.
* **Performance**: O(N) over file size; suitable for typical docs.
* **Idempotency**: repeated runs are no-ops when nothing changes.

---

## 7. Data & Text Model

* **Encoding**: UTF-8 only (non-UTF-8 → error).
* **Newlines**: detect `\r\n` → treat as CRLF; normalize to LF internally; restore on write.
* **Body input**: file or stdin; normalize to LF internally; ensure trailing newline before end marker.

---

## 8. Matching Rules

* **Markers**: exact string match, full line comparison.
* **Block span**: start at begin marker; end after end marker.
* **Section IDs**: exact string match (not regex).

---

## 9. Algorithms (pseudocode)

**Insert**
```
read file → text
begin = "<!-- KUGIRI-BEGIN: {id} -->"
end = "<!-- KUGIRI-END: {id} -->"
if --before: find section with before_id, insert before it
else if --after: find section with after_id, insert after it
else: error (position required)
write_result(file, text, --write)
```

**Update**
```
read file → text
find section by id
if found: replace inner content
else: error (section not found)
write_result(file, text, --write)
```

**Remove**
```
read file → text
find section by id
if found: remove entire section including markers
else: error (section not found)
write_result(file, text, --write)
```

**Extract**
```
read file → text
find section by id
if found: print inner content to stdout
else: error
```

**Trim**
```
read file → text
remove all lines matching KUGIRI markers
print result to stdout
```

---

## 10. Errors & Messages

* Marker mismatch: "Found begin marker but not end marker; file may be corrupt."
* Section not found: "Section with id '{id}' not found"
* Marker not found: "Marker section with id '{id}' not found"
* Conflicting anchors: "Specify only one of --before or --after"
* Missing position: "Must specify --before or --after for insert command"

---

## 11. Edge Cases

* File without trailing newline → add separator newline before new block when appending.
* Duplicate ids → operate on the **first**; discourage duplicates by design.
* Whitespace around markers must match exactly (safer).
* Windows paths & CRLF supported.
* Very large files (>100MB) may require future streaming mode.

---

## 12. Security

* No network access.
* Safe string matching (no regex vulnerabilities).
* Atomic writes for in-place edits.

---

## 13. Testing

* **Unit**: marker matching; section finding; positioning; EOL LF/CRLF; idempotency; error paths.
* **Golden**: Markdown CHANGELOG scenarios; multiple comment styles.
* **CI matrix**: ubuntu-latest, macos-latest, windows-latest; musl static for Linux.

---

## 14. Extensibility (Future)

* `list` (discover ids).
* Batch operations from a manifest (YAML/JSON).
* Integrations (mdBook, MkDocs, Docusaurus).

---

## 15. Packaging & Release

* **Lang**: Rust (MSRV ~1.77+).
* **Crates**: `clap`, `anyhow`, `tempfile`.
* **Binary**: `kugiri`.
* **Author**: haya14busa
* **License**: MIT.
* **Releases**: GitHub Releases (details TBD).
* **Install**: Cargo.

---

## 16. Defaults & Conventions

* Marker format: `<!-- KUGIRI-BEGIN: {id} -->` and `<!-- KUGIRI-END: {id} -->`
* Insert anchors: `<!-- KUGIRI-INSERT: {id} -->`
* CHANGELOG example:

    ```bash
    # First release: insert after marker
    kugiri insert CHANGELOG.md \
      --id "v1.0.0" \
      --after "CHANGELOG-TOP" \
      --body-file release_notes.md \
      -w

    # Subsequent releases: insert after previous version
    kugiri insert CHANGELOG.md \
      --id "v1.1.0" \
      --after "v1.0.0" \
      --body-file release_notes.md \
      -w
    ```

---

## 17. Implementation Sketch (Rust Modules)

```
src/
  main.rs     // CLI wiring (clap)
  io.rs       // read/write, EOL detect, atomic replace
  markers.rs  // marker matching, section finding
  insert.rs   // insert command implementation
  update.rs   // update command implementation
  remove.rs   // remove command implementation
  extract.rs  // extract command implementation
  trim.rs     // trim command implementation
  error.rs    // error types, exit code mapping
  tests/      // unit & golden tests
```
