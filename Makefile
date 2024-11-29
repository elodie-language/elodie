BOOTSTRAP_DIR = ./bootstrap

BOOTSTRAP_RUN_SMOKE_TEST_SCRIPT = ./test/smoke/test-bootstrap-run.sh
BOOTSTRAP_BUILD_SMOKE_TEST_SCRIPT = ./test/smoke/test-bootstrap-build.sh

BOOTSTRAP_SMOKE_TEST_RUNNER_TEST_SCRIPT = ./test/smoke-test-runner/test-bootstrap.sh
BOOTSTRAP_REGRESSION_TEST_SCRIPT = ./test/regression/test-bootstrap.sh

BOOTSTRAP_CORE_RUN_TEST_SCRIPT = ./src/lib/core/test-bootstrap.sh
BOOTSTRAP_STD_TEST_SCRIPT = ./src/lib/std/test-bootstrap.sh

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
test-smoke: bootstrap test-smoke-bootstrap-run test-smoke-bootstrap-build

.PHONY: test-smoke-bootstrap-run
test-smoke-bootstrap-run: bootstrap
	$(BOOTSTRAP_RUN_SMOKE_TEST_SCRIPT) ./test/smoke ./bootstrap/target/debug/bootstrap

.PHONY: test-smoke-bootstrap-build
test-smoke-bootstrap-build: bootstrap
	$(BOOTSTRAP_BUILD_SMOKE_TEST_SCRIPT) ./test/smoke ./bootstrap/target/debug/bootstrap

.PHONY: test-smoke-test-runner
test-smoke-test-runner: bootstrap
	$(BOOTSTRAP_SMOKE_TEST_RUNNER_TEST_SCRIPT) ./test/smoke-test-runner ./bootstrap/target/debug/bootstrap

.PHONY: test-regression
test-regression: bootstrap
	$(BOOTSTRAP_REGRESSION_TEST_SCRIPT) ./test/regression ./bootstrap/target/debug/bootstrap

.PHONY: test-core
test-core: bootstrap
	$(BOOTSTRAP_CORE_RUN_TEST_SCRIPT) ./src/lib/core ./bootstrap/target/debug/bootstrap


.PHONY: test-std
test-std: bootstrap
	$(BOOTSTRAP_STD_TEST_SCRIPT) ./src/lib/std ./bootstrap/target/debug/bootstrap

# Run the tests
.PHONY: test
test: test-bootstrap test-smoke test-smoke-test-runner test-regression test-core test-std