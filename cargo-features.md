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
* **prodash-render-line-crossterm** 
    * The `--verbose` flag will be powered by an interactive progress mechanism that doubles as log as well as interactive progress
      that appears after a short duration.
* **gitoxide-core-tools**
    * A way to enable all `gitoxide-core` tools found in `gix tools`
* _mutually exclusive_    
    * If both are set a compile error is triggered. This also means that `cargo … --all-features` will fail.
    * **gitoxide-core-blocking-client**
        * Use blocking client networking.
    * **gitoxide-core-async-client**
      * Use async client networking.

There are **convenience features**, which combine common choices of the above into one name

* **max** = *fast* + *prodash-render-tui-crossterm* + *prodash-render-line-crossterm* + *http* + *gitoxide-core-tools* + *client-networking*
    * _default_, for unix and windows
* **lean** = *fast* + *prodash-render-line-crossterm* + *gitoxide-core-tools* + *client-networking*
    * cross-platform by nature as this comes with simplified log based progress
* **lean-async** = *fast* + *prodash-render-line-crossterm* + *gitoxide-core-tools* + *client-async-networking*
   * Due to async client-networking not being implemented for most transports, this one supports only the 'git' transport. It uses, however, a fully asynchronous
     networking implementation which can serve a real-world example on how to implement custom async transports.
* **small**
    * As small as it can possibly be, no threading, no fast sha1, log based progress only, rust based zlib implementation.
    * no networking, local operations only.

### gitoxide-core

The library powering the command-line interface.

* **organize**
    * **provides the 'organize' command**
        * Includes `jwalk` to find repositories quickly in order to move into a directory structure automatically.
    * **provides the 'find' command**
        * discover all git repositories within a directory. Particularly useful with [skim][skim].
* **estimate-hours**
    * Derive the amount of time invested akin to [git-hours].
* _mutually exclusive_    
  - if both are set, _blocking-client_ will take precedence.
  - **blocking-client**
    - If set, the client used to connect to git servers will use a blocking API. It supports more transports and is what most would want.
  - **async-client**
    - The client to connect to git servers will be async, while supporting only the 'git' transport itself. It's the most limited and can be seen as example
      on how to use custom transports for custom servers.
* **local-time-support**
    - Functions dealing with time may include the local timezone offset, not just UTC with the offset being zero.

[skim]: https://github.com/lotabout/skim
[git-hours]: https://github.com/kimmobrunfeldt/git-hours

### git-pack

Documented in [its documentation](https://docs.rs/git-pack).

### git-actor

Documented in [its documentation](https://docs.rs/git-actor).

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
    * The `threading` module will contain thread-safe primitives for shared ownership and mutation, otherwise these will be their single threaded counterparts.
        * This way, single-threaded applications don't have to pay for threaded primitives.
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
          CPUs that support it, like AMD Ryzen or Intel Core i3, as well as Apple Silicon like M1.
        * Takes precedence over `rustsha1` if both are specified.
        * A fast SHA1 implementation is critical to `gitoxide's` performance
    * **rustsha1**
        * A standard and well performing pure Rust implementation of Sha1. Will significantly slow down various git operations.

* **cache-efficiency-debug**
    * Caches implement this by default, which costs nothing unless this feature is enabled
    * Count cache hits and misses and print that debug information on drop
* **time**
    * Make the `time` module available with access to the local time as configured by the system.
     
### git-packetline

Documented in [its documentation](https://docs.rs/git-packetline).

### git-transport

Documented in [its documentation](https://docs.rs/git-transport).
    
### git-protocol

Documented in [its documentation](https://docs.rs/git-protocol).
    
### git-object

Documented in [its documentation](https://docs.rs/git-object).
  
### git-repository

Documented in [its documentation](https://docs.rs/git-repository).

### Serialization Support

What follows is feature toggles to control serialization of all public facing simple data types.

* **serde1**
    * Data structures implement `serde::Serialize` and `serde::Deserialize`

The feature above is provided by the crates:

* **git-actor**
* **git-object**
* **git-ref**
* **git-url**
* **git-odb**
* **git-protocol**
* **git-repository**
* **gitoxide-core**
 
