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
   * [x] support for LRU or in-memory cache (time lowered from 12min to 9.5min)
   * [x] figure out multi-threading and how to gate it
* **gio**
   * [x] validate pack file
   * [x] validate index and pack
   * **progress**
     * [x] for `lean` binary behind --verbose flag
     * [x] for `pretty` binary with support for logging and TUI
   * [x] statistics
   * [ ] ~~a verbose mode to list each object in a pack, similar to existing git-verify-pack~~
   * [x] journey tests
   * [x] display object throughput per second
   * [x] change exit code depending on verification success or failure
   * [x] progress that allows TUI to remain open for people to see more log messages
   * [x] support for serde/~~miniserde~~ for all returned data types (features per crate)
   * [x] a way to output things as json (as `--format` flag maybe)
* **stress**
  * [x] first stress test for validation of a big repository, linux maybe, or something smaller but big enough
* **unrelated**
  * [x] Use `argh` in cargo-diet to reduce build times
  * [x] Use `argh` in prodash dashboard example to reduce build times

