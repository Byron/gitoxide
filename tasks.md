### Repository Clone

* **git-odb**
  * [x] all docs, sans examples
  * [x] Rename pack data/pack index `Kind` to `Version` or similar, because that's what it really is.
* **Documentation (with deny(missing_docs))**
  * [x] git-features
  * [ ] git-object
  * [ ] git-url
  * [ ] git-ref
  * [ ] git-packetline
  * [ ] git-protocol
  * [ ] git-transport
  * [ ] git-commitgraph
* **git-config**
  * A complete implementation, writing a the git remote configuration is needed for finalizing the clone
  * [ ] `Config` type which integrates multiple files into one interface, much like a *multi* version of `File`
* **git-ref**
  * [ ] create ref pointing to ID
      * _assure to keep the path towards symbolic refs open, and allow specifying if these should be followed or not_
* **git-index**
  * [ ] Create an index from tree
  * [ ] Checkout index to worktree
* **git-repository**
  * [ ] instance for a valid looking repository
    * [ ] support shallow repos/references
  * [ ] create-update refs as received from clone/git-receive-pack
* **gix clone**
  * [ ] try initializing repo on output path - if so, use that to learn about pack location and place new pack there, allow Repo to create refs somehow.
    * _probably this is done using the repository itself, which steers the whole process and injects it's own delegates.
  * [ ] otherwise create the scaffolding needed for a new repository, probably based on `init` implementation
* **receive pack**
  * [ ] resolve thin pack with Bundle
* **git-repository**
  * [ ] clone from https remote
  
### FSCK an entire repository

* **multi-db** (incorporate object lookup for loose objects and packs)
  * [ ] single threaded
  * [ ] optional object cache
  * [ ] fs-check - verify all object content of a git repository
  
### Commit-Graph

* [x] A plumbing command to extract some value from the current implementation, maybe statistics, or verification
* [ ] Application of the command above in a stress test

* **Questions**
  * What to do with the ['extra-garbage'](https://github.com/Byron/gitoxide/blob/6f90beeb418480f9cd8bb7ae3b5db678b24103cb/git-commitgraph/src/file/init.rs#L248), 
    some code is commented out.
  
### Other

To be picked in any order….

* **prodash**
  * [ ] finish transitioning to futures-lite to get rid of futures-util dependency to reduce compile times
* **criner**
  * [x] upgrade to prodash 11.0
  * [ ] switch to `isahc` or `ureq` (blocking, but could use unblock for that)
    seems to allow async-reading of bodies, allowing to get rid of reqwest and tokio. Redirect is configurable.

[josh-aug-12]: https://github.com/Byron/gitoxide/issues/1#issuecomment-672566602
