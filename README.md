[![Rust](https://github.com/Byron/git-oxide/workflows/Rust/badge.svg)](https://github.com/Byron/git-oxide/actions)

**gio** is a command-line interface (*CLI*) to access git repositories. It's written to optimize the
user-experience, and perform as good or better than the native implementation, and make git tooling more
hackable.

The CLI uses various crates, please see _'Development Status'_ for details.

## Development Status

* **gitoxide** _(CLI)_
  * please note that all functionality comes from the `gitoxide-core` library, which mirrors these capabilities
    and itself relies on all `git-*` crates.
  * repository
    * [x] init
  * plumbing
    * [x] pack verify
    * [x] pack index verify including each object sha1
* **git-repository**
  * [x] initialize
  * [ ] read and write all data types
  * [ ] rev-parsing and ref history
  * [ ] remotes with push and pull
  * [ ] configuration
  * [ ] merging
  * [ ] API documentation with examples
* **git-config**
  * read and write git configuration files
  * [ ] API documentation with examples
* **git-refs**
  * Handle symbolic references and packed references
  * discover them in typical folder structures
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
* **git-object**
  * represent borrowed commits, trees and tags
  * *decode (zero-copy)*
    * [x] commit
    * [x] tree
    * [x] tag
  * encode
    * [ ] commit
    * [ ] tree
    * [ ] tag
  * [ ] API documentation with examples
* **git-odb**
  * **loose objects**
    * [x] traverse
    * [x] read
      * [x] into memory
      * [x] streaming
    * [x] streaming write
  * **packs**
    * [x] traverse pack index
    * [ ] decode
      * [x] full objects
      * [x] deltified objects
      * [ ] Multi-Pack index file (MIDX)
      * [ ] 'bitmap' file
    * [ ] encode
      * [ ] create new packs
    * [ ] verify pack
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
* **Stress Testing**
  * [ ] Verify huge packs
  * [ ] Explode huge packs to disk and validate loose objects
  * [ ] Generate huge back from a lot of loose objects

## Installation

**TBD**

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
     behind a feature flag and once the need transpire.
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

## Roadmap to Future

As you can see from the version numbers, this project dispenses major version generously.

### Roadmap to 1.0

Provide a CLI to for the most basic user journey:

* [x] initialize a repository
* [ ] create a commit
* [ ] add a remote
* [ ] push

### `git-features` guide

A crate to help controlling which capabilities are available from the top-level crate that uses `gitoxide`.
All feature toggles are additive.

* **parallel** (optional)
  * Use scoped threads and channels to parallelize common workloads on multiple objects. If enabled, it is used everywhere
    where it makes sense.
  * As caches are likely to be used and instantiated per thread, more memory will be used on top of the costs for threads.
* **fast-sha1** 
  * a multi-crate implementation that can use hardware acceleration, thus bearing the potential for up to 2Gb/s throughput on 
    CPUs that support it, like AMD Ryzen.
 
## Development Practices

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

## Maintenance Guide

Utilities to aid in keeping the project fresh and in sync can be found in the `Maintenance` section of the `makefile`.

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
  and believe that with `gitoxide` it will be possible to provide the fastest implementation for that problem.

