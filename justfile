check:
	typos
	cargo clippy
	cargo test
	cargo doc

changelog:
	git cliff -o CHANGELOG.md
