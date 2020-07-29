[![Rust](https://github.com/Byron/git-oxide/workflows/Rust/badge.svg)](https://github.com/Byron/git-oxide/actions)
[![Crates.io](https://img.shields.io/crates/v/gitoxide.svg)](https://crates.io/crates/gitoxide)

**gix** is a command-line interface (*CLI*) to access git repositories. It's written to optimize the
_user-experience_, and perform as _good or better than the native implementation_.

Furthermore it provides **an easy and safe to use API** in the form of various small crates for implementing your own tools in a breeze.
Please see _'Development Status'_ for a listing of all crates and their capabilities.

[![asciicast](https://asciinema.org/a/346976.svg)](https://asciinema.org/a/346976)

## Development Status

* **gitoxide** _(CLI)_
  * please note that all functionality comes from the `gitoxide-core` library, which mirrors these capabilities
    and itself relies on all `git-*` crates.
  * limit amount of threads used in operations that support it.
  * repository
    * [x] init
  * plumbing
    * [x] [pack verify](https://asciinema.org/a/346975)
    * [x] [pack index verify](https://asciinema.org/a/346976) including each object sha1 and statistics
    * [x] pack explode, useful for transforming packs into loose objects for inspection or restoration
      * [x] verify written objects (by reading them back from disk)
* **git-object**
  * *decode (zero-copy)* borrowed objects
    * [x] commit
    * [x] tree
    * [x] tag
  * encode owned objects
    * [x] commit
    * [x] tree
    * [x] tag
  * [x] transform borrowed to owned objects
  * [ ] API documentation with examples
* **git-odb**
  * **loose objects**
    * [x] traverse
    * [x] read
      * [x] into memory
      * [x] streaming
      * [x] verify checksum
    * [x] streaming write for blobs
    * [x] buffer write for small in-memory objects/non-blobs
  * **packs**
    * [x] traverse pack index
    * [x] object abstraction
      * [x] decode (zero copy)
      * [x] verify checksum
    * [x] easy and fast pack traversal
    * [x] decode
      * [x] full objects
      * [x] deltified objects
    * **advanced**
      * [ ] Multi-Pack index file (MIDX)
      * [ ] 'bitmap' file
    * [ ] encode
      * [ ] create new pack
      * [ ] create 'thin' pack
    * [x] verify pack with statistics
      * [x] brute force - less memory
      * [x] indexed - more memory
    * [x] pack streaming with `Iterator` 
       * [ ] index from pack
  * [ ] API documentation with examples
  * **sink**
    * [x] write objects and obtain id
  * **alternates**
    * [ ] _database that act as link to other known ODB types on disk_
    * [ ] handles cycles
    * [ ] handles recursive configurations
  * **multi-odb**
    * [ ] _an ODB for object lookup from multiple lower level ODB at once_
  * **promisor**
    * It's vague, but these seems to be like index files allowing to fetch objects from a server on demand.
* **git-repository**
  * [x] initialize
    * [ ] Proper configuration depending on platform (e.g. ignorecase, filemode, …)
  * [ ] read and write all data types
  * [ ] rev-parsing and ref history
  * [ ] remotes with push and pull
  * [ ] configuration
  * [ ] merging
  * [ ] stashing
  * [ ] API documentation with examples
  * [ ] _Commit Graph_ - split and unsplit
* **git-config**
  * read and write git configuration files
  * [ ] API documentation with examples
* **git-ref**
  * Handle symbolic references and packed references
  * discover them in typical folder structures
  * [x] name validation
  * [ ] API documentation with examples
* **git-index**
  * read and write a git-index file
  * add and remove entries
  * [ ] API documentation with examples
* **git-diff**
  * diffing of git-object::Tree structures
  * diffing, merging, working with hunks of data
  * find differences between various states, i.e. index, working tree, commit-tree
  * [ ] API documentation with examples
* **git-transport**
  * [ ] via ssh
    * [ ] push
    * [ ] pull
  * [ ] via https
    * [ ] push
    * [ ] pull
  * [ ] API documentation with examples
* **git-features**
  * **parallel** feature toggle
    * _When on…_
      * `in_parallel`
      * `join`
    * _When off all functions execute serially_
* **git-tui**
  * _a terminal user interface seeking to replace and improve on `tig`_
* **Stress Testing**
  * [x] Verify huge packs
  * [x] Explode a pack to disk 
  * [ ] Generate huge back from a lot of loose objects
* **Ideas for Demos**
  * [ ] A simple [`git-hours`][git-hours-algo] clone
  * [ ] Open up SQL for git using [sqlite virtual tables](https://github.com/rusqlite/rusqlite/blob/master/tests/vtab.rs). Check out gitqlite
        as well. What would an MVP look like? Maybe even something that could ship with gitoxide.

[git-hours-algo]: https://github.com/kimmobrunfeldt/git-hours/blob/8aaeee237cb9d9028e7a2592a25ad8468b1f45e4/index.js#L114-L143

## Installation

### Binary Release

```sh
curl -LSfs https://raw.githubusercontent.com/byron/git-oxide/master/ci/install.sh | \
    sh -s -- --git byron/git-oxide --crate gix-max-termion
```

See the [releases section][releases] for manual installation and various alternative builds that are _slimmer_ or _smaller_, depending
on your needs, for _Linux_, _MacOS_ and _Windows_.

[releases]: https://github.com/Byron/git-oxide/releases

#### Cargo

`cargo` is the Rust package manager which can easily be obtained through [rustup][rustup]. With it, you can build your own binary
effortlessly and for your particular CPU for additional performance gains.

```
# The default installation, 'max'
cargo install gitoxide

# On linux, it's a little faster to compile the termion version, which also results in slightly smaller binaries
cargo install gitoxide --no-default-features --features max-termion

# For smaller binaries and even faster build times that are traded for a less fancy CLI implementation, use `lean`
# or `lean-termion` respectively.
cargo install gitoxide --no-default-features --features lean
```

[releases]: https://github.com/Byron/git-oxide/releases 
[rustup]: https://rustup.rs

## Usage

Once installed, there are two binaries:

* **gix**
  * high level commands, _porcelain_, for every-day use, optimized for a pleasant user experience
* **gixp**
  * low level commands, _plumbing_, for use in more specialized cases

## Project Goals

 * **a pure-rust implementation of git**
   * including *transport*, *object database*, *references* and *cli*
   * a simple command-line interface is provided for the most common git operations, optimized for
     user experience. A *simple-git* if you so will.
   * be the go-to implementation for anyone who wants to solve problems around git, and become
     *the* alternative to `GitPython` in the process.
   * become the foundation for a free distributed alternative to github.
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
   * libraries use light-weight custom errors implemented using `quick-error`.
   * internationalization is nothing we are concerned with right now.
   * IO errors due to insufficient amount of open file handles don't always lead to operation failure
 * **async as opt-in**
   * Making certain capabilities available through `async` APIs allows for abortable operations, which
     may be interesting for interactive user interfaces. Thus it is something worth considering, but only
     behind a feature flag and once the need transpires.
   * Ideally many operations powered by implementors of `std::io::{Read, Write}` and `std::iter::Iterator`,
     which makes unblocking them trivial using the fantastic `blocking` crate. Only when these are used internally,
     providing a separate async version of these operations can be beneficial to make them abortable.

## Non-Goals

 * **replicate `git` command functionality perfectly**
   * `git` is `git`, and there is no reason to not use it. Our path is the one of simplicity to make
     getting started with git easy.
 * **be incompatible to git**
   * the on-disk format must remain compatible, and we will never contend with it.
 * **use async IO everywhere**
   * for the most part, git operations are heavily relying on memory mapped IO as well as CPU to decompress data,
     which doesn't lend itself well to async IO out of the box.
   * When connecting or streaming over TCP connections, especially when receiving on the server, async seems like a must
     though. It should be possible to put it behind a feature flag though.

## Roadmap to Future

### Roadmap to 1.0

Provide a CLI to for the most basic user journey:

* [x] initialize a repository
* [ ] clone a repository
   * [ ] http(s) (or ssh, whatever is easier)
* [ ] create a commit
* [ ] add a remote
* [ ] push
  * [ ] create (thin) pack
  
### Roadmap to 1.1

 * [ ] clone a repository via
   * [ ] ssh

## Cargo features guide

Cargo uses feature toggles to control which dependencies are pulled in, allowing users to specialize crates to fit their usage.
Ideally, these should be additive.
This guide documents which features are available for each of the crates provided here and how they function.

### gitoxide

The top-level command-line interface.

* **fast**
  * Makes the crate execute as fast as possible by supporting parallel computation of otherwise long-running functions
    as well as fast, hardware accelerated hashing.
  * If disabled, the binary will be visibly smaller.
* _(mutually exclusive)_
  * **pretty-cli**
    * Use `clap` + `structopt` to build the prettiest, best documented and most user-friendly CLI at the expense of file size.
    * provides a terminal user interface for detailed and exhaustive progress.
    * provides a line renderer for log-like progress
  * **lean-cli**
    * Use `argh` to produce a usable binary with decent documentation that is smallest in size, usually 300kb less than `pretty-cli`.
    * If `pretty-cli` is enabled as well, `lean-cli` will take precedence, and you pay for building unnecessary dependencies.
    * provides a line renderer for log-like progress
* **prodash-line-renderer-crossterm** or **prodash-line-renderer-termion** _(mutually exclusive)_
  * The `--verbose` flag will be powered by an interactive progress mechanism that doubles as log as well as interactive progress
    that appears after a short duration.
  
There are **convenience features**, which combine common choices of the above into one name

* **max** = *pretty-cli* + *fast* + *prodash-tui-renderer-crossterm*
  * _default_, for unix and windows
* **max-termion** = *pretty-cli* + *fast* + *prodash-tui-renderer-termion*
  * for unix only, faster compile times, a little smaller
* **lean** = *lean-cli* + *fast* + *prodash-line-renderer-crossterm*
  * for unix and windows, significantly smaller than _max_, but without `--progress` terminal user interface.
* **lean-termion** = *lean-cli* + *fast* + *prodash-line-renderer-termion*
  * for unix only, faster compile times, a little smaller
* **light** = *lean-cli* + *fast*
  * crossplatform by nature as this comes with simplified log based progress
* **small** = *lean-cli*
  * As small as it can possibly be, no threading, no fast sha1, log based progress only
    
### git-features

A crate to help controlling which capabilities are available from the top-level crate that uses `gitoxide-core` or any other
`gitoxide` crate that uses `git-features`.
All feature toggles are additive.

* **parallel**
  * Use scoped threads and channels to parallelize common workloads on multiple objects. If enabled, it is used everywhere
    where it makes sense.
  * As caches are likely to be used and instantiated per thread, more memory will be used on top of the costs for threads.
* **fast-sha1** 
  * a multi-crate implementation that can use hardware acceleration, thus bearing the potential for up to 2Gb/s throughput on 
    CPUs that support it, like AMD Ryzen or Intel Core i3.
* **progress-log**
  * Implement the `Progress` trait using the `log` crate. Throttle progress output to one every 0.5 seconds unless messages
    are sent manually.
* **progress-prodash**
  * Implement the `Progress` trait for the tree data structures provided by `prodash`, which enables using a terminal user
    interface for progress.
    
 ### Serialization Support
 
 What follows is feature toggles to control serialization of all public facing simple data types.
 
 * **serde1**
   * Data structures implement `serde::Serialize` and `serde::Deserialize`
   
 The feature above is provided by the crates:
 
 * **git-object**
 * **git-odb**
 * **gitoxide-core**

 
## Development Guide

### Practices 

 * **test-first development**
   * protect against regression and make implementing features easy
   * user docker to test more elaborate user interactions
   * keep it practical, knowing the Rust compiler already has your back
     for the mundane things, like unhappy code paths.
   * *use git itself* as reference implementation, and use their test-cases and fixtures where
     appropriate
   * *use libgit2* test fixtures and cases where appropriate
 * **safety first**
   * handle all errors, never unwrap.
   * provide an error chain and make it easy to understand what went wrong.
 * **strive for an MVP and version 1.0 fast...**
   * ...even if that includes only the most common usecases.
 * **Prefer to increment major version rapidly...**
   * ...instead of keeping major version zero for longer than needed.
  
### Guidelines

* **prepare for SHA256 support by using `owned::Id` and `borrowed::Id`**
  * eventually there will be the need to support both Sha1 and Sha256. We anticipate it by using the `Id` type instead 
    of slices or arrays of 20 bytes. This way, eventually we can support multiple hash digest sizes.
  * Right now it's unclear how Sha256 is going to work in git, so we only support Sha1 for now. It might be an avenue to proactively
    implement it ahead of time once there is a specification to follow.
  * It looks like Git prepares to support it by using compile time, we can support it at runtime though with minimal cost. If needed,
    we can later remove support using a cargo feature toggle.
    
### Sha256

A bunch of notes collected to keep track of what's needed to eventually support it

* read `hash-function-transition.txt`
* [x] support `gpgsig-sha256` field - we won't break, but also don't do anything with it (e.g. `extra_headers`)
* [ ] support index V3
* [ ] Pack file PSRC field
   
## Plumbing vs Porcelain

Both terms are coming from the `git` implementation itself, even though it won't necessarily point out which commands are plumbing and which
are porcelain.
The term *plumbing* refers to lower-level, more rarely used commands that complement porcelain by being invoked by it or for certain use
cases.
The term *porcelain* refers to those with a decent user experience, they are primarily intended for use by humans.

In any case, both types of programs must self-document their capabilities using through the `--help` flag.

From there, we can derive a few rules to try adhere to:

### Plumbing

* does not show any progress or logging output by default
* if supported and logging is enabled, it will show timestamps in UTC

### Porcelain

* Provides output to stderr by default to provide progress information. There is no need to allow disabling it, but it shouldn't show up unless
  the operation takes some time.
* If timestamps are shown, they are in localtime.
* Non-progress information goes to stdout.

## Maintenance Guide

Utilities to aid in keeping the project fresh and in sync can be found in the `Maintenance` section of the `makefile`. Run `make` to
get an overview.

### Creating a release

Run `etc/release.sh` to release all crates in leaf-first order using `cargo release`.

### Which git-version to chase?

Generally, we take the git version installed on ubuntu-latest as the one we stay compatible with (_while maintaining backwards
compatibility_). Certain tests only run on CI, designed to validate certain assumptions still hold against possibly changed
git program versions.

This also means that CI may fail despite everything being alright locally, and the fix depends on the problem at hand.

### How to update fixtures

Fixtures are created by using a line like this which produces a line we ignore via `tail +1` followed by the un-prettified object payload
trailed by a newline.
```sh
echo c56a8e7aa92c86c41a923bc760d2dc39e8a31cf7  | git cat-file --batch | tail +2 > fixture
```

Thus one has to post-process the file by reducing its size by one using `truncate -s -1 fixture`, **removing the newline byte**.


## Shortcomings

* **lean** and **light** and **small** builds don't support non-UTF-8 paths
  * This is because they depend on `argh`, which [does not yet support parsing OsStrings](https://github.com/google/argh/issues/33). We however
    believe it eventually will do so and thus don't move on to [`pico-args`](https://github.com/RazrFalcon/pico-args/blob/master/examples/app.rs).
* **Packfiles use memory maps**
  * Even though they are comfortable to use and fast, they squelch IO errors.
  * _potential remedy_: We could generalize the Pack to make it possible to work on in-memory buffers directly. That way, one
    would initialize a Pack by reading the whole file into memory, thus not squelching IO errors at the expense of latency as well
    as memory efficiency.
* **Packfiles cannot load files bigger than 2^31 or 2^32 on 32 bit systems**
  * As these systems cannot address more memory than that.
  * _potential remedy_: implement a sliding window to map and unmap portions of the file as needed.
* **CRC32** implementation doesn't use SIMD
  * Probably at no cost one could upgrade to the **crc32fast** crate, but it looks unmaintained and has more code.

## Credits

* **itertools** _(MIT Licensed)_
  * We use the `izip!` macro in code
* **deflate2** _(MIT Licensed)_
  * We use various abstractions to implement decompression and compression directly on top of the rather low-level `miniz_oxide` crate

## Fun facts

* Originally I was really fascinated by [this problem](https://github.com/gitpython-developers/GitPython/issues/765#issuecomment-396072153)
  and believe that with `gitoxide` it will be possible to provide the fastest solution for it.
* I have been absolutely blown away by `git` from the first time I experienced git more than 13 years ago, and 
  tried to implement it in [various shapes](https://github.com/gitpython-developers/GitPython/pull/1028) and [forms](https://github.com/byron/gogit)
  multiple [times](https://github.com/Byron/gitplusplus). Now with Rust I finally feel to have found the right tool for the job!
