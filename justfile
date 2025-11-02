# Generate code and run lints and tests
test-local: gen lint test
    @just test --features const-generics

# Produce generated code
gen:
    cargo run --package generate
    cargo fmt

# Update lockfiles
up:
    nix flake update
    cargo update

# Run all lints
lint: fmt-check clippy clippy-all

# Check formatting
fmt-check:
    cargo fmt --all -- --check

# Clippy
clippy:
    cargo clippy -- -D warnings

# Clippy with all features
clippy-all:
    cargo clippy --all-features -- -D warnings

# Run test
test *args:
    cargo test --verbose --features "strict" {{args}}
    cargo doc --features "strict" {{args}}
