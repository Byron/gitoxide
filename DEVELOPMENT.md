## Development Guide

### Practices 

 * **test-first development**
   * protect against regression and make implementing features easy
   * user docker to test more elaborate user interactions
   * keep it practical, knowing the Rust compiler already has your back
     for the mundane things, like unhappy code paths.
   * *use git itself* as reference implementation, and use their test-cases and fixtures where
     appropriate
   * *use libgit2* test fixtures and cases where appropriate
 * **safety first**
   * handle all errors, never unwrap.
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
  
### Guidelines

* **async**
  * **library client-side**
    * Don't use it client side, as operations there are usually bound by the CPU and ultra-fast access to memory mapped files.
      It's no problem to saturate either CPU or the IO system.
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
    * Building a pack is CPU and at some point, IO bound, and it makes no sense to use async to handle more connections - git
      needs a lot of resources and threads will do just fine.
      
* **interruption of long-running operations**
  * Use `git-features::interrupt::*` for building support for interruptions of long-running operations only.
    * It's up to the author to decide how to best integrate it, generally we use a poll-based mechanism to check whether
      an interrupt flag is set.
    * **this is a must if…**
      * …temporary resources like files might otherwise be leaked
    * **this is optional but desirable if…**
      * …there is no leakage otherwise to support user interfaces. They background long-running operations and need them to be cancellable.
      
* **prepare for SHA256 support by using `owned::Id` and `borrowed::Id`**
  * eventually there will be the need to support both Sha1 and Sha256. We anticipate it by using the `Id` type instead 
    of slices or arrays of 20 bytes. This way, eventually we can support multiple hash digest sizes.
  * Right now it's unclear how Sha256 is going to work in git, so we only support Sha1 for now. It might be an avenue to proactively
    implement it ahead of time once there is a specification to follow.
  * It looks like Git prepares to support it by using compile time, we can support it at runtime though with minimal cost. If needed,
    we can later remove support using a cargo feature toggle.
    
### Sha256

A bunch of notes collected to keep track of what's needed to eventually support it

* read `hash-function-transition.txt`
* [x] support `gpgsig-sha256` field - we won't break, but also don't do anything with it (e.g. `extra_headers`)
* [ ] support index V3
* [ ] Pack file PSRC field

### `.unwrap()` vs `.expect(…)`

* don't use unwrap, not even in tests. Instead use `quick_error!()` or `Box<dyn std::error::Error>`.
* Use `expect(…)` as assertion on Options, providing context on *why* the expectations should hold. Or in other words,
  answer "This should work _because_…<expect(…)>"
   
   
## Maintenance Guide

Utilities to aid in keeping the project fresh and in sync can be found in the `Maintenance` section of the `makefile`. Run `make` to
get an overview.

### Creating a release

Run `etc/release.sh` to release all crates in leaf-first order using `cargo release`.

### Which git-version to chase?

Generally, we take the git version installed on ubuntu-latest as the one we stay compatible with (_while maintaining backwards
compatibility_). Certain tests only run on CI, designed to validate certain assumptions still hold against possibly changed
git program versions.

This also means that CI may fail despite everything being alright locally, and the fix depends on the problem at hand.

### How to update fixtures

Fixtures are created by using a line like this which produces a line we ignore via `tail +1` followed by the un-prettified object payload
trailed by a newline.
```sh
echo c56a8e7aa92c86c41a923bc760d2dc39e8a31cf7  | git cat-file --batch | tail +2 > fixture
```

Thus one has to post-process the file by reducing its size by one using `truncate -s -1 fixture`, **removing the newline byte**.


