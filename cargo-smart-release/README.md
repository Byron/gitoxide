# `cargo smart-release`

Fearlessly release workspace crates and with beautiful semi-handcrafted changelogs.

[![asciicast](https://asciinema.org/a/65uPfzoWxGac43zEZ1gQ1yVVe.svg)](https://asciinema.org/a/65uPfzoWxGac43zEZ1gQ1yVVe)

## Key Features

* **zero-configuration**
  * `cargo smart-release` needs no extra flags to _do the right thing™️_ smartly. If your intervention is needed it will let you know before it makes changes.
  * It won't do anything if there are no changes.
* **made for multi-crate workspaces**
  * "Nothing stands by itself, and everything is interconnected" - is how it sees the world, allowing it to efficiently handling complex workspace graphs.
  * _works just as well for single-crate workspaces_
* **changelogs-deluxe**
  * It maintains beautiful changelogs for you while allowing you to edit them for the final polish.
  * See your release notes via in-repository _tag objects_ and in _GitHub Releases_
  * **plays well with `cargo release`**
    * `cargo changelog` writes changelogs non-destructively, and only that, leaving the release workflow to [cargo-release].
  
If seeing is believing, here is [a 12 minute demonstration](https://www.youtube.com/watch?v=EOft_uMDVYE), and the same in 30 minutes is [also available](https://youtu.be/a4CzzxJ7ecE).

## Made for this Workflow

When developing various crates in a workspace, when committing changes and if the edit is breaking, a feature, or another
change I want to see in changelogs, [conventional] git messages will be used. This helps building changelog scaffolding automatically later.

When ready for releasing a particular crate or set of crates of interest, run `cargo smart-release [<crate-name> ...]` to simulate a release. For particularly thorough
but error-prone simulations (as in false positives) one could run `cargo smart-release --dry-run-cargo-publish`. To polish changelogs, run `cargo changelog --write --only <crate-name>`
to update the scaffolding and edit it by hand until it fits.

After evaluating the release procedure and following instructions,
`cargo smart-release --execute` will cause the fully automatic release of one or more crates.

There are various other options that shouldn't be needed in the common case, use `cargo smart-release --help` to see them.

[conventional]: https://www.conventionalcommits.org

## Installation

### Cargo
Via `cargo`, which can be obtained using [rustup][rustup]

```
cargo install cargo-smart-release
```

## Features

* [x] safe to use as actually performing an operation requires the `--execute` flag
* [x] avoid inconsistent states by making operations as atomic as possible, leveraging `gitoxide` technology to the fullest
* [x] handle workspace dependencies and cycles gracefully, allowing one invocation to publish multiple crates
* [x] avoid making any releases if there are no changes
* [x] avoid bumping versions if the current version isn't released, allowing you to control the version by editing the cargo manifest
* [x] [conventional] commit message drive changelog scaffolding and to automatically derive the crate version to publish
* [x] automatically release dependent workspace pre-release crates along with the desired one if they changed since their last release
* [x] automatically adjust manifest versions and update manifests of crates which use those whose versions were incremented
* [x] conservatively bump downstream workspace crates in the light of breaking changes, even though these won't be published, making downstream breakage impossible
* [x] use git tags to know if a crate changed at all, skipping publishes if there is no code change at all
* [ ] Handle pre-release versions and meta-data as per the [stability guide].
* [ ] Support other remote names than 'origin' - currently the latter name is assumed. Fix by getting the remote of the currently checked out branch.
* [ ] handle version specifications correctly [(tables vs values)](https://github.com/Byron/cargo-release/blob/master/src/cargo.rs#L179:L207)
* [ ] handle all version comparators correctly (see [here](https://github.com/Byron/cargo-release/blob/master/src/version.rs#L192:L226) for how it's done)
* [ ] Automatically detect if crate changes are breaking to suggest the correct version increment

## Comparison to `cargo release`

`cargo-release` is the reason this tool exists, as it got me addicted to an all automatic release workflow that knows git. This works perfectly
for simple workspaces or single-crate workspaces, as of 2021-08-12, so use it: `cargo install cargo-release`.

Here is what `cargo smart-release` does differently: "It tries really hard to do what _I_ want most of the time when publishing workspace `gitoxide` crates".

- ~~be safe to execute, so it's disarmed by default~~
- specify one ore more crates, and detect which crates need publishing automatically
- handle dependency cycles in such a way that increases the chances of overall success
- try really hard to not leave the workspace in an inconsistent state when something goes wrong
- be a playground for `gitoxide` to have a reason to make it much more convenient and more feasible for application authors (aka dog-fooding)
- create changelogs non-destructively, along with annotated tags and GitHub releases

## Limitations

* it requires tables to be used when specifying versions, i.e. `crate = { version = "1" }` instead of `crate  = "1".
* it gracefully fails when encountering version requirement comparators which are not `^`, like `=`
* it's tested only by using it on `gitoxide`, there are only very few regression tests with little coverage.
* short object ids in changelogs may be ambiguous, as they are unconditionally truncated to 7 characters.
* changelog rewriting of user content will drop links if they are not of the 'inline' form
* it's very young and probably tries to eat underwear
* it needs a git repository to govern the workspace

## Acknowledgements

Thanks to [cargo-release] for showing the way and for incredible fast response times. I'd recommend everyone to participate there instead of writing your own.

Special thanks go to [git-cliff] which gave me the nudge needed to want to write my own.

[cargo-release]: https://github.com/sunng87/cargo-release/issues/224
[git-cliff]: https://github.com/orhun/git-cliff
[rustup]: https://rustup.rs/
[stability guide]: https://github.com/Byron/gitoxide/blob/stability/STABILITY.md
