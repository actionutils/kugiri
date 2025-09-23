.PHONY: all build test test-unit test-golden clean install help

CARGO = cargo
TARGET = target/release/kugiri

# Default target
all: build test

# Build the project
build:
	@echo "Building kugiri..."
	@$(CARGO) build --release

# Run all tests
test: test-unit test-golden

# Run unit tests
test-unit:
	@echo "Running unit tests..."
	@$(CARGO) test

# Run golden tests
test-golden: build
	@echo "Running golden tests..."
	@cd tests && ./run_golden_tests.sh

# Update golden files (removes them so they get recreated)
update-golden:
	@echo "Removing golden files for regeneration..."
	@rm -f tests/golden/*.golden
	@$(MAKE) test-golden

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@$(CARGO) clean
	@rm -f tests/golden/*.output

# Install the binary
install: build
	@echo "Installing kugiri..."
	@$(CARGO) install --path .

# Help target
help:
	@echo "kugiri - Marker-based block editing CLI"
	@echo ""
	@echo "Available targets:"
	@echo "  make build        - Build the release binary"
	@echo "  make test         - Run all tests (unit + golden)"
	@echo "  make test-unit    - Run unit tests only"
	@echo "  make test-golden  - Run golden tests only"
	@echo "  make update-golden - Regenerate all golden test files"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make install      - Install kugiri binary"
	@echo "  make help         - Show this help message"
