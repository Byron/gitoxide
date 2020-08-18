[![Rust](https://github.com/Byron/git-oxide/workflows/Rust/badge.svg)](https://github.com/Byron/git-oxide/actions)
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
  * **the `gix` program** - convenient and for humans
    * [x] init - initialize a new non-bare repository with a `main` branch
  * **the `gixp` program** _(plumbing)_ - lower level commands for use in automation
    * **pack**
      * [x] [pack verify](https://asciinema.org/a/352942)
      * [x] [pack index verify](https://asciinema.org/a/352945) including each object sha1 and statistics
      * [x] [pack explode](https://asciinema.org/a/352951), useful for transforming packs into loose objects for inspection or restoration
        * [x] verify written objects (by reading them back from disk)
      * [ ] **pack-receive** - receive a pack produced by **pack-send** or _git-upload-pack_
      * [ ] **pack-send** - create a pack and send it using the pack protocol to stdout, similar to 'git-upload-pack', 
            for consumption by **pack-receive** or _git-receive-pack_
    * **pack-index**
      * [x] [index from data](https://asciinema.org/a/352941) - create an index file by streaming a pack file as done during clone
          * [ ] support for thin packs (as needed for fetch/pull)
          
### git-object
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
  
### git-odb
  * **loose objects**
    * [x] traverse
    * [x] read
      * [x] into memory
      * [x] streaming
      * [x] verify checksum
    * [x] streaming write for blobs
    * [x] buffer write for small in-memory objects/non-blobs to bring IO down to open-read-close == 3 syscalls
  * **packs**
    * [x] traverse pack index
    * [x] 'object' abstraction
      * [x] decode (zero copy)
      * [x] verify checksum
    * [x] simple and fast pack traversal
    * [x] decode
      * [x] full objects
      * [x] deltified objects
    * **streaming**
      * _decode a pack from `Read` input_
      * [x] `Read` to `Iterator` of entries
        * _read as is, verify hash, and restore partial packs_
      * [x] create index from pack alone (_much faster than git_)
        * [ ] resolve 'thin' packs
    * [ ] encode
      * [ ] Add support for zlib-ng for 2.5x compression performance and 20% faster decompression
      * [ ] create new pack
      * [ ] create 'thin' pack
    * [x] verify pack with statistics
      * [x] brute force - less memory
      * [x] indexed - faster, but more memory
    * **advanced**
      * [ ] Multi-Pack index file (MIDX)
      * [ ] 'bitmap' file
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

### git-url
  * As documented here: https://www.git-scm.com/docs/git-clone#_git_urls
  * [x] ssh URLs and SCP like syntax
  * [x] file, git, and SSH
  * [x] paths (OS paths, without need for UTF-8)
  * [x] username expansion for ssh and git urls
  * [ ] API documentation with examples
  
### git-protocol
  * No matter what we do here, timeouts must be supported to prevent hanging forever and to make interrupts destructor-safe.
  * Packet lines must be abstracted from the client at least, as the 'dumb' transport doesn't use them.
  * [x] [PKT-Line](https://github.com/git/git/blob/master/Documentation/technical/protocol-common.txt#L52:L52)
    * [x] encode
    * [x] decode (zero-copy)
    * [x] [error line](https://github.com/git/git/blob/master/Documentation/technical/pack-protocol.txt#L28:L28)
    * [x] [V2 additions](https://github.com/git/git/blob/master/Documentation/technical/protocol-v2.txt#L35:L36)
    * [x] [side-band mode](https://github.com/git/git/blob/master/Documentation/technical/pack-protocol.txt#L467:L467)
    * [x] `Read` from packet line sidebands with progress support
  * [ ] shallow clones
  * [ ] **Version 1**
    * [ ] parse and serialize [capabilities](https://github.com/git/git/blob/master/Documentation/technical/protocol-capabilities.txt#L1:L1)
    * [ ] [fetch](https://github.com/git/git/blob/master/Documentation/technical/pack-protocol.txt#L157:L157)
      * [ ] [ref advertisement](https://github.com/git/git/blob/master/Documentation/technical/pack-protocol.txt#L200:L200)
      * [ ] [upload request](https://github.com/git/git/blob/master/Documentation/technical/pack-protocol.txt#L245:L245)
      * [ ] [shallow update](https://github.com/git/git/blob/master/Documentation/technical/pack-protocol.txt#L305:L305)
      * [ ] [upload haves](https://github.com/git/git/blob/master/Documentation/technical/pack-protocol.txt#L333:L333)
        * [ ] 'simple' (multi-ack* is absent)
        * [ ] multi-ack 
        * [ ] multi-ack detailed
      * [ ] [server-response (pack)](https://github.com/git/git/blob/master/Documentation/technical/pack-protocol.txt#L404:L404)
    * [ ] push
  * [ ] [Version 2](https://github.com/git/git/blob/master/Documentation/technical/protocol-v2.txt)
  * [ ] API documentation with examples
  
### git-transport
  * [ ] **[git](https://github.com/git/git/blob/master/Documentation/technical/pack-protocol.txt#L66:L66)**
    * [ ] **initiate**
      * [ ] [extra parameters](https://github.com/git/git/blob/master/Documentation/technical/pack-protocol.txt#L52:L52) via null separated k=v pairs
          * [ ] protocol version definition
  * [ ] **[ssh](https://github.com/git/git/blob/master/Documentation/technical/pack-protocol.txt#L103:L103)**
    * `ssh2` crate with [openssl vendoring support](https://lib.rs/crates/ssh2) for static linkage
    * [ ] **initiate**
      * extra paramaters (via environment variable)
  * [ ] protocol for transfer [via http(s)](https://github.com/git/git/blob/master/Documentation/technical/http-protocol.txt)
    * [ ] 'smart'
    * [ ] ~~dumb~~ (_opt out for now_)
    * [ ] extra parameters
  * [ ] API documentation with examples
  
### git-repository
  * [x] initialize
    * [ ] Proper configuration depending on platform (e.g. ignorecase, filemode, …)
  * [ ] [Signed commits and tags](https://github.com/Byron/gitoxide/issues/12)
  * [ ] read and write all data types
  * [ ] rev-parsing and ref history
  * [ ] remotes with push and pull
  * [ ] configuration
  * [ ] merging
  * [ ] stashing
  * [ ] API documentation with examples
  * [ ] _Commit Graph_ - split and unsplit
  
### git-config
  * read and write git configuration files
  * [ ] API documentation with examples
  
### git-ref
  * Handle symbolic references and packed references
  * discover them in typical folder structures
  * [x] [name validation](https://github.com/git/git/blob/master/Documentation/technical/protocol-common.txt#L23:L23)
  * [ ] API documentation with examples
  
### git-index
  * read and write a git-index file
  * add and remove entries
  * [ ] API documentation with examples
  
### git-diff
  * diffing of git-object::Tree structures
  * diffing, merging, working with hunks of data
  * find differences between various states, i.e. index, working tree, commit-tree
  * [ ] API documentation with examples
  
### git-features
  * **parallel** feature toggle
    * _When on…_
      * `in_parallel`
      * `join`
    * _When off all functions execute serially_
    
### git-tui
  * _a terminal user interface seeking to replace and improve on `tig`_
  
### Stress Testing
  * [x] Verify huge packs
  * [x] Explode a pack to disk 
  * [ ] Generate huge pack from a lot of loose objects
  
### Ideas for Demos
  * [ ] A simple [`git-hours`][git-hours-algo] clone
  * [ ] Open up SQL for git using [sqlite virtual tables](https://github.com/rusqlite/rusqlite/blob/master/tests/vtab.rs). Check out gitqlite
        as well. What would an MVP look like? Maybe even something that could ship with gitoxide.

[git-hours-algo]: https://github.com/kimmobrunfeldt/git-hours/blob/8aaeee237cb9d9028e7a2592a25ad8468b1f45e4/index.js#L114-L143

## Installation

### Binary Release

```sh
curl -LSfs https://raw.githubusercontent.com/Byron/gitoxide/main/ci/install.sh | \
    sh -s -- --git Byron/gitoxide --crate gix-max-termion
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
   * libraries use light-weight custom errors implemented using `quick-error`.
   * internationalization is nothing we are concerned with right now.
   * IO errors due to insufficient amount of open file handles don't always lead to operation failure
 * **Cross platform support, including Windows**
   * With the tools and experience available here there is no reason not to support Windows.
   * [Windows is testsed on CI](https://github.com/Byron/git-oxide/blob/df66d74aa2a8cb62d8a03383135f08c8e8c579a8/.github/workflows/rust.yml#L34)
     and failures do prevent releases.
     

## Non-Goals

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
    * Use `clap` 3.0 to build the prettiest, best documented and most user-friendly CLI at the expense of file size.
    * provides a terminal user interface for detailed and exhaustive progress.
    * provides a line renderer for log-like progress
  * **lean-cli**
    * Use `argh` to produce a usable binary with decent documentation that is smallest in size, usually 300kb less than `pretty-cli`.
    * If `pretty-cli` is enabled as well, `lean-cli` will take precedence, and you pay for building unnecessary dependencies.
    * provides a line renderer for log-like progress
* **prodash-render-line-crossterm** or **prodash-render-line-termion** _(mutually exclusive)_
  * The `--verbose` flag will be powered by an interactive progress mechanism that doubles as log as well as interactive progress
    that appears after a short duration.
  
There are **convenience features**, which combine common choices of the above into one name

* **max** = *pretty-cli* + *fast* + *prodash-render-tui-crossterm*
  * _default_, for unix and windows
* **max-termion** = *pretty-cli* + *fast* + *prodash-render-tui-termion*
  * for unix only, faster compile times, a little smaller
* **lean** = *lean-cli* + *fast* + *prodash-render-line-crossterm*
  * for unix and windows, significantly smaller than _max_, but without `--progress` terminal user interface.
* **lean-termion** = *lean-cli* + *fast* + *prodash-render-line-termion*
  * for unix only, faster compile times, a little smaller
* **light** = *lean-cli* + *fast*
  * crossplatform by nature as this comes with simplified log based progress
* **small** = *lean-cli*
  * As small as it can possibly be, no threading, no fast sha1, log based progress only, no cleanup of temporary files on interrupt
    
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
* _mutually-exclusive_
  * **interrupt-handler**  
    * Listen to interrupts and termination requests and provide long-running operations tooling to allow aborting the input stream.
      * **Note that** `git_features::interrupt::init_handler()` must be called at the start of the application.
    * If the application already sets a handler, this handler will have no effect.
    * If unset, these utilities can still be triggered programmatically. However, interrupting with Ctrl+C or SIGTERM may lead to 
      leaking temporary files.
  * **disable-interrupts** (_takes precedence if **interrupt-handler** is set as well_)
    * If set, interrupts cannot be triggered programmatically and it's up to the user to inject means of supporting interrupts.
    * Useful if there is multiple interruptible operations at the same time that should be triggered independently. After all,
    * this facility is a global one.
    * Probably useful for server implementations.
    
### Serialization Support
 
 What follows is feature toggles to control serialization of all public facing simple data types.
 
 * **serde1**
   * Data structures implement `serde::Serialize` and `serde::Deserialize`
   
 The feature above is provided by the crates:
 
 * **git-object**
 * **git-url**
 * **git-odb**
 * **git-protocol**
 * **gitoxide-core**
 
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
* it does not need a git repository, but instead takes all variables via the command-line 

### Porcelain

* Provides output to stderr by default to provide progress information. There is no need to allow disabling it, but it shouldn't show up unless
  the operation takes some time.
* If timestamps are shown, they are in localtime.
* Non-progress information goes to stdout.

## Shortcomings

* **lean** and **light** and **small** builds don't support non-UTF-8 paths _in the CLI_
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
* **git-url** _might_ be more restrictive than what git allows as for the most part, it uses a browser grade URL parser.
  
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

  
## Unused Performance Optimizations
* **miniz-oxide**
  * **unnecessary buffer reset**
    * In the [`InflateState` struct][miniz-inflatestate], there is a big 32kb buffer which gets zeroed for every decompression attempt.
    * This costs ~4s for 7.5 million objects.
  * **reuse of state between decompressions could be faster**
    * Similar to above, there are several occasions when we decompress in an 'all at once', which also requires to recreate a 32kb buffer
      filled with zeroes. If most of that state could be reused, we would save time when handling millions of objects both during pack
      lookup as well as pack streaming.
    
[miniz-inflatestate]: https://github.com/Frommi/miniz_oxide/blob/7f5aedd7cc553b624902210a7d136440c138dc80/miniz_oxide/src/inflate/stream.rs#L102

## Fun facts

* Originally I was really fascinated by [this problem](https://github.com/gitpython-developers/GitPython/issues/765#issuecomment-396072153)
  and believe that with `gitoxide` it will be possible to provide the fastest solution for it.
* I have been absolutely blown away by `git` from the first time I experienced git more than 13 years ago, and 
  tried to implement it in [various shapes](https://github.com/gitpython-developers/GitPython/pull/1028) and [forms](https://github.com/byron/gogit)
  multiple [times](https://github.com/Byron/gitplusplus). Now with Rust I finally feel to have found the right tool for the job!
