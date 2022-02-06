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

Documented in [its documentation](https://docs.rs/gitoxide-core).

### git-pack

Documented in [its documentation](https://docs.rs/git-pack).

### git-actor

Documented in [its documentation](https://docs.rs/git-actor).

### git-features

Documented in [its documentation](https://docs.rs/git-features).
     
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
 
