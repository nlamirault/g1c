# g1c Makefile

.PHONY: all build release run clean check test lint fmt docs install uninstall help

# Directories
BIN_DIR ?= ./target/release
# Installation directory
PREFIX ?= ~/.local/bin

# Build settings
CARGO ?= cargo
CARGO_FLAGS ?=
RELEASE_FLAGS ?= --release
RUN_FLAGS ?=

# Default target
all: build

# Development builds
build:
	$(CARGO) build $(CARGO_FLAGS)

# Release builds
release:
	$(CARGO) build $(RELEASE_FLAGS)

# Run the application
run:
	RUST_BACKTRACE=1 $(CARGO) run $(CARGO_FLAGS) $(RUN_FLAGS)

# Run with specific project
run-project:
	@echo "Enter project ID:"; \
	read PROJECT_ID; \
	RUST_BACKTRACE=1 $(CARGO) run $(CARGO_FLAGS) -- --project $$PROJECT_ID

# Clean build artifacts
clean:
	$(CARGO) clean
	rm -rf target/

# Check code without building
check:
	$(CARGO) check

# Run tests
test:
	$(CARGO) test $(CARGO_FLAGS)

# Lint the code
lint:
	$(CARGO) clippy -- -D warnings

# Format code
fmt:
	$(CARGO) fmt

# Generate documentation
docs:
	$(CARGO) doc --no-deps
	@echo "Documentation generated in target/doc"

# Install the application
install: release
	@mkdir -p $(PREFIX)
	@cp $(BIN_DIR)/g1c $(PREFIX)/
	@chmod +x $(PREFIX)/g1c
	@echo "Installed g1c to $(PREFIX)/g1c"

# Uninstall the application
uninstall:
	@rm -f $(PREFIX)/g1c
	@echo "Uninstalled g1c from $(PREFIX)/g1c"

# Check environment
check-env:
	@echo "Checking required dependencies..."
	@which gcloud > /dev/null || (echo "Error: gcloud CLI not found. Please install Google Cloud SDK." && exit 1)
	@gcloud auth list --filter=status:ACTIVE --format="value(account)" | grep -q "@" || echo "Warning: No active gcloud account found. Run 'gcloud auth login' to authenticate."
	@PROJECT=$$(gcloud config get-value project 2>/dev/null); \
	if [ -z "$$PROJECT" ] || [ "$$PROJECT" = "(unset)" ]; then \
		echo "Warning: No default project set. Run 'gcloud config set project PROJECT_ID'"; \
	else \
		echo "Using project: $$PROJECT"; \
	fi

# Help documentation
help:
	@echo "g1c - Google Cloud Instances TUI"
	@echo ""
	@echo "Usage:"
	@echo "  make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  all         Build the development version (default)"
	@echo "  build       Build the development version"
	@echo "  release     Build the release version"
	@echo "  run         Run the development version"
	@echo "  run-project Run with a specific project ID (--project)"
	@echo "  clean       Remove build artifacts"
	@echo "  check       Check code without building"
	@echo "  test        Run tests"
	@echo "  lint        Run linter (clippy)"
	@echo "  fmt         Format code"
	@echo "  docs        Generate documentation"
	@echo "  install     Install the application to $(PREFIX)"
	@echo "  uninstall   Uninstall the application"
	@echo "  check-env   Check if environment is properly set up"
	@echo "  help        Show this help message"
	@echo ""
	@echo "Environment variables:"
	@echo "  CARGO_FLAGS    Additional flags for cargo"
	@echo "  RELEASE_FLAGS  Additional flags for release build"
	@echo "  RUN_FLAGS      Additional flags for run target"
	@echo "  PREFIX         Installation directory (default: ~/.local/bin)"
	@echo ""
	@echo "Command line options:"
	@echo "  --project, -p  Specify Google Cloud project ID"
	@echo "  --region, -g   Filter by Google Cloud region"
	@echo "  --refresh, -r  Set auto-refresh interval in seconds"
	@echo "  --config, -c   Path to config file"
