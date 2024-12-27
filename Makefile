BOOTSTRAP_DIR = ./bootstrap

BOOTSTRAP_SMOKE_TEST_SCRIPT = ./test/smoke/bootstrap.sh
BOOTSTRAP_E2E_TEST_SCRIPT = ./test/e2e/bootstrap.sh

# Default target
.PHONY: all
all: test

.PHONY: bootstrap
bootstrap:
	cargo build --manifest-path $(BOOTSTRAP_DIR)/Cargo.toml

.PHONY: test-bootstrap
test-bootstrap: bootstrap
	cargo test --manifest-path $(BOOTSTRAP_DIR)/Cargo.toml

.PHONY: test-smoke
test-smoke:
	$(BOOTSTRAP_SMOKE_TEST_SCRIPT) ./test/smoke ./bootstrap/target/debug/bootstrap

.PHONY: test-e2e
test-e2e:
	$(BOOTSTRAP_E2E_TEST_SCRIPT) ./test/e2e ./bootstrap/target/debug/bootstrap

.PHONY: test
test: test-bootstrap test-smoke test-e2e