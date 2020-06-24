### TODO

* [x] decode commit
* [x] decode tree
* [x] represent blob
* [x] stream blob
* [ ] refactor ID into NewType
* **pack**
   * [ ] validate checksum
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
   * [ ] verbose mode to list each object in pack, similar to existing git-verify-pack

