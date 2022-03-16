### git-actor

* [x] read and write a signature that uniquely identifies an actor within a git repository

### git-hash
* types to represent hash digests to identify git objects.
* used to abstract over different kinds of hashes, like SHA1 and the upcoming SHA256
* [x] API documentation
    * [ ] Some examples

### git-chunk
* [ ] decode the chunk file table of contents and provide convenient API
* [ ] write the table of contents

### git-object
* *decode (zero-copy)* borrowed objects
    * [x] commit
      * [x] parse the title, body, and provide a title summary.
      * [ ] parse [trailers](https://git-scm.com/docs/git-interpret-trailers#_description) 
    * [x] tree
* encode owned objects
    * [x] commit
    * [x] tree
    * [x] tag
      * [x] [name validation][tagname-validation]
* [x] transform borrowed to owned objects
* [x] API documentation
    * [ ] Some examples

### git-pack

* **packs**
    * [x] traverse pack index
    * [x] 'object' abstraction
        * [x] decode (zero copy)
        * [x] verify checksum
    * [x] simple and fast pack traversal
        * [ ] [fast pack traversal works with ref-deltas](https://github.com/Byron/gitoxide/blob/8f9a55bb31af32b266d7c53426bc925361a627b2/git-pack/src/cache/delta/from_offsets.rs#L101-L105)
    * [x] decode
        * [x] full objects
        * [x] deltified objects
    * **decode**
        * _decode a pack from `Read` input_
            * [x] Add support for zlib-ng for 20% faster _decompression_ performance
            * [x] `Read` to `Iterator` of entries
                * _read as is, verify hash, and restore partial packs_
        * [x] create index from pack alone (_much faster than git_)
            * [x] resolve 'thin' packs
    * **encode**
        * [x] Add support for zlib-ng for 2.5x _compression_ performance
        * [x] objects to entries iterator
            * [x] input objects as-is
            * [x] pack only changed objects as derived from input
            * [x] base object compression
            * [ ] delta compression
            * [x] create 'thin' pack, i.e. deltas that are based on objects the other side has.
            * [x] parallel implementation that scales perfectly
        * [x] entries to pack data iterator
        * [ ] write index along with the new pack
    * [x] **verify** pack with statistics
        * [x] brute force - less memory
        * [x] indexed - optimal speed, but more memory
    * **advanced**
        * [x] Multi-Pack index file (MIDX)
            * [x] read
            * [x] write 
            * [x] verify
        * [ ] 'bitmap' file
        * [ ] [special handling for networked packs](https://github.com/git/git/blob/89b43f80a514aee58b662ad606e6352e03eaeee4/packfile.c#L949:L949)
        * [ ] [detect and retry packed object reading](https://github.com/git/git/blob/89b43f80a514aee58b662ad606e6352e03eaeee4/packfile.c#L1268:L1268)
* [x] API documentation
    * [ ] Some examples

### git-odb

* **loose object store**
    * [x] traverse
    * [x] read
        * [x] into memory
        * [x] streaming
        * [x] verify checksum
    * [x] streaming write for blobs
    * [x] buffer write for small in-memory objects/non-blobs to bring IO down to open-read-close == 3 syscalls
* **dynamic store**
    * [x] auto-refresh of on-disk state
    * [x] handles alternates
    * [x] multi-pack indices
    * [x] perfect scaling with cores
    * [x] support for pack caches, object caches and MRU for best per-thread performance.
    * [x] prefix/short-id lookup
* **sink**
    * [x] write objects and obtain id
* **alternates**
    * _resolve links between object databases_
    * [x] safe with cycles and recursive configurations
    * [x] multi-line with comments and quotes
* **promisor**
    * It's vague, but these seems to be like index files allowing to fetch objects from a server on demand.
* [x] API documentation
    * [ ] Some examples
    
### git-diff

Check out the [performance discussion][git-diff-performance] as well.

* **tree**
  * [x] changes needed to obtain _other tree_
  * [ ] case-insensitive comparisons  
  * [ ] rename and copy tracking
  * [ ] readily available caching for 4x+ speedups
* **patches**    
  * There are various ways to generate a patch from two blobs.
  * [ ] any
* diffing, merging, working with hunks of data
* find differences between various states, i.e. index, working tree, commit-tree
* Parallel stat calls to check/update objects in index
* [x] API documentation
  * [ ] Examples
    
[git-diff-performance]: https://github.com/Byron/gitoxide/discussions/74

### git-traverse

Check out the [performance discussion][git-traverse-performance] as well.

* **trees**
  * [x] nested traversal
* **commits**
  * [x] ancestor graph traversal similar to `git revlog`
* [x] API documentation
    * [ ] Examples
    
[git-traverse-performance]: https://github.com/Byron/gitoxide/discussions/76

* **tree**

### git-url
* As documented here: https://www.git-scm.com/docs/git-clone#_git_urls
* **parse**
    * [x] ssh URLs and SCP like syntax
    * [x] file, git, and SSH
    * [x] paths (OS paths, without need for UTF-8)
* [x] username expansion for ssh and git urls
* [x] convert URL to string
* [x] API documentation
    * [ ] Some examples

### git-protocol
* _abstract over protocol versions to allow delegates to deal only with a single way of doing things_
* [x] **credentials**
    * [x] via git-credentials
    * [ ] via pure Rust implementation if no git is installed
* [x] fetch & clone
    * [x] detailed progress
    * [x] control credentials provider to fill, approve and reject
    * [x] command: ls-ref
        * [x] parse V1 refs as provided during handshake
        * [x] parse V2 refs
        * [ ] handle empty refs, AKA PKT-LINE(zero-id SP "capabilities^{}" NUL capability-list)
    * [x] initialize and validate command arguments and features sanely
    * [x] abort early for ls-remote capabilities
    * [x] packfile negotiation
        * [x] delegate can support for all fetch features, including shallow, deepen, etc.
        * [x] receive parsed shallow refs
* [ ] push
* [x] API documentation
    * [ ] Some examples

### git-packetline
* [PKT-Line](https://github.com/git/git/blob/master/Documentation/technical/protocol-common.txt#L52:L52)
* [x] encode
* [x] decode (zero-copy)
* [x] [error line](https://github.com/git/git/blob/master/Documentation/technical/pack-protocol.txt#L28:L28)
* [x] [V2 additions](https://github.com/git/git/blob/master/Documentation/technical/protocol-v2.txt#L35:L36)
* [x] [side-band mode](https://github.com/git/git/blob/master/Documentation/technical/pack-protocol.txt#L467:L467)
* [x] `Read` from packet line with (optional) progress support via sidebands
* [x] `Write` with built-in packet line encoding
* [x] API documentation
    * [ ] Some examples

### git-transport
* No matter what we do here, timeouts must be supported to prevent hanging forever and to make interrupts destructor-safe.
* **client**
    * [x] general purpose `connect(…)` for clients
        * [x] _file://_ launches service application
        * [x] _ssh://_ launches service application in a remote shell using _ssh_
        * [x] _git://_ establishes a tcp connection to a git daemon
        * [x] _http(s)://_ establishes connections to web server
        * [ ] pass context for scheme specific configuration, like timeouts
    * [x] git://<service>
        * [x] V1 handshake
            * [x] send values + receive data with sidebands
            * [ ] ~~support for receiving 'shallow' refs in case the remote repository is shallow itself (I presume)~~
                * Since V2 doesn't seem to support that, let's skip this until there is an actual need. No completionist :D
        * [x] V2 handshake
            * [x] send command request, receive response with sideband support
    * [x] http(s)://<service>
        * [x] set identity for basic authentication
        * [x] V1 handshake
            * [x] send values + receive data with sidebands
        * [x] V2 handshake
            * [x] send command request, receive response with sideband support
        * [ ] ~~'dumb'~~ - _we opt out using this protocol seems too slow to be useful, unless it downloads entire packs for clones?_
    * [x] authentication failures are communicated by io::ErrorKind::PermissionDenied, allowing other layers to retry with authentication
* **server**
    * [ ] general purpose `accept(…)` for servers
* [x] API documentation
    * [ ] Some examples

### git-worktree
* handle the working tree/checkout
  - [x] checkout an index of files, executables and symlinks just as fast as git
     - [x] forbid symlinks in directories
     - [ ] handle submodules
     - [ ] handle sparse directories
     - [ ] handle sparse index
     - [ ] linear scaling with multi-threading up to IO saturation
* manage multiple worktrees
* deal with exclude specifications, like .gitignore and other exclude files.
 
### git-revision
* find specifications for revisions (like `git name-rev`)
* parse specifications into revisions (like `git rev-parse`)
 
### git-submodule

* CRUD for submodules
* try to handle with all the nifty interactions and be a little more comfortable than what git offers, lay a foundation for smarter git submodules.

### git-bitmap

A plumbing crate with shared functionality regarding EWAH compressed bitmaps, as well as other kinds of bitmap implementations.

* **EWAH**
  * `Array` type to read and write bits
     * [x] execute closure for each `true` bit
  * [x] decode on-disk representation
  * [ ] encode on-disk representation

### git-index

The git staging area.

* read 
  * [x] V2 - the default, including long-paths support
  * [x] V3 - extended flags
  * [x] V4 - delta-compression for paths
  * optional threading
    * [x] concurrent loading of index extensions
    * [x] threaded entry reading
  * extensions
    * [x] TREE for speeding up tree generation
    * [x] REUC resolving undo
    * [x] UNTR untracked cache
    * [x] FSMN file system monitor cache V1 and V2
    * [x] 'link' base indices to take information from, split index
    * [x] 'sdir' sparse directory entries - marker
  * [x] verification of entries and extensions as well as checksum
* `stat` update
    * [ ] optional threaded `stat` based on thread_cost (aka preload)
* [ ] handling of `.gitignore` and system file exclude configuration
* [ ] handle potential races
* maintain extensions when altering the cache
    * [ ] TREE for speeding up tree generation
    * [ ] REUC resolving undo
    * [ ] UNTR untracked cache
    * [ ] FSMN file system monitor cache V1 and V2
    * [ ] EOIE end of index entry
    * [ ] IEOT index entry offset table
    * [ ] 'link' base indices to take information from, split index
    * [ ] 'sdir' sparse directory entries
* additional support
    * [ ] non-sparse
    * [ ] sparse (search for [`sparse index` here](https://github.blog/2021-08-16-highlights-from-git-2-33/))
* add and remove entries
* [x] API documentation
    * [ ] Some examples

### git-commitgraph
* [x] read-only access
    * [x] Graph lookup of commit information to obtain timestamps, generation and parents, and extra edges
    * [ ] Bloom filter index
    * [ ] Bloom filter data
* [ ] create and update graphs and graph files
* [x] API documentation
    * [ ] Some examples
    
### git-tempfile

See its [README.md](https://github.com/Byron/gitoxide/blob/main/git-tempfile/README.md).

### git-lock

See its [README.md](https://github.com/Byron/gitoxide/blob/main/git-lock/README.md).

### git-config
* [ ] read
    * line-wise parsing with decent error messages
    * [x] decode value
        * [x] boolean
        * [x] integer
        * [x] color
        * [x] path (incl. resolution)
        * [ ] include
        * [ ] includeIf
* [x] write
    * keep comments and whitespace, and only change lines that are affected by actual changes, to allow truly non-destructive editing
* [ ] `Config` type which integrates multiple files into one interface to support system, user and repository levels for config files
* [x] API documentation
    * [x] Some examples

### git-repository
* [x] utilities for applications to make long running operations interruptible gracefully and to support timeouts in servers.
* [ ] handle `core.repositoryFormatVersion` and extensions
* [x] discovery
  * [ ] option to not cross file systems
  * [ ] handle git-common-dir
* **plumbing**
  * **Repository**  (_plumbing_)
    * [x] discovery
    * [ ] handle other non-discovery modes and provide control over environment variable usage required in applications
    * [x] instantiation
    * [ ] a way to handle `.git` files with `gitdir: <path>` in it
    * [ ] handle `gitdir` and `commondir` files
  * [x] access to refs and objects
  * traverse 
      * [x] commit graphs
      * [ ] make [git-notes](https://git-scm.com/docs/git-notes) accessible
      * [x] tree entries
  * diffs/changes
     * [x] tree with tree
     * [ ] tree with index
     * [ ] index with working tree
  * [x] initialize
      * [ ] Proper configuration depending on platform (e.g. ignorecase, filemode, …)
  * [ ] All mutations are multi-process safe and this is tested and configurable (i.e. abort or wait if lock is encountered)
* support for unicode-precomposition of command-line arguments (needs explicit use in parent application)
* **Easy** (_porcelain_)
  * **Id**
    * [x] short hashes with detection of ambiguity.
       * [ ] solve the [birthday paradox](https://git-scm.com/docs/git-describe#_examples) and make default hex-len depend on the amount of objects.
  * **objects**
    * [x] lookup
    * [x] peel to object kind
    * **trees**
      * [x] lookup path
  * **references**
      * [x] peel to end
  * [ ] [Signed commits and tags](https://github.com/Byron/gitoxide/issues/12)
  * [ ] clone
      * [ ] shallow
      * [ ] namespaces support
  * [ ] sparse checkout support
  * [ ] `git describe` like functionality
  * [ ] execute hooks
  * [ ] .gitignore handling
  * [ ] checkout/stage conversions clean + smudge as in .gitattributes
  * [ ] rev-parsing and ref history
  * **refs**
    * [ ] run transaction hooks and handle special repository states like quarantine
    * [ ] support for different backends like `files` and `reftable`
  * [ ] worktrees
  * [ ] remotes with push and pull
  * [ ] mailmap   
  * [ ] configuration
  * [ ] merging
  * [ ] stashing
  * [ ] Use _Commit Graph_ to speed up certain queries
* subtree
* submodules
* [ ] API documentation
    * [ ] Some examples

### git-bundle
* [ ] create a bundle from an archive
* [ ] extract a branch from a bundle into a repository
* [ ] API documentation
    * [ ] Some examples

### git-validate
* [x] validate ref names
* [x] [validate][tagname-validation] tag names

### git-ref
* [ ] Prepare code for arrival of longer hashes like Sha256. It's part of the [V2 proposal][reftable-v2] but should work for loose refs as well.
* [ ] **revparse** - obtain an object ID from short or long hashes, reference names or reference log [or more][revparse].
* **Stores**
  * [ ] disable transactions during [quarantine]
  * [x] namespaces
    * a server-side feature to transparently isolate refs in a single shared repository, allowing all forks to live in the same condensed repository.
  * **loose file**
    * [x] ref validation
    * [x] find single ref by name
    * [ ] special handling of `FETCH_HEAD` and `MERGE_HEAD`
    * [x] iterate refs with optional prefix
    * [ ] [worktree support]
    * ~~symbolic ref support, using symbolic links~~
        * This is a legacy feature which is not in use anymore.
    * **transactions** 
      * [x] delete, create or update single ref or multiple refs while handling the _reflog_
      * [x] set any valid ref value (not just object ids)
      * [x] reflog changes can be entirely disabled (i.e. for bare repos)
      * [ ] rename or copy references
      * [x] transparent handling of packed-refs during deletion
      * [x] writing loose refs into packed-refs and optionally delete them
      * [ ] initial transaction optimization (a faster way to create clones with a lot of refs)
    * **log**
      * [x] forward iteration
      * [x] backward iteration
      * [ ] expire
    * **ref**
      * [x] peel to id
    * **packed**
      * [x] find single ref by name
      * [x] iterate refs with optional prefix
      * [x] handle unsorted packed refs and those without a header
  * [ ] **[reftable][reftable-spec]**, 
    * see [here for a Go/C implementation][reftable-impl]
* [x] API documentation
    * [ ] Some examples

[revparse]: https://git-scm.com/docs/git-rev-parse#Documentation/git-rev-parse.txt-emem
[reftable-spec]: https://github.com/eclipse/jgit/blob/master/Documentation/technical/reftable.md
[reftable-impl]: https://github.com/google/reftable
[reftable-v2]: https://github.com/google/reftable/blob/master/reftable-v2-proposal.md
[quarantine]: https://github.com/git/git/blob/master/Documentation/git-receive-pack.txt#L223:L223
[worktree support]: https://github.com/git/git/blob/master/refs/files-backend.c#L163:L182


### git-features
* **io-pipe** feature toggle
    * a unix like pipeline for bytes
* **parallel** feature toggle
    * _When on…_
        * `in_parallel`
        * `join`
    * _When off all functions execute serially_
* **fast-sha1**
    * provides a faster SHA1 implementation using CPU intrinsics
* [x] API documentation

### git-tui
* _a terminal user interface seeking to replace and improve on `tig`_
* Can display complex history in novel ways to make them graspable. Maybe [this post] can be an inspiration.

[tagname-validation]: https://github.com/git/git/blob/master/Documentation/technical/protocol-common.txt#L23:L23
[this post]: http://blog.danieljanus.pl/2021/07/01/commit-groups/
