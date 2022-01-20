This file sketches out the tasks required to be able to clone a repository whilst checking out its head ref.

### Repository Clone

* **git-odb**
  * [x] all docs, sans examples
  * [x] Rename pack data/pack index `Kind` to `Version` or similar, because that's what it really is.
* **git-object** refactor
  * [x] split `Id` and everything hash related into `git-hash`
  * [x] use `git-hash` inside of `git-features`, remove cycle
* **git-config**
  * [x] Thanks to a generous contribution it's mostly done and well on the way
  * [ ] Push it towards 1.0
  * [ ] `Config` type which integrates multiple files into one interface, much like a *multi* version of `File`
  * [x] Make `gix organize` use `git-config` on single files (the repository configuration)
* **git-ref**
  * [x] transactional creation of refs
  * [x] iteration of refs
* **git-index**
  * [ ] Create an index from tree
* **git-bitmap**
  * [ ] 
* **git-worktree**
  * [ ] checkout an index
* **git-repository**
  * [x] instance for a valid looking repository
    * [ ] support shallow repos/references
  * [ ] create-update refs as received from clone/git-receive-pack safely (i.e. with required locking)
  * [ ] clone from https remote
* **gix repository clone**
  * [ ] try initializing repo on output path - if so, use that to learn about pack location and place new pack there, allow Repo to create refs somehow.
    * _probably this is done using the repository itself, which steers the whole process and injects it's own delegates_.
  * [ ] otherwise create the scaffolding needed for a new repository, probably based on `init` implementation
* **gix pack receive**
  * [x] resolve thin pack with Bundle

### FSCK an entire repository

* **multi-db** (incorporate object lookup for loose objects and packs)
  * [x] multi-threaded
  * [x] delta-tree cache for speedups
  * [ ] ref-validity check by traversing everything, including reflog, checking reachability of objects accordingly
  * [x] fs-check - verify all object content of a git repository
    * probably this should be based on indexed pack traversal for maximum decoding speed and not on individual
      object lookup
   
### Index, worktree and diffing - `git status` like

* [ ] a complete index implementation
* [ ] an understanding on how worktrees work (also consider `git worktree`) in conjunction git-index
* [ ] diffing of worktree -> index -> tree

### Client side push (client to server)

* **git-odb**
  * [x] basic pack generation based on tree-diff or tree-traversal
* [x] **Traversal** (as building blocks to feed pack generation)
  * [x] Traverse a commit graph (look around to see what's common, if in doubt walk back the commit graph and see how to deal with branching)
  * [x] Traverse trees
  * [ ] best-fit delta objects creation using the [similar][sim-crate]

* **git-protocol**
  * [ ] `ReceivePack` logic for V1 and V2
  * [ ] _async_ & _blocking_ negotiation of commits
  
### Server fetch/pull (server to client)

* **git-odb**

The below is a very early draft - it would be better to study existing implementations first to get a better overview on what (not) to do.
This one starts with the fun part to allow writing tests early and experiment with different diff algorithms and potentially their performance.
  
* [x] generate a pack from objects received by an iterator producing (see [issue][pack-mvp])
  * [x] base objects only
  * [x] re-use existing delta objects
  * [x] A mechanism to declare some bases to be 'out of pack' for thin pack support
* [x] **Iterator** to feed pack generation efficiently
* [x] pack creation

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

    
### gix organize

* [ ] Add journey test to cover case with non-bare repository. Try to only read `non-bare` git config files and see the journey test fail.

### gix cat

* A program to cat objects and pretty-print them, similar to git cat-file. Useful to get a feel for
  'locate(…)' performance and stress test it a little.
* Be sure to escape terminal escape codes  
