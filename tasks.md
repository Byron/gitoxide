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
  * [ ] seriously refactor the header abstraction in the pack file - it's not right at all.
     * pack_offset -> offset_distance_to_base. That way it can be encoded/decoded without further knowledge.
     * RefDelta { oid } -> id
     * data_offset becomes method, the field becomes the header size
     * to_write should be on entry
  * [ ] stress test generating an index for the linux kernel pack (uses 64 bit offsets)
* **cli _index-from-pack_ **
  * [ ] build index from pack
  * [ ] complete pack with trailer
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
* **quality**
  * use `parking_lot` to get rid of poisoned locks (and the unwraps() that go with it)
* **progress - convenience**
  * [ ] Alternate unit - might help doing throughput and things like throughput per second (for downloads)
  * [ ] a way to measure throughput on drop
  * [ ] percentages somewhere near the progress bar
* **criner**
  * [ ] switch to `isahc`
    seems to allow async-reading of bodies, allowing to get rid of reqwest and tokio. Redirect is configurable.
* **quick-error** update
  * We need a new release soon with the latest updates to get rid of our git dependency. Until then we can't release.

