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
VERSION="<VERSION>"
curl -sSfL https://github.com/actionutils/kugiri/releases/download/${VERSION}/install.sh | sh
```

### Secure Installation with Verification

For enhanced security, verify the installation scripts before executing them.

<details>
<summary><b>🔒 Verify latest version with Cosign</b></summary>

```bash
SCRIPT="install.sh"  # or "run.sh"

# Get the latest release tag
LATEST=$(curl -s https://api.github.com/repos/actionutils/kugiri/releases/latest | grep '"tag_name"' | cut -d'"' -f4)
DOWNLOAD_URL="https://github.com/actionutils/kugiri/releases/download/${LATEST}"

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

</details>

<details>
<summary><b>🔒 Verify specific version with Cosign</b></summary>

```bash
VERSION="<VERSION>"
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

</details>

<details>
<summary><b>🔒 Verify latest version with GitHub CLI</b></summary>

```bash
# Get the latest release tag
LATEST=$(curl -s https://api.github.com/repos/actionutils/kugiri/releases/latest | grep '"tag_name"' | cut -d'"' -f4)

curl -sL "https://github.com/actionutils/kugiri/releases/download/${LATEST}/install.sh" | \
    (tmpfile=$(mktemp); cat > "$tmpfile"; \
     gh attestation verify --repo=actionutils/kugiri \
       --signer-workflow='actionutils/trusted-go-releaser/.github/workflows/trusted-release-workflow.yml' \
       "$tmpfile" && \
     sh "$tmpfile"; rm -f "$tmpfile")
```

</details>

<details>
<summary><b>🔒 Verify specific version with GitHub CLI</b></summary>

```bash
VERSION="<VERSION>"

curl -sL "https://github.com/actionutils/kugiri/releases/download/${VERSION}/install.sh" | \
    (tmpfile=$(mktemp); cat > "$tmpfile"; \
     gh attestation verify --repo=actionutils/kugiri \
       --signer-workflow='actionutils/trusted-go-releaser/.github/workflows/trusted-release-workflow.yml' \
       "$tmpfile" && \
     sh "$tmpfile"; rm -f "$tmpfile")
```

</details>

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
