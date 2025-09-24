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

#### Using Cosign (Recommended)

Verify the installation script before executing:

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

#### Using GitHub CLI Attestations

Verify using GitHub's attestation feature:

```bash
VERSION="<VERSION>"

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
