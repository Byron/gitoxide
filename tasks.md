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
* **criner**
  * [ ] switch to `isahc`
    seems to allow async-reading of bodies, allowing to get rid of reqwest and tokio. Redirect is configurable.
* **miniz-oxide**
  * Get [this PR](https://github.com/Frommi/miniz_oxide/pull/91) merged for faster reset performance

[josh-aug-12]: https://github.com/Byron/gitoxide/issues/1#issuecomment-672566602
