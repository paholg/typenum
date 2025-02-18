# Run all CI checks except those that require different platforms
test-local: lint test
    @just test --features const-generics

# Update lockfiles
up:
    nix flake update
    cargo update

# Run all lints
lint: fmt clippy clippy-all

# Check formatting
fmt:
    cargo fmt --all -- --check

# Clippy
clippy:
    cargo clippy -- -D warnings

# Clippy with all features
clippy-all:
    # Allow deprecated because we test the no_std feature.
    cargo clippy --all-features -- -D warnings -A deprecated

# Run test
test *args:
    cargo test --verbose --features "strict" {{args}}
    cargo doc --features "strict" {{args}}
