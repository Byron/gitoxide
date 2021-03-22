### git-hash
* types to represent hash digests to identify git objects.
* used to abstract over different kinds of hashes, like SHA1 and the upcoming SHA256
* [x] API documentation
    * [ ] Some examples

### git-object
* *decode (zero-copy)* borrowed objects
    * [x] commit
    * [x] tree
    * [x] tag
* encode owned objects
    * [x] commit
    * [x] tree
    * [x] tag
* [x] transform borrowed to owned objects
* [x] API documentation
    * [ ] Some examples

### git-odb
* **loose objects**
    * [x] traverse
    * [x] read
        * [x] into memory
        * [x] streaming
        * [x] verify checksum
    * [x] streaming write for blobs
    * [x] buffer write for small in-memory objects/non-blobs to bring IO down to open-read-close == 3 syscalls
* **packs**
    * [x] traverse pack index
    * [x] 'object' abstraction
        * [x] decode (zero copy)
        * [x] verify checksum
    * [x] simple and fast pack traversal
    * [x] decode
        * [x] full objects
        * [x] deltified objects
    * **streaming**
        * _decode a pack from `Read` input_
        * [x] `Read` to `Iterator` of entries
            * _read as is, verify hash, and restore partial packs_
        * [x] create index from pack alone (_much faster than git_)
            * [ ] resolve 'thin' packs
    * [ ] encode
        * [ ] Add support for zlib-ng for 2.5x compression performance and 20% faster decompression
        * [ ] create new pack
        * [ ] create 'thin' pack
    * [x] verify pack with statistics
        * [x] brute force - less memory
        * [x] indexed - faster, but more memory
    * **advanced**
        * [ ] Multi-Pack index file (MIDX)
        * [ ] 'bitmap' file
* [x] API documentation
    * [ ] Some examples
* **sink**
    * [x] write objects and obtain id
* **alternates**
    * _database that act as link to other known git ODBs on disk_
    * [x] safe with cycles and recursive configurations
    * [x] multi-line with comments and quotes
* **multi-odb**
    * [ ] _an ODB for object lookup from multiple lower level ODB at once_
* **promisor**
    * It's vague, but these seems to be like index files allowing to fetch objects from a server on demand.

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

### git-index
* read and write a git-index file
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

### git-config
* [ ] read
    * line-wise parsing with decent error messages
    * [x] decode value
        * [x] boolean
        * [x] integer
        * [x] color
        * [ ] path (incl. resolution)
        * [ ] include
        * [ ] includeIf
* [x] write
    * keep comments and whitespace, and only change lines that are affected by actual changes, to allow truly non-destructive editing
* [ ] `Config` type which integrates multiple files into one interface to support system, user and repository levels for config files
* [x] API documentation
    * [x] Some examples

### git-repository
* [x] initialize
    * [ ] Proper configuration depending on platform (e.g. ignorecase, filemode, …)
* [ ] [Signed commits and tags](https://github.com/Byron/gitoxide/issues/12)
* [ ] clone
    * [ ] shallow
    * [ ] namespaces support
* [ ] sparse checkout support
* [ ] execute hooks
* [ ] .gitignore handling
* [ ] checkout/stage conversions clean + smudge as in .gitattributes
* [ ] rev-parsing and ref history
* [ ] worktree
* [ ] remotes with push and pull
* [ ] configuration
* [ ] merging
* [ ] stashing
* [ ] Use _Commit Graph_ to speed up certain queries
* [ ] API documentation
    * [ ] Some examples

### git-bundle
* [ ] create a bundle from an archive
* [ ] extract a branch from a bundle into a repository
* [ ] API documentation
    * [ ] Some examples

### git-ref
* Handle symbolic references and packed references
* discover them in typical folder structures
* [x] [name validation](https://github.com/git/git/blob/master/Documentation/technical/protocol-common.txt#L23:L23)
* [x] API documentation
    * [ ] Some examples

### git-diff / git-status
* diffing of git-object::Tree structures
* diffing, merging, working with hunks of data
* find differences between various states, i.e. index, working tree, commit-tree
* Parallel stat calls to check/update objects in index
* [ ] API documentation with examples

### git-features
* **interrupt-handler** feature toggle
    * Interruption for computations when receiving SIGTERM and SIGINT
    * can be entirely didsabled with the **disable-interrupts** feature toggle
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
