### Cloning
* **gixp-pack-receive**
  * _a program to (initially only) clone from a given URL_, storing a full pack with index only. No support for `git-refs`.
  * [x] pkt-lines support
  * [x] Don't forcibly remove newlines at the end of pack lines, but make that an extra step.
  * [x] decode band should be able to fail
  * [x] PacketLine 'Iterator' from `Read`
  * [x] `Read` from packet lines with sideband
    * [x] Progress parsing, with 'info(…)' fallback if that fails
    * [ ] SetName in Progress trait (to allow setting the correct progress information)
  * **connect**
    * [ ] via file and `git-upload-pack`
      * [ ] receive pack with V1 protocol
      * [ ] receive pack with V2 protocol
    * [ ] via https
      * [ ] receive pack with V1 protocol
      * [ ] receive pack with V2 protocol
* **git-refs**
  * [ ] a way to know if a ref needs to be parsed (like `ref/name^{}`)
  * [ ] create ref pointing to ID
      * _assure to keep the path towards symbolic refs open, and allow specifying if these should be followed or not_
  * [ ] **gixp-pack-receive** may optionally write received refs to the specified directory
* **git-repository**
  * [ ] instance for a valid looking repository
    * [ ] support shallow repos/references
  * [ ] create-update refs as received from clone/git-receive-pack
* **gix clone**
  * [ ] try initializing repo on output path - if so, use that to learn about pack location and place new pack there, allow Repo to create refs somehow.
    * _probably this is done using the repository itself, which steers the whole process and injects it's own delegates.
  * [ ] otherwise create the scaffolding needed for a new repository, probably based on `init` implementation

### Fetching _more analysis needed after previous block_

* **git-daemon-proxy** - obtain real-world examples for the V1/V2 git protocols
  * a standalone tool to serve as `git-daemon-proxy`, which intercepts all interactions with it and dumps each one
    into files like N.request and N.response.
  * Differentiate request and response delimiters using timeouts
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
  * [ ] switch to `isahc`
    seems to allow async-reading of bodies, allowing to get rid of reqwest and tokio. Redirect is configurable.
* **miniz-oxide**
  * Wait for release so we can use the new reset capability introduced by [this PR](https://github.com/Frommi/miniz_oxide/pull/91)

[josh-aug-12]: https://github.com/Byron/gitoxide/issues/1#issuecomment-672566602
