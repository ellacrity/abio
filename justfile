# Setup
set dotenv-load := true
set windows-shell := ["pwsh", "-NoProfile", "-NoLogo", "-Command"]
set shell := ["bash", "-cu"]

release_mode := "--release"

# List all available recipes
_default:
    @just --list

# Run benchmark suite
@bench:
    cargo +nightly bench

# Build project in debug or release mode
@build *MODE:
    cargo build --workspace --all-targets {{ if MODE != "" { release_mode } else { "" } }}

# Build project in debug or release mode and update Cargo.lock
@build-locked *MODE:
    cargo build --workspace --all-targets --locked {{ if MODE != "" { release_mode } else { "" } }}
    
# Build project (debug and release mode)
@build-all:
    @just build
    @just build release

# Compile, test and lint project, failing on error
@check:
    cargo +nightly fmt --all -- --check && \
    cargo test --workspace --quiet && \
    cargo +nightly clippy --workspace --all-targets -- -D warnings

# Remove build artifacts
@clean:
    cargo clean --verbose

# Compile project
@compile:
    cargo check --workspace --all-targets

# Build documentation, opening in a browser
@docs:
    cargo +nightly doc --workspace --all-features --no-deps --open

# Build documentation, opening in a browser
@docs-all:
    cargo +nightly doc --workspace --all-features --no-deps --document-private-items --open

# Run tests against Miri
@miri:
    cargo miri test

# Run tests against Miri (verbose mode)
@miri-verbose:
    cargo miri test -- --nocapture

# Lint project with clippy
@lint:
    cargo +nightly clippy --workspace --all-targets

# Rebuild project from scratch
@rebuild:
    @just clean
    @just build
    @just build release

# Run project binary
@run name:
    cargo run --bin {{name}}

# Run test suite
@test:
    cargo test --workspace --all-targets --quiet

# Run test suite (release mode)
@test-release:
    cargo test --workspace --quiet --all-targets --release

# Run test suite (debug mode with multi-threaded environment)
@test-verbose:
    cargo test --workspace --all-targets -- --nocapture

# Run test suite (debug mode with single-threaded environment)
@test-debug:
    cargo test --workspace --all-targets -- --test-threads=1 --nocapture

# Get compilation timings [fmt: html, json]
@timings fmt:
    @just clean
    cargo build --workspace --all-targets --release -Z timings={{fmt}}

# Show the versions of required build tools
@versions:
    rustc --version
    cargo --version

# Run recipe in watch mode, restarting on changes
@watch recipe:
    cargo watch -s "just {{ recipe }}"
