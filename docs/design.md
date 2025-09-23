# Design Document: **kugiri** — Marker-based Block Editing CLI

## 1. Overview
**kugiri** is a standalone, static CLI (Rust) for **idempotent edits** to text files using **marker lines**. It supports:
- **Upsert**: replace the contents of a section identified by an `id` if it exists; otherwise insert a new section at an anchor (before/after), BOF, or EOF.
- **Extract**: print the inner contents of a section (without markers).

Primary use cases:
- Maintaining CHANGELOG sections per release version.
- Injecting generated help/benchmark outputs into README files.
- Updating code/doc snippets fenced by comments in any language.

The name “kugiri” (区切り) reflects **boundaries** (BEGIN/END markers) and **sections**.

---

## 2. Goals & Non-Goals
**Goals**
- Idempotent, deterministic edits suitable for CI.
- Single static binary; zero runtime deps.
- Cross-platform (Linux/macOS/Windows), preserving original newline style.
- Safe writes (atomic replace) and optional backups.
- Flexible markers & anchors (Markdown or arbitrary comment syntaxes).

**Non-Goals (v0)**
- Streaming edits for multi-GB files (in-memory processing).
- Encoding conversions (assume UTF-8; binary files unsupported).
- Syntax-aware parsing (pure text/regex).

---

## 3. Terminology
- **Marker template**: format with `{mark}` and `{id}` → expands into exact **begin**/**end** marker lines.  
  Example: `<!-- {mark}: SECTION {id} -->` → `BEGIN/END` markers.
- **Section id**: unique identifier (e.g., `v1.4.0`) embedded in markers.
- **Anchor**: regex indicating where to insert a new section when absent (either `--insertafter` or `--insertbefore`).

---

## 4. CLI UX

```
kugiri <COMMAND> <FILE> \[flags]

Commands:
upsert   Replace or insert a section identified by --id
extract  Print inner content of a section identified by --id

Common flags:
\--id <STRING>
\--marker-template <STRING>  (default: "<!-- {mark}: SECTION {id} -->")

Upsert flags:
\--body-file \<PATH|- >       (default: "-": read from stdin)
\--insertafter <REGEX>
\--insertbefore <REGEX>
\--prepend-if-no-anchor
\--append-if-no-anchor       (default fallback)
\--backup

Exit codes:
0 ok (including no-op)
2 invalid args
3 file I/O error
4 marker mismatch (begin without end)
5 section not found (extract)
6 regex error (anchor)

````

**Examples**
```bash
# Upsert CHANGELOG entry (id = version), insert after an anchor line
kugiri upsert CHANGELOG.md \
  --id v1.4.0 \
  --marker-template "<!-- {mark}: CHANGELOG {id} -->" \
  --insertafter "<!-- CHANGELOG:INSERT-HERE -->" \
  --body-file notes.md --backup

# Upsert README help section, prepend if no anchor
generate-help | kugiri upsert README.md \
  --id CLI-HELP \
  --marker-template "<!-- {mark}: HELP {id} -->" \
  --prepend-if-no-anchor

# Extract an existing section's inner text
kugiri extract README.md \
  --id CLI-HELP \
  --marker-template "<!-- {mark}: HELP {id} -->" > help.txt
````

---

## 5. Functional Requirements

1. **Marker expansion**

   * `marker-template` must contain `{mark}` and `{id}`.
   * Expand with `{mark}=BEGIN/END`; markers matched as **full lines** (`^...$`), whitespace-sensitive.

2. **Upsert**

   * If begin+end (for the given id) exist:

     * Replace **inner content** only; preserve marker lines.
     * Ensure one newline after begin and before end (normalize around body).
   * If absent:

     * Insert new block at:

       1. after first `--insertafter` match, or
       2. before first `--insertbefore` match, or
       3. BOF if `--prepend-if-no-anchor`, else
       4. EOF (default or `--append-if-no-anchor`).
   * Both `--insertafter` and `--insertbefore` → error.
   * Begin without end (or vice versa) → error (exit 4).

3. **Extract**

   * Print inner content (no markers). If not found → exit 5.

4. **Anchors**

   * Rust regex in multiline mode; first match wins.
   * `--insertafter`: insert after the matched line; `--insertbefore`: before.

---

## 6. Non-Functional Requirements

* **Atomic writes** (tempfile + rename).
* **Backup**: optional `<FILE>.bak`.
* **EOL preservation**: detect CRLF vs LF; convert back on write.
* **Performance**: O(N) over file size; suitable for typical docs.
* **Idempotency**: repeated runs are no-ops when nothing changes.

---

## 7. Data & Text Model

* **Encoding**: UTF-8 only (non-UTF-8 → error).
* **Newlines**: detect `\r\n` → treat as CRLF; normalize to LF internally; restore on write.
* **Body input**: file or stdin; normalize to LF internally; ensure trailing newline before end marker.

---

## 8. Matching Rules

* **Markers**: literal match via `regex::escape`, full line (`(?m)^...$`).
* **Block span**: start at begin; end after end (and one following newline if present).
* **Anchors**: user-provided regex; invalid → exit 6.

---

## 9. Algorithms (pseudocode)

**Upsert**

```
read file → text
eol = detect_eol(text); normalized = to_lf(text)
(begin, end) = expand(marker_template, id)
if find_block(normalized, begin, end) = Some((start, end_idx)):
    new = normalized[..start] + begin + "\n" + body + ensure_nl + end + normalized[end_idx..]
else:
    if insertafter && match_after(normalized): insert after matched line
    else if insertbefore && match_before(normalized): insert before matched line
    else if prepend_if_no_anchor: new = block + normalized
    else: new = normalized + ensure_nl + block
write_atomic(file, from_lf(new, eol))
```

**Extract**

```
read file → normalized
(begin, end) = expand(...)
cap = "(?s)<begin>\n?(.*?)\n?<end>" (escaped literals)
if match: print group(1) else exit 5
```

---

## 10. Errors & Messages

* Invalid template: “marker-template must contain {mark} and {id}.”
* Marker mismatch: “Found begin marker but not end marker; file may be corrupt.” (exit 4)
* Anchor regex compile failure: “invalid insertafter/insertbefore regex: …” (exit 6)
* Section not found: “section with id '…' not found” (exit 5)
* Conflicting anchors: “Specify only one of --insertbefore or --insertafter.” (exit 2)

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
* Regex via `regex` crate (safe engine; avoids catastrophic backtracking).
* Atomic writes; optional backups.

---

## 13. Testing

* **Unit**: marker expansion; spans; anchors; EOL LF/CRLF; idempotency; error paths.
* **Golden**: Markdown CHANGELOG scenarios; multiple comment styles.
* **CI matrix**: ubuntu-latest, macos-latest, windows-latest; musl static for Linux.

---

## 14. Extensibility (Future)

* `list` (discover ids), `delete`, `rename`.
* Batch upserts from a manifest (YAML/JSON).
* Extra template vars (`{date}`, etc.).
* Encoding detection (UTF-16) with flags.
* Integrations (mdBook, MkDocs, Docusaurus).

---

## 15. Packaging & Release

* **Lang**: Rust (MSRV \~1.77+).
* **Crates**: `clap`, `regex`, `anyhow`, `tempfile`.
* **Binary**: `kugiri`.
* **License**: MIT/Apache-2.0.
* **Releases**: GitHub Releases + checksums (+ Sigstore/attestations).
* **Install**: Cargo, Homebrew, Scoop.

---

## 16. Defaults & Conventions

* Default template: `<!-- {mark}: SECTION {id} -->`
* CHANGELOG:

  * Anchor: `<!-- CHANGELOG:INSERT-HERE -->` near top.
  * `id` = version tag (e.g., `v1.4.0`).
  * Example:

    ```bash
    kugiri upsert CHANGELOG.md \
      --id "$TAG" \
      --marker-template "<!-- {mark}: CHANGELOG {id} -->" \
      --insertafter "<!-- CHANGELOG:INSERT-HERE -->" \
      --body-file release_notes.md
    ```

---

## 17. Implementation Sketch (Rust Modules)

```
src/
  main.rs     // CLI wiring (clap)
  io.rs       // read/write, EOL detect, atomic replace, backups
  markers.rs  // marker expansion, span finding, extract
  upsert.rs   // upsert algorithm (anchors, fallbacks)
  extract.rs  // extract algorithm
  error.rs    // error types, exit code mapping
  tests/      // unit & golden tests
```

