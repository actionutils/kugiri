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
# Pass arguments to kugiri (use -s -- to pass arguments)
curl -sSfL https://github.com/actionutils/kugiri/releases/latest/download/run.sh | sh -s -- update --help
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
DOWNLOAD_URL="https://github.com/actionutils/kugiri/releases/latest/download"

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
curl -sL "https://github.com/actionutils/kugiri/releases/latest/download/install.sh" | \
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

### Install from crates.io

If you have Rust and Cargo installed:

```bash
cargo install kugiri
```
