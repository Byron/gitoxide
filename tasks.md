### TODO pack decoding + pack exploding
* **parallel**
  * [x] limit parallelism by allowing to set the amount of threads
* **pack**
  * [ ] extract borrowed objects from a pack
  * [ ] support streamed objects (similar to how it's done with loose objects)
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
* **multi-db** (incorporate object lookup for loose objects and packs)
  * [ ] single threaded
  * [ ] optional object cache
  * [ ] fs-check - verify all object content of a git repository

