build:
	cargo build

runFile:
	cargo run -- test

run:
	cargo run

test:
	cargo test

coverage:
	cargo llvm-cov

coverage-html:
	cargo llvm-cov --html
	open target/llvm-cov/html/index.html
