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
  * [ ] The linux kernel verifies `--decode` and `--re-encode`
    * [x] parse pgp_signature and mergetag into 'extra-headers' without further parsing
    * [ ] support easy access to merge-tags and the signature
  * add that to the stress test
* **plumbing - explode pack**
  * [ ] write loose object
  * [ ] single threaded
  * [ ] multi-threaded
  * [ ] progress
  * [ ] statistics
* **multi-db** (incorporate object lookup for loose objects and packs)
  * [ ] single threaded
  * [ ] optional object cache
  * [ ] fs-check - verify all object content of a git repository

