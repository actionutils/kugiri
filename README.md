# kugiri

Marker-based block editing CLI for maintaining sections in text files.

<!-- KUGIRI-BEGIN: installation -->
<!--
  DO NOT EDIT THIS SECTION IN README.md DIRECTLY!

  To update the installation instructions:
  1. Edit this template file: docs/installation-template.md
  2. Run: make update-readme-install VERSION=vX.X.X
     or: ./scripts/update-installation.sh vX.X.X

  This content is automatically inserted into README.md between KUGIRI markers
  using kugiri itself. The VERSION placeholders will be replaced with the
  specified version number.
-->

## Installation

### Quick Install

Install the latest release:

```bash
curl -sSfL https://github.com/actionutils/kugiri/releases/latest/download/install.sh | sh
```

Or run without installation:

```bash
# Run kugiri directly without installation
curl -sSfL https://github.com/actionutils/kugiri/releases/latest/download/run.sh | sh

# Pass arguments to kugiri (use -s -- to pass arguments)
curl -sSfL https://github.com/actionutils/kugiri/releases/latest/download/run.sh | sh -s -- --help

# Example: Update a file section directly
curl -sSfL https://github.com/actionutils/kugiri/releases/latest/download/run.sh | sh -s -- update README.md --id section --body-file content.txt -w

# Example: Extract content from a file
curl -sSfL https://github.com/actionutils/kugiri/releases/latest/download/run.sh | sh -s -- extract README.md --id help-section
```

### Install Specific Version

```bash
VERSION="v0.2.0"
curl -sSfL https://github.com/actionutils/kugiri/releases/download/${VERSION}/install.sh | sh
```

### Secure Installation with Verification

#### Using Cosign (Recommended)

Verify the installation script before executing:

```bash
VERSION="v0.2.0"
SCRIPT="install.sh"  # or "run.sh"
DOWNLOAD_URL="https://github.com/actionutils/kugiri/releases/download/${VERSION}"

curl -sL "${DOWNLOAD_URL}/${SCRIPT}" | \
    (tmpfile=$(mktemp); cat > "$tmpfile"; \
     cosign verify-blob \
       --certificate-identity-regexp '^https://github.com/actionutils/trusted-go-releaser/.github/workflows/trusted-release-workflow.yml@.*$' \
       --certificate-oidc-issuer 'https://token.actions.githubusercontent.com' \
       --certificate "${DOWNLOAD_URL}/${SCRIPT}.pem" \
       --signature "${DOWNLOAD_URL}/${SCRIPT}.sig" \
       "$tmpfile" && \
     sh "$tmpfile"; rm -f "$tmpfile")
```

#### Using GitHub CLI Attestations

Verify using GitHub's attestation feature:

```bash
VERSION="v0.2.0"

curl -sL "https://github.com/actionutils/kugiri/releases/download/${VERSION}/install.sh" | \
    (tmpfile=$(mktemp); cat > "$tmpfile"; \
     gh attestation verify --repo=actionutils/kugiri \
       --signer-workflow='actionutils/trusted-go-releaser/.github/workflows/trusted-release-workflow.yml' \
       "$tmpfile" && \
     sh "$tmpfile"; rm -f "$tmpfile")
```

### Build from Source

If you have Rust and Cargo installed:

```bash
cargo install --git https://github.com/actionutils/kugiri
```

Or clone and build locally:

```bash
git clone https://github.com/actionutils/kugiri.git
cd kugiri
cargo install --path .
```
<!-- KUGIRI-END: installation -->

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