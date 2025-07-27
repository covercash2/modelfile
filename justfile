# list recipes
default:
  just --list

# run full checks
check:
	typos
	cargo fmt --check
	cargo clippy
	cargo test
	cargo doc

# generate a changelog
changelog:
	git cliff -o CHANGELOG.md
