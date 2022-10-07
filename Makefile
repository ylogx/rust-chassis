all: lint test

run:
	cargo run

.PHONY: check-git-clean
check-git-clean:
	@# git status --porcelain --untracked-files=no && echo "ERROR: Modified files in working tree. Stash modified files using: git stash." && exit -1
	@# git diff-index --quiet HEAD
	git diff --quiet src/

install: release
release: check-git-clean
	cargo build --release
	cp -prv ./target/release/template-rust ~/bin/template-rust

runner:
	./target/release/template-rust

init:
	echo "Install rust via rustup e.g. brew install rustup-init && rustup-init"
	rustup component add rustfmt clippy

lint:
	cargo fmt
	cargo clippy

test:
	cargo test
