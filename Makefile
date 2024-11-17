BOOTSTRAP_DIR = ./bootstrap
BOOTSTRAP_SMOKE_TEST_SCRIPT = ./test/smoke/test-bootstrap.sh
BOOTSTRAP_REGRESSION_TEST_SCRIPT = ./test/regression/test-bootstrap.sh

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
	$(BOOTSTRAP_SMOKE_TEST_SCRIPT) ./test/smoke ./bootstrap/target/debug/bootstrap

# Run the regression tests
.PHONY: test-regression
test-regression: bootstrap
	$(BOOTSTRAP_REGRESSION_TEST_SCRIPT) ./test/regression ./bootstrap/target/debug/bootstrap

# Run the tests
.PHONY: test
test: test-bootstrap test-smoke test-regression