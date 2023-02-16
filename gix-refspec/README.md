# `gix-refspec`

### Testing

#### Fuzzing

`cargo fuzz` is used for fuzzing, installable with `cargo install cargo-fuzz`.

Targets can be listed with `cargo fuzz list` and executed via `cargo +nightly fuzz run <target>`,
where `<target>` can be `parse` for example.

