**cargo smart-release**

Fearlessly release workspace crates without dealing with dependencies or versions.

### Installation

#### Cargo
Via `cargo`, which can be obtained using [rustup][rustup]

```
cargo install cargo-smart-release
```

### Usage

```bash
# simulate the release process but don't persist operations
cargo smart-release workspace-package-1 another-workspace-package

# perform the actual release, resulting in all changed dependent workspace crates to be published.
cargo smart-release workspace-package-1 another-workspace-package --execute

# learn more…
cargo smart-release --help
```

### Features

* [x] safe to use as actually performing an operation requires the `--execute` flag
* [x] avoid inconsistent states by making operations as atomic as possible, leveraging `gitoxide` technology to the fullest
* [x] handle workspace dependencies and cycles gracefully, allowing one invocation to publish multiple crates
* [x] automatically release dependent workspace pre-release crates along with the desired one if they changed since their last release
* [x] automatically adjust manifest versions and update manifests of crates which use those whose versions were incremented
* [x] use git tags to know if a crate changed at all, skipping publishes if there is no code change at all
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

* it's very young and probably tries to eat underwear
* it needs a git repository to govern the workspace
* it requires tables to be used when specifying versions, i.e. `crate = { version = "1" }` instead of `crate  = "1".
* it gracefully fails when encountering version requirement comparators which are not `^`, like `=`
* it's tested only by using it on `gitoxide`, there are no regression tests at all
* it was hacked together in two days, and there definitely are rough edges and assumptions

### Acknowledgements

Thanks to [cargo release] for showing the way, but also motivating me to finally take full control of releases.

[cargo-release]: https://github.com/sunng87/cargo-release/issues/224
[rustup]: https://rustup.rs/
