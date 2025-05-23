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
	cargo flamegraph --test loading_benchmark

.PHONY: build-maturin
build-maturin:
	.venv/bin/python -m maturin develop

.PHONY: test-python
test-python:
	.venv/bin/python -m pytest python/test

.PHONY: test
test: print-sep test-rust build-maturin test-python print-sep
