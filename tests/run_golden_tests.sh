#!/bin/bash
set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# Build the project
echo "Building kugiri..."
cargo build --release 2>/dev/null
KUGIRI="../target/release/kugiri"

# Change to tests directory
cd "$(dirname "$0")"

# Create golden directory if it doesn't exist
mkdir -p golden

# Test counter
TOTAL=0
PASSED=0
FAILED=0

# Function to run a test
run_test() {
    local name="$1"
    local cmd="$2"
    local golden_file="golden/${name}.golden"
    local output_file="/tmp/${name}.output"

    TOTAL=$((TOTAL + 1))

    echo -n "Testing $name... "

    # Run command and capture output
    if eval "$cmd" > "$output_file" 2>&1; then
        if [ -f "$golden_file" ]; then
            # Normalize line endings for comparison (handle Windows vs Unix)
            # Create temporary normalized files
            local golden_normalized="/tmp/${name}.golden.normalized"
            local output_normalized="/tmp/${name}.output.normalized"

            # Convert to Unix line endings for comparison
            tr -d '\r' < "$golden_file" > "$golden_normalized"
            tr -d '\r' < "$output_file" > "$output_normalized"

            # Compare with golden file
            if diff -u "$golden_normalized" "$output_normalized" > /dev/null 2>&1; then
                echo -e "${GREEN}PASS${NC}"
                PASSED=$((PASSED + 1))
            else
                echo -e "${RED}FAIL${NC} (output differs)"
                echo "  Expected vs Actual:"
                diff -u "$golden_normalized" "$output_normalized" | head -20
                FAILED=$((FAILED + 1))
            fi

            # Clean up temporary files
            rm -f "$golden_normalized" "$output_normalized"
        else
            # Create golden file if it doesn't exist
            cp "$output_file" "$golden_file"
            echo "CREATED golden file"
            PASSED=$((PASSED + 1))
        fi
    else
        echo -e "${RED}FAIL${NC} (command failed)"
        cat "$output_file"
        FAILED=$((FAILED + 1))
    fi
}

echo "Running golden tests..."
echo "========================"

# Extract tests
run_test "extract_basic" \
    "$KUGIRI extract fixtures/basic.md --id section1"

run_test "extract_nested_outer" \
    "$KUGIRI extract fixtures/nested.md --id outer"

run_test "extract_nested_inner" \
    "$KUGIRI extract fixtures/nested.md --id inner"

run_test "extract_indented" \
    "$KUGIRI extract fixtures/indented.md --id indented-section"

# Update tests
run_test "update_basic" \
    "echo 'Updated content' | $KUGIRI update fixtures/basic.md --id section1"

run_test "update_indented" \
    "echo 'Updated indented content' | $KUGIRI update fixtures/indented.md --id four-space-section"

run_test "update_nested" \
    "echo 'Updated inner' | $KUGIRI update fixtures/nested.md --id inner"

# Insert tests
run_test "insert_after_section" \
    "echo 'New section content' | $KUGIRI insert fixtures/basic.md --id new-section --after section1"

run_test "insert_before_section" \
    "echo 'New section content' | $KUGIRI insert fixtures/basic.md --id new-section --before section2"

run_test "insert_after_insert_marker" \
    "echo 'Content after INSERT' | $KUGIRI insert fixtures/with_insert.md --id new-section --after top-insert"

run_test "insert_indented" \
    "echo 'New indented content' | $KUGIRI insert fixtures/with_insert.md --id new-section --after indented-insert"

# Remove tests
run_test "remove_section" \
    "$KUGIRI remove fixtures/basic.md --id section1"

run_test "remove_nested" \
    "$KUGIRI remove fixtures/nested.md --id inner"

# Trim test
run_test "trim_all_markers" \
    "$KUGIRI trim fixtures/basic.md"

# Wrap test
run_test "wrap_content" \
    "echo 'Content to wrap' | $KUGIRI wrap --id wrapped-section"

run_test "wrap_multiline" \
    "echo -e 'Line 1\nLine 2\nLine 3' | $KUGIRI wrap --id multiline-section"

# Upsert tests
run_test "upsert_existing" \
    "echo 'Upserted content' | $KUGIRI upsert fixtures/basic.md --id section1 --after section2"

run_test "upsert_new" \
    "echo 'New upserted content' | $KUGIRI upsert fixtures/basic.md --id new-section --after section1"

# Summary
echo "========================"
echo "Test Results:"
echo "  Total:  $TOTAL"
echo -e "  Passed: ${GREEN}$PASSED${NC}"
if [ $FAILED -gt 0 ]; then
    echo -e "  Failed: ${RED}$FAILED${NC}"
    exit 1
else
    echo -e "  Failed: $FAILED"
fi

echo ""
if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
else
    echo -e "${RED}Some tests failed.${NC}"
    echo "To update golden files, delete the failed .golden files and re-run the tests."
fi
