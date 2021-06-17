# Development Guide

## Practices 

 * **test-first development**
   * protect against regression and make implementing features easy.
   * user docker to test more elaborate user interactions
   * keep it practical, knowing the Rust compiler already has your back
     for the mundane things, like unhappy code paths.
   * *use git itself* as reference implementation, and use their test-cases and fixtures where
     appropriate.
   * *use libgit2* test fixtures and cases where appropriate.
 * **safety first**
   * handle all errors, never `unwrap()`. If needed, `expect("why")`.
   * provide an error chain and make it easy to understand what went wrong.
 * **strive for an MVP and version 1.0 fast...**
   * ...even if that includes only the most common usecases.
 * **Prefer to increment major version rapidly...**
   * ...instead of keeping major version zero for longer than needed.
 * **stability**
   * we adhere to semantic versioning
   * while below 1.0, expect a greater amount of breaking changes, which are announced with minor versions
   * From 1.0, we will try hardest to keep the API and user interface non-breaking the closer to the user a library is. Thus the CLI should remain at version
    1 for a long times. However, crates that make it up can change more rapidly and may see more major version changes over time.
  
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


