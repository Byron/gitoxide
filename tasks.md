This file sketches out the tasks required to be able to clone a repository whilst checking out its head ref.

### Client side push (client to server)

* **git-odb**
  * [x] basic pack generation based on tree-diff or tree-traversal

* **git-protocol**
  * [ ] `ReceivePack` logic for V1 and V2
  * [ ] _async_ & _blocking_ negotiation of commits
  
### Client fetch/pull (server to client)

* **git-odb**

The below is a very early draft - it would be better to study existing implementations first to get a better overview on what (not) to do.
This one starts with the fun part to allow writing tests early and experiment with different diff algorithms and potentially their performance.
  
* [x] generate a pack from objects received by an iterator producing (see [issue][pack-mvp])
  * [x] base objects only
  * [ ] re-use existing delta objects
  * [ ] best-fit delta objects using the [similar][sim-crate]
  * [ ] A mechanism to declare some bases to be 'out of pack' for thin pack support
* [x] **Traversal** (as building blocks to feed pack generation)
  * [x] Traverse a commit graph (look around to see what's common, if in doubt walk back the commit graph and see how to deal with branching)
  * [x] Traverse trees
* [x] **Iterator** to feed pack generation efficiently

[pack-mvp]: https://github.com/Byron/gitoxide/issues/67

* **git-transport**

Certainly needs more research, but roughly…
  
* [ ] Server side `accept()`
  * [ ] http(s)
  * [ ] ssh
  * [ ] ~~daemon~~ probaby only used in testing, and we might implement it if it's useful for us as well
  
* **git-protocol**
  
  * [ ] Server side chatter to negotiate a pack for
    * [ ] protocol V2
    * [ ] protocol V1 (_probably not worth it, let's see_)
  
* **gix-serve**

Probably more like a toy at first merely for testing operation against various git clients.
  
  * [ ] A server able to answer via
    * [ ] http(s)
    * [ ] file protocol (or remote invocation via SSH)
  
[sim-crate]: https://crates.io/crates/similar

### Repository Clone

* **git-odb**
  * [x] all docs, sans examples
  * [x] Rename pack data/pack index `Kind` to `Version` or similar, because that's what it really is.
* **git-object** refactor
  * [x] split `Id` and everything hash related into `git-id`
  * [x] use `git-id` inside of `git-features`, remove cycle
* **git-config**
  * [x] Thanks to a generous contribution it's mostly done and well on the way
  * [ ] Push it towards 1.0
  * [ ] `Config` type which integrates multiple files into one interface, much like a *multi* version of `File`
  * [x] Make `gix organize` use `git-config` on single files (the repository configuration)
* **git-ref**
  * [ ] create ref pointing to ID
      * _assure to keep the path towards symbolic refs open, and allow specifying if these should be followed or not_
* **git-index**
  * [ ] Create an index from tree
  * [ ] Checkout index to worktree
* **git-repository**
  * [ ] instance for a valid looking repository
    * [ ] support shallow repos/references
  * [ ] create-update refs as received from clone/git-receive-pack safely (i.e. with required locking)
  * [ ] clone from https remote
* **gix clone**
  * [ ] try initializing repo on output path - if so, use that to learn about pack location and place new pack there, allow Repo to create refs somehow.
    * _probably this is done using the repository itself, which steers the whole process and injects it's own delegates_.
  * [ ] otherwise create the scaffolding needed for a new repository, probably based on `init` implementation
* **gixp receive-pack**
  * [ ] resolve thin pack with Bundle

### FSCK an entire repository

* **multi-db** (incorporate object lookup for loose objects and packs)
  * [x] ~~single~~multi-threaded
  * [x] ~~object~~pack-cache for speedups
  * [ ] fs-check - verify all object content of a git repository 
     * probably this should be based on indexed pack traversal for maximum decoding speed and not on individual
       object lookup
    
### gix organize

* [ ] Add journey test to cover case with non-bare repository. Try to only read `non-bare` git config files and see the journey test fail.

### gixp cat

* A program to cat objects and pretty-print them, similar to git cat-file. Useful to get a feel for
  'locate(…)' performance and stress test it a little.
* Be sure to escape terminal escape codes  

### Feature Flags

* [ ] configure the `small` feature set so that the flate2 backend is miniz-oxide instead of zlib-ng, allowing a 'pure' rust build. 
      This might mean that the `fast` feature contains zlib-ng.

* **Questions**
  * What to do with the ['extra-garbage'](https://github.com/Byron/gitoxide/blob/6f90beeb418480f9cd8bb7ae3b5db678b24103cb/git-commitgraph/src/file/init.rs#L248),
    some code is commented out.
