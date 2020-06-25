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
   * [ ] validate index checksum
   * [ ] validate pack checksum
   * [ ] validate each object sha1/crc2
* **gio**
   * [ ] validate pack file
   * [ ] validate index and pack
   * [ ] a verbose mode to list each object in a pack, similar to existing git-verify-pack

