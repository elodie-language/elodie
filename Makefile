BOOTSTRAP_DIR = ./bootstrap
E2E_TEST_SCRIPT = ./test/e2e/test.sh
SMOKE_TEST_SCRIPT = ./test/smoke/test.sh

# Default target
.PHONY: all
all: test

# Build the Rust project in the bootstrap directory
.PHONY: bootstrap
bootstrap:
	cargo build --manifest-path $(BOOTSTRAP_DIR)/Cargo.toml

# Run the bootstrap tests
.PHONY: test-bootstrap
test-bootstrap: bootstrap
	cargo test --manifest-path $(BOOTSTRAP_DIR)/Cargo.toml

# Run the smoke tests
.PHONY: test-smoke
test-smoke: bootstrap
	$(SMOKE_TEST_SCRIPT) ./test/smoke ./bootstrap/target/debug/bootstrap

# Run the e2e tests
.PHONY: test-e2e
test-e2e: bootstrap
	$(E2E_TEST_SCRIPT) ./test/e2e ./bootstrap/target/debug/bootstrap

# Run the tests
.PHONY: test
test: test-bootstrap test-smoke test-e2e