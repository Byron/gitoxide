# `gix-pathspec`

### Testing

#### Fuzzing

`cargo fuzz` is used for fuzzing, installable with `cargo install cargo-fuzz`.

Targets can be listed with `cargo fuzz list` and executed via `cargo +nightly fuzz run <target>`, 
where `<target>` can be `parse` for example.

### Notes

- There is one additional keyword that `git` can parse, but that this crate doesn't support yet: the `prefix` keyword

  [Here is a commit](https://github.com/git/git/commit/5be4efbefafcd5b81fe3d97e8395da1887b4902a) in which `prefix` is somewhat explained.
