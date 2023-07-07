#!/usr/bin/env -S just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.

default:
    {{ just_executable() }} --list

alias t := test
alias c := check

# run all tests, clippy, including journey tests, try building docs
test: clippy check doc unit-tests journey-tests-pure journey-tests-small journey-tests-async journey-tests journey-tests-smart-release

# run all tests, without clippy, including journey tests, try building docs
ci-test: check doc unit-tests journey-tests-pure journey-tests-small journey-tests-async journey-tests journey-tests-smart-release

# Run cargo clippy on all crates
clippy *clippy-args:
    cargo clippy --all --tests --examples -- {{ clippy-args }}
    cargo clippy --all --no-default-features --features small -- {{ clippy-args }}
    cargo clippy --all --no-default-features --features max-pure -- {{ clippy-args }}
    cargo clippy --all --no-default-features --features lean-async --tests -- {{ clippy-args }}

# Run cargo clippy on all crates, fixing what can be fixed, and format all code
clippy-fix:
    cargo clippy --fix --all --tests --examples
    cargo clippy --fix --allow-dirty --all --no-default-features --features small
    cargo clippy --fix --allow-dirty --all --no-default-features --features max-pure
    cargo clippy --fix --allow-dirty --all --no-default-features --features lean-async --tests
    cargo fmt --all

# Build all code in suitable configurations
check:
    cargo check --all
    cargo check --no-default-features --features small
    # assure compile error occurs
    if cargo check --features lean-async 2>/dev/null; then false; else true; fi
    if cargo check -p gitoxide-core --all-features 2>/dev/null; then false; else true; fi
    if cargo check -p gix-packetline --all-features 2>/dev/null; then false; else true; fi
    if cargo check -p gix-transport --all-features 2>/dev/null; then false; else true; fi
    if cargo check -p gix-protocol --all-features 2>/dev/null; then false; else true; fi
    cargo check --no-default-features --features lean
    cargo check --no-default-features --features lean-async
    cargo check --no-default-features --features max
    cargo check -p gitoxide-core
    cargo check -p gitoxide-core --features blocking-client
    cargo check -p gitoxide-core --features async-client
    cd gix-hash; \
        set -ex; \
        cargo check --all-features; \
        cargo check
    cd gix-object; \
        set -ex; \
        cargo check --all-features; \
        cargo check --features verbose-object-parsing-errors
    cd gix-attributes && cargo check --features serde
    cd gix-glob && cargo check --features serde
    cd gix-worktree && cargo check --features serde
    cd gix-actor && cargo check --features serde
    cd gix-date && cargo check --features serde
    cargo check -p gix-tempfile --features signals
    cargo check -p gix-tempfile --features hp-hashmap
    cargo check -p gix-tempfile
    cargo check -p gix-pack --features serde
    cargo check -p gix-pack --features pack-cache-lru-static
    cargo check -p gix-pack --features pack-cache-lru-dynamic
    cargo check -p gix-pack --features object-cache-dynamic
    cargo check -p gix-pack
    cargo check -p gix-packetline
    cargo check -p gix-packetline --features blocking-io
    cargo check -p gix-packetline --features async-io
    cargo check -p gix-index --features serde
    cargo check -p gix-credentials --features serde
    cargo check -p gix-sec --features serde
    cargo check -p gix-revision --features serde
    cargo check -p gix-mailmap --features serde
    cargo check -p gix-url --all-features
    cargo check -p gix-url
    cargo check -p gix-features --all-features
    cargo check -p gix-features --features parallel
    cargo check -p gix-features --features fs-walkdir-parallel
    cargo check -p gix-features --features rustsha1
    cargo check -p gix-features --features fast-sha1
    cargo check -p gix-features --features progress
    cargo check -p gix-features --features io-pipe
    cargo check -p gix-features --features crc32
    cargo check -p gix-features --features zlib
    cargo check -p gix-features --features zlib,zlib-ng
    cargo check -p gix-features --features zlib,zlib-ng-compat
    cargo check -p gix-features --features zlib-stock
    cargo check -p gix-features --features zlib,zlib-stock
    cargo check -p gix-features --features cache-efficiency-debug
    cargo check -p gix-commitgraph --all-features
    cargo check -p gix-commitgraph
    cargo check -p gix-config-value --all-features
    cargo check -p gix-config-value
    cargo check -p gix-config --all-features
    cargo check -p gix-config
    cargo check -p gix-transport
    cargo check -p gix-transport --features blocking-client
    cargo check -p gix-transport --features async-client
    cargo check -p gix-transport --features async-client,async-std
    cargo check -p gix-transport --features http-client
    cargo check -p gix-transport --features http-client-curl
    cargo check -p gix-transport --features http-client-reqwest
    cargo check -p gix-protocol
    cargo check -p gix-protocol --features blocking-client
    cargo check -p gix-protocol --features async-client
    cargo check -p gix --no-default-features --features async-network-client
    cargo check -p gix --no-default-features --features async-network-client-async-std
    cargo check -p gix --no-default-features --features blocking-network-client
    cargo check -p gix --no-default-features --features blocking-http-transport-curl
    cargo check -p gix --no-default-features --features blocking-http-transport-reqwest
    cargo check -p gix --no-default-features --features max-performance
    cargo check -p gix --no-default-features --features max-performance-safe
    cargo check -p gix --no-default-features --features progress-tree
    cargo check -p gix --no-default-features
    cargo check -p gix-odb --features serde
    cargo check -p cargo-smart-release --all
    cargo check -p cargo-smart-release --features vendored-openssl
    cargo check --no-default-features --features max-control

# Run cargo doc on all crates
doc $RUSTDOCFLAGS="-D warnings":
    cargo doc --all --no-deps
    cargo doc --features=max,lean,small --all --no-deps

# run all unit tests
unit-tests:
    cargo test --all
    cd gix-object; \
      set -ex; \
      cargo test; \
      cargo test --features verbose-object-parsing-errors
    cd gix-worktree; \
      set -ex; \
      cargo test; \
      cargo test --features "internal-testing-gix-features-parallel"
    cargo test -p gix-tempfile --features signals
    cargo test -p gix-tempfile
    cargo test -p gix-features
    cargo test -p gix-features --all-features
    cargo test -p gix-ref-tests --all-features
    cargo test -p gix-odb
    cargo test -p gix-odb --all-features
    cargo test -p gix-pack --all-features
    cargo test -p gix-pack-tests
    cargo test -p gix-pack-tests --features "internal-testing-gix-features-parallel"
    cargo test -p gix-index-tests
    cargo test -p gix-index-tests --features "internal-testing-gix-features-parallel"
    cargo test -p gix-packetline
    cargo test -p gix-packetline --features blocking-io,maybe-async/is_sync --test blocking-packetline
    cargo test -p gix-packetline --features "async-io" --test async-packetline
    cargo test -p gix-transport
    cargo test -p gix-transport --features http-client-curl,maybe-async/is_sync
    cargo test -p gix-transport --features http-client-reqwest,maybe-async/is_sync
    cargo test -p gix-transport --features async-client
    cargo test -p gix-protocol --features blocking-client
    cargo test -p gix-protocol --features async-client
    cargo test -p gix-protocol
    cargo test -p gix
    cargo test -p gix --features async-network-client
    cargo test -p gix --features blocking-network-client
    cargo test -p gix --features regex
    cargo test -p gitoxide-core --lib

# These tests aren't run by default as they are flaky (even locally)
unit-tests-flaky:
    cargo test -p gix --features async-network-client-async-std

jtt := "target/debug/jtt"

# run journey tests (max)
journey-tests:
    cargo build
    cargo build -p gix-testtools --bin jtt
    ./tests/journey.sh target/debug/ein target/debug/gix {{ jtt }} max

# run journey tests (max-pure)
journey-tests-pure:
    cargo build --no-default-features --features max-pure
    cargo build -p gix-testtools --bin jtt
    ./tests/journey.sh target/debug/ein target/debug/gix {{ jtt }} max-pure

# run journey tests (small)
journey-tests-small:
    cargo build --no-default-features --features small
    cargo build -p gix-testtools
    ./tests/journey.sh target/debug/ein target/debug/gix {{ jtt }} small

# run journey tests (lean-async)
journey-tests-async:
    cargo build --no-default-features --features lean-async
    cargo build -p gix-testtools
    ./tests/journey.sh target/debug/ein target/debug/gix {{ jtt }} async

# run journey tests (cargo-smart-release)
journey-tests-smart-release:
    cargo build -p cargo-smart-release --bin cargo-smart-release
    cd cargo-smart-release && ./tests/journey.sh ../target/debug/cargo-smart-release

# Run cargo-diet on all crates to see that they are still in bound
check-size:
    ./etc/check-package-size.sh

# Check the minimal support rust version for currently installed Rust version
ci-check-msrv:
    rustc --version
    cargo check -p gix
    cargo check -p gix --no-default-features --features async-network-client,max-performance

# Enter a nix-shell able to build on macos
nix-shell-macos:
    nix-shell -p pkg-config openssl libiconv darwin.apple_sdk.frameworks.Security darwin.apple_sdk.frameworks.SystemConfiguration

# run various auditing tools to assure we are legal and safe
audit:
    cargo deny check advisories bans licenses sources

# run tests with `cargo nextest` (all unit-tests, no doc-tests, faster)
nextest:
    cargo nextest run --all

# run nightly rustfmt for its extra features, but check that it won't upset stable rustfmt
fmt:
    cargo +nightly fmt --all -- --config-path rustfmt-nightly.toml
    cargo +stable fmt --all -- --check
    just --fmt --unstable
