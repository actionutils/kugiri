#!/bin/bash
# Script to update README.md installation section using kugiri
# Usage: ./update-installation.sh [version]
# Example: ./update-installation.sh v0.2.0

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Get the directory where the script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Get version from argument or use default
VERSION="${1:-v0.2.0}"

# Validate version format
if [[ ! "$VERSION" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}Error: Invalid version format. Expected format: v0.0.0${NC}"
    echo "Usage: $0 [version]"
    echo "Example: $0 v0.2.0"
    exit 1
fi

echo -e "${GREEN}Updating installation instructions for version: ${VERSION}${NC}"

# Change to project directory
cd "$PROJECT_DIR"

# Check if kugiri is available
if ! command -v kugiri &> /dev/null && [ ! -f "./target/debug/kugiri" ]; then
    echo -e "${YELLOW}kugiri not found. Building from source...${NC}"
    cargo build
fi

# Use local build if kugiri is not in PATH
if command -v kugiri &> /dev/null; then
    KUGIRI="kugiri"
else
    KUGIRI="./target/debug/kugiri"
fi

# Create temporary file with version replaced
TEMP_FILE=$(mktemp)
trap "rm -f $TEMP_FILE" EXIT

# Replace <VERSION> placeholder with actual version
sed "s/<VERSION>/${VERSION}/g" docs/installation-template.md > "$TEMP_FILE"

# Update the installation section
echo -e "${GREEN}Updating README.md installation section...${NC}"
$KUGIRI update README.md --id installation --body-file "$TEMP_FILE" -w

echo -e "${GREEN}✓ README.md installation section updated with version ${VERSION}${NC}"

# Optionally, show the diff
if command -v git &> /dev/null && git rev-parse --is-inside-work-tree &> /dev/null; then
    if ! git diff --quiet README.md; then
        echo -e "\n${YELLOW}Changes made:${NC}"
        git diff --stat README.md
        echo -e "\n${YELLOW}To see full diff, run:${NC} git diff README.md"
    else
        echo -e "${GREEN}No changes needed - installation section is up to date${NC}"
    fi
fi
