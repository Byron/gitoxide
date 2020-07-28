### TODO pack decoding + pack exploding
* **parallel**
  * [x] limit parallelism by allowing to set the amount of threads
* **pack**
  * [x] extract borrowed objects from a pack
  * [ ] ~~support streamed objects (similar to how it's done with loose objects)~~ - no need, all slices support io::Read, and we don't
        actually support streaming, so let's net unify 'interfaces' on a low level like this.
* **owned objects**
  * [x] encode object
    * [x] blob
    * [x] tag
    * [x] tree
    * [x] commit
* **pack verify**
  * [x] add '--some-flag' to run every non-blob through a decode/encode cycle to see if all objects can be parsed
  * [x] The Linux Kernel verifies `--decode` and `--re-encode`
    * [x] parse pgp_signature and mergetag into 'extra-headers' without further parsing
  * [x] The Rust repo verifies `--decode` and `--re-encode`
  * [x] add re-encode to all stress tests
  * [x] support easy access to merge-tags and the signature
* **stream pack for pack-verify**
  * [x] support for multi-threading
  * [x] choice of algorithm in pack-verify
  * [x] use pack streaming in pack-verify by default
  * [x] set some unit tests cases to use the streaming algorithm
* **progress - convenience**
  * [x] `inc()` method and`inc_by(step)` method
* **plumbing - explode pack**
  * _useful for receiving packs that are small enough to warrant living as loose objects for a while
    preventing pack buildup_
  * **write loose object to…**
     * [x] sink
     * [x] Deflate stream
     * [x] disk - with decent errors
     * [x] size as u64 (properly)
  * **cli**
     * [x] generalize pack reading algorithm
     * [x] write objects from pack (multi-threaded comes for free)
        * [x] to sink
        * [x] to disk
     * [x] progress
     * [x] option to compress sink input too
     * [ ] unrelated: see if delta-decode buffer optimization can work easily
     * [ ] --verify
     * [ ] statistics
     
### Cloning

* **index-from-pack**
  * _capability to receive packs from a stream_
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
* **progress - convenience**
  * [ ] a way to measure throughput on drop
  * [ ] percentages somewhere near the progress bar
* **criner**
  * [ ] switch to `isahc`
    seems to allow async-reading of bodies, allowing to get rid of reqwest and tokio. Redirect is configurable.
* **quick-error** update
  * We need a new release soon with the latest updates to get rid of our git dependency. Until then we can't release.

