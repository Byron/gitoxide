# git-config

**git-config is a library for interacting with `git-config` files.**

This crate intents to be a performant Rust implementation for reading and
writing `git-config` files. It exposes tiers of abstractions, from simple
config value wrappers to a high level reader and writer.

The highlight of this crate is the zero-copy parser. We employ techniques to
avoid copying where necessary, and reads that do not need normalization are
guaranteed to be zero-copy. Higher level abstractions maintain this guarantee,
and utilizes acceleration structures for increased performance.

Currently, this is _not_ a binary. While we do intent to have a drop-in
replacement for the `git config` sub-command, we're currently missing
system-level abstractions to do so.

## Examples

Reading and writing to a config:

```rust
use git_config::file::GitConfig;
use git_config::values::Boolean;
use std::fs::read_to_string;

let input = r#"
[core]
  some-bool = true

[other "internal"]
  hello = world
"#;
let mut config = GitConfig::from(input)?;
let boolean = config.get_config::<Boolean>("core", None, "some-bool");
config.set_value("other", Some("internal"), "hello", "clippy!");
```

## Contributing

Contributions are always welcome!

### Code quality

This repository enables pedantic, cargo, and nursery `clippy` lints. Make sure
to run `cargo clean && cargo clippy` (the clean stage is very important!) to
ensure your code is linted.

### Testing

Since this is a performance oriented crate, in addition to well tested code via
`cargo test`, we also perform benchmarks to measure notable gains or losses in
performance. We use [`criterion`] so benches can be run via `cargo bench` after
installing it via `cargo install cargo-criterion`.

Changes to `parser.rs` may include a request to fuzz to ensure that it cannot
panic on inputs. This can be done by executing `cargo fuzz parser` after
installing the `fuzz` sub-command via `cargo install cargo-fuzz`.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in git-config by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

[`criterion`]: https://github.com/bheisler/criterion.rs