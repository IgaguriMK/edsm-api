
.PHONY: check
check: soft-clean
	cargo fmt -- --check
	cargo test -- --test-threads=1
	cargo clippy -- -D warnings

.PHONY: soft-clean
soft-clean:
	cargo clean -p edsm-api
