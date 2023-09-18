test: test-library test-docs

test-library:
  cargo nextest run --all-features

test-docs:
  cargo test --doc

build:
  cargo build --release

coverage:
  cargo llvm-cov nextest --html --open
