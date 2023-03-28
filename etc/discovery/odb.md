# Discovery: data consistency and resource usage

## Summary

Concurrent writes to packs observable by applications pose a challenge to the current implementation and we need to find ways around that. There may be other limitations
related to any resource that changes often, most prominently refs.

## Motivation

Use this document to shed light on the entire problem space surrounding data consistency and resource usage of packed objects to aid in finding solutions
that are best for various use cases without committing to high costs in one case or another.

## Git as database

Databases are well understood and there is common semantics they typically adhere to. The following is an exploration of the various kinds of databases that make up a git
repository along with some of their interplay.

### Object Databases

In repositories there may be millions to 10th of millions of objects like commits, blobs, trees and tags, and accessing them quickly is relevant to many of git's features.

#### Loose object database

Each object is stored in a single file on disk, partitions by the first byte of its content hash. All implementations handle it similarly.

#### Packed object database

Packs are single files containing one or more objects. Delta-compression allows them to be very size-efficient, e.g. 90x compression for the linux kernel.

Packs and the object database is inherently append-only, i.e. objects are never *[0] deleted, allowing concurrent readers to observe a consistent state even in presence
of writers. Writers create new pack files and may remove them after adding all changes successfully, without the need for locking.

`gitoxide`s implementation as of 2021-11-19 is known to be unsuitable in the presence of concurrent writes to packs due to its inability to automatically respond
to changed packs on disk if objects cannot be found on the first attempt. Other implementations like `libgit2` and canonical `git` handle this by using
thread-safe interior mutability at the cost of scalability.

`gitoxide`s implementation may be insufficient in that regard, but it shows how read-only data allows to scale across core as well as the CPU architecture allows.

The usage of system resources like file handles is simple but potentially wasteful as all packs are memory-mapped in full immediately. Lazy and partial memory-mapping
of packs is used in other implementations. Laziness allows for more efficiency and partial mapping allows to handle big packs on 32 bit systems.

Failure to acquire a memory map due to limits in the amount of open file handles results in an error when initializing the pack database in the `gitoxide`s implementation.
To my knowledge, this is handled similarly in other implementations. All implementations assume there is unlimited memory, but the effect of running out of memory is only
known to me in case of `gitoxide` which will panic.


[0] deletion is possible but doesn't happen instantly, instead requiring time to pass and calls to git-maintenance and for them to be unreachable, i.e. not used in the
entire repository.

### Reference databases

References are pointers to objects or other references and are crucial to `git` as they form the entry points in all git operations. If objects aren't reachable by starting
graph traversal at a reference (or its historical reflog information) it is effectively lost forever, i.e. unreachable.

They may be just a couple to hundreds of thousands of references in a repository which are changed right after new objects are added to the object database.

#### Loose reference database

References are stored one at a time in files, one reference at a time or multiple ones in a single well known file, `packed-refs`.
`packed-refs` is updated during maintenance to keep keep direct references only.

Multiple references can change at the same time, but multiple changes aren't atomic as changes are made a file at a time. All implementations may observe intermediate states
where some but not all references are updated.

`packed-refs` may change during maintenance or upon deletion of references. All implementations cache the `packed-refs` file but check for a stale cache (i.e. see if file on disk
changed in the mean time) before each use of the cached data.

The database read, i.e. accessing individual reference values or iterating references,
performance is heavily limited by disk-IO when accessing loose files. Handling og `packed-refs` is speedy even in the presence of hundreds of thousands
of references due to optimizations performed in all implementations.

The reference update/add performance is parallel per reference as long as the set of writers don't overlap in references to change, but bound by disk-IO,
due to writes happening one file at a time. `packed-refs` is not changed, but typically read to validate write constraints that help with atomicity,
i.e. only change a value if it matches the previously observed one.

Deletions are slow and a worst-case scenario as not only the loose reference(s) will be deleted but potentially the `packed-refs` file updated if
it contained the (possibly only) copy/ies. An update implies rewriting `packed-refs` file entirely. During that time it is locked, blocking or failing other writers, forming
a choking point.

`gitoxide`s implementation keeps one `packed-refs` cache handle to the underlying repository, which costs a file handle for a memory map if the `packed-refs` file is larger than
32kB, theoretically configurable but currently hardcoded based on the default in `git`.
Other implementations maintain one per repository instance (libgit2) or one per process (git).

Even the biggest transactions will only additionally open one loose reference file at a time, and close it right after use.


| **Operation**   | **read loose file**                 | **locks**             | **costs**                                                                                                                                                                                                                                                                        | **read packed-refs**                                                        | **concurrency granularity**     | **Limit/Bottleneck**                         |
|-----------------|-------------------------------------|-----------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------|---------------------------------|----------------------------------------------|
| **Add/Update**  | only if comparing to previous value | loose ref             | .lock file per reference; write new value; move into place; create intermediate directories as needed; delete empty directories in the way as needed; read packed-refs; possibly append to reflog                                                                                | only if comparing to previous value and loose reference file isn't present. | per reference                   | disk-IO                                      |
| **Read**        | always                              |                       | read loose file; read packed-refs                                                                                                                                                                                                                                                | if loose file didn't exist                                                  | n/a                             | disk-IO                                      |
| **Delete**      | only if asserting previous value   | loose ref; packed-ref | .lock file per reference; delete loose file; delete lock file; .lock for packed-refs, rewrite packed-refs.lock; move packed-refs.lock into place; possibly delete reflog per ref                                                                                                 | always                                                                      | per reference (and packed-refs) | disk-IO (and CPU if packed-refs is involved) |
| **maintenance** | always all (or by filter)           | loose ref; packed-ref | .lock file per reference, read loose reference; .lock for packed-refs; read entire packed-refs; insert loose reference values; write entire altered packed-refs into packed-refs.lock; move packed-refs.lock into place; delete existing loose references and delete their .lock | always                                                                      | per reference and packed-refs   | disk-IO and CPU                              |

Failures to add/update/delete may occur if the constraint isn't met. It's possible to wait in the presence of a lock file instead of failing immediately,
which is beneficial if there is no value constraint.
Typically value constraints will be used for safety though, so waiting for a lock to be acquired usually results in failure right after as a change
caused by a value mismatch. However, in the presence of deletions, it is always useful to wait for locks as after deletion, the previous value can't be checked anymore
causing the operation to succeed.

Races exist do not exist for writers, but do exist for readers as they may observe intermediate states of transactions involving multiple updates.

Semantically, `gitoxide`s implementation of this database is equivalent to the one of `git`.

#### Ref-table database

Even though `gitoxide` doesn't have an implementation yet it's useful to understand its semantics as a possible path forward.

As a major difference to the _loose reference database_ it doesn't operate on user editable files but uses a binary format optimized for read performance and consistency, allowing
readers to always have a consistent view. There can be any amount of readers.

All changes like updates, additions or deletions are fully transactional, but there can only be one writer at a time. This has the potential for contention in busy
(mono-)repositories as file based locking mechanisms aren't fair and waiting strategies with exponential backoff may cause some writers to wait forever.

Due to the lack of experience with this database details around resource consumption in terms of file handles can't be provided at this time.

### Configuration

`git` employs cascaded configuration files to affect processes running on the repository. Their effect can vary widely, and so can their use in applications handling repositories.

`gitoxide`s implementation currently uses only the repository git configuration file when initializing a repository to correctly determine whether or not it is _bare.
It does not expose git configuration at this time and doesn't use git configuration when repository initialization is complete.

We have no reason to believe that other implementations use git configuration beyond first initialization either or do anything to assure it remains up to date in memory
after reading it.

### Index

The `index` is a single file for the purpose acting as a staging area for building and manipulating upcoming commits.

It is created and updated when checking out files into the working tree, and is used to keep track of working tree states while no commit object
was created yet, i.e. during `git add …`.

`gitoxide` neither implements it nor it is used in concurrent environments, which is why we exclude it from this discovery.

## Known technical problems and their solutions

Before looking at how changes affect data consistency and resource usage affects reliability, let's list all known technical issues thus far.

Solutions aren't always mutually exclusive despite the form of presentation suggesting it.

| **Database**      | **Problem**                                                                                                                                  | **Solution**                                                                                                                                                                                                | **Benefits**                                                                                                                  | **shortcomings**                                                                                                                                                                                                                                     | **Example Implementation**                                                  |
|-------------------|----------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------|
| **pack**          | **1. initialization**                                                                                                                        | 1. map all packs at once                                                                                                                                                                                    | read-only possible; same latency for all objects                                                                              | worst case init delay, highest resource usage, some packs may never be read, some applications might run out of system resources early even though they would not have needed all packs.                                                             | gitoxide                                                                    |
|                   |                                                                                                                                              | 2. map packs later on object access miss                                                                                                                                                                    | nearly no init delay,  no unnecessary work, resource usage as needed                                                          | needs mutability;  first access of some objects may be slow                                                                                                                                                                                          | libgit2, git                                                                |
| **pack**          | **2. file limit hit**                                                                                                                        | 1. fail                                                                                                                                                                                                     | read-only possible                                                                                                            |                                                                                                                                                                                                                                                      | gitoxide                                                                    |
|                   |                                                                                                                                              | 2. free resources and retry, then possibly fail                                                                                                                                                             | higher reliability                                                                                                            | needs mutability                                                                                                                                                                                                                                     | libgit2 (only on self-imposed limit)                                        |
| **pack**          | **3. file handle reduction/avoid hitting file limit**                                                                                        | 1. do not exceed internal handle count                                                                                                                                                                      | some control over file handles                                                                                                | the entire application needs to respect count, needs sync with actual OS imposed limit,  no sharing across multiple in-process pack databases                                                                                                        | libgit2 (only within pack database)                                         |
|                   |                                                                                                                                              | 2. process-wide pooling of memory maps                                                                                                                                                                      | share packs across multiple repositories instances                                                                            | pack databases aren't self-contained anymore                                                                                                                                                                                                         |                                                                             |
|                   |                                                                                                                                              | 3. map whole packs at once     (instead of non-overlapping parts of them)                                                                                                                                   | Only consume one file handle per pack (as opposed to one per region)                                                          | cannot handle packs larger than 4GB on 32bit systems                                                                                                                                                                                                 | gitoxide                                                                    |
|                   |                                                                                                                                              | 4. Multi-pack index files                                                                                                                                                                                   | A single index can be used N packs, saving N-1 packs for N>1 packs                                                            |                                                                                                                                                                                                                                                      |                                                                             |
| **pack**          | **4. object miss**                                                                                                                           | 1. fail                                                                                                                                                                                                     | fast if object misses are expected                                                                                            | incorrect or a burden in user code if miss is due to changed packs                                                                                                                                                                                   | gitoxide                                                                    |
|                   |                                                                                                                                              | 2. lazy-load more packs, retry,      refresh known packs, retry, then fail                                                                                                                                  | always correct even in the light of writers                                                                                   | can cause huge amount of extra work if object misses are expected; does not handle problem 5.                                                                                                                                                        | libgit2                                                                     |
|                   |                                                                                                                                              | 3. catch error, force a pack refresh, repeat                                                                                                                                                                | can work in conjunction with similar shortcomings of loose reference database                                                 | needs mutability, burden on the API user;                                                                                                                                                                                                            |                                                                             |
|                   |                                                                                                                                              | 4. writers force an update of the process-wide pool of packs after creating new packs    and before updating references with the new objects                                                                |                                                                                                                               | high implementation complexity; assumes complete control of one process over git repository, excluding running git-maintenance; new readers aren't allowed for a while until the new pack is placed causing some moments of unresponsiveness/waiting |                                                                             |
| **pack**          | ~~5. race when creating/altering more than a pack at a time~~                                                                                | 1. ignore                                                                                                                                                                                                   |                                                                                                                               | a chance for occasional object misses                                                                                                                                                                                                                | all of them                                                                 |
|                   |                                                                                                                                              | 2. retry more than one time                                                                                                                                                                                 | greatly reduced likelihood of object misses                                                                                   |                                                                                                                                                                                                                                                      |                                                                             |
| **pack**          | **6.too many (small) packs (i.e. due to pack-receive)      reduce lookup performance**                                                       | 1. explode pack into loose objects (and deal with them separately)                                                                                                                                          | can run in parallel (but is typically bound by max IOP/s)                                                                     | might take a while if many objects are contained in the pack due to file IOP/s; needs recompression and looses delta compression; risk of too many small objects                                                                                    |                                                                             |
|                   |                                                                                                                                              | 2. combine multiple packs into one                                                                                                                                                                          | keep all benefits of packs; very fast if pack-to-pack copy is used; can run in parallel (but is typically bound by max IOP/s) | combining with big packs takes has to write a lot of data; can be costly if pack delta compression is used                                                                                                                                           |                                                                             |
|                   |                                                                                                                                              | 3. Just-in-time maintenance after writes                                                                                                                                                                    | tuned to run just at the right time to run just as much as needed                                                             | an implementation isn't trivial as there must only be one maintenance operation per repository at a time, so some queue should be made available to not skip maintenance just because one is running already.                                        |                                                                             |
| **loose refs**    | **7. readers of multiple references observe in-between states of transactions      which change a subset more than one of these references** | 1. switch to ref-table database                                                                                                                                                                             |                                                                                                                               | switches to shortcomings of ref-table                                                                                                                                                                                                                |                                                                             |
|                   |                                                                                                                                              | 2. repeat reference resolution until it's stable twice in a row                                                                                                                                             | works without ref-table                                                                                                       | cost for reference resolution at least doubles if more than one reference is needed; burden on API user                                                                                                                                              |                                                                             |
| **ref-table**     | **8. contention of writers around unfair lock**                                                                                              | 1. implement process-internal queuing mechanism                                                                                                                                                             | minimal cost and high throughput with  minimal, in-process synchronization                                                    | doesn't help with contention caused by multiple processes                                                                                                                                                                                            |                                                                             |
|                   |                                                                                                                                              | 2. use ref-table partitions                                                                                                                                                                                 | works for in-process and multi-process case, even though it might be slower than queues.                                      | I can't find information about it anymore                                                                                                                                                                                                            |                                                                             |
| **loose objects** | **9. too many loose objects reduce overall performance**                                                                                     | 1. use packs                                                                                                                                                                                                |                                                                                                                               | needs scheduled or otherwise managed maintenance, and winning strategies depend on the size and business of the repository                                                                                                                           |                                                                             |
|                   |                                                                                                                                              | 2. Just-in-time maintenance after writes                                                                                                                                                                    | tuned to run just at the right time to run just as much as needed                                                             | an implementation isn't trivial as there must only be one maintenance operation per repository at a time, so some queue should be made available to not skip maintenance just because one is running already.                                        |                                                                             |
| **all**           | **10. disk full/write failure**                                                                                                              | 1. write temporary files first, with robust auto-removal,     move into place when completed; partial transactions are robustly rolled back    or stray files aren't discoverable or are valid on their own |                                                                                                                               |                                                                                                                                                                                                                                                      | gitoxide; git (not libgit2, it leaves partial packs on receive for example) |
| **loose refs**    | **11. namespace is stored in database instance, so different `Easy` handles share it**                                                       | 1. Have one loose ref database per state (optionally)                                                                                                                                                       |                                                                                                                               | A default must be chosen and either one might be surprising to some, i.e. shared namespace as preference depends on the use case entirely, but seems like an unsafe default.                                                                         |                                                                             |

### Amendum problem 5.

Refreshing packs if an object is missed is the current way of handling writes to the pack database. As outlined in
how [geometric repacking works](https://github.blog/2021-04-29-scaling-monorepo-maintenance/#geometric-repacking) it can indeed
happen that multiple packs are changed which isn't atomic. However, since this will be done in an additive fashion, first adding the new packs based on existing packs
and loose objects, and then removing the packs and loose objects they replace, there is no race happening as all objects stay reachable at all times.

## Applying changes to repositories while maintaining consistency

A change to any file in a git repository has the potential to affect the processes operating on it, hence it's important to perform them so these
observe only valid repository states. For the sake of simplicity, we assume atomicity for all operations, but these still have to be ordered correctly
to assure a consistent state.

**Adding objects to the database**

1. add objects
    * this is safe as these objects aren't reachable yet.
    * object databases are append, only, so no existing state is ever affected.
2. adjust reference(s)
    * point related references to the entry-points of the newly added objects (typically commits or tags)
    * now the new objects can be discovered

**Multiple writers** will run into conflicts with each other if the references they are handling overlap.
**Readers** will see the object graph from a snapshot of a set of references.

**Reorganize objects or references for higher performance**

This happens regularly during maintenance operations, which don't alter the repository content but its structure.

1. find objects or references to include in the reorganization
2. write the reorganized data
3. remove the now obsolete data

Multiple of these operations must synchronize or else unpredictable outcomes may occur.
**Writers** may add packs which remain untouched, the same applies to loose objects.
**Readers** will see the object graph from a snapshot of a set of references.

**Read references and find objects reachable through them**

1. obtain object entry-points pointed to by references
2. traverse object graph with entry points

Multiple readers are always safe.

It's worth repeating that as long as all of the numbered items are truly atomic, readers will always observe a consistent state in the presence of writers.

### What breaks consistency

#### Packs

If all objects were loose, there would be no issue. However, for the above changes to work one must always be able to see all objects.
This isn't the case when packs are involved, as accessing them is too costly to redo the entire pack initialization work each time an object is retrieved
(e.g. list packs directory, memory map packs).

Thus packs are cached in order to reuse them each time an object is accessed. Staleness is typically checked if an object isn't present, as it's assumed
that all objects accessed are reachable through the object graph, which means they ought to be present, triggering a pack refresh and retrying the retrieval.

This assumption isn't necessarily the case (_anymore_) due to git features causing partial object repositories:

- depth-pruned clones (i.e. `git clone --depth X`) where all commits after a certain topological distance to the latest commit are left out.
  Running into this case during traversal is cheap despite triggering a refresh to detect that the object is truly not present, as it running a single time
  while stopping an entire traversal.
- partial clones with `git clone --filter=blob` which can be used to download entire histories but blobs only for a portion of the (usually recent) history.
  Operations like `git diff`, without special precautions, may run into a lot of object misses causing costly refreshes each time.

Any writer that eventually creates a pack may break readers that operate on a stale cache, making strategies for mitigation like the one above mandatory.
Such a mitigation, however, requires interior mutability as object access is a read-only operation, which in turn is costly in multi-threaded applications or
for applications that don't need it, like CLIs.

#### Loose References

Writing loose references isn't actually atomic, so readers may observe some references in an old and some in a new state. This isn't always a breaking issue like it is
the case for packs, the program can still operate and is likely to produce correct (enough) outcomes.

Mitigations are possible with careful programming on the API user's side or by using the `ref-table` database instead.

### What reduces reliability

System resources are limited and avoiding waste (along with decent configuration of `ulimits` and maintenance of disk-space) is one of the ways to keep an application
from failing.

The biggest consumer of system resources most certainly is the pack database due to its cache of memory mapped pack and index files, and avoiding to duplicate these within
a process is paramount.

## Use-Cases

We will look at typical access patterns holistically based on various use-cases, look at the problems they would run into and pick their preferred solution that optimizes
for efficiency/performance/reduction of waste.

### Multi-threaded CLI tool operating on filtered partial repository clone

The tool displays the reference graph, shows differences and creates 'blame' like views on files. As it's operated by the user, other writers are unlikely
to happen while it is running.

Scaling near perfectly with added CPU cores and great performance even in the biggest mono-repos are its hallmark.

As the program is written with missing objects being the default, it gracefully handles and expects such cases. While running in TUI mode, it offers a manual
refresh to the user in case they fetched or pulled in the meantime, to refresh the screen and update its object database to make all newly added objects available.

**Problems and Solutions**

* The program only deals with _1) initialization_ and _4) object misses_, where the latter are expected and handled gracefully. 1) is handled with solution 1., spending time to make
  all packs available to get the best and most consistent multi-threaded object access performance.

**Drawbacks**

The program could benefit of using 1.2 instead of 1.1 which could cause exhaustion of file handles despite the user having no interest in evaluating all available objects,
but ideally that is possible without losing performance during multi-threading.

### Professional git-hosting mono-repo server with git-maintenance tasks and just-in-time replication

A single server process handles all reads and writes concurrently and receives and sends using the git protocol using an async TCP/IO framework over a custom transport.
All operations calling into `gitoxide` are unblocked using a thread pool. There is only one repository with a reference namespace for each client repository of which it
contains 50.000, each averaging 5000 objects and 5 references for a total of 250.000.000 objects and 250.000 references. As many of these repositories have PNG files and
binaries checked in, they average to about 25MB of storage space for a total of 1.25TB.

At night there are about 1 send-pack (fetch) and 0.25 send-pack (clone) operations and 0.5 received-pack per second, ramping up to 6 send-pack (fetch) and 1.5 send-pack (clone)
and 3 receive-packs per second. At peak times during the day, these values are 10 times higher due to "push and ~~run~~ go home", causing 30 receive-packs per second for an hour or two.
_(assumption is 50.000 * 5 pushes and 10 fetches and 2.5 clones)_. Most client-side push operations are sending just 1 or 2 commits with a couple of changes, averaging pack sizes
of about 20 objects for a total of 5.000.000 new objects per day or an average of ~208.000 per hour. Only 1 branch is typically updated for 250.000 per day or ~10400 per hour.

After each receive-pack was run and the client connection was informed of success, a background job is started to push the changes using the git protocol to a sibling server
which also takes part in serving clients and also pushes its changes to this server (the writes caused by it are already part of the above measurements).

`git-maintenance` runs every 5 minutes using the built-in scheduling mechanism to repack references, place loose objects into packs, create and update reverse index caches
and reachability bitmaps and repacks existing packs geometrically. Every 24h it recompresses the newly created packs created during geometric repacking.

**Problems and Solutions**

* **1.1** pack database - initialization → all at once
    - as the server comes up, there is a moment to map all 27 to 28 packs and their indices, totalling up to 56 required file handles, along with maybe twice that many
      additional packs in case of many pushes of new repositories happening within the `git-maintenance` time window.
    - there is traffic at all times with some portions of all objects needed in the course of any 24h period
* **2.1** pack database - file limit hit  → fail
    - The worst case scenario for the amount of required file handles can be estimated and overcommitted.
    - Update the `ulimit` if there is not enough available handles for peak operations.
* **3.2 & 3.3**  pack database - file handle reduction/avoid hitting file limit → process wide memory map pooling & mapping entire packs/indices at once
    - Note that multipack indices are related to maximizing object-count performance in conjunction with bitmaps, which is a separate topic, but in short: yes, want that, too :)
* **4.3 & 4.4** pack database - object miss  → catch error, refresh packs, retry & writers force update shared memory map pool
    - we read references at well known times and assure their objects exists right away. If not, a forced (but efficient) pack refresh should make the new objects available.
    - For readers this mostly happens if external processes change the pack database, like git-maintenance, as writers keep the view on memory mapped packs fresh (efficiently)
      by informing about the exact changes to avoid directory scanning.
* **6.1 & 6.2** pack database - reduced object lookup performance/too many small packs - explode small packs to loose objects & keep packs with more than 10.000 objects
    - The trick is to strike the balance to keeping the amount of small packs low and to try not to have too many loose objects due to file system performance limitations with
      directories containing a lot of files. Within the maintenance window,
* **7.1 or 7.2** loose reference database - readers of multiple references observe in-between states of transactions […] → switch to ref-table or read references at least twice
    - With about 3 ref writes per second ref-table should not suffer from lock-contention, and should over better overall performance.
    - Otherwise, _7.2_ seems doable as well even though using the loose ref database definitely is a nightmare for reference deletions.
* **8** ref-table - contention of writers around unfair lock - deal-breaking contention would be unexpected
    - With the luxury of having a ref-table at all it would be even more of a luxury to use more advanced features to increase write performance. It's all a little bit too unknown
      right now.
* **9** loose object database - too many loose objects reduce overall performance - fixed with git-maintenance
    - See drawbacks though
* **10** - disk full
    - It will be interesting to think of solutions involving the introduction of a bigger machine and migration of all repositories to there before the disk space runs out.
      Otherwise we believe that managing disk space is part of operation and not the server process itself.
* **10** - write failure - fail connection
    - write failures aren't specifically handled but result in typical Rust error behaviour probably alongside error reporting on the respective channels of the gix-transport sideband.
    - `gitoxide` is made to cleanup on failure and leave nothing behind that could accumulate.
* **11** - loose ref database - namespace isn't per connection
    - This needs fixing in `gitoxide` to probably be unshared by default. Namespaces are most useful on the server, which would use an `EasyArcExclusive` per connection.
      Sharing ref namespaces would be surprising and wrong.

**Drawbacks**

* running `git-maintenance` every 5 minutes during off-hours seems like overkill and it seems better to a built-in repacking scheduler that is based on actual load.

### Self-hosted git server with front-end and zero-conf and auto-maintenance

This server is typically used by teams and runs within the intranet. Teams love it because it's easy to setup and usually 'just works' thanks to a local sqlite database
and a directory to store repositories in. Some teams have it running for 10 years without issues.

It provides a front-end which displays repository data and allows team-members to create issues and comment on each others merge requests, among other things. This browser
application uses websockets to keep a connection to the server through which data can be retrieved using a simple request-response protocol. After some time of inactivity
it automatically disconnects, but is automatically revived if the browser tab is activated again. There is prominent warnings if the disk space is low along with suggestions
for a fix.

The implementation prides itself for showing each commit message affecting the files currently displayed without caching them in the database due to its clever use of
multithreading, offloading segments of the history to all threads available for processing, sending the results back as they come in and stopping the processing once all
files and directories are annotated. It uses a single `Repository` instance per thread which changes as the client browses to different repositories, and expects all
objects to exists even in presence of pushes happening in the meantime. It checks for the latter by regularly polling if the commit of the current
branch changed compared to the previous time it checked.

Clients can push and fetch to the server via SSH and HTTP(S) transports. The SSH transport is implemented with a helper program that ultimately
calls `git receive-pack` and `git upload-pack`. When HTTP(S) is used, the serve program handles the connection itself, using one thread per connection and
opens up a new `Repository` for each of them. Multi-threading is used to build packs when sending or resolve them when receiving. After writing a pack the server
will schedule a background maintenance process to keep repositories fast and small.

The default favors speed and using all available cores, but savvy users can run it with `--threads 1`  to only ever use a single thread for processing.

**Problems and Solutions**

* **1.2** pack database - map packs later on object access miss
    - The server generally creates one `Repository` instance per connection which won't know the objects to access in advance. Minimizing initialization times is paramount.
    - The code is willing to differentiate between single- and multi-threaded mode when both are sufficiently different, but otherwise uses multi-threading compatible types
      even if using only one thread. As it doesn't expect to use too many cores at once this seems acceptable.
* **2.1** pack database - file limit hit  → fail
    - Since `Repository` instances aren't shared across connection, there is no way to free files. The system relies heavily on lazy-loading of pack data to not use system
      resources only when needed.
    - Update the `ulimit` if there is not enough available handles for peak operations.
* **3.2 & 3.3**  pack database - file handle reduction/avoid hitting file limit → process wide memory map pooling & mapping entire packs/indices at once
    - Note that 3.2 is very interesting as a way to deduplicate memory maps of multiple connections to the same repository. It should be fine to do without such optimization though
      and just increase the limit for file handles.
* **4.2** pack database - object miss  → lazy-load next pack, retry, repeat until there is no more pack to load
    - client code doesn't want to know about internal optimizations and thus prefers lazy-loading. It's notable that none-existing objects will force loading all packs that way,
      but that isn't expected on a server that definitely holds the entire object database.
* **6.1 & 6.2 & 6.3** pack database - reduced object lookup performance/too many small packs - explode small packs to loose objects & keep packs with more than 10.000 objects & just-in-time maintenance
    - Even in low-traffic servers its important to maintain them to avoid running into unavailability.
* **7.1** loose reference database - readers of multiple references observe in-between states of transactions […] → switch to ref-table
    - concurrency issues are less likely on a low-traffic server with people mostly pushing a single branch at a time. However, switching to ref-table solves a potential
      issue that should be chosen if there is no other reason to use a loose reference database
* **8** ref-table - contention of writers around unfair lock - n/a
    - not an issue as there isn't enough traffic here
* **9.2** loose object database - too many loose objects reduce overall performance - just-in-time maintenance
* **10** - disk full - display early warnings in the front-end to every user to get it fixed
    - This solution is implemented on application side (and not in `gitoxide`), it's interesting enough to mention though for systems that operate themselves.
    - One could also imagine that it tries to spend the nights aggressively compression repositories, some low-hanging fruits there.
* **10** - write failure - fail connection
    - write failures aren't specifically handled but result in typical Rust error behaviour probably alongside error reporting on the respective channels of the gix-transport sideband.
    - `gitoxide` is made to cleanup on failure and leave nothing behind that could accumulate.
* **11** - loose ref database - namespace isn't per connection
    - Needs fixing in `gitoxide`.

## Learnings

### Loose references database

- When deleting (with or without value constraint), wait for locks instead of failing to workaround `packed-refs` as choking point. It's certainly worth it to split transactions
  so that deletions are done separately from updates to allow usage of the most suitable locking strategies.
- When adding/updating references, prefer to fail immediately as the chance for the same change being made concurrently is low, and failure
  would occur after waiting for the lock due to constraint mismatch.

### Git configuration

- some application types might never use it, which is why it should be controllable how and what is loaded (when we actually implement it).

## Action Items

Please note that these are based on the following value system:

- We value _highly_ to scale object access performance with cores.
- We value _more_ to offer a choice of trade-offs than to aim for a one-size-fits-all solution, unless the latter has no shortcomings.*

- We don't value the handling of out of memory situations differently than panicking. This might change if `gitoxide` should fly to Mars or land in the linux kernel though.
- We don't value enabling 32 bit applications to deal with pack files greater than 4GB and leave this field entirely to the other implementations.

1. **per `Easy…`  ref namespace**
    - As loose ref databases are cheap, one should live on each state by default, and if these namespaces were cloned with the `Easy…` it would be straightforward to propagate
      this configuration.
    - There is still the open question how this should work when `ref-table` appears on the scene and its multi-file database that can most certainly benefit from shared memory maps
      similarly to pack databases, thus sharing it on the `repo()`. Maybe it would not contain the namespace but pass it as a parameter every time, which for consistency would be
      best ported to the loose ref database as well. That way, there would be one API governing both, unifying sharing on the `repo()`.
    - Ref databases could have methods like `find_in_namespace()' along with the current ones, whereas the current ones delegate to the ones with namespace which may be `None`,
      to accommodate for the fact that most won't use namespaces.
    - Use this opportunity to implement a general `Store` for later use with ref-table, and for immediate use in the `Easy` state. The packed-buffer along with the stat logic
      should definitely be placed in there, too, I think, as technically without a maintained packed buffer the whole loose ref DB is quite useless AND the `Store` API won't be
      equivalent between different implementations. AKA, hide implementation details which the packed ref buffer is. This would also give it its own Buffer for logs, but that's
      quite alright, after all these are quite separate from object buffers.

2. **Parameterize some sort of Policy into linked::ODB/compound::ODB**
    - First off, this needs an experiment to try it out quickly.
    - **initial thoughts**
        - ❌ Depending on the actual implementation of `Policy`, `Repository/Easy` will or will not be thread-safe. This excludes using a `Box<…>` there as it has different
          trait bounds (once with and once without `Send + Sync`. I would like to avoid more feature toggles in `gix`, but could live with it.
            - ✔️ `Repository` would end up with type parameters if feature toggles aren't used, which could be compensated for with typedefs for the few known policies. However, this
              would also lead in a type-explosion for `Easy` and may force it to have a type parameter too.
        - ❌ To keep the `Repository` free of type parameters we could boil policies down to typical policies, like Eager, Lazy, LazyThreadSafe, PooledLazy, PooledLazyThreadSafe,
          all with different tradeoffs. On that level, maybe 3 to 5 feature toggles would work, but who likes feature toggles especially if they aren't additive?
        - ✔️ `contains(oid)` is not actually exposed in any trait and not used much in `git` either, even though it is optimized for by loading pack data only on demand. We, however,
          use `gix_pack::Bundle` as smallest unit, which is a mapped index _and_ data file, thus forcing more work to be done in some cases. There is only one multi-pack index
          per repository, but that would force all packs to be loaded if it was implemented similarly, but that shows that Bundle's probably aren't the right abstraction or
          have to make their pack data optional. If that happens, we definitely need some sort of policy to make this work. Definitely put `contains(oid)` into the `Find` trait
          or a separate trait to enforce us dealing with this and keep multi-pack indices in mind.
        - ✔️ Some operations rely on pack-ids and or indices into a vector to find a pack, and this must either be stable or stable for long enough especially in the presence
          of refreshes. Keep pack-ids stable by matching them with their pack hashes. We still have to assure they won't be unloaded even after deletion on disk if somebody still
          refers to them.
            - Make sure pack-ids are always incrementing, which is how it's currently implemented, too (more or less, it always restarts from 0 which should be fine but why risk it).
        - ✔️ Can we be sure that there won't be another type parameter in `Repository` for the refs database? If yes, we basically say that `ref-table` will work read-only or
          hides its interior mutability behind RwLocks. It's probably going to be the latter as it should be fast enough, but it's sad there is inevitably some loss :/.
            - Technically it has a similar problem as the pack database, except that it's not append only which tips the scale towards interior mutability or `find(…)`
              implementations that are `&mut self`. However, that would exclude sharing of the ref-db, which probably is acceptable given that its usually just used to kick off
              multi-threaded computations. We don't even want to it hide its mutability as it's probably a catastrophe if multiple threads tried to read from it. So it would have
              to be behind a good old RWLock and maybe that's something we can live with for the Multi-RefDB that works for either loose refs or ref-table. A single lock to
              make the ref-table version sync. Unknown how that would relate to supporting parallel writing some time, but let's just say that _we don't think that another type
              parameter will be necessary for this one_, ever. It might even be an option to have one per `Easy::state` if it's reasonably fast, so yes, let's not bother with that now.
            - The ref-table DB could probably must be `&mut` for finding references and then be placed behind an Rc<RefCell> or Arc<Mutex> respectively. The `Repository` will just be
              share storage container and maybe there will be a better name for it.
        - ✔️ There is also the idea of 'views' which provide an owned set of bundles to iterate over so that pack access doesn't have to go through a lock most of the time
          unless there is the need for a refresh. This means bundles are not owned by the compound::Store anymore, but rather their container is.
            - There should be a way to gradually build up that container, so that one says: get next pack while we look for an object, otherwise the first refresh would
              map all files right away. Ideally it's index by index for `contains()` and index + data of a bundle at a time for `find()`.
            - ❌ This also means that if `contains()` is even a possibility, that on each call one will have to refresh the `view`. This might mean we want to split out that functionality
              into their own traits and rather hand people an object which is created after the `view` was configured - i.e. after calls to `contains()` one has to set the
              view to also contain packs. Ideally that happens on demand though… right now indices and packs are quite coupled so maybe this has to go away.
            - If there are views, these really should be per-thread because only then we can turn RwLocks/Mutexes into RefCells for mutation of the internal view, which is then
              also specific to the access pattern of the actual reader _and_ will be discarded when done (as opposed to a shared view in the Repository which lives much longer).
              Or in other words, `Policy` implementation could optionally be thread-safe, whereas the actual object repo is not, but the policy could be shared then if behind `Borrow`.
              Doing this would also mean that the Repository doesn't even have a repository anymore, but just a `pack::Policy`.
        - It's still unclear how to best control repeated refreshes in presence of the possibility of blobs missing due to `--filter=blob`. Maybe this is the moment where one would
          access the policy directly to turn off refreshes for the duration of the operation.
        - `Views` work if they are placed in the state and are thread-local for that reason, with interior mutability. A `view` will just be the linked odb implementation itself.
            - It should contain a borrowed `Policy` which is owned in the shared `Repository`. The latter should contains a list of paths to object databases (i.e. alternates) to
              allow seeing all multi-pack indices and indices like it is one repository.
        - `Repository`  turns into `RepositoryLocal` with a `Rc<dyn Policy>` that isn't `Sync` and adds a `Repository` type that does the same but with `Arc<dyn Policy + Sync + 'static>`.
            - each of these repository types has their own `Easy` types.
            - _Difficulties_: Some `Platform` types store `repo: Access::RepoRef` and use `repo.deref().odb`, but that now only has a `Policy` from which a new `State` should be created
              or they store the State right away… .
        - The default `Policy` should be the least surprising, hence incur mapping costs over time and automatically refresh itself if needed.
        - Make sure auto-refresh can be turned off on policy level to allow users to probe for objects despite `--filter=blob` or similar without slowing down to a crawl due to
          a refresh each time an object is missing.
        - The way this is going, `Deref` can panic in single-threaded applications only if recursion is used. Thread-safe versions of this will hang
          unless a reentrant mutex is used. Since we don't call ourselves recursively (i.e. during `find_object(…)`, this won't be an issue. It should also be impossible in single-threaded
          mode even with multiple `Easy` instances.
        - memory map pooling across repositories can work if the odb informs about the entry-point path when asking for more indices to check
        - It's OK to use a feature toggle to switch between Rc<RefCel> and Arc<Mutex>
    - **in the clear**
        - Algorithmically, object access starts out with `Indices`, fetching more until the object is contained within, then the corresponding the pack data is loaded (if needed)
          to possibly extract object data.
            - The result of a contains check would have to be a pack id to allow looking it up in the own cache and then ask it to be loaded/returned by the `Policy`, along with
              maybe the indices required to fetch it from the pack, maybe just a `bundle::Location` of sorts. It could also be a struct to encode the pack-id and the index within it.
            - `pack::Bundle` won't be used within the ODB anymore as it doesn't allow such separation and won't work well with multi pack indices.
            - a `Policy` could implement the building blocks needed by that algorithm.
            - The `Policy` should go through `Deref` to allow for different ways of internal shared ownership of actual indices, but that would also mean multiple implementations
              would either duplicate code or forward to even more generic implementations.
        - It looks like building a configurable 'can-do-it-call' store is more like it and would use compile-time types to avoid generics entirely. This could live in the Repository
          as before.
            - Having it in a shared-ownership configurable `Policy` is probably the way to go as it would allow sharing mappings across repositories while implementing ways of handling
              them.
            - when building packs, it's vital that the pack indices stay stable and don't go through a refresh (which isn't observable for the one finding objects). Thus it's vital
              that packs are built with their own object database that is configured to not refresh packs or better even, eager policy without refresh. The latter requires a well-maintained
              object database due to lots of additional file handles needed, or alternatively an algorithm which fails if a refresh would be needed but instead retries if an object wasn't found,
              for example when a pack can't be (lazy) loaded as it was removed during maintenance. In other words, those creating packs definitely have to deal with failure specifically and
              probably just retry
        - Also it seems that the existing implementation has merit, but should probably be altered to be a single store (without policy) instead also to fix latent issues around
          addressing packs in alternate repositories.
            - The current store is Sync, and can easily be passed around behind an `Arc`, which is actually how it works currently as well even though the `Arc` is on the `Repository`
              instead.
            - Shared-ownership `Policy` based stores would work just like it, but each one has its own thread-local interior mutable cache.
        - The new ODB implementation and the previous one are pretty incompatible because the old one is Sync, but the new one is designed not to be (i.e.) use thread-local storage.
          Existing implementations need to adjust their trait bounds and operate differently to accommodate. Counting objects and packing would be a good benchmark though, even though
          the latter doesn't even scale that well due to the required dashmap to check for existing objects. In other words, currently there seems to be no actual benchmark for parallel
          usage.
            - In single-threaded operation the trait-bounds would prevent creation of packs unless they are adjusted as well, leading to `gix-pack` requiring its own feature toggle
              which we really try hard to avoid, but probably can be placed on application level, which has to use that to setup gix-features accordingly, making it bearable. This
              means though that we need to implement single-threaded and multi-threaded versions of everything important, like pack generation based on the count (which already has
              a single-threaded version).
            - Maybe… after some benchmarks, we can entirely drop the single-threaded version if it's not significantly faster on a single thread (without thread primiives) than the
              same with multi-threading (but a single-thread).
        - Servers would be split into readers and writers, where…
            - …readers (receive-pack) share a common pool and thus all maps, with lazy loading and refresh (but their pack-ids change due to that, and packs might disappear, which they don't mind)
            - …writers (upload-pack) use a lazy loaded repository pool like readers during negotiation, but when…
                - …cloning use an eagerly loaded Repository just for that particular clone for stable pack ids
                - …fetches use a lazy-loaded Repository with refresh disabled, and full retries if the pack they were referring to goes away. Maybe there can be a policy for that to keep
                  pack ids stable despite refresh, which would also solve clones which could then lazy-load.
        - `Repository` must remain `Sync`.
        - The new general/policy store must always be sync, and can't use the OwnShared, etc, abstractions. Being able to build programs for single threads only is a feature we keep
          though, but we don't for this store as it must always be possible to use multi-threading as some algorithms might never be ported to single-threaded versions.
            - this is only for one reason: it's impossible to get rid of or make dynamic the Send + Sync trait bounds.
