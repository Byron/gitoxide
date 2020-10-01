### Repository Clone

* **git-odb**
  * [x] compound-odb 
    * [x] Write forwarding to loose odb
    * [x] locate object and figure out how to deal with differences of ODBs databases
    * [x] make NLL issue work
    * [x] Nice access to compound::Object
  * [x] Add #![deny(rust_2018_idioms)] everywhere
  * [x] Where 'thiserror' is available, use it for all Errors. It is more powerful, and if we paid for it already, let's use it.
  * [x] alternate DB (location - it's really must following the chain until a compound DB can be created)
    * [x] circular dependencies test
    * [x] basic test
    * [x] multiple alternates in a single file
    * [x] comments
    * [x] quotes 
    * [x] support for relative directories
    * [x] lookup uses alternates
  * [x] loose upgrade: jwalk powered iteration behind a feature flag
  * [ ] full docs
* **git-commitgraph** review
  * [x] adjust tests to disable gpgsignatures
  * [ ] make tests depend on checked-in fixtures, instead of generating them (and depend on git on CI), making it easy to recreate them
* **git-config**
  * A complete implementation, writing a the git remote configuration is needed for finalizing the clone
* **git-ref**
  * [ ] create ref pointing to ID
      * _assure to keep the path towards symbolic refs open, and allow specifying if these should be followed or not_
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
  
### Other

To be picked in any order….

* **prodash**
  * [ ] finish transitioning to futures-lite to get rid of futures-util dependency to reduce compile times
* **criner**
  * [x] upgrade to prodash 9.0
  * [ ] switch to `isahc` or `ureq` (blocking, but could use unblock for that)
    seems to allow async-reading of bodies, allowing to get rid of reqwest and tokio. Redirect is configurable.

[josh-aug-12]: https://github.com/Byron/gitoxide/issues/1#issuecomment-672566602
