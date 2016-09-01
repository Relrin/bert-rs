build:
	cd bert && cargo build

rebuild_all:
	cd bert && cargo clean && cargo build

test:
	cd bert_tests && rustup run nightly cargo test --no-default-features --features serde_macros