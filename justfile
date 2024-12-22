check:
	typos
	cargo fmt --check
	cargo clippy
	cargo test
	cargo doc

changelog:
	git cliff -o CHANGELOG.md
