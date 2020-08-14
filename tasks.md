### Cloning

* **JTT upload-pack-proxy** - obtaining real-world examples for the V1/V2 git protocols
  * a plumbing program to serve as `git-proxy` when invoked like this
    * `PATH="$PWD:$PATH" git -c core.gitproxy="git-proxy" clone git://localhost:9419/small-test`
    * to be used to obtain more real-world samples of typical git interactions for use in the test-suite
  * **git-protocol**
    * [ ] pkt-lines support
    * [ ] basic V1 parsing to understand data frames to allow placing them into individual files
* **a way to intercept git-http communication**
  * Maybe with a custom proxy as well, can't hurt to try APIs in real-world programs
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
