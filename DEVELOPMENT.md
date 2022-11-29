# Development Guide

## Practices 

 * **test-first development**
   * protect against regression and make implementing features easy.
   * user containers to test more elaborate user interactions
   * keep it practical, knowing the Rust compiler already has your back
     for the mundane things, like unhappy code paths.
   * *use git itself* as reference implementation, and use their test-cases and fixtures where
     appropriate. At the very least, try to learn from them.
      * Run the same test against git whenever feasible to assure git agrees with our implementation. 
        See `git-glob` for examples.
   * *use libgit2* test fixtures and cases where appropriate, or learn from them.
 * **safety first**
   * handle all errors, never `unwrap()`. If needed, `expect("why")`.
   * provide an error chain and make it easy to understand what went wrong. 
   * We `thiserror` generally.
 * Adhere to the [stability guide](https://github.com/Byron/gitoxide/blob/main/STABILITY.md)

## Commit Messages

We use a style I'd call 'purposeful [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/)', and instead of classifying
every commit using _conventional commit_ messaging, we do so only if the message should show up in the changelog.

The _subject_ usually informs about the *what* and the body provides details and explains the *why*.

Commit messages _must_ show up in the changelog in case of breaking changes. Examples for that are:

- change!: rename `Foo` to `Bar`. (#123)
  
  And this is why we do it in the body.
- remove!: `Repository::obsolete()`.
 
  Nobody used this method.

Features or other changes that are visible and people should know about look like this:

- feat: add `Repository::foo()` to do great things. (#234)

  And here is how it's used and some more details.
- fix: don't panic when calling `foo()` in a bare repository. (#456)

Everything else, particularly refactors or chores, don't use _conventional commits_ as these don't affect users of the API.
Examples could be:

- make test module structure similar to the modules they are testing for consistency
- `make fmt`
- thanks clippy

Please refrain from using `chore:` or `refactor:` prefixes as for the most part, users of the API don't care about those. When a `refactor` 
changes the API in some way, prefer to use `feat`, `change`, `rename` or `remove` instead, and most of the time the ones that are not `feat`
are breaking so would be seen with their _exclamation mark_ suffix, like `change!`.

### Reasoning

Commit messages are used for guiding `cargo smart-release` to do most of the release work for us. This includes changelog generation
as well as picking the right version bump for each crate.

## Commit splitting on breaking changes.

Knowing that `cargo smart-release` is driven by commit messages and affects their versions with per-crate granularity, it becomes important
to split edits into multiple commits to clearly indicate which crate is actually broken.

Typical patterns include making a breaking change in one crate and then fix all others to work with it. For changelogs to look proper
and version bumps to be correct, the first commit would contain only the breaking changes themselves, 
like "rename: `foo()` to `bar()`", and the second commit would contain all changes to adapt to that and look like "adapt to changes in `<crate name>`".

## Commit History

We generally follow a 'track everything' approach and there is a lot of freedom leading to more commits rather than less. There 
is no obligation to squash commits or to otherwise tune the history.

We use feature branches and PRs most of the time to be able to take advantage of CI and GitHub review tools, and merge with merge commits
to make individual efforts stand out. There is no need for linearizing history or tuning it in any other way. However, each commit 
_must_ follow the guidelines laid out in the `Commit Messages` paragraph.

There is value in organizing commits by topic and [_Stacked Git_](https://stacked-git.github.io) is hereby endorsed to do that.
  
## Configuration and overrides

As a general rule, respect and implement all applicable [git-config](https://git-scm.com/docs/git-config) by default, but allow the
caller to set overrides. How overrides work depends on the goals of the particular API so it can be done on the main call path,
forcing a choice, or more typically, as a side-lane where overrides can be done on demand.

Note that it should be possible to obtain the current configuration for modification by the user for selective overrides, either
by calling methods or by obtaining a data structure that can be set as a whole using a `get -> modify -> set` cycle.

Note that without any of that, one should document that with `config_snapshot_mut()` any of the relevant configuration can be
changed in memory before invoking a method in order to affect it.

Parameters which are not available in git or specific to `gitoxide` or the needs of the caller can be passed as parameters or via
`Options` or `Context` structures as needed.

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
* **when to use interior mutability**
  - in **plumbing**, do not use it at all but instead provide the mutable part (like caches, buffers) as arguments, pushing their handling entirely to the caller.
  - Set on top an optional abstraction that manages the above for you using **interior mutability only if part of the mutable state has to be returned as borrow**
    or if otherwise it wouldn't be possible to borrowcheck. Or in other words: start without interior mutability and try to do it the standard way, but switch when needed.
  - When using primitives to support interior mutability, use the provided ones and utility functions in `git_features::threading::*` exclusively to allow switching between
    thread-safe and none-threadsafe versions at compile time.
      - The preferred way of using it is to start out as upgradable reader, and upgrading to write if needed, keeping contention to a minimum.
  - If _shared ownership_ is involved, one always needs _interior mutability_, but may still decide to use an API that requires `&mut self` if locally stored caches are involved.
  - Types that are not thread-local must be `Sync`, but only if the `git-features/parallel` is enabled due to the usage of `git_features::threading::…` primitives which won't
    be thread-safe without the feature.
* **when to use shared ownership**
  - Use `git_features::threading::OwnShared` particularly when shared resources supposed to be used by thread-local handles. Going through a wrapper for shared ownership is fast
    and won't be the bottleneck, as it's only about 16% slower than going through a shared reference on a single core.
* **Path encoding**
  - For `git`, paths are just bytes no matter on which platform. We assume that on windows its path handling goes through some abstraction layer like `MSYS2`
    which avoids it to seeing UTF-16 encoded paths (and writing them). Thus it should be safe to assume `git`s path encoding is byte oriented.
  - Assuming UTF8-ish bytes in paths produced by `git` even on windows due to `MSYS2`, we use `os_str_bytes` to convert these back into `OsStr` and derivatives like `Path`
    as needed even though it might not always be the case depending on the actual encoding used by `MSYS2` or other abstraction layers, or avoiding to use std types altogether
    using our own instead.
    
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
  
## `Options` vs `Context`

- Use `Options` whenever there is something to configure in terms of branching behaviour. It can be defaulted, and if it can't these fields should be parameters.
- Use `Context` when potential optional data is required to perform an operation at all. See `git_config::path::Context` as reference. It can't be defaulted and the
  fields could also be parameters.

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
  * Manual command-line parsing is OK
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

## Reviewing PRs

- be sure to clone locally and run tests with `GITOXIDE_TEST_IGNORE_ARCHIVES=1` to assure new fixture scripts (if there are any) are validated
  on _MacOS_ and _Windows_. Note that linux doesn't need to be tested that way as CI on linux ignores them by merit of not checking them out
  via `git-lfs`.

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

### For object data

Fixtures are created by using a line like this which produces a line we ignore via `tail +1` followed by the un-prettified object payload
trailed by a newline.
```sh
echo c56a8e7aa92c86c41a923bc760d2dc39e8a31cf7  | git cat-file --batch | tail +2 > fixture
```

Thus one has to post-process the file by reducing its size by one using `truncate -s -1 fixture`, **removing the newline byte**.

# Tips & Tricks

## Git debug mode cranked up to 11

```
GIT_TRACE=true \
GIT_TRACE_PACK_ACCESS=true \
GIT_TRACE_PACKET=true \
GIT_TRACE_PACKFILE=true \
GIT_TRACE_PERFORMANCE=true \
GIT_TRACE_SHALLOW=true \
GIT_TRACE_SETUP=true \
GIT_CURL_VERBOSE=true \
GIT_SSH_COMMAND="ssh -VVV" \
git <command>
```

