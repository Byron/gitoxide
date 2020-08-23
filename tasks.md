### Cloning
* **gixp-pack-receive**
  * _a program to (initially only) clone from a given URL_, storing a full pack with index only. No support for `git-refs`.
  * **git-packetline**
    * [x] pkt-lines support
    * [x] Don't forcibly remove newlines at the end of pack lines, but make that an extra step.
    * [x] decode band should be able to fail
    * [x] PacketLine 'Iterator' from `Read`
    * [x] `Read` from packet lines with sideband
      * [x] Progress parsing, with 'info(…)' fallback if that fails
      * [x] SetName in Progress trait (to allow setting the correct progress information)
      * [x] Disable sideband support (e.g. github http V2 doesn't have it)
    * [x] don't coerce line delimiters into empty slices.
    * [x] Make 'ERR <error>' handling as error optional, as it may occur only in certain contexts.
    * Reader improvements
      * [x] Allow to enable error handling in `Reader`
      * [x] Allow peeking one line
    * [x] `Write` with packet line encoding
  * **git-transport**
    * [x] parse capabilities
    * [ ] git://git-upload-pack
      * [x] V1 handshake
      * [ ] V2 handshake
    * [ ] http://git-upload-pack
      * **prerequisites**
        * [x] pipe to link writers with readers using bytes
        * [x] piped iterator
        * [x] HTTP trait for simple gets and post implemented for Curl
        * [x] propagate HTTP status code
        * [x] non-OK is propagated
        * [ ] test for auto-reset on ReadWithProgress drop
        * [ ] timeout configuration
      * [ ] V1 handshake
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
  
### Notes

* response-end packets of V2 are [not sent in stateless-rpc mode](https://github.com/git/git/blob/master/serve.c#L246:L246) (i.e. http mode) and maybe even V2 daemons
* protocol V2 [is always stateless](https://github.com/git/git/blob/master/builtin/upload-pack.c#L54:L54)
* The negotiation is the meat of the fetch algorithm, and even though not important for clones, the back-and-forth seems very relevant 
  to how the transfer interface should be built. I feel it must be on packet line level.

### Other

To be picked in any order….

* **git-odb**
  * Find errors with `Box<dyn std::error::Error>` fields and see if these could be generic with the use of `thiserror`
    * [ ] [definitely here](https://github.com/Byron/gitoxide/blob/8f8d14f4606e99dc710eb352a985db48c00ea4f4/git-odb/src/pack/index/traverse/mod.rs#L142)
    * [ ] _…keep looking…_
* **prodash**
  * [ ] finish transitioning to futures-lite to get rid of futures-util dependency to reduce compile times
* **criner**
  * [ ] upgrade to prodash 9.0
  * [ ] switch to `isahc`
    seems to allow async-reading of bodies, allowing to get rid of reqwest and tokio. Redirect is configurable.
* **miniz-oxide**

[josh-aug-12]: https://github.com/Byron/gitoxide/issues/1#issuecomment-672566602
