# kugiri

Marker-based block editing CLI for maintaining sections in text files.

## Installation

```bash
cargo install --path .
```

## Usage

kugiri uses markers to identify sections in files:
- `<!-- KUGIRI-BEGIN: {id} -->` - Start of a section
- `<!-- KUGIRI-END: {id} -->` - End of a section
- `<!-- KUGIRI-INSERT: {id} -->` - Insertion point marker

### Commands

#### Insert
Add a new section before or after an existing marker:

```bash
# Insert after a marker
kugiri insert CHANGELOG.md --id v1.0.0 --after HEADER --body-file release.md -w

# Insert before a marker with content from stdin
echo "New content" | kugiri insert README.md --id section1 --before footer
```

#### Update
Replace the content of an existing section:

```bash
# Update section from file
kugiri update README.md --id help-section --body-file help.txt -w

# Update from stdin
generate-help | kugiri update README.md --id help-section -w
```

#### Remove
Remove a section and its markers:

```bash
kugiri remove README.md --id deprecated-section -w
```

#### Extract
Print the content of a section (without markers):

```bash
kugiri extract README.md --id help-section > help.txt
```

#### Trim
Remove all KUGIRI markers from a file:

```bash
kugiri trim README.md > clean.md
```

#### Wrap
Wrap content with KUGIRI markers:

```bash
# Wrap stdin content
echo "Some content" | kugiri wrap --id section-name

# Wrap content from a file
kugiri wrap --id section-name --body-file content.txt
```

### Options

- `-w, --write`: Write changes in-place (default: output to stdout)
- `--body-file <PATH|->`: Content source file (default: `-` for stdin)
- `--before <ID>`: Insert before this marker ID
- `--after <ID>`: Insert after this marker ID
- `--id <ID>`: Section identifier

## Examples

### Managing a CHANGELOG

```bash
# First release
echo "## v1.0.0 - Initial release" | kugiri insert CHANGELOG.md \
  --id v1.0.0 --after HEADER -w

# Subsequent releases
echo "## v1.1.0 - Bug fixes" | kugiri insert CHANGELOG.md \
  --id v1.1.0 --after v1.0.0 -w
```

### Updating generated documentation

```bash
# Generate and update help section
./generate-help.sh | kugiri update README.md --id CLI-HELP -w

# Extract for separate file
kugiri extract README.md --id CLI-HELP > docs/cli.md
```

## License

MIT
