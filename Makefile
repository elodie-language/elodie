BOOTSTRAP_DIR = ./bootstrap
TEST_SCRIPT = ./test/test.sh

# Default target
.PHONY: all
all: test

# Build the Rust project in the bootstrap directory
.PHONY: bootstrap
bootstrap:
	cargo build --manifest-path $(BOOTSTRAP_DIR)/Cargo.toml

# Run the tests
.PHONY: test
test: bootstrap
	cargo test --manifest-path $(BOOTSTRAP_DIR)/Cargo.toml
	$(TEST_SCRIPT) ./test ./bootstrap/target/debug/bootstrap