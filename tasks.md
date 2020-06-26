### TODO

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
   * [ ] validate each object sha1/crc2
   * [ ] support for LRU or in-memory cache
   * [ ] figure out multi-threading and how to gate it (and let others control it)
* **gio**
   * [x] validate pack file
   * [ ] validate index and pack
   * [ ] a verbose mode to list each object in a pack, similar to existing git-verify-pack

