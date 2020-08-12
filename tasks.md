### Cloning

* **git-transport**
  * [ ] transport layer
  * [ ] receive a pack
* **git-refs**
  * Enough to handle a git fetch - maybe we can just hardcode things for now…
* **receive pack**
  * [ ] resolve thin pack with Bundle
* **git-repository**
  * [ ] clone from https remote
  
### FSCK an entire repository

* **multi-db** (incorporate object lookup for loose objects and packs)
  * [ ] single threaded
  * [ ] optional object cache
  * [ ] fs-check - verify all object content of a git repository
* **cli**
  * [ ] Clone into valid repository
  
### Other

To be picked in any order….

* **prodash**
  * [ ] finish transitioning to futures-lite to get rid of futures-util dependency to reduce compile times
* **gitoxide performance**
  * [ ] @joshtriplett writes: "Regarding decompression performance, try replacing miniz_oxide with a better zlib decoder. Build with libz-sys, and then try substituting zlib-ng built with --zlib-compat. (I'm working on making that easier.) That should substantially improve decompression."
    * potential [savings: MASSIVE](https://github.com/Byron/gitoxide/issues/1#issuecomment-672626465) 
    * Note that this should only be feature toggled. Using any of the above would replace a pure Rust implementation, which we would always like to keep as an option for those who want maximum safety.
  * [ ] Add '--chunk|batch-size' flag to `pack-verify` and `index-from-pack` to allow tuning sizes for large amounts of cores
    * potential savings: medium
  * [ ] Add more control over the amount of memory used for the `less-memory` algorithm of `pack-verify` to increase cache hit rate at the cost of memory.
    Note that depending on this setting, it might not be needed anymore to iterated over sorted offsets, freeing 150MB of memory in the process
    that could be used for the improved cache. With the current cache and no sorted offsets, the time nearly triples.
  * [ ] _progress measuring costs when using 96 cores_ (see [this comment][josh-aug-12])
    * potential savings: low
* **criner**
  * [ ] switch to `isahc`
    seems to allow async-reading of bodies, allowing to get rid of reqwest and tokio. Redirect is configurable.
* **miniz-oxide**
  * Get [our PR](https://github.com/Frommi/miniz_oxide/pull/92) merged

[josh-aug-12]: https://github.com/Byron/gitoxide/issues/1#issuecomment-672566602
