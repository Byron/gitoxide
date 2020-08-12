### Pack streaming and restoring

* **index-from-pack**
  * _capability to receive packs from a stream_
  * [x] stream pack with iterator interface
    * [x] provide seen compressed bytes
  * [x] repair pack - write trailer for all complete objects
  * [x] build index from pack
      * [x] header
      * [x] fan-out table
      * [x] oids
      * [x] crc32
          * [x] encode pack header (for needed for CRC32)
      * [x] 32bit offsets
      * [x] 64 bit offset extension
  * [x] seriously refactor the header abstraction in the pack file - it's not right at all.
  * [ ] ~~refactor pack:data::iter::Entry to duplicate less of pack::data::Entry~~
  * [x] use progress (one for reading bytes, one for objects, solving the long-standing problem on how to visualize this)
     * [x] use new show_throughput method everywhere
* **cli index-from-pack**
  * [x] build index from pack
     * [x] from stdin()
  * [x] complete pack with trailer
  * **fixes**
    * [x] figure out why resolving the linux pack is so slow and fix it
    * [x] Allow to provide a pre-resolve phase to initialize the resolver
    * [x] Use Tree in verify impl
    * [x] ~~fix lookup todos~~ - it's nearly twice as slow
    * [x] per-object counts for statistics (and use that to optimize order when matching on object types)
    * [x] handle ctrl+c similarly to the pretty version to prevent leakage (or find a way to use
      system temp files)
         * [x] for lean mode
         * [x] for pretty mode
         * [x] allow interrupting the resolution phase too
         * [x] fix typo :D - thanks IJ for confusing me
  * [x] move --verbose, --progress and --progress-keep-open to the top-level 
  * [x] journey test for command-line capabilities
  * [x] unit tests for bundle index write
  * [ ] ~~nicer errors with descriptive messages~~ I don't feel it now, and it's trivial to add them when needed
  * [x] stress test generating an index for the linux kernel pack (uses 64 bit offsets)
* **cli - verify**
   * [ ] break progress up into bytes decompressed and compressed bytes read
* **progress - convenience**
  * [x] use usize for progress counting
  * [x] Unit conversion probably using a new Unit enum (to support more than just bare strings)
  * [x] percentages somewhere near the progress bar
  * [x] Find a way to measure live throughput (in line renderer and tui renderer)
  * [ ] ~~a way to measure throughput on drop~~ - we don't know if the operation was successful, should not be automatic, ever
* **parallel**
  * [ ] ~~sequential version of eager iterator~~ - let's always keep it eager as this really helps with offloading IO and some processing
        from the consumers.
  * [x] auto-sequential eager iterator if there is not enough objects to process
**bugs & improvements**
  * [x] can abort sha1 of pack/index and ctrlc/interrupt is not intercepted.
  * [x] better usability for units
  * [x] When verifying, there is still 'sha1 of index' flying around despite being done
  * [x] parallelize initial verification of the pack and interrupt the rest on failure to trigger stopping everything else
* **asciinemas**
   * [ ] explode
   * [ ] index-from-pack
  
### Cloning

* **receive pack**
  * [ ] resolve thin pack with Bundle
* **git-transport**
  * [ ] transport layer
  * [ ] receive a pack
* **git-refs**
  * Enough to handle a git fetch - maybe we can just hardcode things for now…
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
  * finish transitioning to futures-lite to get rid of futures-util dependency to reduce compile times
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
* **quick-error** update
  * We need a new release soon with the latest updates to get rid of our git dependency. Until then we can't release.

[josh-aug-12]: https://github.com/Byron/gitoxide/issues/1#issuecomment-672566602
