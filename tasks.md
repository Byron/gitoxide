### TODO pack decoding + pack exploding
* **deployment**
  * produce binaries on tag for all OSs for
    * [ ] pretty + fast
    * [ ] lean + fast
    * [ ] lean + small
* **progress**
  * [ ] one-line progress indicator, maybe implemented in prodash (similar to what `git` does when receiving)
* **initial release**
* **pack**
  * [ ] extract borrowed objects from a pack
  * [ ] support streamed objects (similar to how it's done with loose objects)
* **borrowed objects**
  * [ ] write loose
    * [ ] blob
    * [ ] tag
    * [ ] tree
    * [ ] commit
* **plumbing - explode pack**
  * [ ] single threaded
  * [ ] multi-threaded
  * [ ] progress
  * [ ] statistics
* **multi-db** (incorporate object lookup into)

