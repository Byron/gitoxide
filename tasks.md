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
    * [ ] per-object counts for statistics (and use that to optimize order when matching on object types)
    * [ ] nicer errors with descriptive messages
    * [ ] handle ctrl+c similarly to the pretty version to prevent leakage (or find a way to use
      system temp files)
  * [ ] unit tests for bundle index write
  * [ ] journey test for command-line capabilities
  * [ ] stress test generating an index for the linux kernel pack (uses 64 bit offsets)
* **cli - verify**
   * break progress up into bytes decompressed and compressed bytes read
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
* **prodash**
  * [ ] use usize for progress counting
  * [ ] support custom formatters using boxed 'display' types for units
* **quality**
  * [x] use `parking_lot` to get rid of poisoned locks (and the unwraps() that go with it)
* **progress - convenience**
  * [ ] Unit conversion probably using a new Unit enum (to support more than just bare strings)
    * [ ] Find a  way to measure live throughput
  * [ ] a way to measure throughput on drop
  * [ ] percentages somewhere near the progress bar
* **criner**
  * [ ] switch to `isahc`
    seems to allow async-reading of bodies, allowing to get rid of reqwest and tokio. Redirect is configurable.
* **quick-error** update
  * We need a new release soon with the latest updates to get rid of our git dependency. Until then we can't release.

