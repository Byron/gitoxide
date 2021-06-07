## Cargo features guide

Cargo uses feature toggles to control which dependencies are pulled in, allowing users to specialize crates to fit their usage.
Ideally, these should be additive.
This guide documents which features are available for each of the crates provided here and how they function.

### gitoxide

The top-level command-line interface.

* **fast**
    * Makes the crate execute as fast as possible by supporting parallel computation of otherwise long-running functions
      as well as fast, hardware accelerated hashing, along with a faster zlib backend.
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
* **gitoxide-core-organize**
    * An alias for `gitoxide-core/organize`.

There are **convenience features**, which combine common choices of the above into one name

* **max** = *pretty-cli* + *fast* + *prodash-render-tui-crossterm* + *http* + *gitoxide-core-organize*
    * _default_, for unix and windows
* **max-termion** = *pretty-cli* + *fast* + *prodash-render-tui-termion* + *http* + *gitoxide-core-organize*
    * for unix only, faster compile times, a little smaller
* **lean** = *lean-cli* + *fast* + *prodash-render-line-crossterm* + *gitoxide-core-organize*
    * for unix and windows, significantly smaller than _max_, but without `--progress` terminal user interface.
* **lean-termion** = *lean-cli* + *fast* + *prodash-render-line-termion* + *gitoxide-core-organize*
    * for unix only, faster compile times, a little smaller
* **light** = *lean-cli* + *fast* + *gitoxide-core-organize*
    * crossplatform by nature as this comes with simplified log based progress
* **small** = *lean-cli*
    * As small as it can possibly be, no threading, no fast sha1, log based progress only, no cleanup of temporary files on interrupt, rust based zlib implementation.

### gitoxide-core

The library powering the command-line interface.

* **organize**
    * **provides the 'organize' subcommand**
        * Includes `jwalk` to find repositories quickly in order to move into a directory structure automatically.
    * **provides the 'find' subcommand**
        * discover all git repositories within a directory. Particularly useful with [skim][skim].

[skim]: https://github.com/lotabout/skim

### git-odb

* **pack-cache-lru-static**
    * Provide a fixed-size allocation-free LRU cache for packs. It's useful if caching is desired while keeping the memory footprint
      for the LRU-cache itself low.
* **pack-cache-lru-dynamic**
    * Provide a hash-map based LRU cache whose eviction is based a memory cap calculated from object data.

### git-features

A crate to help controlling which capabilities are available from the top-level crate that uses `gitoxide-core` or any other
`gitoxide` crate that uses `git-features`.
All feature toggles are additive.

* **progress**
    * Provide traits and utilities for providing progress information. These can then be rendered using facilities provided by
      the `prodash` crate.
* **parallel**
    * Use scoped threads and channels to parallelize common workloads on multiple objects. If enabled, it is used everywhere
      where it makes sense.
    * As caches are likely to be used and instantiated per thread, more memory will be used on top of the costs for threads.
* **crc32**
    * provide a proven and fast `crc32` implementation.
* **io-pipe**
    * an in-memory unidirectional pipe using `bytes` as efficient transfer mechanism
* **zlib**
    * Enable the usage of zlib related utilities to compress or decompress data.
    * By default it uses a pure rust implementation which is slower than the **zlib-ng-compat** version, but might be relevant if you prefer a pure-rust build
      and reduced performance is acceptable. Note that a competitive Zlib implementation is critical to `gitoxide's` performance.
    * Additional backends are supported, each of which overriding the default Rust backend.
      * _mutually-exclusive_
       * **zlib-ng-compat**
         * Use a C-based backend which can compress and decompress significantly faster.
      * **cloudflare-zlib**
         * Another incarnation of a faster-than-normal zlib backend.
      * **_zlib-rust-backend_**
         * available for completeness even though it's the default - it may be chosen for more maintainable feature flags.
    
* **walkdir**
    * Makes facilities of the `walkdir` crate partially available.
    * In conjunction with the **parallel** feature, directory walking will be parallel instead behind a compatible interface.
* _mutually-exclusive_
    * **fast-sha1**
        * a multi-crate implementation that can use hardware acceleration, thus bearing the potential for up to 2Gb/s throughput on
          CPUs that support it, like AMD Ryzen or Intel Core i3.
        * Takes precedence over `sha1` if both are specified.
        * A fast SHA1 implementation is critical to `gitoxide's` performance
    * **sha1**
        * A standard and well performing pure Rust implementation of Sha1. Will significantly slow down various git operations.
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

### git-packetline

By default, all IO related capabilities will be missing unless one of the following is chosen.

* _mutually exclusive_
  * If both are present, _blocking-io_ is chosen.
  * **blocking-io**
    * If set, all IO will become blocking. The same types will be used preventing side-by-side usage of blocking and non-blocking IO
  * **async-io**
    * Implement IO traits from `futures-io` 
    

### git-transport

The _client_ portion of transport can be blocking or async. If none is selected, it will be missing entirely.

- _mutually exclusive_
    - **blocking-client**
      - If set, blocking implementations of the typical git transports become available in `crate::client`
      - **http-client-curl**
          - Adds support for the http and https transports using the Rust bindings for `libcurl`
  - **async-client**
      - If set, an async implementations of the git transports becomes available in `crate::client`.
      - Suitable for implementing your own transports while using git's way of communication, typically in conjunction with a custom server.
         - **Note** that the _blocking_ client has a wide range of available transports, with the _async_ version of it supporting only the TCP based `git` transport leaving you
            with the responsibility to providing such an implementation of `futures-io::AsyncRead/AsyncWrite` yourself.
    
### git-protocol

The _client_ portion of the protocol uses `git-transport` to communicate to a server. For it to be usable, one of the following features must
be selected.

- _mutually exclusive_ 
    - Specifying both causes a compile error, preventing the use of `--all-features`.
    * **blocking-client**
      * If set, blocking command implementations are available and will use the blocking version of the `git-transport` crate.
    * **async-client**
      * As above, but provides async implementations instead.

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
 
