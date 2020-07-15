### TODO pack decoding + pack exploding
* **parallel**
  * [x] limit parallelism by allowing to set the amount of threads
* **pack**
  * [x] extract borrowed objects from a pack
  * [ ] ~~support streamed objects (similar to how it's done with loose objects)~~ - no need, all slices support io::Read, and we don't
        actually support streaming, so let's net unify 'interfaces' on a low level like this.
* **borrowed objects**
  * [ ] write loose
    * [ ] blob
    * [ ] tag
    * [ ] tree
    * [ ] commit
* **plumbing - explode pack**
  * [ ] single threaded
  * [ ] multi-threaded
  * [ ] progress
  * [ ] statistics
* **pack verify**
  * add '--some-flag' to run every non-blob through a decode/encode cycle to see if all objects can be parsed
  * add that to the stress test
* **multi-db** (incorporate object lookup for loose objects and packs)
  * [ ] single threaded
  * [ ] optional object cache
  * [ ] fs-check - verify all object content of a git repository

