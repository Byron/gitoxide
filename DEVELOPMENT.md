# Development Guide

## Practices 

 * **test-first development**
   * protect against regression and make implementing features easy.
   * user containers to test more elaborate user interactions
   * keep it practical, knowing the Rust compiler already has your back
     for the mundane things, like unhappy code paths.
   * *use git itself* as reference implementation, and use their test-cases and fixtures where
     appropriate. At the very least, try to learn from them.
   * *use libgit2* test fixtures and cases where appropriate, or learn from them.
 * **safety first**
   * handle all errors, never `unwrap()`. If needed, `expect("why")`.
   * provide an error chain and make it easy to understand what went wrong. We use `quick-error` for non-generic errors and `thiserror` when generalization is
     needed.
 * Adhere to the [stability guide](https://github.com/Byron/gitoxide/blob/main/STABILITY.md)
  
## General

* **async**
  * **library client-side**
    * ~~Don't use it client side, as operations there are usually bound by the CPU and ultra-fast access to memory mapped files.
      It's no problem to saturate either CPU or the IO system.~~
      * Provide `async` clients as opt-in using feature toggles to help integrating into an existing async codebase.
  * **User Interfaces**
    * User interfaces can greatly benefit from using async as it's much easier to maintain a responsive UI thread that way thanks
      to the wonderful future combinators.
    * `blocking` can be used to make `Read` and `Iterator` async, or move any operation onto a thread which blends it into the 
      async world. 
       * Most operations are fast and 'interrupting' them is as easy as ignoring their result by cancelling their task.
       * Long-running operations can be roughly interacted with using `git_features::interrupt::trigger()` function, and after a moment
         of waiting the flag can be unset with the `…::uninterrupt()` function to allow new long-running operations to work. 
         Every long running operation supports this.
  * **server-side**
    * ~~Building a pack is CPU and at some point, IO bound, and it makes no sense to use async to handle more connections - git
      needs a lot of resources and threads will do just fine.~~
      * Support async out of the box without locking it into particular traits using conditional complication. This will make integrating
        into an async codebase easier, which we assume is given on the server side _these days_.
  * **usage of `maybe_async`**
    * Right not we intentionally only use it in tests to allow one set of test cases to test both blocking and async implementations. This is the
      only way to prevent drift of otherwise distinct implementations.
    * **Why not use it to generate blocking versions of traits automatically?**
      * This would require `maybe_async` and its dependencies to always be present, increasing compile times. For now we chose a little more code to handle
        over increasing compile times for everyone. This stance may change later once compile times don't matter that much anymore to allow the removal of code.
      
* **`Default` trait implementations**
  * These can change only if the effect is contained within the callers process.
    This means **changing the default of a file version** is a **breaking change**.
* **Using the `Progress` trait**
  * When receiving a `Progress` implementation
     * without calling `add_child(…)` then use it directly to communicate progress, leaving
       control of the name to the caller. However, call `.init(…)` to configure the iteration.
     * and when calling `add_child(…)` don't use the parent progress instance for anything else.  
* **interruption of long-running operations**
  * Use `git-features::interrupt::*` for building support for interruptions of long-running operations only.
    * It's up to the author to decide how to best integrate it, generally we use a poll-based mechanism to check whether
      an interrupt flag is set.
    * **this is a must if…**
      * …temporary resources like files might otherwise be leaked.
    * **this is optional but desirable if…**
      * …there is no leakage otherwise to support user interfaces. They background long-running operations and need them to be cancellable.
      
* **prepare for SHA256 support by using `git_hash::ObjectId` and `git_hash::oid`**
  * eventually there will be the need to support both Sha1 and Sha256. We anticipate it by using the `Id` type instead 
    of slices or arrays of 20 bytes. This way, eventually we can support multiple hash digest sizes.
  * Right now it's unclear how Sha256 is going to work in git, so we only support Sha1 for now. It might be an avenue to proactively
    implement it ahead of time once there is a specification to follow.
  * It looks like Git prepares to support it by using compile time, we can support it at runtime though with minimal cost. If needed,
    we can later remove support using a cargo feature toggle.
* **symbolic links do not exist** as far as we are concerned
  * in older, probably linux only, git versions symbolic links were used for symbolic references for example. This required special handling
    in some places. We don't implement that and assume more modern repositories.
    
## Sha256

A bunch of notes collected to keep track of what's needed to eventually support it

* read `hash-function-transition.txt`
* [x] support `gpgsig-sha256` field - we won't break, but also don't do anything with it (e.g. `extra_headers`)
* [ ] support index V3
* [ ] Pack file PSRC field

## `.unwrap()` vs `.expect(…)`

* don't use unwrap, not even in tests. Instead use `quick_error!()` or `Box<dyn std::error::Error>`.
* Use `expect(…)` as assertion on Options, providing context on *why* the expectations should hold. Or in other words,
  answer "This should work _because_…<expect(…)>"
  
## Examples, Experiments, Porcelain CLI and Plumbing CLI - which does what?

### Plumbing vs Porcelain

Both terms are coming from the `git` implementation itself, even though it won't necessarily point out which commands are plumbing and which
are porcelain.
The term *plumbing* refers to lower-level, more rarely used commands that complement porcelain by being invoked by it or by hand for certain use
cases.
The term *porcelain* refers to those with a decent user experience, they are primarily intended for use by humans.

In any case, both types of programs must self-document their capabilities using through the `--help` flag.

From there, we can derive a few rules to adhere to unless there are good reasons not to:

#### Plumbing

* does not show any progress or logging output by default
* if supported and logging is enabled, it will show timestamps in UTC
* it does not need a git repository, but instead takes all required information via the command-line

#### Porcelain

* Provides output to stderr by default to provide progress information. There is no need to allow disabling it, but it shouldn't show up unless
  the operation takes some time.
* If timestamps are shown, they are in localtime.
* Non-progress information goes to stdout.

#### Summary

Here is the hierarchy of programs - each level requires more polish and generally work to be done.
_Experiments_ are the quickest ways to obtain some insights. _Examples_ are materialized ideas that others can learn from but that don't quite have
the polish (or the potential) to move up to _plumbing_ or _porcelain_. _Plumbing_ is programs for use in scripts, whereas _porcelain_ is for use
by humans.

* **Experiments**
  * quick, potentially one-off programs to learn about an aspect of gitoxide potentially in comparison to other implementations like `libgit2`.
  * No need for tests of any kind, but it must compile and be idiomatic Rust and `gitoxide`.
  * Manual commmand-line parsing is OK
  * no polish
  * make it compile quickly, so no extras
* **Examples**
  * An implementation of ideas for actual occasional use and the first step towards getting integrated into Porcelain or Plumbing CLIs.
  * Proper command-line parsing with Clap.  
  * No tests or progress. 
  * High quality Rust code along with idiomatic `gitoxide` usage so people can learn from it.  
* **Plumbing CLI**    
  * Use Clap AND Argh for command-line parsing via feature toggles to allow for tiny builds as plumbing is mostly for scripts.
  * Journey tests 
  * Progress can be turned on using the `--verbose` flag, quiet by default.
  * Examples can be turned into plumbing by adding journey tests and `argh` command-line parsing, as well as progress.
* **Porcelain CLI**
  * Use Clap for command-line parsing for the best quality CLI experience - it's for the user.
  * Journey tests.
  * Support for `--quiet` and `--progress`.
  * Verbose by default.
  * Examples can be turned into plumbing by adding journey tests and progress.

# Maintenance Guide

Utilities to aid in keeping the project fresh and in sync can be found in the `Maintenance` section of the `makefile`. Run `make` to
get an overview.

## Creating a release

Run `make publish-all` to publish all crates in leaf-first order using `cargo release` based on the currently set version.
For this to work, you have to run `cargo release minor|major` each time you break the API of a crate but abort it during package verification.
That way, `cargo release` updates all the dependents for you with the new version, without actually publishing to crates.io.

## Which git-version to chase?

Generally, we take the git version installed on ubuntu-latest as the one we stay compatible with (_while maintaining backwards
compatibility_). Certain tests only run on CI, designed to validate certain assumptions still hold against possibly changed
git program versions.

This also means that CI may fail despite everything being alright locally, and the fix depends on the problem at hand.

## How to update fixtures

Fixtures are created by using a line like this which produces a line we ignore via `tail +1` followed by the un-prettified object payload
trailed by a newline.
```sh
echo c56a8e7aa92c86c41a923bc760d2dc39e8a31cf7  | git cat-file --batch | tail +2 > fixture
```

Thus one has to post-process the file by reducing its size by one using `truncate -s -1 fixture`, **removing the newline byte**.

# Discovery: data consistency and resource usage of packed objects

## Summary

Concurrent writes to packs observable by applications pose a challenge to the current implementation and we need to find ways around that. There may be other limitations
related to any resource that changes often, most prominently refs.

## Motivation

Use this document to shed light on the entire problem space surrounding data consistency and resource usage of packed objects to aid in finding solutions
that are best for various use cases without committing to high costs in one case or another.

## Status Quo

### Object Databases

In repositories there may be millions of objects and accessing them quickly is relevant to many of git's features. 

#### Packed object database

As packs and the object database is inherently append-only, i.e. objects are never *[0] deleted, allowing concurrent readers to observe a consistent state even in presence
of writers. Writers create new pack files and may remove them after adding all changes successfully.

`gitoxide`s implementation as of 2021-11-19 is known to be unsuitable in the presence of concurrent writes to packs due to its inability to automatically respond
to changed packs on disk if objects cannot be found on the first attempt. Other implementations like `git2` and canonical `git` handle this by using 
thread-safe interior mutability at the cost of scalability.

`gitoxide`s implementation may be insufficient in that regard, but it shows how read-only data allows to scale across core as well as the CPU architecture allows.

The usage of system resources like file handles is simple but potentially wasteful as all packs are memory-mapped in full immediately. Lazy and partial memory-mapping 
of packs is used in other implementations. Laziness allows for more efficiency and partial mapping allows to handle big packs on 32 bit systems.

Failure to acquire a memory map due to limits in the amount of open file handles results in an error when initializing the pack database in the `gitoxide`s implementation.
To my knowledge, this is handled similarly in other implementations. All implementations assume there is unlimited memory, but the effect of running out of memory is only 
known to me in case of `gitoxide` which will panic.


[0] deletion is possible but doesn't happen instantly, instead requiring time to pass and calls to git-maintenance and for them to be unreachable, i.e. not used in the
    entire repository.

#### Loose object database

Each object is stored in a single file on disk, partitions by the first byte of its content hash. All implementations handle it similarly.

### Loose reference database

References, i.e. pointers to objects or other references, are stored one at a time in files, one reference at a time or multiple ones in a single well known file, `packed-refs`. 
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


| **Operation**   | **read loose file**                 | **locks**             | **costs**                                                                                                                                                                                                                                                                        | **read packed-refs**                                                        | **concurrency granularity**     | **Limit/Bottleneck**                         |
|-----------------|-------------------------------------|-----------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------|---------------------------------|----------------------------------------------|
| **Add/Update**  | only if comparing to previous value | loose ref             | .lock file per reference; write new value; move into place; create intermediate directories as needed; delete empty directories in the way as needed; read packed-refs                                                                                                           | only if comparing to previous value and loose reference file isn't present. | per reference                   | disk-IO                                      |
| **Read**        | always                              |                       | read loose file; read packed-refs                                                                                                                                                                                                                                                | if loose file didn't exist                                                  | n/a                             | disk-IO                                      |
| **Delete**      | only if assserting previous value   | loose ref; packed-ref | .lock file per reference; delete loose file; delete lock file; .lock for packed-refs, rewrite packed-refs.lock; move packed-refs.lock into place                                                                                                                                 | always                                                                      | per reference (and packed-refs) | disk-IO (and CPU if packed-refs is involved) |
| **maintenance** | always all (or by filter)           | loose ref; packed-ref | .lock file per reference, read loose reference; .lock for packed-refs; read entire packed-refs; insert loose reference values; write entire altered packed-refs into packed-refs.lock; move packed-refs.lock into place; delete existing loose references and delete their .lock | always                                                                      | per reference and packed-refs   | disk-IO and CPU                              |

Failures to add/update/delete may occur if the constraint isn't met. It's possible to wait in the presence of a lock file instead of failing immediately, 
which is beneficial if there is no value constraint. 
Typically value constraints will be used for safety though, so waiting for a lock to be acquired usually results in failure right after as a change
caused by a value mismatch. However, in the presence of deletions, it is always useful to wait for locks as after deletion, the previous value can't be checked anymore
causing the operation to succeed.

Races exist do not exist for writers, but do exist for readers as they may observe intermediate states of transactions involving multiple updates.

Semantically, `gitoxide`s implementation of this database is equivalent to the one of `git`.

### Ref-table database

**TBD**


## Understanding changes to repositories

A change to any file in a git repository has the potential to affect the process operating on it. Related changes to multiple files are never atomic, and can be observed
in an in-between state.

**TBD**


## Values

- We value _highly_ to scale object access performance with cores.
- We value _more_ to offer a choice of trade-offs than to aim for a one-size-fits-all solution, unless the latter has no shortcomings.

- We don't value the handling of out of memory situations differently than panicking. This might change if `gitoxide` should fly to Mars or land in the linux kernel though.
- We don't value enabling 32 bit applications to deal with pack files greater than 4GB and leave this field entirely to the other implementations.

## Existing technical problems and their solutions

| **Problem**                                                   | **Solution**                                                               | **Benefits**                                                         | **shortcomings**                                                                                                                             | **Example Implementation**           |
|---------------------------------------------------------------|----------------------------------------------------------------------------|----------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------|
| **1. initialization**                                         | 1. map all packs at once                                                   | read-only possible; same latency for all objects                     | worst case init delay, highest resource usage, some packs may never be read                                                                  | gitoxide                             |
|                                                               | 2. map packs later on object access miss                                   | nearly no init delay,  no unnecessary work, resource usage as needed | needs mutability;  first access of some objects may be slow                                                                                  | libgit2, git                         |
| **2. file limit hit**                                         | 1. fail                                                                    | read-only possible                                                   |                                                                                                                                              | gitoxide                             |
|                                                               | 2. free resources and retry, then possibly fail                            | higher reliability                                                   | needs mutability                                                                                                                             | libgit2 (only on self-imposed limit) |
| **3. file handle reduction/avoid hitting file limit**         | 1. do not exceed internal handle count                                     | some control over file handles                                       | the entire application needs to respect count, needs sync with actual OS imposed limit, no sharing across multiple in-process pack databases | libgit2 (only within pack database)  |
|                                                               | 2. process-wide pooling of memory maps                                     | share packs across multiple repositories instances                   | pack databases aren't self-contained anymore                                                                                                 |                                      |
| **4. object miss**                                            | 1. fail                                                                    | fast if object misses are expected                                   | incorrect or a burden in user code if miss is due to changed packs                                                                           | gitoxide                             |
|                                                               | 2. lazy-load more packs, retry,      refresh known packs, retry, then fail | always correct even in the light of writers                          | can cause huge amount of extra work if object misses are expected; does not handle problem 5.                                                | libgit2                              |
| ~~5. race when creating/altering more than a pack at a time~~ | 1. ignore                                                                  |                                                                      | a chance for occasional object misses                                                                                                        | all of them                          |
|                                                               | 2. retry more than one time                                                | greatly reduced likelyhood of object misses                          |                                                                                                                                              |                                      |
### Amendum problem 5.

Refreshing packs if an object is missed is the current way of handling writes to the pack database. As outlined in
how [geometric repacking works](https://github.blog/2021-04-29-scaling-monorepo-maintenance/#geometric-repacking) it can indeed
happen that multiple packs are changed which isn't atomic. However, since this will be done in an additive fashion, first adding the new packs based on existing packs
and loose objects, and then removing the packs and loose objects they replace, there is no race happening as all objects stay reachable at all times.

## Approach

We will look at typical access patterns holistically based on various use-cases and decide which existing technical solution would fit best.

## Problems and solutions

### Loose references database has inconsistent reads

When updating multiple references in a single transaction, writers may observe an intermediate states with some refs pointing to the previous value, some pointing to the new.

The known **solution** is to switch to the `ref-table` implementation.

## Learnings

### Loose references database

- When deleting (with or without value constraint), wait for locks instead of failing to workaround `packed-refs` as chocking point.
- When adding/updating references, prefer to fail immediately as the chance for the same change being made concurrently is low, and failure 
  would occur after waiting for the lock due to constraint mismatch.
