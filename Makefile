.PHONY: install
install:
	.venv/bin/python -m pip install -Ur test-requirements.txt


.PHONY: print-sep
print-sep:
	@echo ""
	@echo ""
	@echo ""
	@echo ""
	@echo "==============================================="
	@echo ""

.PHONY: test-rust
test-rust:
	cargo test

.PHONY: bench-rust
bench-rust:
	cargo bench --bench benchmark

.PHONY: bench-python
bench-python:
	asv run

.PHONY: profile-rust
profile-rust:
	cargo install flamegraph
	cargo flamegraph --bench benchmark -- --bench

.PHONY: build-maturin
build-maturin:
	.venv/bin/python -m maturin develop

.PHONY: test-python
test-python:
	.venv/bin/python -m pytest python/test

.PHONY: test
test: print-sep test-rust build-maturin test-python print-sep
