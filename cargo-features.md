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
    * As small as it can possibly be, no threading, no fast sha1, log based progress only, no cleanup of temporary files on interrupt

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

By default, all IO related capabilities will use async IO traits from `futures-io`.

* **blocking-io**
  If set, all IO will become blocking. The same types will be used preventing side-by-side usage of blocking and non-blocking IO

### git-transport

The _client_ portion of the transport layer is _async_ by default, i.e. if no feature toggles are set.

* **blocking-client**
  * If set, blocking implementations of the typical git transports become available in `crate::client`
  * **http-client-curl**
    * Adds support for the http and https transports using the Rust bindings for `libcurl`
    
**Note** that the _blocking_ client has a great choice of available transports, with the _async_ version of it supporting only the TCP based `git` transport leaving you
with the responsibility to providing such an implementation of `futures-io::AsyncRead/AsyncWrite` yourself.
    
### git-protocol

The _client_ portion of the protocol is _async_ by default, i.e. if no feature toggles are set.

* **blocking-client**
  * If set, blocking implementations are available and will use the blocking version of the `git-transport` crate.

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
 
