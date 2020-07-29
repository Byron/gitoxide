### Pack streaming and restoring

* **index-from-pack**
  * _capability to receive packs from a stream_
  * [x] stream pack with iterator interface
  * [ ] build index from pack
  * [ ] repair pack - write trailer for all complete objects
* **cli**
  * [ ] build index from pack
  * [ ] complete pack with trailer
  
### Cloning

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

