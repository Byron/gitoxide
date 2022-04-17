[![Rust](https://github.com/Byron/gitoxide/workflows/Rust/badge.svg)](https://github.com/Byron/gitoxide/actions)
[![Crates.io](https://img.shields.io/crates/v/gitoxide.svg)](https://crates.io/crates/gitoxide)

**gix** is a command-line interface (*CLI*) to access git repositories. It's written to optimize the
_user-experience_, and perform as _good or better than the canonical implementation_.

Furthermore it provides **an easy and safe to use API** in the form of various small crates for implementing your own tools in a breeze.
Please see _'Development Status'_ for a listing of all crates and their capabilities.

[![asciicast](https://asciinema.org/a/352942.svg)](https://asciinema.org/a/352942)

## Development Status

### gitoxide _(CLI)_
  * please note that all functionality comes from the `gitoxide-core` library, which mirrors these capabilities
    and itself relies on all `git-*` crates.
  * limit amount of threads used in operations that support it.
  * choose between 'human' and 'json' output formats
  * **the `ein` program** - convenient and for humans
    * [x] **init** - initialize a new non-bare repository with a `main` branch
    * [ ] **clone** - initialize a local copy of a remote repository
    * **tools**  
      * [x] **organize** - find all git repositories and place them in directories according to their remote paths
      * [x] **find** - find all git repositories in a given directory - useful for tools like [skim][skim]
      * [x] **estimate-hours** - estimate the time invested into a repository by evaluating commit dates.
          * Based on the [git-hours] algorithm.
          * See the [discussion][git-hours-discussion] for some performance data.
  * **the `gix` program** _(plumbing)_ - lower level commands for use in automation
    * **pack**
      * [x] [verify](https://asciinema.org/a/352942)
      * [x] [index verify](https://asciinema.org/a/352945) including each object sha1 and statistics
      * [x] [explode](https://asciinema.org/a/352951), useful for transforming packs into loose objects for inspection or restoration
        * [x] verify written objects (by reading them back from disk)
      * [x] [receive](https://asciinema.org/a/359321) - receive a whole pack produced by **pack-send** or _git-upload-pack_, useful for `clone` like operations.
      * [x] **create** - create a pack from given objects or tips of the commit graph.
      * [ ] **send** - create a pack and send it using the pack protocol to stdout, similar to 'git-upload-pack', 
            for consumption by **pack-receive** or _git-receive-pack_
      - **multi-index**
          * [x] **info** - print information about the file
          * [x] **create** - create a multi-index from pack indices
          * [x] **verify** - check the file for consistency
          * [x] **entries** - list all entries of the file
      - **index**
          * [x] [create](https://asciinema.org/a/352941) - create an index file by streaming a pack file as done during clone
             * [x] support for thin packs (as needed for fetch/pull)
    * **commit-graph**
      * [x] **verify** - assure that a commit-graph is consistent
    * **mailmap**
      * [x] **verify** - check entries of a mailmap file for parse errors and display them
    * **repository**
      * **verify** - validate a whole repository, for now only the object database.
      * **commit**
         * [x] **describe** - identify a commit by its closest tag in its past
      * **tree**
         * [x] **entries** - list tree entries for a single tree or recursively
         * [x] **info** - display tree statistics
      * **odb**
         * [x] **info** - display odb statistics
         * [x] **entries** - display all object ids in the object database
      * **mailmap**
          * [x] **entries** - display all entries of the aggregated mailmap git would use for substitution
    * **index**
      * [x] **entries** - show detailed entry information for human or machine consumption (via JSON)
      * [x] **verify** - check the index for consistency
      * [x] **info** - display general information about the index itself, with detailed extension information by default
         * [x] detailed information about the TREE extension
         * [ ] …other extensions details aren't implemented yet
      * [x] **checkout-exclusive** - a predecessor of `git worktree`, providing flexible options to evaluate checkout performance from an index and/or an object database.
    * **remote**
      * [ref-list](https://asciinema.org/a/359320) - list all (or given) references from a remote at the given URL

[skim]: https://github.com/lotabout/skim
[git-hours]: https://github.com/kimmobrunfeldt/git-hours/blob/8aaeee237cb9d9028e7a2592a25ad8468b1f45e4/index.js#L114-L143
[git-hours-discussion]: https://github.com/Byron/gitoxide/discussions/78

### Crates

Follow linked crate name for detailed status. Please note that all crates follow [semver] as well as the [stability guide].

### Production Grade

* **Stability Tier 1**
  - [git-lock](https://github.com/Byron/gitoxide/blob/main/git-lock/README.md)
     
* **Stability Tier 2**
  - [git-tempfile](https://github.com/Byron/gitoxide/blob/main/git-tempfile/README.md)
   
### Stabilization Candidates

Crates that seem feature complete and need to see some more use before they can be released as 1.0.

* [git-mailmap](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-mailmap)
* [git-chunk](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-chunk)

### Initial Development
* **usable**
  * [git-actor](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-actor)
  * [git-hash](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-hash)
  * [git-object](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-object)
  * [git-validate](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-validate)
  * [git-url](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-url)
  * [git-glob](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-glob)
  * [git-packetline](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-packetline)
  * [git-transport](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-transport)
  * [git-protocol](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-protocol)
  * [git-pack](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-pack)
  * [git-odb](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-odb)
  * [git-commitgraph](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-commitgraph)
  * [git-diff](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-diff)
  * [git-traverse](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-traverse)
  * [git-config](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-config)
  * [git-features](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-features)
  * [git-ref](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-ref)
  * [git-repository](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-repository)
  * `gitoxide-core`
* **very early**    
  * [git-index](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-index)
  * [git-worktree](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-worktree)
  * [git-bitmap](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-bitmap)
  * [git-revision](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-revision)
  * [git-attributes](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-attributes)
  * [git-quote](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-quote)
* **idea**
  * [git-note](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-note)
  * [git-date](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-date)
  * [git-lfs](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-lfs)
  * [git-rebase](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-rebase)
  * [git-pathspec](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-pathspec)
  * [git-submodule](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-submodule)
  * [git-tui](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-tui)
  * [git-tix](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-tix)
  * [git-bundle](https://github.com/Byron/gitoxide/blob/main/crate-status.md#git-bundle)
  
### Stress Testing
  * [x] Verify huge packs
  * [x] Explode a pack to disk 
  * [x] Generate and verify large commit graphs
  * [ ] Generate huge pack from a lot of loose objects
  
### Cargo features

Many crates use feature flags to allow tuning the compiled result based on your needs. Have a [look at the guide][cargo-features] for more information.

[cargo-features]: https://github.com/Byron/gitoxide/blob/main/cargo-features.md#git-config

### Stability and MSRV

Our [stability guide] helps to judge how much churn can be expected when depending on crates in this workspace.

[stability guide]: https://github.com/Byron/gitoxide/blob/main/STABILITY.md

## Installation

### Download a Binary Release

Using `cargo quickinstall`, one is able to fetch [binary releases][releases]. You can install it via `cargo install cargo-quickinstall`, assuming 
the [rust toolchain][rustup] is present.

Then install gitoxide with `cargo quickinstall gitoxide`.

See the [releases section][releases] for manual installation and various alternative builds that are _slimmer_ or _smaller_, depending
on your needs, for _Linux_, _MacOS_ and _Windows_.

[releases]: https://github.com/Byron/gitoxide/releases

### From Source via Cargo

`cargo` is the Rust package manager which can easily be obtained through [rustup]. With it, you can build your own binary
effortlessly and for your particular CPU for additional performance gains.

The minimum supported Rust version is [documented in the CI configuration](https://github.com/Byron/gitoxide/blob/main/.github/workflows/msrv.yml#L23),
the latest stable one will work as well.

```
# The default installation, 'max'
cargo install gitoxide

# For smaller binaries and even faster build times that are traded for a less fancy CLI implementation, use `lean`
# or `lean-termion` respectively.
cargo install gitoxide --no-default-features --features lean
```

The following installs the latest unpublished release directly from git:

```sh
cargo install --git https://github.com/Byron/gitoxide  gitoxide
```

[releases]: https://github.com/Byron/gitoxide/releases 
[rustup]: https://rustup.rs

## Usage

Once installed, there are two binaries:

* **ein**
  * high level commands, _porcelain_, for every-day use, optimized for a pleasant user experience
* **gix**
  * low level commands, _plumbing_, for use in more specialized cases

## Project Goals

Project goals can change over time as we learn more, and they can be challenged.

 * **a pure-rust implementation of git**
   * including *transport*, *object database*, *references*, *cli* and *tui*
   * a simple command-line interface is provided for the most common git operations, optimized for
     user experience. A *simple-git* if you so will.
   * be the go-to implementation for anyone who wants to solve problems around git, and become
     *the* alternative to `GitPython` in the process.
   * become the foundation for a free distributed alternative to GitHub, and maybe even GitHub itself
 * **learn from the best to write the best possible idiomatic Rust**
   * *libgit2* is a fantastic resource to see what abstractions work, we will use them
   * use Rust's type system to make misuse impossible
 * **be the best performing implementation**
   * use Rust's type system to optimize for work not done without being hard to use
   * make use of parallelism from the get go
 * **assure on-disk consistency**
   * assure reads never interfere with concurrent writes
   * assure multiple concurrent writes don't cause trouble
 * **take shortcuts, but not in quality**
   * binaries may use `anyhow::Error` exhaustively, knowing these errors are solely user-facing.
   * libraries use light-weight custom errors implemented using `quick-error` or `thiserror`.
   * internationalization is nothing we are concerned with right now.
   * IO errors due to insufficient amount of open file handles don't always lead to operation failure
 * **Cross platform support, including Windows**
   * With the tools and experience available here there is no reason not to support Windows.
   * [Windows is tested on CI](https://github.com/Byron/gitoxide/blob/df66d74aa2a8cb62d8a03383135f08c8e8c579a8/.github/workflows/rust.yml#L34)
     and failures do prevent releases.
     

## Non-Goals

Project non-goals can change over time as we learn more, and they can be challenged.

 * **replicate `git` command functionality perfectly**
   * `git` is `git`, and there is no reason to not use it. Our path is the one of simplicity to make
     getting started with git easy.
 * **be incompatible to git**
   * the on-disk format must remain compatible, and we will never contend with it.
 * **use async IO everywhere**
   * for the most part, git operations are heavily relying on memory mapped IO as well as CPU to decompress data,
     which doesn't lend itself well to async IO out of the box.
   * Use `blocking` as well as `git-features::interrupt` to bring operations into the async world and to control 
     long running operations.
   * When connecting or streaming over TCP connections, especially when receiving on the server, async seems like a must
     though, but behind a feature flag.

## Contributions

If what you have seen so far sparked your interest to contribute, then let us say: We are happy to have you and help you to get started.

We recommend running `make tests check-size` during the development process to assure CI is green before pushing.

A backlog for work ready to be picked up is [available in the Project's Kanban board][project-board], which contains instructions on how 
to pick a task. If it's empty or you have other questions, feel free to [start a discussion][discussions] or reach out to @Byron [privately][keybase].

For additional details, also take a look at the [collaboration guide].

[collaboration guide]: https://github.com/Byron/gitoxide/blob/main/COLLABORATING.md
[project-board]: https://github.com/Byron/gitoxide/projects
[discussions]: https://github.com/Byron/gitoxide/discussions
[keybase]: https://keybase.io/byronbates

### Getting started with Video Tutorials

- [Learning Rust with Gitoxide](https://youtube.com/playlist?list=PLMHbQxe1e9Mk5kOHrm9v20-umkE2ck_gE)
   - In 17 episodes you can learn all you need to meaningfully contirbute to `gitoxide`.
- [Getting into Gitoxide](https://youtube.com/playlist?list=PLMHbQxe1e9MkEmuj9csczEK1O06l0Npy5)
   - Get an introduction to `gitoxide` itself which should be a good foundation for any contribution, but isn't a requirement for contributions either.

#### Other Media

- [Rustacean Station Podcast](https://rustacean-station.org/episode/055-sebastian-thiel/)

## Roadmap

### Features for 1.0

Provide a CLI to for the most basic user journey:

* [x] initialize a repository
* clone a repository
   - [ ] bare
   - [ ] with working tree
* [ ] create a commit
* [ ] add a remote
* [ ] push
  * [x] create (thin) pack

### Ideas for Examples

* [ ] `gix tool open-remote` open the URL of the remote, possibly after applying known transformations to go from `ssh` to `https`.
* [ ] `tix` as example implementation of `tig`, displaying a version of the commit graph, useful for practicing how highly responsive GUIs can be made.
* [ ] Open up SQL for git using [sqlite virtual tables](https://github.com/rusqlite/rusqlite/blob/master/tests/vtab.rs). Check out gitqlite
  as well. What would an MVP look like? Maybe even something that could ship with gitoxide. See [this go implementation as example](https://github.com/filhodanuvem/gitql).
* [ ] A truly awesome history rewriter which makes it easy to understand what happened while avoiding all pitfalls. Think BFG, but more awesome, if that's possible.
* [ ] `git-tui` should learn a lot from [fossil-scm] regarding the presentation of data. Maybe [this](https://github.com/Lutetium-Vanadium/requestty/) can be used for prompts. Probably [magit] has a lot to offer, too.

### Ideas for Spin-Offs

* [ ] A system to integrate tightly with `git-lfs` to allow a multi-tier architecture so that assets can be stored in git and are accessible quickly from an intranet location
  (for example by accessing the storage read-only over the network) while changes are pushed immediately by the server to other edge locations, like _the cloud_ or backups. Sparse checkouts along with explorer/finder integrations
  make it convenient to only work on a small subset of files locally. Clones can contain all configuration somebody would need to work efficiently from their location,
  and authentication for the git history as well as LFS resources make the system secure. One could imagine encryption support for untrusted locations in _the cloud_
  even though more research would have to be done to make it truly secure.
* [ ] A [syncthing] like client/server application. This is to demonstrate how lower-level crates can be combined into custom applications that use
  only part of git's technology to achieve their very own thing. Watch out for big file support, multi-device cross-syncing, the possibility for
  untrusted destinations using full-encryption, case-insensitive and sensitive filesystems, and extended file attributes as well as ignore files.
* An event-based database that uses commit messages to store deltas, while occasionally aggregating the actual state in a tree. Of course it's distributed by nature, allowing
  people to work offline.
    - It's abstracted to completely hide the actual data model behind it, allowing for all kinds of things to be implemented on top.
    - Commits probably need a nanosecond component for the timestamp, which can be added via custom header field.
    - having recording all changes allows for perfect merging, both on the client or on the server, while keeping a natural audit log which makes it useful for mission critical
      databases in business.
    * **Applications**
      - Can markdown be used as database so issue-trackers along with meta-data could just be markdown files which are mostly human-editable? Could user interfaces
        be meta-data aware and just hide the meta-data chunks which are now editable in the GUI itself? Doing this would make conflicts easier to resolve than an `sqlite`
        database.
      - A time tracker - simple data, very likely naturally conflict free, and interesting to see it in terms of teams or companies using it with maybe GitHub as Backing for authentication.
        - How about supporting multiple different trackers, as in different remotes?

[syncthing]: https://github.com/syncthing/syncthing
[fossil-scm]: https://www.fossil-scm.org
[magit]: https://magit.vc


## Shortcomings & Limitations

* **fetches using protocol V1 and stateful connections, i.e. ssh, git, file, may hang**
  * This can be fixed by making response parsing.
  * Note that this does not affect cloning, which works fine.
* **lean** and **light** and **small** builds don't support non-UTF-8 paths _in the CLI_
  * This is because they depend on `argh`, which [does not yet support parsing OsStrings](https://github.com/google/argh/issues/33). We however
    believe it eventually will do so and thus don't move on to [`pico-args`](https://github.com/RazrFalcon/pico-args/blob/master/examples/app.rs).
  * Only one level of sub-commands are supported due to a limitation of `argh`, which forces porcelain to limit itself as well despite using `clap`.
    We deem this acceptable for plumbing commands and think that porcelain will be high-level and smart enough to not ever require deeply nested sub-commands.
* **Packfiles use memory maps**
  * Even though they are comfortable to use and fast, they squelch IO errors.
  * _potential remedy_: We could generalize the Pack to make it possible to work on in-memory buffers directly. That way, one
    would initialize a Pack by reading the whole file into memory, thus not squelching IO errors at the expense of latency as well
    as memory efficiency.
* **Packfiles cannot load files bigger than 2^31 or 2^32 on 32 bit systems**
  * As these systems cannot address more memory than that.
  * _potential remedy_: implement a sliding window to map and unmap portions of the file as needed.
    * However, those who need to access big packs on these systems would rather resort to `git` itself, allowing
      our implementation to be simpler and potentially more performant.
* **Objects larger than 32 bits cannot be loaded on 32 bit systems**
  * in-memory representations objects cannot handle objects greater than the amount of addressable memory.
  * This should not affect git LFS though.
* **git-url** _might_ be more restrictive than what git allows as for the most part, it uses a browser grade URL parser.
  * Thus far there is no proof for this, and as _potential remedy_ we could certainly re-implement exactly what git does
    to handle its URLs.
* **local time** is currently impeded by [this issue](https://github.com/time-rs/time/issues/293#issuecomment-909158529) but it's planned to resolve it eventually.
  
## Credits

* **itertools** _(MIT Licensed)_
  * We use the `izip!` macro in code
* **deflate2** _(MIT Licensed)_
  * We use various abstractions to implement decompression and compression directly on top of the rather low-level `miniz_oxide` crate
  
## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

## Fun facts

* Originally @Byron was really fascinated by [this problem](https://github.com/gitpython-developers/GitPython/issues/765#issuecomment-396072153)
  and believes that with `gitoxide` it will be possible to provide the fastest solution for it.
* @Byron has been absolutely blown away by `git` from the first time he experienced git more than 13 years ago, and 
  tried to implement it in [various shapes](https://github.com/gitpython-developers/GitPython/pull/1028) and [forms](https://github.com/byron/gogit)
  multiple [times](https://github.com/Byron/gitplusplus). Now with Rust @Byron finally feels to have found the right tool for the job!
