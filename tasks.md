### TODO pack decoding + pack exploding
* **deployment**
  * port prodash to using crossterm as well
  * produce binaries on tag for all OSs for
    * [x] pretty + fast
    * [x] lean + fast
    * [x] lean + small
  * [x] `--version|-v` option
* **progress**
  * [ ] one-line progress indicator, maybe implemented in prodash (similar to what `git` does when receiving)
    * [ ] in Termion
    * [ ] in crossterm
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
* **multi-db** (incorporate object lookup for loose objects and packs)
  * [ ] single threaded
  * [ ] optional object cache
  * [ ] fs-check - verify all object content of a git repository

