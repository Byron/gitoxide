### TODO pack validation

* [x] decode commit
* [x] decode tree
* [x] represent blob
* [x] stream blob
* [x] refactor ID into NewType
* **sha1**
   * [x] minimal dependencies
   * [x] maximum performance
* **pack**
   * [x] validate checksum
   * [x] decode full pack entries
   * [x] decode full objects
   * [x] decode delta objects
* **index**
   * [x] validate index checksum
   * [x] validate pack checksum
   * [x] validate each object sha1
   * [x] validate each object crc32
   * [x] lookup by full oid to resolve [this](https://github.com/Byron/git-oxide/blob/053045bb23e2a85e2a1d16eeb65c399dfabba5b4/git-odb/tests/pack/index.rs#L27)
   * [ ] support for LRU or in-memory cache
   * [ ] figure out multi-threading and how to gate it (and let others control it)
* **gio**
   * [x] validate pack file
   * [ ] validate index and pack
   * [ ] a verbose mode to list each object in a pack, similar to existing git-verify-pack
* **stress**
  * [ ] first stress test for validation of a big repository, linux maybe, or something smaller but big enough

