[![Rust](https://github.com/Byron/gitoxide/workflows/Rust/badge.svg)](https://github.com/Byron/gitoxide/actions)
[![Crates.io](https://img.shields.io/crates/v/gitoxide.svg)](https://crates.io/crates/gitoxide)

**gix** is a command-line interface (*CLI*) to access git repositories. It's written to optimize the
_user-experience_, and perform as _good or better than the canonical implementation_.

Furthermore it provides **an easy and safe to use API** in the form of various small crates for implementing your own tools in a breeze.
Please see _'Development Status'_ for a listing of all crates and their capabilities.

[![asciicast](https://asciinema.org/a/352942.svg)](https://asciinema.org/a/352942)

## Development Status

**Please note** that from 2020-09-17, the development speed will be reduced greatly. I will do my best to create at least
one commit per day ramp it up from there to eventually arrive at a new baseline velocity. It will be lower than what it was before, and
I hope 1/2 to 2/3 of that will be realistic.

This is entirely unrelated to the project and I still can't wait to use `gitoxide` on a daily basis once the first high-level commands
become available.

### gitoxide _(CLI)_
  * please note that all functionality comes from the `gitoxide-core` library, which mirrors these capabilities
    and itself relies on all `git-*` crates.
  * limit amount of threads used in operations that support it.
  * choose between 'human' and 'json' output formats
  * **the `gix` program** - convenient and for humans
    * [x] **init** - initialize a new non-bare repository with a `main` branch
    * [ ] **clone** - initialize a local copy of a remote repository
  * **the `gixp` program** _(plumbing)_ - lower level commands for use in automation
    * **pack**
      * [x] [pack verify](https://asciinema.org/a/352942)
      * [x] [pack index verify](https://asciinema.org/a/352945) including each object sha1 and statistics
      * [x] [pack explode](https://asciinema.org/a/352951), useful for transforming packs into loose objects for inspection or restoration
        * [x] verify written objects (by reading them back from disk)
      * [x] [pack-receive](https://asciinema.org/a/359321) - receive a whole pack produced by **pack-send** or _git-upload-pack_, useful for `clone` like operations.
      * [ ] **pack-send** - create a pack and send it using the pack protocol to stdout, similar to 'git-upload-pack', 
            for consumption by **pack-receive** or _git-receive-pack_
    * **pack-index**
      * [x] [index from data](https://asciinema.org/a/352941) - create an index file by streaming a pack file as done during clone
          * [ ] support for thin packs (as needed for fetch/pull)
    * **commit-graph**
      * [x] **verify** - assure that a commit-graph is consistent
    * [remote-ref-list](https://asciinema.org/a/359320)
      * [x] list all (or given) references from a remote at the given URL
          
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
  * [x] API documentation
    * [ ] Some examples
  * **sink**
    * [x] write objects and obtain id
  * **alternates**
    * _database that act as link to other known git ODBs on disk_
    * [x] safe with cycles and recursive configurations
    * [x] multi-line with comments and quotes
  * **multi-odb**
    * [ ] _an ODB for object lookup from multiple lower level ODB at once_
  * **promisor**
    * It's vague, but these seems to be like index files allowing to fetch objects from a server on demand.

### git-url
  * As documented here: https://www.git-scm.com/docs/git-clone#_git_urls
  * **parse**
    * [x] ssh URLs and SCP like syntax
    * [x] file, git, and SSH
    * [x] paths (OS paths, without need for UTF-8)
  * [x] username expansion for ssh and git urls
  * [x] convert URL to string
  * [ ] API documentation with examples
  
### git-protocol
  * _abstract over protocol versions to allow delegates to deal only with a single way of doing things_
  * [x] **credentials**
    * [x] via git-credentials
    * [ ] via pure Rust implementation if no git is installed
  * [x] fetch & clone
    * [x] detailed progress
    * [x] control credentials provider to fill, approve and reject
    * [x] command: ls-ref
      * [x] parse V1 refs as provided during handshake
      * [x] parse V2 refs
      * [ ] handle empty refs, AKA PKT-LINE(zero-id SP "capabilities^{}" NUL capability-list)
    * [x] initialize and validate command arguments and features sanely
    * [x] abort early for ls-remote capabilities
    * [x] packfile negotiation
      * [x] delegate can support for all fetch features, including shallow, deepen, etc.
      * [x] receive parsed shallow refs
  * [ ] push
  * [ ] API documentation with examples
  
### git-packetline
  * [PKT-Line](https://github.com/git/git/blob/master/Documentation/technical/protocol-common.txt#L52:L52)
  * [x] encode
  * [x] decode (zero-copy)
  * [x] [error line](https://github.com/git/git/blob/master/Documentation/technical/pack-protocol.txt#L28:L28)
  * [x] [V2 additions](https://github.com/git/git/blob/master/Documentation/technical/protocol-v2.txt#L35:L36)
  * [x] [side-band mode](https://github.com/git/git/blob/master/Documentation/technical/pack-protocol.txt#L467:L467)
  * [x] `Read` from packet line with (optional) progress support via sidebands
  * [x] `Write` with built-in packet line encoding

### git-transport
  * No matter what we do here, timeouts must be supported to prevent hanging forever and to make interrupts destructor-safe.
  * **client**
    * [x] general purpose `connect(…)` for clients
      * [x] _file://_ launches service application
      * [x] _ssh://_ launches service application in a remote shell using _ssh_
      * [x] _git://_ establishes a tcp connection to a git daemon
      * [x] _http(s)://_ establishes connections to web server
      * [ ] pass context for scheme specific configuration, like timeouts
    * [x] git://<service>
      * [x] V1 handshake
        * [x] send values + receive data with sidebands
        * [ ] ~~support for receiving 'shallow' refs in case the remote repository is shallow itself (I presume)~~
          * Since V2 doesn't seem to support that, let's skip this until there is an actual need. No completionist :D
      * [x] V2 handshake
        * [x] send command request, receive response with sideband support
    * [x] http(s)://<service>
      * [x] set identity for basic authentication
      * [x] V1 handshake
        * [x] send values + receive data with sidebands
      * [x] V2 handshake
        * [x] send command request, receive response with sideband support
      * [ ] ~~'dumb'~~ - _we opt out using this protocol seems too slow to be useful, unless it downloads entire packs for clones?_
    * [x] authentication failures are communicated by io::ErrorKind::PermissionDenied, allowing other layers to retry with authentication
  * **server**
      * [ ] general purpose `accept(…)` for servers
  * [ ] API documentation with examples

### git-index
  * handle git index files for primary use by the git-repository while crafting new commits
  * [ ] API documentation with examples
  
### git-commitgraph
  * [x] read-only access
     * [x] Graph lookup of commit information to obtain timestamps, generation and parents, and extra edges
     * [ ] Bloom filter index
     * [ ] Bloom filter data
  * [ ] create and update graphs and graph files
  * [ ] API documentation with examples

### git-config
  * [ ] read
    * line-wise parsing with decent error messages
    * [ ] decode value
      * [ ] boolean
      * [ ] integer
      * [ ] color
      * [ ] path (incl. resolution)
      * [ ] include
      * [ ] includeIf
  * [ ] write
    * keep comments and whitespace, and only change lines that are affected by actual changes, to allow truly non-destructive editing
  * [ ] API documentation with examples
  
### git-repository
  * [x] initialize
    * [ ] Proper configuration depending on platform (e.g. ignorecase, filemode, …)
  * [ ] [Signed commits and tags](https://github.com/Byron/gitoxide/issues/12)
  * [ ] clone
    * [ ] shallow
    * [ ] namespaces support
  * [ ] sparse checkout support
  * [ ] execute hooks
  * [ ] .gitignore handling
  * [ ] checkout/stage conversions clean + smudge as in .gitattributes
  * [ ] read and write all data types
  * [ ] rev-parsing and ref history
  * [ ] worktree
  * [ ] remotes with push and pull
  * [ ] configuration
  * [ ] merging
  * [ ] stashing
  * [ ] Use _Commit Graph_ to speed up certain queries
  * [ ] API documentation with examples
  
### git-bundle
  * [ ] create a bundle from an archive
  * [ ] extract a branch from a bundle into a repository
  
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

[releases]: https://github.com/Byron/gitoxide/releases

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

[releases]: https://github.com/Byron/gitoxide/releases 
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
   * libraries use light-weight custom errors implemented using `quick-error` or `thiserror`.
   * internationalization is nothing we are concerned with right now.
   * IO errors due to insufficient amount of open file handles don't always lead to operation failure
 * **Cross platform support, including Windows**
   * With the tools and experience available here there is no reason not to support Windows.
   * [Windows is testsed on CI](https://github.com/Byron/gitoxide/blob/df66d74aa2a8cb62d8a03383135f08c8e8c579a8/.github/workflows/rust.yml#L34)
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
* **http**
  * support synchronous 'http' and 'https' transports (e.g. for clone, fetch and push) at the expense of compile times and binary size
* _(mutually exclusive)_
  * **pretty-cli**
    * Use `clap` 3.0 to build the prettiest, best documented and most user-friendly CLI at the expense of binary size.
    * provides a terminal user interface for detailed and exhaustive progress.
    * provides a line renderer for leaner progress
  * **lean-cli**
    * Use `argh` to produce a usable binary with decent documentation that is smallest in size, usually 300kb less than `pretty-cli`.
    * If `pretty-cli` is enabled as well, `lean-cli` will take precedence, and you pay for building unnecessary dependencies.
    * provides a line renderer for lean but pretty progress
* **prodash-render-line-crossterm** or **prodash-render-line-termion** _(mutually exclusive)_
  * The `--verbose` flag will be powered by an interactive progress mechanism that doubles as log as well as interactive progress
    that appears after a short duration.
  
There are **convenience features**, which combine common choices of the above into one name

* **max** = *pretty-cli* + *fast* + *prodash-render-tui-crossterm* + *http*
  * _default_, for unix and windows
* **max-termion** = *pretty-cli* + *fast* + *prodash-render-tui-termion* + *http*
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
    * Useful if there is multiple interruptible operations at the same time that should be triggered independently. After all, this facility is a global one.
    * Probably useful for server implementations.
* **io-pipe**
  * an in-memory unidirectional pipe using `bytes` as efficient transfer mechanism
    
### git-transport

* **http-client-curl**
  * Adds support for the http and https transports using the Rust bindings for `libcurl`
  
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
The term *plumbing* refers to lower-level, more rarely used commands that complement porcelain by being invoked by it or by hand for certain use
cases.
The term *porcelain* refers to those with a decent user experience, they are primarily intended for use by humans.

In any case, both types of programs must self-document their capabilities using through the `--help` flag.

From there, we can derive a few rules to adhere to unless there are good reasons not to:

### Plumbing

* does not show any progress or logging output by default
* if supported and logging is enabled, it will show timestamps in UTC
* it does not need a git repository, but instead takes all required information via the command-line 

### Porcelain

* Provides output to stderr by default to provide progress information. There is no need to allow disabling it, but it shouldn't show up unless
  the operation takes some time.
* If timestamps are shown, they are in localtime.
* Non-progress information goes to stdout.

## Shortcomings

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
* **CRC32** implementation doesn't use SIMD
  * Probably at no cost one could upgrade to the **crc32fast** crate, but it looks unmaintained and builds more slowly.
* **git-url** _might_ be more restrictive than what git allows as for the most part, it uses a browser grade URL parser.
  * Thus far there is no proof for this, and as _potential remedy_ we could certainly re-implement exactly what git does
    to handle its URLs.
  
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

* Originally I was really fascinated by [this problem](https://github.com/gitpython-developers/GitPython/issues/765#issuecomment-396072153)
  and believe that with `gitoxide` it will be possible to provide the fastest solution for it.
* I have been absolutely blown away by `git` from the first time I experienced git more than 13 years ago, and 
  tried to implement it in [various shapes](https://github.com/gitpython-developers/GitPython/pull/1028) and [forms](https://github.com/byron/gogit)
  multiple [times](https://github.com/Byron/gitplusplus). Now with Rust I finally feel to have found the right tool for the job!
