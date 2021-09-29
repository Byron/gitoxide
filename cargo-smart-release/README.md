**cargo smart-release**

Fearlessly release workspace crates without dealing with dependencies or versions.

[![asciicast](https://asciinema.org/a/cJyZGUCCUIe75pQ9QZO2aZSOl.svg)](https://asciinema.org/a/cJyZGUCCUIe75pQ9QZO2aZSOl)

### Installation

#### Cargo
Via `cargo`, which can be obtained using [rustup][rustup]

```
cargo install cargo-smart-release
```

### Preferred workflow

When developing various crates in a workspace I tend to edit code, and depending on the kind of edit, either increase the patch level, minor version or major version
right after, depending on the basic rules of `semver`. Use `cargo smart-release --bump minor --skip-tag --skip-publish --execute` for this which has the benefit of 
it automatically adjusting other workspace dependencies when needed.

When ready for relasing a particular crate or set of crates of interest, run `cargo smart-release [<crate-name> ...]` to simulate a release. For particularly thorough
but error-prone simulations (as in false positives) one could run `cargo smart-release --dry-run-cargo-publish`. After evaluating the release procedure and following
instructions, one should end up with a fully automatic release of multiple crates. There are various other options that shouldn't be needed in the common case, use
`cargo smart-release --help` to see them.

### Features

* [x] safe to use as actually performing an operation requires the `--execute` flag
* [x] avoid inconsistent states by making operations as atomic as possible, leveraging `gitoxide` technology to the fullest
* [x] handle workspace dependencies and cycles gracefully, allowing one invocation to publish multiple crates
* [x] avoid making any releases if there are no changes
* [x] avoid bumping versions if the current version isn't released, allowing you to control the version by editing the cargo manifest
* [x] automatically release dependent workspace pre-release crates along with the desired one if they changed since their last release
* [x] automatically adjust manifest versions and update manifests of crates which use those whose versions were incremented
* [x] use git tags to know if a crate changed at all, skipping publishes if there is no code change at all
* [ ] Handle pre-release versions and meta-data as per the [stability guide].
* [ ] Support other remote names than 'origin' - currently the latter name is assumed. Fix by getting the remote of the currently checked out branch.
* [ ] handle version specifications correctly [(tables vs values)](https://github.com/Byron/cargo-release/blob/master/src/cargo.rs#L179:L207)
* [ ] handle all version comparators correctly (see [here](https://github.com/Byron/cargo-release/blob/master/src/version.rs#L192:L226) for how it's done)
* [ ] Automatically detect if crate changes are breaking to suggest the correct version increment

### Comparison to `cargo release`

`cargo-release` is the reason this tool exists, as it got me addicted to an all automatic release workflow that knows git. This works perfectly
for simple workspaces or single-crate workspaces, as of 2021-08-12, so use it: `cargo install cargo-release`.

Here is what `cargo smart-release` does differently: "It tries really hard to do what _I_ want most of the time when publishing workspace `gitoxide` crates".

- be safe to execute, so it's disarmed by default
- specify one ore more crates, and detect which crates need publishing automatically
- handle dependency cycles in such a way that increases the chances of overall success
- try really hard to not leave the workspace in an inconsistent state when something goes wrong
- be a playground for `gitoxide` to have a reason to make it much more convenient and more feasible for application authors (aka dog-fooding)

### Limitations

* it requires tables to be used when specifying versions, i.e. `crate = { version = "1" }` instead of `crate  = "1".
* it gracefully fails when encountering version requirement comparators which are not `^`, like `=`
* it's tested only by using it on `gitoxide`, there are only very few regression tests with little coverage.
* short object ids in changelogs may be ambiguous, as they are unconditionally truncated to 7 characters.
* it's very young and probably tries to eat underwear
* it needs a git repository to govern the workspace

### Acknowledgements

Thanks to [cargo-release] for showing the way and for incredible fast response times. I'd recommend everyone to participate there instead of writing your own.

[cargo-release]: https://github.com/sunng87/cargo-release/issues/224
[rustup]: https://rustup.rs/
[stability guide]: https://github.com/Byron/gitoxide/blob/stability/STABILITY.md
