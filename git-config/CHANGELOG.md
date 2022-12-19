# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.13.0 (2022-12-19)

### New Features

 - <csr-id-5b9bffe8a5eec738e892224a7e18f98c8430d8a4/> `SectionMut::push_with_comment(key, comment)` to add a new variable with a comment.
   This is useful for providing more information about a value at hand, especially if it was
   added programmatically and then shows up in the configuration.
 - <csr-id-e4bf8f0072e60a7a2df94690c8d0b13b1f3038bb/> Add the `Source::EnvOverride` to have a place for 'terminal' overrides.
   That way environment variables represented via git-configuration
   can be integrated into git configuration, making clearer what's
   going to happen even when looking at the configuration via
   `gix config`.
   
   The implementation has to be careful though about assureing there
   is no more specific configuration key, like `http.<URL>.proxy` that
   would override the one from the environment, which always has
   the final word.
 - <csr-id-5fa95460db843f7dcfe68002b303b8b7649846dd/> comfort API like `string_by_key(key)` takes a key like `"remote.origin.url"`, add `section_by_key("remote.origin")` as well.
   That way it's the most comfortable way to query values and very
   similar to how git does it, too.
   
   Additionally, sections can be obtained by section key, both mutably and immutably for completeness.

### New Features (BREAKING)

 - <csr-id-2b36d99eaf3ed24ce4cb736a3dd48440dc0c73b7/> `File::new_section()` and related now returns their `id` as well.
   That way it's possible to more easily interact with it later, for instance
   when one wants to delete it.

### Bug Fixes (BREAKING)

 - <csr-id-0c98ec8fc7d8cc3195472a04fde4a681f620725f/> subsections are identified as `&BStr` in entire API.
   Technically they can be any value (except for newlines and unescaped double quotes),
   and these values might be paths and everything that comes with it, like
   illformed UTF8. In order to be able to represent everything that
   git can represent, we don't enforce UTF8 anymore for subsection names.
   
   Note that section names and key names are required to be valid UTF8
   (and even alphanumeric ascii), which makes illformed UTF8 very unlikely
   there.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 27 calendar days.
 - 27 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Merge branch 'adjustments-for-cargo' ([`083909b`](https://github.com/Byron/gitoxide/commit/083909bc7eb902eeee2002034fdb6ed88280dc5c))
    - adjust to changes in `git-testtools` ([`4eb842c`](https://github.com/Byron/gitoxide/commit/4eb842c7150b980e1c2637217e1f9657a671cea7))
    - make fmt ([`747008d`](https://github.com/Byron/gitoxide/commit/747008d9d370844574dda94e5bec1648c4deb57e))
    - Merge branch 'main' into http-config ([`6b9632e`](https://github.com/Byron/gitoxide/commit/6b9632e16c416841ffff1b767ee7a6c89b421220))
    - `File::new_section()` and related now returns their `id` as well. ([`2b36d99`](https://github.com/Byron/gitoxide/commit/2b36d99eaf3ed24ce4cb736a3dd48440dc0c73b7))
    - `SectionMut::push_with_comment(key, comment)` to add a new variable with a comment. ([`5b9bffe`](https://github.com/Byron/gitoxide/commit/5b9bffe8a5eec738e892224a7e18f98c8430d8a4))
    - Release git-features v0.24.1, git-actor v0.14.1, git-index v0.9.1 ([`7893502`](https://github.com/Byron/gitoxide/commit/789350208efc9d5fc6f9bc4f113f77f9cb445156))
    - Add the `Source::EnvOverride` to have a place for 'terminal' overrides. ([`e4bf8f0`](https://github.com/Byron/gitoxide/commit/e4bf8f0072e60a7a2df94690c8d0b13b1f3038bb))
    - thanks clippy ([`10f4f21`](https://github.com/Byron/gitoxide/commit/10f4f2149830734cc551ea96a3d087f07d43fe29))
    - comfort API like `string_by_key(key)` takes a key like `"remote.origin.url"`, add `section_by_key("remote.origin")` as well. ([`5fa9546`](https://github.com/Byron/gitoxide/commit/5fa95460db843f7dcfe68002b303b8b7649846dd))
    - subsections are identified as `&BStr` in entire API. ([`0c98ec8`](https://github.com/Byron/gitoxide/commit/0c98ec8fc7d8cc3195472a04fde4a681f620725f))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
</details>

## 0.12.0 (2022-11-21)

### New Features

 - <csr-id-7d7bd02d4e0678565f58c5da83fd1ad88c60e911/> read worktree specific configuration of main worktrees.
   Supporting this is useful when interacting with worktrees that have been
   created with sparse worktree support, which moves some configuration
   values into those for the worktree at hand.
   
   Note that linked worktrees are not supported - for that use
   `git-repository` instead.

### New Features (BREAKING)

 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `git-features` and `git-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 2 calendar days.
 - 4 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - read worktree specific configuration of main worktrees. ([`7d7bd02`](https://github.com/Byron/gitoxide/commit/7d7bd02d4e0678565f58c5da83fd1ad88c60e911))
    - refactor ([`747b9e9`](https://github.com/Byron/gitoxide/commit/747b9e9ee6c467c85a6fd4246ad0fea216176cbc))
    - upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
</details>

## 0.11.0 (2022-11-17)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 10 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-glob v0.4.2, git-config-value v0.8.2, git-lock v2.2.0, git-ref v0.19.0, git-config v0.11.0, git-discover v0.8.0, git-index v0.8.0, git-transport v0.22.0, git-protocol v0.23.0, git-worktree v0.8.0, git-repository v0.28.0, gitoxide-core v0.20.0, gitoxide v0.18.0, safety bump 9 crates ([`0c253b1`](https://github.com/Byron/gitoxide/commit/0c253b15143dcedfe4c66d64ab1ea6e097030651))
    - prepare changelogs prior to release ([`fe5721f`](https://github.com/Byron/gitoxide/commit/fe5721f888c64c79fe9a734a9e33b94a282f8d97))
    - order matters ([`166f349`](https://github.com/Byron/gitoxide/commit/166f349b387c219431e5ef0410d9b1402e58dc09))
    - update docs ([`aab6a33`](https://github.com/Byron/gitoxide/commit/aab6a3359fea2858e54d38073b8714d61be2c699))
    - naive approach to loading worktree configs… ([`9d8cb1f`](https://github.com/Byron/gitoxide/commit/9d8cb1f5689cabe5535888b439437581261d9c3b))
</details>

## 0.10.0 (2022-11-06)

### Bug Fixes

 - <csr-id-839f776454d7a8522c0f2887c90fc41b20576bd9/> `File::boolean()` now correctly handles implicit bools across sections.
   This means it will return the last implicit value as true, instead of
   ignoring it to fall back onto an explicit boolean value that might be
   false.
 - <csr-id-7d92c61abc2c54f4a64c35b37bcd1843a8d1da9c/> `file::SectionMut::push()` now properly creates empty values.
   Previously, when calling `.push("key", None)`, the resulting internal
   state would be slighly wrong causing certain invariants and expectations
   to be unfulfilled even though it might have looked OK to most users.
   
   Now it will uphold the invariant for empty values, those without an
   `=` character.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 20 calendar days.
 - 27 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - `File::boolean()` now correctly handles implicit bools across sections. ([`839f776`](https://github.com/Byron/gitoxide/commit/839f776454d7a8522c0f2887c90fc41b20576bd9))
    - be sure to globally order section ids when traversed by name. ([`21a5229`](https://github.com/Byron/gitoxide/commit/21a522902257efb82eeea277d31e0b5ca67d6ef4))
    - `file::SectionMut::push()` now properly creates empty values. ([`7d92c61`](https://github.com/Byron/gitoxide/commit/7d92c61abc2c54f4a64c35b37bcd1843a8d1da9c))
    - refactor ([`041ede9`](https://github.com/Byron/gitoxide/commit/041ede9fe1581a733ea47033ba58265c2ef88bcd))
 * **Uncategorized**
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - prepare changelogs prior to release ([`423af90`](https://github.com/Byron/gitoxide/commit/423af90c8202d62dc1ea4a76a0df6421d1f0aa06))
    - Merge branch 'main' into write-sparse-index (upgrade to Rust 1.65) ([`5406630`](https://github.com/Byron/gitoxide/commit/5406630466145990b5adbdadb59151036993060d))
    - thanks clippy ([`04cfa63`](https://github.com/Byron/gitoxide/commit/04cfa635a65ae34ad6d22391f2febd2ca7eabca9))
    - adjust memory-size expectations to deal with Rust 1.65 and below ([`a93c470`](https://github.com/Byron/gitoxide/commit/a93c4703699ea61a646c82b861c9345715a6c057))
    - Merge branch 'main' into write-sparse-index ([`c4e6849`](https://github.com/Byron/gitoxide/commit/c4e68496c368611ebe17c6693d06c8147c28c717))
    - make fmt ([`ea2136b`](https://github.com/Byron/gitoxide/commit/ea2136b065979cecb3a1fdbf7b20ed7514128d9a))
    - Merge branch 'gix-clone' ([`def53b3`](https://github.com/Byron/gitoxide/commit/def53b36c3dec26fa78939ab0584fe4ff930909c))
    - thanks clippy ([`d2f56df`](https://github.com/Byron/gitoxide/commit/d2f56df5405f6c27ebf7d51f33381f2c548433fb))
</details>

## 0.9.0 (2022-10-10)

### New Features

 - <csr-id-aa5d66f60bd6c9ef404ebc120b613e0cf055b2c9/> add `parse::section::header::is_valid_subsection()` function.
   It can be useful to validate subsection names without having to
   construct an entire `Header` (which also includes a name).
 - <csr-id-5df2a2a5a9addbda7dcc68b2f8f7f4a48d9720c6/> Add various methods to iterate sections along with their id, and mutate them.
   As section names are not unique, it was previously not possible to
   iterate sections and then mutate them as one wouldn't be able to refer
   to the exact section that was just traversed, after all, there can be
   many sections named `remote "origin"`.
   
   With the new methods it's possible to uniquely refer to each section
   for mutation and removal.

### Bug Fixes

 - <csr-id-9c1e639979a9615fd8334ce0e3a809df137776f6/> greatly improve whitespace handling when removing values.
   Previously, newlines would remain past a value, and whitespace could
   remain before one.
   
   Now both are removed to simulate removing an actual line.
 - <csr-id-e533993e8f861ba7a6600aab114ddfecc8a85ee2/> `File::remove_section()` was fixed to allow re-adding a similarly named section.
   We also add `File::remove_section_by_id()` to make it possible to remove
   specific sections.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 20 calendar days.
 - 20 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - adapt to changes in `git-ref` ([`d40beb3`](https://github.com/Byron/gitoxide/commit/d40beb3b5744139b56ed68de4caa62a242df2d3a))
 * **Uncategorized**
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - Merge branch 'fix-smart-release' ([`aa80b60`](https://github.com/Byron/gitoxide/commit/aa80b606e5570f327660cca42ea81581a6e9d5e3))
    - make fmt ([`7b9c065`](https://github.com/Byron/gitoxide/commit/7b9c06547b75929e3e5bf4240f43c7e9bc7d54e0))
    - Merge branch 'clone' ([`507dc7e`](https://github.com/Byron/gitoxide/commit/507dc7e706cb3c9d89d048b3aff5df239a9b6788))
    - add `parse::section::header::is_valid_subsection()` function. ([`aa5d66f`](https://github.com/Byron/gitoxide/commit/aa5d66f60bd6c9ef404ebc120b613e0cf055b2c9))
    - greatly improve whitespace handling when removing values. ([`9c1e639`](https://github.com/Byron/gitoxide/commit/9c1e639979a9615fd8334ce0e3a809df137776f6))
    - Add various methods to iterate sections along with their id, and mutate them. ([`5df2a2a`](https://github.com/Byron/gitoxide/commit/5df2a2a5a9addbda7dcc68b2f8f7f4a48d9720c6))
    - `File::remove_section()` was fixed to allow re-adding a similarly named section. ([`e533993`](https://github.com/Byron/gitoxide/commit/e533993e8f861ba7a6600aab114ddfecc8a85ee2))
    - improve clarity docs related to mutating sections ([`769e897`](https://github.com/Byron/gitoxide/commit/769e89790d8c4146263c84d4d5c9dff7d5018919))
    - Merge branch 'main' into new-http-impl ([`702a161`](https://github.com/Byron/gitoxide/commit/702a161ef11fc959611bf44b70e9ffe04561c7ad))
    - Merge branch 'fetch-pack' ([`3c49400`](https://github.com/Byron/gitoxide/commit/3c49400809c7c2120f4ce704c19a0421545b5acd))
    - Merge branch 'fix-git-features' ([`82fd251`](https://github.com/Byron/gitoxide/commit/82fd251ac80d07bc9da8a4d36e517aa35580d188))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
</details>

## 0.8.0 (2022-09-20)

### Changed

 - <csr-id-5ad296577d837b0699b4718fa2be3d0978c4e342/> `git-config` now uses `git-config-value`.

### Changed (BREAKING)

 - <csr-id-27fb1ce27d2985eb1ee8bee5fffaf759902571fb/> Add `Kind::GitInstallation` for a way to obtain special git-installation configuration paths.
   Note that these are lazily cached as they call the `git` binary.
 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 22 calendar days.
 - 22 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - Add `Kind::GitInstallation` for a way to obtain special git-installation configuration paths. ([`27fb1ce`](https://github.com/Byron/gitoxide/commit/27fb1ce27d2985eb1ee8bee5fffaf759902571fb))
    - upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
    - `git-config` now uses `git-config-value`. ([`5ad2965`](https://github.com/Byron/gitoxide/commit/5ad296577d837b0699b4718fa2be3d0978c4e342))
    - port tests over as well ([`9b28df2`](https://github.com/Byron/gitoxide/commit/9b28df22b858b6f1c9ca9b07a5a1c0cc300b50f0))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **Uncategorized**
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - make fmt ([`429cccc`](https://github.com/Byron/gitoxide/commit/429cccc5831c25a7205a12dc7a0443ac48616e2c))
    - Merge branch 'filter-refs' ([`3773b92`](https://github.com/Byron/gitoxide/commit/3773b92b8372c9a40a74d281149ca65b057a7da9))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Release git-features v0.22.6 ([`c9eda72`](https://github.com/Byron/gitoxide/commit/c9eda729d8f8bc266c7516c613d38acfb83a4743))
    - upgrade all dependencies, except for `windows` ([`2968181`](https://github.com/Byron/gitoxide/commit/29681819ffe53d3926d631dc482f71d6200cb549))
    - make fmt ([`535e967`](https://github.com/Byron/gitoxide/commit/535e967666c6da657ff1b7eff7c64ab27cafb182))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - Merge branch 'main' into filter-refs-by-spec ([`9aa1d3d`](https://github.com/Byron/gitoxide/commit/9aa1d3dc46d4b1c76af257f573aff3aeef2d3fa8))
    - Release git-features v0.22.4, git-url v0.8.0, safety bump 4 crates ([`1d4600a`](https://github.com/Byron/gitoxide/commit/1d4600ae51475c2e225f96c16c41e2c4a2b3f2aa))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Release git-path v0.4.2, git-config-value v0.7.0 ([`c48fb31`](https://github.com/Byron/gitoxide/commit/c48fb3107d29f9a06868b0c6de40567063a656d1))
    - Merge branch 'main' into filter-refs-by-spec ([`cef0b51`](https://github.com/Byron/gitoxide/commit/cef0b51ade2a3301fa09ede7a425aa1fe3527e78))
</details>

## 0.7.1 (2022-08-28)

Maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 4 calendar days.
 - 4 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#XXX](https://github.com/Byron/gitoxide/issues/XXX)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#XXX](https://github.com/Byron/gitoxide/issues/XXX)**
    - prepare changelogs prior to release ([`8c0bca3`](https://github.com/Byron/gitoxide/commit/8c0bca37ff9fbaadbe55561fb2b0d649980c95b1))
 * **Uncategorized**
    - Release git-object v0.20.3, git-ref v0.15.4, git-config v0.7.1, git-diff v0.18.0, git-traverse v0.16.3, git-pack v0.22.0, git-odb v0.32.0, git-url v0.7.3, git-transport v0.19.3, git-protocol v0.19.1, git-refspec v0.1.1, git-repository v0.23.0, safety bump 6 crates ([`85a3bed`](https://github.com/Byron/gitoxide/commit/85a3bedd68d2e5f36592a2f691c977dc55298279))
    - Release git-attributes v0.3.3, git-ref v0.15.3, git-index v0.4.3, git-worktree v0.4.3, git-testtools v0.8.0 ([`baad4ce`](https://github.com/Byron/gitoxide/commit/baad4ce51fe0e8c0c1de1b08148d8303878ca37b))
    - Release git-features v0.22.3, git-revision v0.4.4 ([`c2660e2`](https://github.com/Byron/gitoxide/commit/c2660e2503323531ba02519eaa51124ee22fec51))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
</details>

## 0.7.0 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-69ec5940d3f37eb4dace8f1ed7616b5988984d15/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes

### New Features

 - <csr-id-9937d0e00df3a523484c7ae2850be2712a1a4c9a/> `File::set_raw_value_filter()` to set values only in sections passing a filter.
 - <csr-id-17455c9d93ad38bfee2560f5a4e60324dee3b4e5/> `File::section_mut_or_create_new_filter()` to allow chosing which sections to add values to.
 - <csr-id-5902f54b93101a6290fcf89f9f13fdbea3678e00/> `File::section_mut_or_create_new(…)` to obtain an existing or new section for mutation.
 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs

### Bug Fixes

 - <csr-id-08c50a47fa901457194539c7db74ad47ab2f8b60/> Properly handle boolean values such that `a` is true but `a=` is false.
   This is even consistent when no booleans are used, such that `a` has no
   value as if it is not present, it's only available for booleans which
   must be specified.
 - <csr-id-7c585162454c476fe93f032c8a2329cffd7c054f/> Keep track of a severe limitation and prepare tests for fixing it.
   This also changes behaviour, but merely removes a hack in `Boolean`
   which considered empty strings true, even though they are supposed to be
   false.

### Changed (BREAKING)

 - <csr-id-2b2357e9cc54539e0dbe7c0e22802f2b884160d8/> Add `File::set_raw_value()` to unconditionally set single values, and make the value itself easier to provide.

### New Features (BREAKING)

 - <csr-id-b6cd6ace412b0c0df4bacbe7ed7ef6608f27909c/> `file::SectionMut::push()` now supports values without key-value separator.
   These make a difference as those without `=` are considered boolean
   true.
   Currently pushing onto a section is the only way to write them.

### Other (BREAKING)

 - <csr-id-69ec5940d3f37eb4dace8f1ed7616b5988984d15/> `File::set_raw_[multi_]value()` to `::set_existing_raw_[multi_]value`.
   This makes clear that the method will fail if the value doesn't yet
   exist.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 5 calendar days.
 - 6 days passed between releases.
 - 11 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - Properly handle boolean values such that `a` is true but `a=` is false. ([`08c50a4`](https://github.com/Byron/gitoxide/commit/08c50a47fa901457194539c7db74ad47ab2f8b60))
    - fix config tests on windows ([`7a871c2`](https://github.com/Byron/gitoxide/commit/7a871c2a5691ae973804ff66af9608070733b366))
    - Keep track of a severe limitation and prepare tests for fixing it. ([`7c58516`](https://github.com/Byron/gitoxide/commit/7c585162454c476fe93f032c8a2329cffd7c054f))
    - `file::SectionMut::push()` now supports values without key-value separator. ([`b6cd6ac`](https://github.com/Byron/gitoxide/commit/b6cd6ace412b0c0df4bacbe7ed7ef6608f27909c))
    - refactor ([`5415449`](https://github.com/Byron/gitoxide/commit/541544953c52ff3df8c8e21f6aca366840faca3e))
    - `File::set_raw_value_filter()` to set values only in sections passing a filter. ([`9937d0e`](https://github.com/Byron/gitoxide/commit/9937d0e00df3a523484c7ae2850be2712a1a4c9a))
    - `File::section_mut_or_create_new_filter()` to allow chosing which sections to add values to. ([`17455c9`](https://github.com/Byron/gitoxide/commit/17455c9d93ad38bfee2560f5a4e60324dee3b4e5))
    - Add `File::set_raw_value()` to unconditionally set single values, and make the value itself easier to provide. ([`2b2357e`](https://github.com/Byron/gitoxide/commit/2b2357e9cc54539e0dbe7c0e22802f2b884160d8))
    - `File::section_mut_or_create_new(…)` to obtain an existing or new section for mutation. ([`5902f54`](https://github.com/Byron/gitoxide/commit/5902f54b93101a6290fcf89f9f13fdbea3678e00))
    - `File::set_raw_[multi_]value()` to `::set_existing_raw_[multi_]value`. ([`69ec594`](https://github.com/Byron/gitoxide/commit/69ec5940d3f37eb4dace8f1ed7616b5988984d15))
 * **Uncategorized**
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Release git-path v0.4.1 ([`5e82346`](https://github.com/Byron/gitoxide/commit/5e823462b3deb904f5d6154a7bf114cef1988224))
    - Merge branch 'remote-ls-refs' ([`39d585d`](https://github.com/Byron/gitoxide/commit/39d585d9f9ac6f3ecf51359c8e37f0a50e21ed45))
    - thanks clippy ([`2770431`](https://github.com/Byron/gitoxide/commit/2770431f8742d6249574f53f06ec0026f9d241d6))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - use docsrs feature in code to show what is feature-gated automatically on docs.rs ([`b1c40b0`](https://github.com/Byron/gitoxide/commit/b1c40b0364ef092cd52d03b34f491b254816b18d))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - pass --cfg docsrs when compiling for https://docs.rs ([`5176771`](https://github.com/Byron/gitoxide/commit/517677147f1c17304c62cf97a1dd09f232ebf5db))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
</details>

## 0.6.1 (2022-08-17)

A maintenance release without user facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 17 commits contributed to the release over the course of 26 calendar days.
 - 26 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#482](https://github.com/Byron/gitoxide/issues/482)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#482](https://github.com/Byron/gitoxide/issues/482)**
    - refactor ([`1ee9918`](https://github.com/Byron/gitoxide/commit/1ee991847a5adeaaeb6e80ae626c28b0ba89e0af))
 * **Uncategorized**
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Merge branch 'submodule-open' ([`8f5f3ab`](https://github.com/Byron/gitoxide/commit/8f5f3ab588cf0165d50a82365119ad5804745017))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge branch 'main' into write-index-v2 ([`a938986`](https://github.com/Byron/gitoxide/commit/a938986877302c197d1aed087594c5605416fe5f))
    - Merge branch 'main' into remote-ls-refs ([`de61c4d`](https://github.com/Byron/gitoxide/commit/de61c4db7855d6925d66961f62ae3d12cc4acf78))
    - thanks clippy ([`4bd747c`](https://github.com/Byron/gitoxide/commit/4bd747cb3e126fe5b1d540270cfbd731cffd42ef))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - commit strange changes to be able to do anything. ([`237b21d`](https://github.com/Byron/gitoxide/commit/237b21da8465200dcb8b5f7dc324a97bf653a23d))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - commit strange crlf-file that makes everything impossible ([`1dd9f9a`](https://github.com/Byron/gitoxide/commit/1dd9f9a9320813fe5b40578ee4826a1da575c05c))
    - make fmt ([`47724c0`](https://github.com/Byron/gitoxide/commit/47724c0edb382c036a3fc99884becfd2b0740d4b))
    - Merge branch 'kianmeng-fix-typos' ([`4e7b343`](https://github.com/Byron/gitoxide/commit/4e7b34349c0a01ad8686bbb4eb987e9338259d9c))
    - Fix typos ([`e9fcb70`](https://github.com/Byron/gitoxide/commit/e9fcb70e429edb2974afa3f58d181f3ef14c3da3))
</details>

## 0.6.0 (2022-07-22)

<csr-id-32d5b3c695d868ba93755123a25b276bfbe55e0a/>
<csr-id-9cd99337333f5ef4b30e0ec9461fc087699576e6/>
<csr-id-0076dcf9b37f1d633bdad5573b40d34a9fbaba90/>
<csr-id-a8604a237782f8d60a185d4730db57bad81424a6/>

### New Features

 - <csr-id-1bc96bf378d198b012efce9ec9e5b244a91f62bc/> following includes is now non-fatal by default
   Otherwise it would be relatively easy to fail gitoxide startup,
   and we want to be closer to the behaviour in git which ignores
   most of the errors.
 - <csr-id-f9ce1b5411f1ac788f71060ecf785dda9dfd87bf/> `File::from_git_dir()` as comfortable way to instantiate most complete git configuration.
 - <csr-id-14a68a6a78a09f8ae56e30e3b7501de66ef31fdc/> `File` now compares actual content, ignoring whitespace and comments.
 - <csr-id-7dadfd82494d47e36d3f570988eaf3c6b628977f/> `File::new_environment_overrides()` to easily instantiate overrides from the environment.
 - <csr-id-146eeb064822839bc46fd37a247a1b9a84f64e40/> `File::new_globals()` can instantiate non-local configuration with zero-configuration.
 - <csr-id-e701e053fd05850973930be0cefe73e8f3604d40/> `Source::storage_location()` to know where files should be located.
 - <csr-id-fff088485dd5067976cc93d525903b39aafea76a/> `file::ValueMut::(section|into_section_mut)()` to go from value to the owning section.
   This can be useful if the value was obtained using `raw_value_mut()`.
 - <csr-id-f5f2d9b3fef98d9100d713f9291510fa4aa27867/> `Source::is_in_repository()` to find out if a source is in the repository.
 - <csr-id-91e718f0e116052b64ca436d7c74cea79529e696/> `parse::key` to parse a `remote.origin.url`-like key to identify a value
 - <csr-id-26147a7a61a695eda680808ee4aab44a890b2964/> Add `File::detect_newline_style()`, which does at it says.
 - <csr-id-0ad1c9a5280cc172432b5258e0f79898721bac68/> `File::frontmatter()` and `File::sections_and_postmatter()`.
 - <csr-id-fc7e311b423c5fffb8240d9d0f917ae7139a6133/> `parse::Event::to_bstr_lossy()` to get a glimpse at event content.
 - <csr-id-09966a8ea4eaa3e0805e04188de86dd1bac9f388/> `File::append()` can append one file to another rather losslessly.
   The loss happens as we, maybe for the wrong reasons, automatically
   insert newlines where needed which can only be done while we still know
   the file boundaries.
 - <csr-id-56ae5744e8957e617f3a0ebc4d725846b18d93f8/> `file::Section::meta()` to access a section's metadata.
 - <csr-id-6f97bf0c3e7164855cf5aa53462dbc39c430e03f/> `File::sections()` to obtain an iterator over all sections, in order.
 - <csr-id-5418bc70e67476f8778656f2d577f1f9aa65ffbe/> place spaces around `key = value` pairs, or whatever is used in the source configuration.
 - <csr-id-8118644625dc25b616e5f33c85f5100d600766e4/> proper escaping of value bytes to allow round-tripping after mutation
 - <csr-id-9f59356b4f6a1f5f7f35a62c9fbe4859bf8e8e5f/> whitespace in newly pushed keys is derived from first section value.
   That way, newly added key-value pairs look like they should assuming
   all keys have the same indentation as the first key in the section.
   
   If there is no key, then the default whitespace will be double-tabs
   like what's commmon in git.
 - <csr-id-db1f34dfb855058ac08e97d4715876b5db712f61/> `File::from_str()` implementation, to support `let config: File = "[core]".parse()?`
 - <csr-id-9157717c2fb143b5decbdf60d18cc2bd99dde775/> whitespace in mutable sections can be finely controlled, and is derived from existing sections
 - <csr-id-ae3895c7882e0a543a44693faee5f760b49b54d7/> `parse::Header::new(…)` with sub-section name validation
 - <csr-id-d087f12eec73626eb327eaacef8ebb3836b02381/> Add `parse::(Event|section::Header|Comment)::write_to(…)`.
   Now it's possible to serialize these types in a streaming fashion and
   without arbitrarily enforcing UTF-8 on it
 - <csr-id-5a8f242ee98793e2467e7bc9806f8780b9d320ce/> `serde1` feature to add limited serde support

### Bug Fixes

 - <csr-id-6c1588fd1a2fa80fd866787cbf4bcc6e5b51abe6/> maintain insertion order of includes on per-section basis at least.
   Note that git inserts values right after the include directive,
   'splitting' the section, but we don't do that and insert new values
   after the section. Probably no issue in practice while keeping
   our implementation simple.
 - <csr-id-f7bd2caceb87a179288030e0771da2e4ed6bd1e4/> maintain newline format depending on what's present or use platform default.
   Previously implicit newlines when adding new sections or keys to
   sections was always `\n` which isn't correct on windows.
   
   Now the newline style is detected and used according to what's present,
   or in the lack of content, defaults to what's correct for the platform.
 - <csr-id-0d07ef1aa4a9e238c20249d4ae2ed19e6740308a/> validate incoming conifguration keys when interpreting envirnoment variables.
 - <csr-id-6b901843cb18b3d31f8b0b84bb9ebbae279aff19/> `Boolean` can use numbers to indicate true or false, drops support for `one` and `zero`.
 - <csr-id-94dde44e8dd1a0b8d4e11f2627a3f6b345a15989/> `file::MutableSection::remove()` now actually removes keys _and_ values.
 - <csr-id-048b92531eb877a5a128e702504891bf1e31becf/> `file::MutableMultiValue` escapes input values and maintains key separator specific whitespace.
 - <csr-id-f911707b455ba6f3800b85f667f91e4d56027b91/> value normalization (via `value::normalize()` handles escape sequences.
   The latter ones are `\n`, `\t` and `\b` which are the only supported
   ones in values of git-config files.
 - <csr-id-44dfec07480cc2ac6fd01674b748cc03af51fed1/> stable sort order for `File::sections_by_name_with_header()`
 - <csr-id-1ea919d5ff81ab7b01b8201386ef63c7e081b537/> count newlines (for error display) in multi-line values as well
 - <csr-id-1e71e71c984289f0d7e0a39379ee6728918b7dc5/> auto-normalize string values to support quote removal in case of strings.
   Related to https://github.com/starship/starship/pull/3883 .

### Other

 - <csr-id-32d5b3c695d868ba93755123a25b276bfbe55e0a/> :Events::from_bytes()` with `filter` support.

### Changed (BREAKING)

 - <csr-id-17c83d55f8942788aac5eb1bea22a48daa045bf4/> add `File::resolve_includes()` and move its error type to `file::includes`.
 - <csr-id-5221676e28f2b6cc1a7ef1bdd5654b880965f38c/> add `File::from_bytes_owned()` and remove `File::from_path_with_buf()`
 - <csr-id-98d45c2f59863fdee033b38e757cec09593f6892/> remove `File::from_env_paths()`.
   It's replaced by its more comfortable `new_globals()`.
 - <csr-id-230a523593afcfb8720db965ff56265aaceea772/> untangle `file::init::…` `Option` and `Error` types.
   This moves types to where they belong which is more specific instead
   of having a catch-all `Error` and `Options` type.
 - <csr-id-3f3ff11a6ebe9775ee5ae7fc0ec18a94b5b46d61/> rename `parse::Comment::(comment_tag|comment)` to `::tag|text` and `parse::Section::section_header` to `::header`.
 - <csr-id-6f4eea936d64fb9827277c160f989168e7b1dba2/> Associate `file::Metadata` with each `File`.
   This is the first step towards knowing more about the source of each
   value to filter them based on some properties.
   
   This breaks various methods handling the instantiation of configuration
   files as `file::Metadata` typically has to be provided by the caller
   now or be associated with each path to read configuration from.
 - <csr-id-b672ed7667a334be3d45c59f4727f12797b340da/> rename `file::SectionBody` to `file::section::Body`.
 - <csr-id-3bea26d7d2a9b5751c6c15e1fa9a924b67e0159e/> Remove `File::sections_by_name_with_header()` as `::sections_by_name()` now returns entire sections.
 - <csr-id-41b3e622ee71943c285eadc518150fc7b6c92361/> create `resolve_includes` options to make space for more options when loading paths.
 - <csr-id-cabc8ef0e31c954642525e7693009a7fe4b4c465/> rename `path::Options` into `path::Context`.
   It's not an option if it's required context to perform a certain
   operation.
 - <csr-id-3de0cfd81523e4ba7cc362d8625f85ebf8fd9172/> All accessors in `File` are now using `impl AsRef<str>` where possible for added comfort.
 - <csr-id-3d25fe6c7a52529488fab19c927d64a1bc75838f/> Much more comfortable API `file::*Mut` types thanks to `impl Into/AsRef`.
 - <csr-id-393b392d515661e5c3e60629319fdab771c3d3f0/> Rename `Mutable*` into `$1Mut` for consistency.
 - <csr-id-0a7391a6575f4035c51a46d34fa20c69e9d078e9/> conform APIs of `file::MutableValue` and `file::MutableMultiValue`.
   There are more renames and removals than worth mentioning here given the
   current adoption of the crate.
 - <csr-id-83a0922f06081312b79908835dac2b7f4e849bb3/> rename `file::MutableSection::set_leading_space()` to `set_leading_whitespace()`.
   The corresponding getter was renamed as well to `leading_whitespace()`.
 - <csr-id-219cf7ae0b35b3ac92f97974be52cd022698e01f/> Enforce `parse::section::Header::new()` by making its fields private.
 - <csr-id-4f6cd8cf65c2d8698bffe327a19031c342b229a6/> Add `File::write_to()` and `File::to_bstring()`; remove some `TryFrom` impls.
   Now `File` can be serialized in a streaming fashion and without the
   possibility for UTF8 conversion issues.
   
   Note that `Display` is still imlpemented with the usual caveats.
 - <csr-id-0e392f81e99c8c0ff29f41b9b86afd57cd99c245/> remove `Integer::to_bstring()` as well as some `TryFrom` impls.
   Note that it can still display itself like before via
   `std::fmt::Display`.
 - <csr-id-b22732a2ab17213c4a1020859ec41f25ccabfbfc/> remove `Boolean::to_bstring()` along with a few `From` impls.
   These were superfluous and aren't useful in practice.
   Note that serialization is still implemented via `Display`.
 - <csr-id-65c520c4de8187884f87059adf5cef9cbdcd90a2/> allocation free `File::sections_by_name()` and `File::sections_by_name_with_header()`.
 - <csr-id-ac57c4479e7b6867e8b8e71f7cf76de759dc64a2/> `Path::interpolate()` now takes `path::interpolate::Options` instead of three parameters.
 - <csr-id-0915051798dd782b40617a1aa16abd71f6db1175/> remove `String` type in favor of referring to the `File::string()` method.
   The wrapper had no effect whatsoever except for adding complexity.
 - <csr-id-9cadc6f0cbaad0ac23f5469db2f040aecfbfb82c/> Simplify `Boolean` to be a wrapper around `bool`.
   Previously it tried hard not to degenerate information, making it a
   complicated type.
   
   However, in practice nobody cares about the exact makeup of the boolean,
   and there is no need to serialize a boolean faithfully either.
   
   Instead, those who want to set a value just set any value as a string,
   no need for type safety there, and we take care of escaping values
   properly on write.
 - <csr-id-703922dd4e1e5b27835298217ff4eb8ef1dc57ce/> Use bitflags for `color::Attribute` instead of `Vec` of enums.
   This is less wasteful and sufficient for git, so it should be sufficient
   for us, especially since attributes are indeed a set and declaring
   one twice has no effect.
 - <csr-id-3fc4ac04f46f869c6e3a94ce4bb8a5737aa0c524/> simplify `Color` API.
   For now we only parse and serialize for display, but more uses are
   enabled when needed and trivially.
 - <csr-id-14149eea54e2e8a25ac0ccdb2f6efe624f6eaa22/> remove `parse::Events::from_path` and `File::at`
   The latter has been replaced with `File::from_path_with_buf(…)` and
   is a low-level way to load just a single config file, purposefully
   uncomfortable as it will not resolve includes.
   
   The initialization API will need some time to stabilize.
 - <csr-id-73adceeae12270c0d470d4b7271c1fd6089d5c2d/> Slim down API surface of `parse::Events`.
   It's more of a 'dumb' structure now than before, merely present
   to facilitate typical parsing than something special on its own.
 - <csr-id-2e47167e4a963743494b2df6b0c15800cb876dd0/> remove `File::new()` method in favor of `File::default()`.
 - <csr-id-ea6765093b5475912ba1aa81d4440cbf5dd49fb6/> rename `parse::event::List` to `parse::Events`
 - <csr-id-89f5fca843d999c5bea35fb3fe2a03dc3588f74e/> rename `parse::State` to `parse::event::List`
 - <csr-id-3cdb0890b71e62cfa92b1ed1760c88cb547ec729/> move `value::*` into the crate root, except for `Error` and `normalize_*()`.
 - <csr-id-748d921efd7469d5c19e40ddcb9099e2462e3bbc/> rename `value::parse::Error` to `value::Error`.
 - <csr-id-7e8a22590297f2f4aab76b53be512353637fb651/> rename `value::TrueVariant` to `value::boolean::True`
 - <csr-id-8bcaec0599cf085a73b344f4f53fc023f6e31430/> rename `IntegerSuffix` to `integer::Suffix`
 - <csr-id-d085037ad9c067af7ce3ba3ab6e5d5ddb45b4057/> rename `value::Color(Attribute|Value)` to `value::color::Attribute` and `value::color::Name`.
 - <csr-id-a0f6252343a62b0b55eef02888ac00c09100687a/> Turn `parse::ParseOrIoError` into `parse::state::from_path::Error`
 - <csr-id-b6b31e9c8dd8b3dc4860431069bb1cf5eacd1702/> rename `parse::ParsedComment` into `parse::Comment`
 - <csr-id-239cbfb450a8cddfc5bec1de21f3dc54fab914ce/> rename `parse::Section*` related types.
   These are now located in `section::*`.
 - <csr-id-60af4c9ecb1b99f21df0e8facc33e5f6fc70c424/> rename `parse::Parser` to `parse::State`.
   Furthermore, make `State` the entry point for all parsing, removing
   all free-standing functions that returned a `State`.
 - <csr-id-3724850e0411f1f76e52c6c767fd8cebe8aea0f6/> rename `parser` module to `parse`
 - <csr-id-58b22152a0295998935abb43563e9096589ef53e/> rename `normalize_cow()` to `normalize()` and move all `normalize*` functions from `values` to the `value` module
 - <csr-id-767bedccdae1f3e6faf853d59ecf884a06cc3827/> move `Path` from `values` to `value` module
 - <csr-id-6033f3f93d2356399a661567353a83a044662699/> Move `Boolean` and `String` from `values` into `value` module
 - <csr-id-d4444e18042891b0fe5b9c6e6813fed26df6c560/> move `values::Integer` into `value` module
 - <csr-id-38f31174e8c117af675cdfbc21926133b821ec38/> move `Color` to own `value` module
 - <csr-id-aa630ad6ec2c6306d3307d5c77e272cb24b00ddd/> remove `values::Bytes` - use `values::String` instead.
   Note that these values are always normalized and it's only possible
   to get a raw values using the `raw_value()` API.

### New Features (BREAKING)

 - <csr-id-d003c0f139d61e3bd998a0283a9c7af25a60db02/> Support for `lossy` load mode.
   There is a lot of breaking changes as `file::from_paths::Options` now
   became `file::init::Options`, and the same goes for the error type.
 - <csr-id-1ea26d80f392114349d25ebf88a7b260ee822aa1/> add `_filter()` versions to most access methods.
   That way it's possible to filter values by their origin.
   
   Note that the `remove_section()` methods now return the entire
   removed section, not just the body, which yields more information
   than before including section metadata.
 - <csr-id-cfd974f46d2cbb99e7784a05f5e358fed0d4bcab/> section names are now validated.
 - <csr-id-6ba2f8060768978ad7204e162fb2253ca8843879/> filtering supportort for `parse::Events`.
   That way it's possible to construct Files which are not destined to be
   written back as they only keep events necessary for value access,
   greatly reducing allocations.
 - <csr-id-311d4b447daf8d4364670382a20901468748d34d/> change mostily internal uses of [u8] to BString/BStr
 - <csr-id-edd226719cd04a480274cb7d983b6d5d8bfdbb13/> Path-interpolation makes `home-dir` configurable.
   That way the caller has full control over how the environment is used,
   which also allows more fine-grained control over which config files
   can be included.

### Bug Fixes (BREAKING)

 - <csr-id-a93a156655d640ae63ff7c35b0a1f5d67a5ca20f/> Simplify specifying keys when mutating config values.
 - <csr-id-895ce40aabbe6d6af5b681a0d0942303fd6549a2/> `File::rename_section()` with validation of input arguments.
 - <csr-id-4a01d983f54a7713dea523f6032cbf5bb2b9dde8/> improve normalization; assure no extra copies are made on query.
   We now return our own content, rather than the originals with their
   lifetimes, meaning we bind lifetimes of returned values to our own
   `File` instance. This allows them to be referenced more often, and
   smarter normalization assures we don't copy in the simple cases
   either.
   
   More tests were added as well.
   This is breaking as lifetime changes can cause distruptions, and
   `values?_as()` was removed as well as it's somewhat duplicate
   to higher-level APIs and it wasn't tested at all.
 - <csr-id-c9933c0b0f51d21dc8244b2acc33d7dc8a33f6ce/> Remove `git-config` test utilities from `git-path`.

### Other (BREAKING)

 - <csr-id-9cd99337333f5ef4b30e0ec9461fc087699576e6/> `File::raw_multi_value()` to `File::raw_values()`
 - <csr-id-0076dcf9b37f1d633bdad5573b40d34a9fbaba90/> `File::raw_multi_value_mut()` to `File::raw_values_mut()`
 - <csr-id-a8604a237782f8d60a185d4730db57bad81424a6/> `File::multi_value()` to `File::values()`.
   The latter is better in line with `string()/strings()`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 324 commits contributed to the release over the course of 33 calendar days.
 - 39 days passed between releases.
 - 93 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#331](https://github.com/Byron/gitoxide/issues/331)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 19 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - final documentation review + adjustments prior to release candidate ([`06b86e0`](https://github.com/Byron/gitoxide/commit/06b86e05dd9a712d26456b43c8da0a11870f08df))
    - refactor ([`4dc6594`](https://github.com/Byron/gitoxide/commit/4dc6594686478d9d6cd09e2ba02048624c3577e7))
    - exclude particular assertion which fails on the linux CI. ([`5e0f889`](https://github.com/Byron/gitoxide/commit/5e0f889c1edb862d698a2d344a61f12ab3b6ade7))
    - first sketch of using configuration and environment variables for author/committer ([`330d0a1`](https://github.com/Byron/gitoxide/commit/330d0a19d54aabac868b76ef6281fffdbdcde53c))
    - remove `Permissions` as there is no need for that here. ([`1954ef0`](https://github.com/Byron/gitoxide/commit/1954ef096a58aedb9f568a01e439d5a5cb46c40d))
    - following includes is now non-fatal by default ([`1bc96bf`](https://github.com/Byron/gitoxide/commit/1bc96bf378d198b012efce9ec9e5b244a91f62bc))
    - Allow to skip non-existing input paths without error ([`989603e`](https://github.com/Byron/gitoxide/commit/989603efcdf0064e2bb7d48100391cabc810204d))
    - `File::from_git_dir()` as comfortable way to instantiate most complete git configuration. ([`f9ce1b5`](https://github.com/Byron/gitoxide/commit/f9ce1b5411f1ac788f71060ecf785dda9dfd87bf))
    - Add a way to load multiple configuration files without allocating a read buffer ([`acb4520`](https://github.com/Byron/gitoxide/commit/acb4520a88ab083640c80a7f23a56a2ca3cda335))
    - refactor ([`ec21e95`](https://github.com/Byron/gitoxide/commit/ec21e95f4d9ffac771410947923f27187e88321a))
    - move `Env` test utility into `git-testtools` ([`bd3f4d0`](https://github.com/Byron/gitoxide/commit/bd3f4d014dd7df7a1e25defa8eea7253eec1560a))
    - refactor ([`b073e29`](https://github.com/Byron/gitoxide/commit/b073e2930bed60ccedadd1709cfaa8889e02ffe3))
    - another failing tests that can't be fixed without a refactor ([`e4d8fd7`](https://github.com/Byron/gitoxide/commit/e4d8fd72f1f648a29e56e487827f2328bfc08d03))
    - an attempt to hack newline handling into place for windows newlines ([`dac1463`](https://github.com/Byron/gitoxide/commit/dac146343a0fbe96b6c0990f4fd4e976e0359a7e))
    - Serialize lossily-read configuration files correctly anyway. ([`cfda0c3`](https://github.com/Byron/gitoxide/commit/cfda0c335d759cae0b23cef51f7b85a5f4b11e82))
    - multi-path include test ([`3d89a46`](https://github.com/Byron/gitoxide/commit/3d89a46bf88b1fb5b4aa5da9fd12c7e310be3f9d))
    - refactor ([`8a7fb15`](https://github.com/Byron/gitoxide/commit/8a7fb15f78ce16d5caedd7656e8aa98e72f248a6))
    - fix windows tests ([`fbcf40e`](https://github.com/Byron/gitoxide/commit/fbcf40e16b8fc1ff97dbed2bc22b64bd44a8b99d))
    - finally proper whitespace handling in all the right places for perfect roundtripping to/from string ([`97e5ede`](https://github.com/Byron/gitoxide/commit/97e5ededb0390c1b4f296a35903433de9c519821))
    - serializations maintains some invariants about whitespace where possible. ([`ee10dd5`](https://github.com/Byron/gitoxide/commit/ee10dd5a8ae0dabfee21c1ce146e92c3c9635e8a))
    - refactor ([`9c248ee`](https://github.com/Byron/gitoxide/commit/9c248eeb015495f910f48ce5df3c8fcce905dba7))
    - `File` now compares actual content, ignoring whitespace and comments. ([`14a68a6`](https://github.com/Byron/gitoxide/commit/14a68a6a78a09f8ae56e30e3b7501de66ef31fdc))
    - maintain insertion order of includes on per-section basis at least. ([`6c1588f`](https://github.com/Byron/gitoxide/commit/6c1588fd1a2fa80fd866787cbf4bcc6e5b51abe6))
    - allow insertion of sections while preserving order ([`f5580a3`](https://github.com/Byron/gitoxide/commit/f5580a3635289d96e662aab00e60d801c4e34e1c))
    - a test showing that include ordering isn't correct compared to the including config. ([`4e47df5`](https://github.com/Byron/gitoxide/commit/4e47df5332810f6e46ab682a68e870220ba3a6fb))
    - add `File::resolve_includes()` and move its error type to `file::includes`. ([`17c83d5`](https://github.com/Byron/gitoxide/commit/17c83d55f8942788aac5eb1bea22a48daa045bf4))
    - add `File::from_bytes_owned()` and remove `File::from_path_with_buf()` ([`5221676`](https://github.com/Byron/gitoxide/commit/5221676e28f2b6cc1a7ef1bdd5654b880965f38c))
    - make it necessary to deal with the possibility of no-input in `from_paths_metadata()` . ([`612645f`](https://github.com/Byron/gitoxide/commit/612645f74ffc49229ccd783361b4d455e2284ac0))
    - Don't fail on empty input on the comfort level ([`61ecaca`](https://github.com/Byron/gitoxide/commit/61ecaca43fb871eaff5cf94a8e7f9cc9413a5a77))
    - `File::new_environment_overrides()` to easily instantiate overrides from the environment. ([`7dadfd8`](https://github.com/Byron/gitoxide/commit/7dadfd82494d47e36d3f570988eaf3c6b628977f))
    - prepare for supporting comfortable version of environment overrides ([`45c964a`](https://github.com/Byron/gitoxide/commit/45c964a3f581dc7d3090bbbe26f188d553783fb3))
    - remove `File::from_env_paths()`. ([`98d45c2`](https://github.com/Byron/gitoxide/commit/98d45c2f59863fdee033b38e757cec09593f6892))
    - `File::new_globals()` can instantiate non-local configuration with zero-configuration. ([`146eeb0`](https://github.com/Byron/gitoxide/commit/146eeb064822839bc46fd37a247a1b9a84f64e40))
    - Classify `Source` in accordance for what git actually does. ([`97374e4`](https://github.com/Byron/gitoxide/commit/97374e4d867e82d7be04da2eaa6ef553e0d9a7ff))
    - `Source::storage_location()` to know where files should be located. ([`e701e05`](https://github.com/Byron/gitoxide/commit/e701e053fd05850973930be0cefe73e8f3604d40))
    - `file::ValueMut::(section|into_section_mut)()` to go from value to the owning section. ([`fff0884`](https://github.com/Byron/gitoxide/commit/fff088485dd5067976cc93d525903b39aafea76a))
    - `Source::is_in_repository()` to find out if a source is in the repository. ([`f5f2d9b`](https://github.com/Byron/gitoxide/commit/f5f2d9b3fef98d9100d713f9291510fa4aa27867))
    - `parse::key` to parse a `remote.origin.url`-like key to identify a value ([`91e718f`](https://github.com/Byron/gitoxide/commit/91e718f0e116052b64ca436d7c74cea79529e696))
    - maintain newline format depending on what's present or use platform default. ([`f7bd2ca`](https://github.com/Byron/gitoxide/commit/f7bd2caceb87a179288030e0771da2e4ed6bd1e4))
    - prepare for passing through newline ([`3c06f88`](https://github.com/Byron/gitoxide/commit/3c06f8889854860b731735a8ce2bf532366003ef))
    - Add `File::detect_newline_style()`, which does at it says. ([`26147a7`](https://github.com/Byron/gitoxide/commit/26147a7a61a695eda680808ee4aab44a890b2964))
    - fix docs ([`78e85d9`](https://github.com/Byron/gitoxide/commit/78e85d9786a541aa43ad7266e85dc1da5e71a412))
    - a test for lossy File parsing ([`5e8127b`](https://github.com/Byron/gitoxide/commit/5e8127b395bd564129b20a1db2d59d39307a2857))
    - 'lossy' is now inherited by includes processing ([`88c6b18`](https://github.com/Byron/gitoxide/commit/88c6b185b2e51858b140e4378a5b5730b5cb4075))
    - untangle `file::init::…` `Option` and `Error` types. ([`230a523`](https://github.com/Byron/gitoxide/commit/230a523593afcfb8720db965ff56265aaceea772))
    - Support for `lossy` load mode. ([`d003c0f`](https://github.com/Byron/gitoxide/commit/d003c0f139d61e3bd998a0283a9c7af25a60db02))
    - :Events::from_bytes()` with `filter` support. ([`32d5b3c`](https://github.com/Byron/gitoxide/commit/32d5b3c695d868ba93755123a25b276bfbe55e0a))
    - try to fix attributes, once more ([`a50a396`](https://github.com/Byron/gitoxide/commit/a50a3964dbf01982b5a2c9a8ccd469332b6f9ca1))
    - `File::frontmatter()` and `File::sections_and_postmatter()`. ([`0ad1c9a`](https://github.com/Byron/gitoxide/commit/0ad1c9a5280cc172432b5258e0f79898721bac68))
    - add `_filter()` versions to most access methods. ([`1ea26d8`](https://github.com/Byron/gitoxide/commit/1ea26d80f392114349d25ebf88a7b260ee822aa1))
    - even better handling of newlines ([`50c1753`](https://github.com/Byron/gitoxide/commit/50c1753c6389f29279d278fbab1afbd9ded34a76))
    - refactor ([`df94c67`](https://github.com/Byron/gitoxide/commit/df94c6737ba642fff40623f406df0764d5bd3c43))
    - rename `parse::Comment::(comment_tag|comment)` to `::tag|text` and `parse::Section::section_header` to `::header`. ([`3f3ff11`](https://github.com/Byron/gitoxide/commit/3f3ff11a6ebe9775ee5ae7fc0ec18a94b5b46d61))
    - `parse::Event::to_bstr_lossy()` to get a glimpse at event content. ([`fc7e311`](https://github.com/Byron/gitoxide/commit/fc7e311b423c5fffb8240d9d0f917ae7139a6133))
    - finally fix newline behaviour ([`c70e135`](https://github.com/Byron/gitoxide/commit/c70e135ecbbce8c696a6ab542ae20f5b5981dfdf))
    - Be smarter about which newline style to use by guessing it based onprior events ([`25ed92e`](https://github.com/Byron/gitoxide/commit/25ed92e66bf4345f852e7e84741079c61ae896c8))
    - `File::append()` can append one file to another rather losslessly. ([`09966a8`](https://github.com/Byron/gitoxide/commit/09966a8ea4eaa3e0805e04188de86dd1bac9f388))
    - A test to validate frontmatter isn't currently handled correctly when appending ([`4665e87`](https://github.com/Byron/gitoxide/commit/4665e876df4ac6ab9135c10ee69b5408b89b5313))
    - `file::Section::meta()` to access a section's metadata. ([`56ae574`](https://github.com/Byron/gitoxide/commit/56ae5744e8957e617f3a0ebc4d725846b18d93f8))
    - refactor ([`d60025e`](https://github.com/Byron/gitoxide/commit/d60025e317d2b5f34f3569f321845bbb557ba2e7))
    - `File::sections()` to obtain an iterator over all sections, in order. ([`6f97bf0`](https://github.com/Byron/gitoxide/commit/6f97bf0c3e7164855cf5aa53462dbc39c430e03f))
    - Associate `file::Metadata` with each `File`. ([`6f4eea9`](https://github.com/Byron/gitoxide/commit/6f4eea936d64fb9827277c160f989168e7b1dba2))
    - rename `file::SectionBody` to `file::section::Body`. ([`b672ed7`](https://github.com/Byron/gitoxide/commit/b672ed7667a334be3d45c59f4727f12797b340da))
    - Remove `File::sections_by_name_with_header()` as `::sections_by_name()` now returns entire sections. ([`3bea26d`](https://github.com/Byron/gitoxide/commit/3bea26d7d2a9b5751c6c15e1fa9a924b67e0159e))
    - A way to more easily set interpolation even without following includes. ([`9aa5acd`](https://github.com/Byron/gitoxide/commit/9aa5acdec12a0721543c6bcc39ffe6bd734f9a69))
    - create `resolve_includes` options to make space for more options when loading paths. ([`41b3e62`](https://github.com/Byron/gitoxide/commit/41b3e622ee71943c285eadc518150fc7b6c92361))
    - rename `path::Options` into `path::Context`. ([`cabc8ef`](https://github.com/Byron/gitoxide/commit/cabc8ef0e31c954642525e7693009a7fe4b4c465))
    - try to fix attributes, once more ([`207e483`](https://github.com/Byron/gitoxide/commit/207e483620b29efb029c6ee742c0bb48d54be020))
    - validate incoming conifguration keys when interpreting envirnoment variables. ([`0d07ef1`](https://github.com/Byron/gitoxide/commit/0d07ef1aa4a9e238c20249d4ae2ed19e6740308a))
    - try to fix filter settings, but it doesn't seem to work ([`9750b7a`](https://github.com/Byron/gitoxide/commit/9750b7a1f01d6f0690221c6091b16c51784df0a3))
    - sketch new section and metadata ([`9cb9acb`](https://github.com/Byron/gitoxide/commit/9cb9acb7b7ebada4d6bb3eef199337912ceeaa36))
    - add `Source` type to allow knowing where a particular value is from. ([`c92d5c6`](https://github.com/Byron/gitoxide/commit/c92d5c6a223e377c10c2ca6b822e7eeb9070e12c))
    - `Boolean` can use numbers to indicate true or false, drops support for `one` and `zero`. ([`6b90184`](https://github.com/Byron/gitoxide/commit/6b901843cb18b3d31f8b0b84bb9ebbae279aff19))
    - All accessors in `File` are now using `impl AsRef<str>` where possible for added comfort. ([`3de0cfd`](https://github.com/Byron/gitoxide/commit/3de0cfd81523e4ba7cc362d8625f85ebf8fd9172))
    - Much more comfortable API `file::*Mut` types thanks to `impl Into/AsRef`. ([`3d25fe6`](https://github.com/Byron/gitoxide/commit/3d25fe6c7a52529488fab19c927d64a1bc75838f))
    - Rename `Mutable*` into `$1Mut` for consistency. ([`393b392`](https://github.com/Byron/gitoxide/commit/393b392d515661e5c3e60629319fdab771c3d3f0))
    - `file::MutableSection::remove()` now actually removes keys _and_ values. ([`94dde44`](https://github.com/Byron/gitoxide/commit/94dde44e8dd1a0b8d4e11f2627a3f6b345a15989))
    - many more tests for MutableSection ([`ac843cb`](https://github.com/Byron/gitoxide/commit/ac843cbef4a6322be706b978e6691bc36c5e458f))
    - refactor ([`701266e`](https://github.com/Byron/gitoxide/commit/701266e6e52456c0c1938732c260be19ec8029c9))
    - conform APIs of `file::MutableValue` and `file::MutableMultiValue`. ([`0a7391a`](https://github.com/Byron/gitoxide/commit/0a7391a6575f4035c51a46d34fa20c69e9d078e9))
    - `file::MutableMultiValue` escapes input values and maintains key separator specific whitespace. ([`048b925`](https://github.com/Byron/gitoxide/commit/048b92531eb877a5a128e702504891bf1e31becf))
    - place spaces around `key = value` pairs, or whatever is used in the source configuration. ([`5418bc7`](https://github.com/Byron/gitoxide/commit/5418bc70e67476f8778656f2d577f1f9aa65ffbe))
    - avoid extra copies when setting values and escaping them ([`a7eff01`](https://github.com/Byron/gitoxide/commit/a7eff0166f200a403d4dba320280f20a70e9afc7))
    - refactor ([`15cd1d2`](https://github.com/Byron/gitoxide/commit/15cd1d2ba447ff27819f6cf398d31e96ff11b213))
    - more empty-value tests ([`511985a`](https://github.com/Byron/gitoxide/commit/511985a8084f2a00e0550e5f2a85c93779385a1b))
    - default space is just a single tab, not two ones ([`7e03b83`](https://github.com/Byron/gitoxide/commit/7e03b835bd6f0f5b3f00dbc63e7960ce6364eaef))
    - proper escaping of value bytes to allow round-tripping after mutation ([`8118644`](https://github.com/Byron/gitoxide/commit/8118644625dc25b616e5f33c85f5100d600766e4))
    - refactor ([`afa736a`](https://github.com/Byron/gitoxide/commit/afa736aba385bd52e7f11fd89538aea99787ac9d))
    - a few tests for `MutableValue` showing that it's too buggy right now ([`5e6f9d9`](https://github.com/Byron/gitoxide/commit/5e6f9d909db41926e829e464abc53ef05fbf620b))
    - rename `file::MutableSection::set_leading_space()` to `set_leading_whitespace()`. ([`83a0922`](https://github.com/Byron/gitoxide/commit/83a0922f06081312b79908835dac2b7f4e849bb3))
    - whitespace in newly pushed keys is derived from first section value. ([`9f59356`](https://github.com/Byron/gitoxide/commit/9f59356b4f6a1f5f7f35a62c9fbe4859bf8e8e5f))
    - `File::from_str()` implementation, to support `let config: File = "[core]".parse()?` ([`db1f34d`](https://github.com/Byron/gitoxide/commit/db1f34dfb855058ac08e97d4715876b5db712f61))
    - whitespace in mutable sections can be finely controlled, and is derived from existing sections ([`9157717`](https://github.com/Byron/gitoxide/commit/9157717c2fb143b5decbdf60d18cc2bd99dde775))
    - refactor ([`c88eea8`](https://github.com/Byron/gitoxide/commit/c88eea87d7ece807ca5b1753b47ce89d3ad6a502))
    - refactor ([`a0d6caa`](https://github.com/Byron/gitoxide/commit/a0d6caa243aa293386d4ad164e1604f0e71c2cf3))
    - auto-compute whitespace for sections, even though it probably needs to be better than that ([`ee9ac95`](https://github.com/Byron/gitoxide/commit/ee9ac953180886cc483e1125b7f4e172af92c3ce))
    - validation for Keys and header names ([`59ec7f7`](https://github.com/Byron/gitoxide/commit/59ec7f7bf019d269573f8cc69f6d34b9458b1f1a))
    - Simplify specifying keys when mutating config values. ([`a93a156`](https://github.com/Byron/gitoxide/commit/a93a156655d640ae63ff7c35b0a1f5d67a5ca20f))
    - `File::rename_section()` with validation of input arguments. ([`895ce40`](https://github.com/Byron/gitoxide/commit/895ce40aabbe6d6af5b681a0d0942303fd6549a2))
    - re-add newlines after multi-line values ([`9a2f597`](https://github.com/Byron/gitoxide/commit/9a2f59742cf94643c5b9967b76042bcc7a4e1a71))
    - more header escaping tests ([`12cf005`](https://github.com/Byron/gitoxide/commit/12cf0052d92ee5bee1926f50c879526b5903c175))
    - Enforce `parse::section::Header::new()` by making its fields private. ([`219cf7a`](https://github.com/Byron/gitoxide/commit/219cf7ae0b35b3ac92f97974be52cd022698e01f))
    - `parse::Header::new(…)` with sub-section name validation ([`ae3895c`](https://github.com/Byron/gitoxide/commit/ae3895c7882e0a543a44693faee5f760b49b54d7))
    - section names are now validated. ([`cfd974f`](https://github.com/Byron/gitoxide/commit/cfd974f46d2cbb99e7784a05f5e358fed0d4bcab))
    - prepare for validation of `parse::section::Header` ([`00592f6`](https://github.com/Byron/gitoxide/commit/00592f6b80abe15a32a890ddc2b1fbf6701798d8))
    - basic escaping of subsection names during serialization ([`00d1a9b`](https://github.com/Byron/gitoxide/commit/00d1a9b741845b49d8691262bef6e5c21876567e))
    - refactor ([`9fac8e0`](https://github.com/Byron/gitoxide/commit/9fac8e0066c9b1845d9e06fb30b61ca9e9d64555))
    - new roundtrip test on file level ([`78bb93c`](https://github.com/Byron/gitoxide/commit/78bb93cf35b6a990bac64bbfc56144799ad36243))
    - Add `File::write_to()` and `File::to_bstring()`; remove some `TryFrom` impls. ([`4f6cd8c`](https://github.com/Byron/gitoxide/commit/4f6cd8cf65c2d8698bffe327a19031c342b229a6))
    - remove `Integer::to_bstring()` as well as some `TryFrom` impls. ([`0e392f8`](https://github.com/Byron/gitoxide/commit/0e392f81e99c8c0ff29f41b9b86afd57cd99c245))
    - remove `Boolean::to_bstring()` along with a few `From` impls. ([`b22732a`](https://github.com/Byron/gitoxide/commit/b22732a2ab17213c4a1020859ec41f25ccabfbfc))
    - Add `parse::(Event|section::Header|Comment)::write_to(…)`. ([`d087f12`](https://github.com/Byron/gitoxide/commit/d087f12eec73626eb327eaacef8ebb3836b02381))
    - fix tests on windows ([`3d7fc18`](https://github.com/Byron/gitoxide/commit/3d7fc188914337074775863acc1d6c15f47e913c))
    - value normalization (via `value::normalize()` handles escape sequences. ([`f911707`](https://github.com/Byron/gitoxide/commit/f911707b455ba6f3800b85f667f91e4d56027b91))
    - refactor normalization and more tests ([`cf3bf4a`](https://github.com/Byron/gitoxide/commit/cf3bf4a3bde6cdf20c63ffee1a5ae55a1f4e1742))
    - more escape characters for normalization ([`b92bd58`](https://github.com/Byron/gitoxide/commit/b92bd580de45cb58cd2b3c4af430273e96139c79))
    - review docs of `file::mutating` ([`2d5703e`](https://github.com/Byron/gitoxide/commit/2d5703e5909946e4327e0372097273facaeca759))
    - stable sort order for `File::sections_by_name_with_header()` ([`44dfec0`](https://github.com/Byron/gitoxide/commit/44dfec07480cc2ac6fd01674b748cc03af51fed1))
    - review `file::raw` module ([`6acf4a4`](https://github.com/Byron/gitoxide/commit/6acf4a43fd63c1c5e24b2e21702dc79827e3d11e))
    - don't over-normalize in comfort layer - all values are normalized now ([`b979a3b`](https://github.com/Byron/gitoxide/commit/b979a3b318faada23a6cf073953b13f7828398af))
    - docs for comfort level File API ([`eafc6ce`](https://github.com/Byron/gitoxide/commit/eafc6ce14a9f3d3dbc585e34e465609385f07f69))
    - review and refactor 'File::value' module ([`7aa8a0b`](https://github.com/Byron/gitoxide/commit/7aa8a0b66f3508336e8c20a1a0d2b481e7b9bde8))
    - allocation free `File::sections_by_name()` and `File::sections_by_name_with_header()`. ([`65c520c`](https://github.com/Byron/gitoxide/commit/65c520c4de8187884f87059adf5cef9cbdcd90a2))
    - refactor ([`2abffd6`](https://github.com/Byron/gitoxide/commit/2abffd6f2224edd98f806b5dbd4fc0e1c60019c5))
    - refactor ([`539c2f6`](https://github.com/Byron/gitoxide/commit/539c2f67bede1247478ce75429690c2904915a89))
    - refactor ([`f1668e9`](https://github.com/Byron/gitoxide/commit/f1668e9d9e94f166fa05164612eab9ee26357d12))
    - refactor ([`2599680`](https://github.com/Byron/gitoxide/commit/2599680f7479e18612b4379efbe918139dde2345))
    - refactor ([`879fad5`](https://github.com/Byron/gitoxide/commit/879fad5afdcd90e248934e9c3b973d7bd438d1f9))
    - fix docs ([`b2b82da`](https://github.com/Byron/gitoxide/commit/b2b82da6c6d3c71b249c9ff2055cd98a58f1d988))
    - once again zero-allocation for SectionBodyIter ([`ba69124`](https://github.com/Byron/gitoxide/commit/ba691243778b3eb89452fd1277c50dfe83d0075f))
    - refactor ([`33efef6`](https://github.com/Byron/gitoxide/commit/33efef6de375e399fe33a02e7b6dace1a679ac7e))
    - docs and refactor ([`700d6aa`](https://github.com/Byron/gitoxide/commit/700d6aa34f2604ee72e619afb15c1bb6ce1697f2))
    - `Path::interpolate()` now takes `path::interpolate::Options` instead of three parameters. ([`ac57c44`](https://github.com/Byron/gitoxide/commit/ac57c4479e7b6867e8b8e71f7cf76de759dc64a2))
    - refactor `from_env` ([`c8693f9`](https://github.com/Byron/gitoxide/commit/c8693f9058765671804c93ead1eea1175a94f87c))
    - make fmt ([`a7d7751`](https://github.com/Byron/gitoxide/commit/a7d7751822a1a8ac89930031707af57ad95d9cbd))
    - more doc adjustments ([`95fc20a`](https://github.com/Byron/gitoxide/commit/95fc20a377aeb914d6b527c1d1b8e75d8c42c608))
    - review docs of 'parse' module; refactor ([`a361c7f`](https://github.com/Byron/gitoxide/commit/a361c7ff290cdae071a12351330013ad0043b517))
    - refactor ([`8e84fda`](https://github.com/Byron/gitoxide/commit/8e84fdadfc49ba61f258286acb0a707bfb2a396b))
    - `File::raw_multi_value()` to `File::raw_values()` ([`9cd9933`](https://github.com/Byron/gitoxide/commit/9cd99337333f5ef4b30e0ec9461fc087699576e6))
    - `File::raw_multi_value_mut()` to `File::raw_values_mut()` ([`0076dcf`](https://github.com/Byron/gitoxide/commit/0076dcf9b37f1d633bdad5573b40d34a9fbaba90))
    - `File::multi_value()` to `File::values()`. ([`a8604a2`](https://github.com/Byron/gitoxide/commit/a8604a237782f8d60a185d4730db57bad81424a6))
    - remove `String` type in favor of referring to the `File::string()` method. ([`0915051`](https://github.com/Byron/gitoxide/commit/0915051798dd782b40617a1aa16abd71f6db1175))
    - fix docs ([`8fa7600`](https://github.com/Byron/gitoxide/commit/8fa7600847da6946784466213cea4c32ff9f7f92))
    - refactor ([`b78e3fa`](https://github.com/Byron/gitoxide/commit/b78e3fa792fad4f3e3f9d5c668afccd75bc551e0))
    - change! Add `home_for_user` in `Path::interpolate(…)`. ([`f9e0ef3`](https://github.com/Byron/gitoxide/commit/f9e0ef38e97fbc1e123d310dc696270d496438b6))
    - Simplify `Boolean` to be a wrapper around `bool`. ([`9cadc6f`](https://github.com/Byron/gitoxide/commit/9cadc6f0cbaad0ac23f5469db2f040aecfbfb82c))
    - Use bitflags for `color::Attribute` instead of `Vec` of enums. ([`703922d`](https://github.com/Byron/gitoxide/commit/703922dd4e1e5b27835298217ff4eb8ef1dc57ce))
    - A bitflag version of color attributes ([`23ec673`](https://github.com/Byron/gitoxide/commit/23ec673baaf666fc38fda2f3b1ace9a8cf6816b8))
    - refactor ([`4f21d1e`](https://github.com/Byron/gitoxide/commit/4f21d1ed145bfd0d56d31be73fade25b104bab53))
    - simplify `Color` API. ([`3fc4ac0`](https://github.com/Byron/gitoxide/commit/3fc4ac04f46f869c6e3a94ce4bb8a5737aa0c524))
    - deduplicate ([`c1b9cd4`](https://github.com/Byron/gitoxide/commit/c1b9cd443ec103a01daee8b8226a53f560d62498))
    - first tests for colors specifically; fix space between tokens ([`e2bd055`](https://github.com/Byron/gitoxide/commit/e2bd0557d9ab68a02216c252ab20aaec2e4efd4e))
    - count newlines (for error display) in multi-line values as well ([`1ea919d`](https://github.com/Byron/gitoxide/commit/1ea919d5ff81ab7b01b8201386ef63c7e081b537))
    - zero-copy for section names ([`25b9760`](https://github.com/Byron/gitoxide/commit/25b9760f9a6a79c6e28393f032150e37d5ae831e))
    - prepare for copy-on-write subsections ([`7474997`](https://github.com/Byron/gitoxide/commit/7474997216df2616a034fb9adc0938590f3ab7ed))
    - another normalization case ([`637fe8f`](https://github.com/Byron/gitoxide/commit/637fe8fca2ce36e07ad671a4454da512b709045c))
    - allow backspaces in value parser ([`199e546`](https://github.com/Byron/gitoxide/commit/199e5461cb85b11ce0b9a0e727fab40a49b78456))
    - another failing test pointing at issues with normalization/escaping in parser ([`3c29321`](https://github.com/Byron/gitoxide/commit/3c2932167aa45a89974be79123932bc964fe3ea9))
    - found failing test with complex multi-line value ([`117401d`](https://github.com/Byron/gitoxide/commit/117401ddb9dea1d78b867ddbafe57c2b37ec10f4))
    - review `git-config::File` docs and rename some internal symbols ([`5a8b111`](https://github.com/Byron/gitoxide/commit/5a8b111b9a3bba2c01d7d5e32fc58fd8a64b81ad))
    - more correctness for sub-section parsing ([`910af94`](https://github.com/Byron/gitoxide/commit/910af94fe11bc6e1c270c5512af9124f8a2e0049))
    - reduce top-level docs ([`cdfb13f`](https://github.com/Byron/gitoxide/commit/cdfb13f5984c92c8e7f234e7751b66930291b461))
    - refactor; remove unnecessary docs ([`c95e0b9`](https://github.com/Byron/gitoxide/commit/c95e0b9331282e029ef6188880d11a892ed1b4bf))
    - assure no important docs are missed ([`f5026fb`](https://github.com/Byron/gitoxide/commit/f5026fb3b64bccf26bc8d5a74dbc5e89b98d9959))
    - filtering supportort for `parse::Events`. ([`6ba2f80`](https://github.com/Byron/gitoxide/commit/6ba2f8060768978ad7204e162fb2253ca8843879))
    - deduplicate events instantiation ([`ead757c`](https://github.com/Byron/gitoxide/commit/ead757c2a4b737d2f617cf23c370e2ca5c46b08b))
    - unclutter lifetime declarations ([`e571fdb`](https://github.com/Byron/gitoxide/commit/e571fdb4630ff373ece02efcd963724c05978ede))
    - remove redundant documentation about errors ([`183c7ae`](https://github.com/Byron/gitoxide/commit/183c7ae0d5f44bb468954a7ad18cc02a01d717bc))
    - adjust to changes in `git-config` ([`c52cb95`](https://github.com/Byron/gitoxide/commit/c52cb958f85b533e791ec6b38166a9d819f12dd4))
    - remove `parse::Events::from_path` and `File::at` ([`14149ee`](https://github.com/Byron/gitoxide/commit/14149eea54e2e8a25ac0ccdb2f6efe624f6eaa22))
    - try to strike a balance between allocations and memory footprint ([`52bd1e7`](https://github.com/Byron/gitoxide/commit/52bd1e7455d2b09811ea0ac5140c3693d3c1e1f7))
    - allocation-free parsing as callback is passed through ([`ed00e22`](https://github.com/Byron/gitoxide/commit/ed00e22cbdfea1d69d1d4c2b829effc26b493185))
    - foundation for allocation free (and smallvec free) parsing ([`307c1af`](https://github.com/Byron/gitoxide/commit/307c1afebfba952a4931a69796686b8a998c4cd9))
    - Slim down API surface of `parse::Events`. ([`73adcee`](https://github.com/Byron/gitoxide/commit/73adceeae12270c0d470d4b7271c1fd6089d5c2d))
    - remove `File::new()` method in favor of `File::default()`. ([`2e47167`](https://github.com/Byron/gitoxide/commit/2e47167e4a963743494b2df6b0c15800cb876dd0))
    - a greatly simplified Events->File conversion ([`c5c4398`](https://github.com/Byron/gitoxide/commit/c5c4398a56d4300c83c5be2ba66664bd11f49d5e))
    - fix docs ([`5022be3`](https://github.com/Byron/gitoxide/commit/5022be3bb7fa54c97e5110f74aaded9e2f1b6ca5))
    - about 30% faster parsing due to doing no less allocations for section events ([`050d0f0`](https://github.com/Byron/gitoxide/commit/050d0f0dee9a64597855e85417460f6e84672b02))
    - allocation-free fuzzing, with optimized footprints ([`2e149b9`](https://github.com/Byron/gitoxide/commit/2e149b982ec57689c161924dd1d0b22c4fcb681f))
    - allocation-free sections ([`d3a0c53`](https://github.com/Byron/gitoxide/commit/d3a0c53864ccc9f8d2851d06f0154b9e8f9bcda7))
    - allocation-free frontmatter ([`6c3f326`](https://github.com/Byron/gitoxide/commit/6c3f3264911042e88afa0819414eb543a3626d11))
    - remove last duplicate of top-level parse function ([`cd7a21f`](https://github.com/Byron/gitoxide/commit/cd7a21f8381385833f5353925dc57c05c07e718d))
    - workaround lack of GAT! ([`4fb327c`](https://github.com/Byron/gitoxide/commit/4fb327c247f1c0260cb3a3443d81063b71e87fe4))
    - remove duplication of top-level parser ([`0f5c99b`](https://github.com/Byron/gitoxide/commit/0f5c99bffdb61e4665e83472275c5c8b0383650b))
    - a minimally invasive sketch of a parse Delegate ([`5958ffb`](https://github.com/Byron/gitoxide/commit/5958ffbfec7724c1a47be8db210df03cf54c9374))
    - fix docs ([`2186456`](https://github.com/Byron/gitoxide/commit/218645618429258e48cb0fdb2bbfba3daa32ee2d))
    - fix fuzz crash in parser ([`86e1a76`](https://github.com/Byron/gitoxide/commit/86e1a76484be50f83d06d6c8a176107f8cb3dea6))
    - rename `parse::event::List` to `parse::Events` ([`ea67650`](https://github.com/Byron/gitoxide/commit/ea6765093b5475912ba1aa81d4440cbf5dd49fb6))
    - rename `parse::State` to `parse::event::List` ([`89f5fca`](https://github.com/Byron/gitoxide/commit/89f5fca843d999c5bea35fb3fe2a03dc3588f74e))
    - update fuzz instructions and make it work ([`19300d5`](https://github.com/Byron/gitoxide/commit/19300d5f37c201aba921a6bff9760996fec2108e))
    - improve normalization; assure no extra copies are made on query. ([`4a01d98`](https://github.com/Byron/gitoxide/commit/4a01d983f54a7713dea523f6032cbf5bb2b9dde8))
    - refactor; assure `normalize` doesn't copy unnecessarily ([`ce069ca`](https://github.com/Byron/gitoxide/commit/ce069ca0b6b44cd734f4d8b4525916d1ddb0de0b))
    - normalize values in all the right places ([`91ba2dd`](https://github.com/Byron/gitoxide/commit/91ba2ddcd3de63aa22dc6e863b26ce1893a36995))
    - avoid unnecessary clones ([`e684488`](https://github.com/Byron/gitoxide/commit/e68448831a94574ee3ca2fa36788f603c91d57a0))
    - adapt to changes in `git-config` ([`363a826`](https://github.com/Byron/gitoxide/commit/363a826144ad59518b5c1a3dbbc82d04e4fc062d))
    - move `value::*` into the crate root, except for `Error` and `normalize_*()`. ([`3cdb089`](https://github.com/Byron/gitoxide/commit/3cdb0890b71e62cfa92b1ed1760c88cb547ec729))
    - rename `value::parse::Error` to `value::Error`. ([`748d921`](https://github.com/Byron/gitoxide/commit/748d921efd7469d5c19e40ddcb9099e2462e3bbc))
    - rename `value::TrueVariant` to `value::boolean::True` ([`7e8a225`](https://github.com/Byron/gitoxide/commit/7e8a22590297f2f4aab76b53be512353637fb651))
    - rename `IntegerSuffix` to `integer::Suffix` ([`8bcaec0`](https://github.com/Byron/gitoxide/commit/8bcaec0599cf085a73b344f4f53fc023f6e31430))
    - rename `value::Color(Attribute|Value)` to `value::color::Attribute` and `value::color::Name`. ([`d085037`](https://github.com/Byron/gitoxide/commit/d085037ad9c067af7ce3ba3ab6e5d5ddb45b4057))
    - refactor ([`a0f7f44`](https://github.com/Byron/gitoxide/commit/a0f7f44c4fca20d3c9b95a3fafe65cef84c760e7))
    - refactor ([`0845c84`](https://github.com/Byron/gitoxide/commit/0845c84b6f694d97519d5f86a97bca49739df8bf))
    - keep str in value API ([`ef5b48c`](https://github.com/Byron/gitoxide/commit/ef5b48c71e0e78fa602699a2f8ca8563c10455c4))
    - Keep BStr even though str could be used. ([`aeca6cc`](https://github.com/Byron/gitoxide/commit/aeca6cce7b4cfe67b18cd80abb600f1271ad6057))
    - Turn `parse::ParseOrIoError` into `parse::state::from_path::Error` ([`a0f6252`](https://github.com/Byron/gitoxide/commit/a0f6252343a62b0b55eef02888ac00c09100687a))
    - rename `parse::ParsedComment` into `parse::Comment` ([`b6b31e9`](https://github.com/Byron/gitoxide/commit/b6b31e9c8dd8b3dc4860431069bb1cf5eacd1702))
    - Allocation-free hashing for section keys and names ([`44d0061`](https://github.com/Byron/gitoxide/commit/44d00611178a4e2f6a080574c41355a50b79b181))
    - allocation-free case-inequality tests for section keys and names ([`94608db`](https://github.com/Byron/gitoxide/commit/94608db648cd717af43a97785ea842bc75361b7e))
    - rename `parse::Section*` related types. ([`239cbfb`](https://github.com/Byron/gitoxide/commit/239cbfb450a8cddfc5bec1de21f3dc54fab914ce))
    - adjustments required due to changed in `git-config` ([`41bfd3b`](https://github.com/Byron/gitoxide/commit/41bfd3b4122e37370d268608b60cb00a671a8879))
    - rename `parse::Parser` to `parse::State`. ([`60af4c9`](https://github.com/Byron/gitoxide/commit/60af4c9ecb1b99f21df0e8facc33e5f6fc70c424))
    - rename `parser` module to `parse` ([`3724850`](https://github.com/Byron/gitoxide/commit/3724850e0411f1f76e52c6c767fd8cebe8aea0f6))
    - fix docs ([`b05aed1`](https://github.com/Byron/gitoxide/commit/b05aed1cfc15a2e29d7796bad4c9a6d4019f4353))
    - refactor ([`8bd9cd6`](https://github.com/Byron/gitoxide/commit/8bd9cd695d608d05859d8bff4033883e71ce7caa))
    - refactor ([`90dd2ce`](https://github.com/Byron/gitoxide/commit/90dd2cec8ea88980365bfd08a16614d145e87095))
    - fix docs ([`0d1be2b`](https://github.com/Byron/gitoxide/commit/0d1be2b893574f2a9d4ba35ac4f2b3da710d4b03))
    - rename `normalize_cow()` to `normalize()` and move all `normalize*` functions from `values` to the `value` module ([`58b2215`](https://github.com/Byron/gitoxide/commit/58b22152a0295998935abb43563e9096589ef53e))
    - Documentation for feature flags ([`26e4a9c`](https://github.com/Byron/gitoxide/commit/26e4a9c83af7550eab1acaf0256099774be97965))
    - `serde1` feature to add limited serde support ([`5a8f242`](https://github.com/Byron/gitoxide/commit/5a8f242ee98793e2467e7bc9806f8780b9d320ce))
    - remove unused serde feature ([`66a8237`](https://github.com/Byron/gitoxide/commit/66a8237ff284c2cf7f80cc909c7b613b599e1358))
    - move `Path` from `values` to `value` module ([`767bedc`](https://github.com/Byron/gitoxide/commit/767bedccdae1f3e6faf853d59ecf884a06cc3827))
    - Move `Boolean` and `String` from `values` into `value` module ([`6033f3f`](https://github.com/Byron/gitoxide/commit/6033f3f93d2356399a661567353a83a044662699))
    - move `values::Integer` into `value` module ([`d4444e1`](https://github.com/Byron/gitoxide/commit/d4444e18042891b0fe5b9c6e6813fed26df6c560))
    - move `Color` to own `value` module ([`38f3117`](https://github.com/Byron/gitoxide/commit/38f31174e8c117af675cdfbc21926133b821ec38))
    - Make symlink tests so that they test real-path conversion ([`d4fbf2e`](https://github.com/Byron/gitoxide/commit/d4fbf2ea71ee1f285c195dd00bfa4e21bf429922))
    - adjustments due to breaking changes in `git_path` ([`4420ae9`](https://github.com/Byron/gitoxide/commit/4420ae932d5b20a9662a6d36353a27111b5cd672))
    - a test to validate relative includepaths aren't valid for includeIf ([`7d27dd5`](https://github.com/Byron/gitoxide/commit/7d27dd5e3558a22865e0c9159d269577431097f3))
    - reuse the initialized environment for a little speed ([`6001613`](https://github.com/Byron/gitoxide/commit/600161324edc370707613841ce9228320c700bf6))
    - Also test against git baseline ([`adcddb0`](https://github.com/Byron/gitoxide/commit/adcddb0056c14302f0133de251fa07e877b6f509))
    - refactor ([`0229e25`](https://github.com/Byron/gitoxide/commit/0229e2583ed7beccaf59dc0c82893c5b67c285dd))
    - prevent race when calling `git` around `GIT_CONFIG_*` env vars ([`53efbf5`](https://github.com/Byron/gitoxide/commit/53efbf54364c373426a7790c28c74c787670877a))
    - remove duplicate gitdir tests that don't have a baseline ([`5c71394`](https://github.com/Byron/gitoxide/commit/5c713946b1f35675bacb27bd5392addf25010942))
    - remove unmotivated forward-slash conversion ([`3af09e5`](https://github.com/Byron/gitoxide/commit/3af09e5800648df87cdaf22191dd4d1dc4b278a3))
    - improved slash/backslash handling on windows ([`a3b7828`](https://github.com/Byron/gitoxide/commit/a3b7828e8bf9d90775f10b0d996fc7ad82f92466))
    - fix build warnings on windows ([`9d48b2f`](https://github.com/Byron/gitoxide/commit/9d48b2f51777de37cc996ad54261f2d20f417901))
    - fix windows test ([`a922f0a`](https://github.com/Byron/gitoxide/commit/a922f0a817d290ef4a539bbf99238a4f96d443f9))
    - refactor ([`d76aee2`](https://github.com/Byron/gitoxide/commit/d76aee22498cb980ab0b53295a2e51af04a8cb7c))
    - conforming subsection parsing handling backslashes like git ([`6366148`](https://github.com/Byron/gitoxide/commit/6366148f538ee03314dd866e083157de810d4ad4))
    - Only copy pattern if required ([`b3a752a`](https://github.com/Byron/gitoxide/commit/b3a752a0a873cf9d685e1893c8d35255d7f7323a))
 * **Uncategorized**
    - Release git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`aa639d8`](https://github.com/Byron/gitoxide/commit/aa639d8c43f3098cc4a5b50614c5ae94a8156928))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge branch 'config-cascade' ([`f144eaf`](https://github.com/Byron/gitoxide/commit/f144eaf5863ae5cac63103f0db51c35fcf03a948))
    - thanks clippy ([`fddc720`](https://github.com/Byron/gitoxide/commit/fddc7206476423a6964d61acd060305572ecd02b))
    - thanks fuzzy ([`15a379a`](https://github.com/Byron/gitoxide/commit/15a379a85d59d83f3a0512b9e9fbff1774c9f561))
    - thanks clippy ([`15fee74`](https://github.com/Byron/gitoxide/commit/15fee74fdfb5fc84349ac103cd5727332f3d2230))
    - thanks clippy ([`0b05be8`](https://github.com/Byron/gitoxide/commit/0b05be850d629124f027af993e316b9018912337))
    - Merge branch 'config-sec-access' ([`b420eba`](https://github.com/Byron/gitoxide/commit/b420eba520ecc31fb2d07c939fa64f1a7be5737e))
    - Merge branch 'config-reduce-events' ([`fd046f4`](https://github.com/Byron/gitoxide/commit/fd046f463a9200d0d8f1a5c3e5f452792f015bd5))
    - thanks clippy ([`693e304`](https://github.com/Byron/gitoxide/commit/693e304a2c38130ed936d5e4544faaa858665872))
    - fix git-config/tests/.gitattributes ([`a741766`](https://github.com/Byron/gitoxide/commit/a7417664ca1e41936f9de8cf066e13aeaf9b0d75))
    - Merge branch 'config-metadata' ([`453e9bc`](https://github.com/Byron/gitoxide/commit/453e9bca8f4af12e49222c7e3a46d6222580c7b2))
    - forced checkin to fix strange crlf issue ([`5d0a5c0`](https://github.com/Byron/gitoxide/commit/5d0a5c0712fbd8fcc00aff54563c83281afc9476))
    - thanks clippy ([`e5ba0f5`](https://github.com/Byron/gitoxide/commit/e5ba0f532bf9bfee46d2dab24e6a6503df4d239d))
    - thanks clippy ([`00bfbca`](https://github.com/Byron/gitoxide/commit/00bfbca21e2361008c2e81b54424a9c6f09e76e9))
    - thanks clippy ([`09e2374`](https://github.com/Byron/gitoxide/commit/09e23743035b9d4463f438378aed54677c03311f))
    - thanks clippy ([`e842633`](https://github.com/Byron/gitoxide/commit/e84263362fe0631935379a0b4e8d8b1fcf6ac81b))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Merge branch 'config-comfort' ([`84b98d9`](https://github.com/Byron/gitoxide/commit/84b98d94177ceaf931aaa521e44eca0fa484d2d3))
    - thanks clippy ([`3ca8027`](https://github.com/Byron/gitoxide/commit/3ca8027e07a835e84a704688778cfb82c956643b))
    - make fmt ([`aa9fdb0`](https://github.com/Byron/gitoxide/commit/aa9fdb0febfb29f906eb81e4378f07ef01b03e05))
    - Merge branch 'config-output' ([`20e188f`](https://github.com/Byron/gitoxide/commit/20e188ff3b06ac7e866956ea5216b00dcffd1307))
    - thanks clippy ([`c9a2390`](https://github.com/Byron/gitoxide/commit/c9a239095511ae95fb5efbbc9207293641b623f7))
    - thanks clippy ([`badd00c`](https://github.com/Byron/gitoxide/commit/badd00c402b59994614e653b28bb3e6c5b70d9d1))
    - make fmt ([`0700b09`](https://github.com/Byron/gitoxide/commit/0700b09d6828849fa2470df89af1f75a67bfb27d))
    - thanks clippy ([`b246f0a`](https://github.com/Byron/gitoxide/commit/b246f0ade5aa42413cc387470b35df357b1136bc))
    - thanks clippy ([`08441de`](https://github.com/Byron/gitoxide/commit/08441def5d1738bbf13b68979f2d1ff7ff3b4153))
    - thanks clippy ([`8b29dda`](https://github.com/Byron/gitoxide/commit/8b29ddaa627048b9ca130b52221709a575f50d3a))
    - thanks clippy ([`cff6e01`](https://github.com/Byron/gitoxide/commit/cff6e018a8f0c3b6c78425f99a204d29d72a65aa))
    - thanks clippy ([`f7be3b0`](https://github.com/Byron/gitoxide/commit/f7be3b0f79bf19faf5a3b68032f764c0b7a12d7e))
    - thanks clippy ([`7a2a31e`](https://github.com/Byron/gitoxide/commit/7a2a31e5758a2be8434f22cd9401ac00539f2bd9))
    - Allow backslashes in subsections ([`6f4f325`](https://github.com/Byron/gitoxide/commit/6f4f325a42656800c8c76c8eae075508c31657be))
    - fix build after changes to `git-url` and `git-config` ([`1f02420`](https://github.com/Byron/gitoxide/commit/1f0242034071ce317743df75cc685e7428b604b0))
    - thanks clippy ([`9b6a67b`](https://github.com/Byron/gitoxide/commit/9b6a67bf369fcf51c6a3289784c3ef8ab366bee7))
    - remove `values::Bytes` - use `values::String` instead. ([`aa630ad`](https://github.com/Byron/gitoxide/commit/aa630ad6ec2c6306d3307d5c77e272cb24b00ddd))
    - change mostily internal uses of [u8] to BString/BStr ([`311d4b4`](https://github.com/Byron/gitoxide/commit/311d4b447daf8d4364670382a20901468748d34d))
    - Definitely don't unconditionally convert to forward slashes ([`146eb0c`](https://github.com/Byron/gitoxide/commit/146eb0c831ce0a96e215b1ec6499a86bbf5902c9))
    - avoid panics and provide errors instead of just not matching ([`a0f842c`](https://github.com/Byron/gitoxide/commit/a0f842c7f449a6a7aedc2742f7fc4f74a12fdd17))
    - Merge branch 'main' into pathspec ([`f4fe879`](https://github.com/Byron/gitoxide/commit/f4fe879ab3161fdb135354939b85408197d8a953))
    - try to fix git-config tests on windows even harder ([`16778d4`](https://github.com/Byron/gitoxide/commit/16778d478d6941ab86571de0bd99aaab816ffe67))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - try once more to get failing tests under control on windows ([`c26c2e9`](https://github.com/Byron/gitoxide/commit/c26c2e962aa6a93c0e06b900dc719f9cd92f6137))
    - thanks clippy ([`27b2dde`](https://github.com/Byron/gitoxide/commit/27b2dde9a299aca112347f988fa21d797f64552b))
    - fix test with brute force; take some notes for later ([`2eda529`](https://github.com/Byron/gitoxide/commit/2eda5296ad9ee58756d564225e98e64a800f46d7))
    - Merge branch 'main' into cont_include_if ([`daa71c3`](https://github.com/Byron/gitoxide/commit/daa71c3b753c6d76a3d652c29237906b3e28728f))
    - Take GitEnv by ref. ([`937d7ee`](https://github.com/Byron/gitoxide/commit/937d7eea84e92467fcc8a6a72c78fe6c060dd95d))
    - Merge branch 'normalize-values' ([`4e8cc7a`](https://github.com/Byron/gitoxide/commit/4e8cc7a5b447656c744cd84e6521e620d0479acb))
    - remove leftover debug printing ([`7d1cf34`](https://github.com/Byron/gitoxide/commit/7d1cf34e4535721db97f566734f68014ebc7c3e8))
    - auto-normalize string values to support quote removal in case of strings. ([`1e71e71`](https://github.com/Byron/gitoxide/commit/1e71e71c984289f0d7e0a39379ee6728918b7dc5))
    - refactor ([`02fba2c`](https://github.com/Byron/gitoxide/commit/02fba2c124f3665112102469d41d476b6cf48dcd))
    - refactor ([`1d6ba9b`](https://github.com/Byron/gitoxide/commit/1d6ba9bd719ad01ce22573cabd8022ddb675c5fc))
    - avoid unwrap() more as the test code matures ([`c2d7e80`](https://github.com/Byron/gitoxide/commit/c2d7e800abe022f5a2663176f0f6b3ac90eacf0e))
    - refactor ([`b5c0b30`](https://github.com/Byron/gitoxide/commit/b5c0b3011d2c0e63c933be42753aea65b88ca569))
    - make '..' related tests work ([`5f11318`](https://github.com/Byron/gitoxide/commit/5f11318dc55b8dd8da016a4053cc4ad34b13fa97))
    - find a few cases that aren't according to spec by failing (and ignored) tests ([`f0e6ea9`](https://github.com/Byron/gitoxide/commit/f0e6ea9086ebfa134044568114bb578120eb5da9))
    - refactor ([`62e5396`](https://github.com/Byron/gitoxide/commit/62e5396ac9221f13437c87f06715c98989981785))
    - generally avoid using `target_os = "windows"` in favor of `cfg(windows)` and negations ([`91d5402`](https://github.com/Byron/gitoxide/commit/91d54026a61c2aae5e3e1341d271acf16478cd83))
    - Invoke git only when necessary ([`556c7cf`](https://github.com/Byron/gitoxide/commit/556c7cff5f813e885598b4bd858c6c22cedf688b))
    - Also use git_path::realpath() in other places that used canonicalize before ([`08af648`](https://github.com/Byron/gitoxide/commit/08af648923c226a0330f0025784c42914d4fea7f))
    - Our own git_path::realpath doesn't have the questionmark? issue on windows ([`cfe196b`](https://github.com/Byron/gitoxide/commit/cfe196b23051e639cb1332f88f1ec917608fbbe1))
    - fix windows tests ([`47f10fe`](https://github.com/Byron/gitoxide/commit/47f10feb2b143b9b429237cf6a4a7424c2b9ab13))
    - more debugging for windows failures ([`e0a72e6`](https://github.com/Byron/gitoxide/commit/e0a72e65e4bbe76755aea1a905d69d74d01f543a))
    - no need for serial anymore ([`34bb715`](https://github.com/Byron/gitoxide/commit/34bb7152ca5992fc35be5f51016565a568916e7c))
    - Make a note to be sure we use the home-dir correctly in git-repository; avoid `dirs` crate ([`0e8cf19`](https://github.com/Byron/gitoxide/commit/0e8cf19d7f742f9400afa4863d302ba18a452adc))
    - finally all tests work without the need for dirs::home_dir() ([`180ce99`](https://github.com/Byron/gitoxide/commit/180ce99a016c17641990eb41b6bbe3b2407ab271))
    - refactor ([`00ba5d8`](https://github.com/Byron/gitoxide/commit/00ba5d8a53aae1c4adbb379c076651756e1af68d))
    - refactor ([`0eb7ced`](https://github.com/Byron/gitoxide/commit/0eb7ced6ec49fe6303659bdcab29952c5cea41bd))
    - Path-interpolation makes `home-dir` configurable. ([`edd2267`](https://github.com/Byron/gitoxide/commit/edd226719cd04a480274cb7d983b6d5d8bfdbb13))
    - refactor ([`aab9865`](https://github.com/Byron/gitoxide/commit/aab98656ee5c4abf65f79d403c1f0cb36fd0ee88))
    - Change last test to new simplified symlink  setup ([`a40e3c9`](https://github.com/Byron/gitoxide/commit/a40e3c999baf203c92d0e5e53ee61c0032e32e51))
    - refactor ([`67677b0`](https://github.com/Byron/gitoxide/commit/67677b0edfa1faa0c011a225d41d78dbde3c5f15))
    - assure the IDE doesn't confuse a module with a test ([`7be0b05`](https://github.com/Byron/gitoxide/commit/7be0b05ff3a5bbea9d9712e4d13ee08cf9979861))
    - refactor ([`1203a14`](https://github.com/Byron/gitoxide/commit/1203a14eba79d335137c96d4ee573739df30b067))
    - refactor ([`a721efe`](https://github.com/Byron/gitoxide/commit/a721efecd36984064b4b31c715bbe011df2538ad))
    - refactor ([`2c8c6e5`](https://github.com/Byron/gitoxide/commit/2c8c6e53fd4681289c9fa2308735c779ed4eace5))
    - refactor ([`eb0ace1`](https://github.com/Byron/gitoxide/commit/eb0ace14a92899002749d6dbd99dac3a35d73c25))
    - refactor ([`8f8f873`](https://github.com/Byron/gitoxide/commit/8f8f873ae711eb5ae62f192f6731653f2bb7ff4b))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-path v0.3.0, safety bump 14 crates ([`400c9be`](https://github.com/Byron/gitoxide/commit/400c9bec49e4ec5351dc9357b246e7677a63ea35))
    - Remove `git-config` test utilities from `git-path`. ([`c9933c0`](https://github.com/Byron/gitoxide/commit/c9933c0b0f51d21dc8244b2acc33d7dc8a33f6ce))
    - Add repo_dir to EnvOverwrite. ([`ed5c442`](https://github.com/Byron/gitoxide/commit/ed5c442cc4f0c546834f2e0e9dc553a221b6985d))
    - Use EnvOverwrite struct. ([`f2e124f`](https://github.com/Byron/gitoxide/commit/f2e124f60f8f9a0d517fddb029d795fa91bcda5a))
    - tempdir lives long enough for sure. ([`a41002f`](https://github.com/Byron/gitoxide/commit/a41002fe4004485fac429d904bc4e8b6842eaf3c))
    - Disable symlink tests on windows. ([`8de6b3d`](https://github.com/Byron/gitoxide/commit/8de6b3d42c89c741195e4add273a2d1e7b48fad9))
</details>

## 0.5.0 (2022-06-13)

### New Features (BREAKING)

 - <csr-id-266d4379e9132fd7dd21e6c8fccb36e125069d6e/> Make `realpath()` easier to use by introducing `realpath_opt()`.
   That way there is consistency about how many symlinks to follow.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 40 commits contributed to the release over the course of 18 calendar days.
 - 22 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#436](https://github.com/Byron/gitoxide/issues/436)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#436](https://github.com/Byron/gitoxide/issues/436)**
    - Remove outdated examples ([`cb9529e`](https://github.com/Byron/gitoxide/commit/cb9529e18b222b9fd9f8c1bb0dba8038a6ea1d4b))
 * **Uncategorized**
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - make fmt ([`cd4f727`](https://github.com/Byron/gitoxide/commit/cd4f7279678678fa6f2e55d4e7681a2075f1d6cf))
    - Temp ignore symlink tests. ([`ec40b94`](https://github.com/Byron/gitoxide/commit/ec40b94bffda14b7b991dd57cd36d893f1f6962b))
    - fmt. ([`82ea726`](https://github.com/Byron/gitoxide/commit/82ea7261cfb75a01992489aa7631e2e6d807be06))
    - Use `dirs::home_dir()` ([`5767a50`](https://github.com/Byron/gitoxide/commit/5767a505f2f2cc3515eb604e39da673fa2e09454))
    - Try fix windows home. ([`393758e`](https://github.com/Byron/gitoxide/commit/393758e14a1b5ff14301f153807fe45623d9f973))
    - Add more tests. ([`db1204d`](https://github.com/Byron/gitoxide/commit/db1204d74b16ff7e905fb5b2d91d9ecb109bca07))
    - Add debug output. ([`52db5e8`](https://github.com/Byron/gitoxide/commit/52db5e8894c5033ec3d58894a7cf17b4f29e03f4))
    - Tests like git: https://github.com/git/git/blob/master/t/t1305-config-include.sh ([`c3a0454`](https://github.com/Byron/gitoxide/commit/c3a04548b08b6972ea0999b0030017d1a6002de2))
    - Start extracting gitdir tests cont. ([`22e5cbe`](https://github.com/Byron/gitoxide/commit/22e5cbece0206da6cf8890a831fd82847526396a))
    - remove `pwd` crate dependency in favor of using libc directly ([`4adfa11`](https://github.com/Byron/gitoxide/commit/4adfa11d70cf78bed541fa59707e8a5082dda245))
    - Drop non-existent config paths before parsing ([`475d6fa`](https://github.com/Byron/gitoxide/commit/475d6fab2467ad0499db7df2d4c99f74e43682fc))
    - Start extracting gitdir tests. ([`5aaf7ba`](https://github.com/Byron/gitoxide/commit/5aaf7ba93857f1e5570f64f4a9539cd3d547b81d))
    - thanks clippy ([`cfa577f`](https://github.com/Byron/gitoxide/commit/cfa577f84c45c7fbed27e6d59ef361f9ac5c2614))
    - refactor ([`da23958`](https://github.com/Byron/gitoxide/commit/da239580fca76011f91a45ae502af88c67d429a4))
    - Finalize onbranch tests; remove mixed ones in favor of specific cases ([`26680c4`](https://github.com/Byron/gitoxide/commit/26680c48951a82d5119f54c57b4e7045d2c20649))
    - refactor ([`11c417f`](https://github.com/Byron/gitoxide/commit/11c417fdc03331db2c4a778bc3e8038ffd0aff89))
    - More tests for branch matching prior to making tests clearer ([`31e6db8`](https://github.com/Byron/gitoxide/commit/31e6db8cdc959549a6c2754692d2471103ada64f))
    - Basic test-setup for more specialized tests ([`b4374d2`](https://github.com/Byron/gitoxide/commit/b4374d21882eca637ddbb80cdde1dac7bc68560e))
    - refactor ([`04da720`](https://github.com/Byron/gitoxide/commit/04da7207a7e44175dc96e4ea850274b2cc5a6d84))
    - Fix including .. path. ([`8891fea`](https://github.com/Byron/gitoxide/commit/8891feac0341960a6339ee86c671fc80c3133b4e))
    - Fix case-insensitive. ([`ca05802`](https://github.com/Byron/gitoxide/commit/ca058024e1e19818261fea39099c893d666928dc))
    - Fix \\ test. ([`ab555b5`](https://github.com/Byron/gitoxide/commit/ab555b557f4bd68b491a552a14cd4549c6a625bc))
    - fix tests on windows ([`bb3b4f0`](https://github.com/Byron/gitoxide/commit/bb3b4f013c862a4c017c65075919e1df59cc1986))
    - refactor ([`e1ba36f`](https://github.com/Byron/gitoxide/commit/e1ba36fab772417d9b60bf89cc49b45fbb7252f9))
    - Merge branch 'main' into svetli-n-cont_include_if ([`315c87e`](https://github.com/Byron/gitoxide/commit/315c87e18c6cac0fafa7b4e59fdd3c076a58a45a))
    - refactor ([`e47fb41`](https://github.com/Byron/gitoxide/commit/e47fb412a136d087c79710e7490d3e1c97d1f955))
    - refactor ([`56eadc8`](https://github.com/Byron/gitoxide/commit/56eadc8b565b2f8a272080bc8814d6665b3f1205))
    - refactor ([`0ccd8ae`](https://github.com/Byron/gitoxide/commit/0ccd8ae0ab01cdb5ae33dd79f486edfcee2b176a))
    - Try fix windows test. ([`e2e94db`](https://github.com/Byron/gitoxide/commit/e2e94db2cee237168d5c56db5c5e94a8b4317991))
    - Refactor include sequence test. ([`b4e657e`](https://github.com/Byron/gitoxide/commit/b4e657ed02cf062b1c2cb1f6c15abdf5d777c177))
    - Extract include_paths. ([`c078671`](https://github.com/Byron/gitoxide/commit/c0786717c4979810002365a68d31abbf21d90f2d))
    - Make `realpath()` easier to use by introducing `realpath_opt()`. ([`266d437`](https://github.com/Byron/gitoxide/commit/266d4379e9132fd7dd21e6c8fccb36e125069d6e))
    - Adjust test structure to mirror the new code structure. ([`984b58e`](https://github.com/Byron/gitoxide/commit/984b58ee1dac58fe0dfd0b80f990ca37d323cad7))
    - Refact. ([`d5d81bc`](https://github.com/Byron/gitoxide/commit/d5d81bc16116b4c58f628e0e5c66d5d0a59b7816))
    - Read include and incideIf sections in correct order. ([`a4a7ebd`](https://github.com/Byron/gitoxide/commit/a4a7ebdb6fcb5f6183917719d6c93f54eea72e85))
    - Refact. ([`a342e53`](https://github.com/Byron/gitoxide/commit/a342e53dac58cea1787a94eaa1a9d24fb1389df2))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
</details>

## 0.4.0 (2022-05-21)

### Changed (BREAKING)

 - <csr-id-553f87225363903e6acdb3e7eaa8cc66a91110f1/> `File::len()` -> `File::num_values()`
   The same is true for `Section::len()` which now is
   `Section::num_values()`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 24 commits contributed to the release over the course of 2 calendar days.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-path v0.1.2, git-sec v0.1.1, git-config v0.4.0, git-discover v0.1.1, git-pack v0.19.1, git-repository v0.18.0, cargo-smart-release v0.10.0, safety bump 2 crates ([`ceb6dff`](https://github.com/Byron/gitoxide/commit/ceb6dff13362a2b4318a551893217c1d11643b9f))
    - fix benchmark compilation ([`53adcfe`](https://github.com/Byron/gitoxide/commit/53adcfea1942e9dd32a7d84d02a83c9a08408fad))
    - Merge branch 'svetli-n-git_includeif' ([`cf24fbe`](https://github.com/Byron/gitoxide/commit/cf24fbe4b62d67b06138243d470dcc1805ebd55b))
    - Bring init functions back to `File` type ([`f1f69d8`](https://github.com/Byron/gitoxide/commit/f1f69d8f983e2505990e7ee21cbd7f64ac7ba766))
    - disallow Rust 2018 idioms ([`81aca45`](https://github.com/Byron/gitoxide/commit/81aca458f4b7f6768e14da5719ada772f419f1b5))
    - fix most of docs ([`1fe053f`](https://github.com/Byron/gitoxide/commit/1fe053f60fa4843e7da6a6328fc293b4bcd25277))
    - thanks clippy ([`409a95b`](https://github.com/Byron/gitoxide/commit/409a95b6505db8568bfea24bc62c92640a5c3cbf))
    - dissolve git_config module in favor of `file` module ([`2d4a19b`](https://github.com/Byron/gitoxide/commit/2d4a19b0c72c4aab79cd3b18430710909ba1af5f))
    - refactor ([`6cc5c20`](https://github.com/Byron/gitoxide/commit/6cc5c20aba825a5a712b33740ea2c7f44387f3f9))
    - refactor ([`3471f95`](https://github.com/Byron/gitoxide/commit/3471f95b5e490d22bb42b6c4204446c52812e4fc))
    - `File::len()` -> `File::num_values()` ([`553f872`](https://github.com/Byron/gitoxide/commit/553f87225363903e6acdb3e7eaa8cc66a91110f1))
    - refactor ([`2626e0c`](https://github.com/Byron/gitoxide/commit/2626e0ca58947eb846128507ffb254e9ebd91ee1))
    - refactor ([`07e0f5e`](https://github.com/Byron/gitoxide/commit/07e0f5e91b3c41614b9182cf9716120fe41ddf40))
    - Split git_config into modules. ([`a85d864`](https://github.com/Byron/gitoxide/commit/a85d8643cbfbfc4bd4d4c1fb17ae3672b8b36931))
    - Fix linux test. ([`e0d063e`](https://github.com/Byron/gitoxide/commit/e0d063ebdfa8effabd53c6a51818617abe4a0b4e))
    - Fix test. ([`ed5de9e`](https://github.com/Byron/gitoxide/commit/ed5de9e8d2e225313ef8e60003797c5466d81273))
    - Temp ignore test. ([`9b70eca`](https://github.com/Byron/gitoxide/commit/9b70eca08aaa36e3f803da1685ac85bab40f0b03))
    - Tryfix windows test. Includes module. ([`b02d147`](https://github.com/Byron/gitoxide/commit/b02d147468f902597d4022c1fce3424213cb9eb8))
    - Tryfix windows test. ([`4098278`](https://github.com/Byron/gitoxide/commit/40982788f88267f0885513fffb112467e2f3b370))
    - Tryfix windows test. ([`17a296f`](https://github.com/Byron/gitoxide/commit/17a296ffc5af08c6c0455b3028d275b9ebe7c18c))
    - Tryfix windows test. ([`a29657a`](https://github.com/Byron/gitoxide/commit/a29657a8118300f11db4e0783800eeadf838532c))
    - Fix merge. ([`07bc9a8`](https://github.com/Byron/gitoxide/commit/07bc9a869d501b78c060e4ed18d4003c287560a8))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
    - Tryfix windows test. ([`300ecbc`](https://github.com/Byron/gitoxide/commit/300ecbc75fbf8d94d7e21a35e12b93f0a954515d))
</details>

## 0.3.0 (2022-05-18)

### Bug Fixes

 - <csr-id-36e2fc0aff4e6aaa35335da90108918882d4cd16/> Use `std::env::var_os()` to avoid potential decode errors

### Changed

 - <csr-id-b04a3465ed20d8f3088e5d3faf11e98e5595f219/> `GitConfig::from_paths(<paths>, …)` accepts more inputs
   `<paths>` is more flexible and is easier to use.

### New Features

 - <csr-id-7c75eac149c6ecb99c3dd7355d76d8d3e8b59cd0/> `GitConfig::path()` for direct access to paths.
   Very similar to `string()`, but as path, whose query can never fail.
 - <csr-id-031bd2f401199a05d6465c0260ceed3cc849c7ac/> add suppport for android
   Do not interpolate `~user/` on Android (Termux).
   There is no meaning of it. It is single user system.
 - <csr-id-dc3dc3b41b5de3ec17429769747bf99bb2bdd03d/> support for `try_value()`, `boolean()` and `string()` access`.
   Support for a convenient way of knowing if a value does or doesn't exist
   via `try_value()`, which can only fail if the conversion fails.
   
   Lastly, `string()` is a special case which doesn't fail as there is
   no conversion, and `boolean()` allows to obtain a plain boolean value
   if it was a valid boolean representation.
 - <csr-id-13554f8d21beb241e0fbdeb56b8414957cbee28a/> new hierarchical errors for value lookup
 - <csr-id-4726bb524c1b0935d35770c907d40a0a16dbb8b5/> `GitConfig::integers()`
   Get multiple fully validated integer values, with their suffix
   interpreted and checked for overflow.
 - <csr-id-ae22a4de486676f11469cec84be403903758b48b/> add `GitConfig::integer()`
   A way to quickly obtain a valid integer with suffixes resolved
   and overflow checked.
 - <csr-id-bfc263797226d027e04daaf6426e57183773d7c3/> `GitConfig::strings()` for multi-value strings.

### Changed (BREAKING)

 - <csr-id-38dfdcf80f9b7368ccaa10f4b78b2129849848d0/> remove `values::*Error` in favor of `value::parse::Error`.
   This makes it easier to work with errors in practice, we are either
   interested in the value that failed to parse to try something else
   or want a nice user message.
   
   Having one decode error type facilitates that.
 - <csr-id-a98a7a7af69482e9ef63f106184049049939459d/> switch from quickerror to thiserror.
   This allows for generic types for sources of errors and allows to
   workaround a limitation with associated type constraints in the MSRV
   of 1.54.
   
   Using thiserror makes this work and brings the crate more closely
   to the rest of the gitoxide crates (which now prefer thiserror over
   quickerror).
 - <csr-id-a86b2541561674df5dbef4120d3e03483cb80117/> remove all `get_` prefixes from methods
   That way the API is more idiomatic and fits better into the
   existing `gitoxide` crates.
 - <csr-id-f9aaac11f0734afbd791132369eb5601bfc7efe9/> use `lookup::Error` and `lookup::existing::Error`
   Use the newly introduced structured error to replace the 'catch-all'
   `GitConfigError` while getting closer to naming conventions in other
   `gitoxide` crates.
 - <csr-id-c7fcb5e1db225aefc3eeab4f29f3fb85c670894a/> `GitConfig::from_paths(…, <option>)` is now owned.
   The type is `Copy`, so no need to pass it by reference.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 89 commits contributed to the release over the course of 40 calendar days.
 - 43 days passed between releases.
 - 14 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 5 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#331](https://github.com/Byron/gitoxide/issues/331), [#386](https://github.com/Byron/gitoxide/issues/386), [#404](https://github.com/Byron/gitoxide/issues/404)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 7 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - upgrade dependencies ([`b039d39`](https://github.com/Byron/gitoxide/commit/b039d39613bb14d49670c4d8b586f76ffb420d03))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - finished refactoring ([`4163c7f`](https://github.com/Byron/gitoxide/commit/4163c7fe0a98b77998fa263458d06bdeb435996d))
    - refactor ([`a359cfd`](https://github.com/Byron/gitoxide/commit/a359cfd86ffae9feab11b45e3167fe28f22dbac8))
    - `GitConfig::from_paths(…, <option>)` is now owned. ([`c7fcb5e`](https://github.com/Byron/gitoxide/commit/c7fcb5e1db225aefc3eeab4f29f3fb85c670894a))
    - Use `std::env::var_os()` to avoid potential decode errors ([`36e2fc0`](https://github.com/Byron/gitoxide/commit/36e2fc0aff4e6aaa35335da90108918882d4cd16))
    - `GitConfig::from_paths(<paths>, …)` accepts more inputs ([`b04a346`](https://github.com/Byron/gitoxide/commit/b04a3465ed20d8f3088e5d3faf11e98e5595f219))
    - refactor unconditional include tests and stabilize them ([`72a5a02`](https://github.com/Byron/gitoxide/commit/72a5a027dd8120b27909efea339dfc7919a865be))
    - `GitConfig::integers()` ([`4726bb5`](https://github.com/Byron/gitoxide/commit/4726bb524c1b0935d35770c907d40a0a16dbb8b5))
    - add `GitConfig::integer()` ([`ae22a4d`](https://github.com/Byron/gitoxide/commit/ae22a4de486676f11469cec84be403903758b48b))
    - refactor ([`c139479`](https://github.com/Byron/gitoxide/commit/c13947977205828dcda177686362e25867fdfe44))
    - refactor ([`4408f17`](https://github.com/Byron/gitoxide/commit/4408f17736052c899a9c98af41485d7dde9a297f))
    - `GitConfig::strings()` for multi-value strings. ([`bfc2637`](https://github.com/Byron/gitoxide/commit/bfc263797226d027e04daaf6426e57183773d7c3))
    - refactor ([`7ea17e1`](https://github.com/Byron/gitoxide/commit/7ea17e1e16346239032844b8f4be9e9c22c6be8e))
    - initial refactoring ([`43a34a5`](https://github.com/Byron/gitoxide/commit/43a34a5bdae53fbb53d3ae095f03c9456115a013))
    - Some notes about of 'path' will soon have to be amended with more safety ([`97e53f6`](https://github.com/Byron/gitoxide/commit/97e53f63df2c0262f23af3d7d997f148d23474be))
    - `GitConfig::path()` for direct access to paths. ([`7c75eac`](https://github.com/Byron/gitoxide/commit/7c75eac149c6ecb99c3dd7355d76d8d3e8b59cd0))
    - remove `values::*Error` in favor of `value::parse::Error`. ([`38dfdcf`](https://github.com/Byron/gitoxide/commit/38dfdcf80f9b7368ccaa10f4b78b2129849848d0))
    - A sketch of what can be a general value decode error ([`4612fca`](https://github.com/Byron/gitoxide/commit/4612fca79446c6f92f0e6a4163bc895fc346b30d))
    - Remove IntegerSuffix error which wasn't ever used ([`732c0fa`](https://github.com/Byron/gitoxide/commit/732c0fa6e1832efcc0de4adc894e820b3bd27b8f))
    - support for `try_value()`, `boolean()` and `string()` access`. ([`dc3dc3b`](https://github.com/Byron/gitoxide/commit/dc3dc3b41b5de3ec17429769747bf99bb2bdd03d))
    - fix build warnings ([`4496b5a`](https://github.com/Byron/gitoxide/commit/4496b5a26abaf91fd4844e0494aaa1b4cce73628))
    - switch from quickerror to thiserror. ([`a98a7a7`](https://github.com/Byron/gitoxide/commit/a98a7a7af69482e9ef63f106184049049939459d))
    - remove all #[inline] attributes ([`8aef1d3`](https://github.com/Byron/gitoxide/commit/8aef1d313dc9d3ac0004e790b6f91ad0c7ac99b0))
    - remove all `get_` prefixes from methods ([`a86b254`](https://github.com/Byron/gitoxide/commit/a86b2541561674df5dbef4120d3e03483cb80117))
    - use `lookup::Error` and `lookup::existing::Error` ([`f9aaac1`](https://github.com/Byron/gitoxide/commit/f9aaac11f0734afbd791132369eb5601bfc7efe9))
    - new hierarchical errors for value lookup ([`13554f8`](https://github.com/Byron/gitoxide/commit/13554f8d21beb241e0fbdeb56b8414957cbee28a))
    - adapt to changes in git-path ([`cc2d810`](https://github.com/Byron/gitoxide/commit/cc2d81012d107da7a61bf4de5b28342dea5083b7))
    - adapt to all changes in git-path with bstr support ([`f158648`](https://github.com/Byron/gitoxide/commit/f158648aef8ad94d86550ceb2eeb20efb3df7596))
    - Use `git-path` crate instead of `git_features::path` ([`47e607d`](https://github.com/Byron/gitoxide/commit/47e607dc256a43a3411406c645eb7ff04239dd3a))
    - make fmt ([`5fc5459`](https://github.com/Byron/gitoxide/commit/5fc5459b17b623726f99846c432a70106464e970))
    - make fmt ([`50ff7aa`](https://github.com/Byron/gitoxide/commit/50ff7aa7fa86e5e2a94fb15aab86470532ac3f51))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - refactor ([`92fe564`](https://github.com/Byron/gitoxide/commit/92fe56486c349a4b08bcefa3e3355c591e281afb))
    - Remove untested error case in integger parsing ([`2b21a35`](https://github.com/Byron/gitoxide/commit/2b21a35e1ba31caea227515ddebc7608cdcca245))
    - validate underflow as well ([`83eda34`](https://github.com/Byron/gitoxide/commit/83eda3443a1b64ff7bc672fbfe16e3a69def1c6d))
    - Case-insensitive integer suffix handling ([`9034bd4`](https://github.com/Byron/gitoxide/commit/9034bd45bba0aa7c6c5691c2e592c389949dd5d6))
    - refactor tests ([`f943d2a`](https://github.com/Byron/gitoxide/commit/f943d2aeb0773752adbb68d731305586ab2ce686))
 * **[#386](https://github.com/Byron/gitoxide/issues/386)**
    - Sketch `Permissions` for git-config ([`8443330`](https://github.com/Byron/gitoxide/commit/8443330b051c109742fe55928e2afd36fc0172fd))
 * **[#404](https://github.com/Byron/gitoxide/issues/404)**
    - Add test to clarify underscores in sections headers aren't allowed ([`47079d4`](https://github.com/Byron/gitoxide/commit/47079d470e44b1adf515ae4df2bed945b7e91108))
 * **Uncategorized**
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Temp ignore test ([`6f35866`](https://github.com/Byron/gitoxide/commit/6f35866725f52eee368042a4293ac74a82752331))
    - update lifetime ([`fd24e2c`](https://github.com/Byron/gitoxide/commit/fd24e2cc1c10ef5c65a5b923ad30806e91427117))
    - Add includeIf test with symlink. ([`5d74404`](https://github.com/Byron/gitoxide/commit/5d744049286632f3141ec07fa3f128093480d1c0))
    - Refactor condition match. ([`15ac22a`](https://github.com/Byron/gitoxide/commit/15ac22a9b28577d2c4175bc752eb7099a3b128fa))
    - Fix realpath tests. ([`0426f4d`](https://github.com/Byron/gitoxide/commit/0426f4deb5d73fd88529530f9a6d01ba55eeadc4))
    - thanks clippy ([`da13aff`](https://github.com/Byron/gitoxide/commit/da13affabe34c3d691b18a70ce61eb00319668c5))
    - Merge branch 'main' into git_includeif ([`05eb340`](https://github.com/Byron/gitoxide/commit/05eb34023933918c51c03cf2afd774db89cc5a33))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - vec -> Option in tests ([`538de54`](https://github.com/Byron/gitoxide/commit/538de54aab0ab0352fbff95e0334c89c415627e9))
    - Tests use `options_with_git_dir()`. ([`9abbac1`](https://github.com/Byron/gitoxide/commit/9abbac1b1a1e7af7c5219f84a9edc1594deda55a))
    - thanks clippy ([`60da03c`](https://github.com/Byron/gitoxide/commit/60da03c3edc38d14601ac2dfbeb3b3045958f860))
    - thanks clippy ([`4d0e29c`](https://github.com/Byron/gitoxide/commit/4d0e29c25fd53421487a624b90072c8553509d45))
    - Merge branch 'git_includeif' of https://github.com/svetli-n/gitoxide into svetli-n-git_includeif ([`0e01da7`](https://github.com/Byron/gitoxide/commit/0e01da74dffedaa46190db6a7b60a2aaff190d81))
    - Merge branch 'worktree-stack' ([`98da8ba`](https://github.com/Byron/gitoxide/commit/98da8ba52cef8ec27f705fcbc84773e5bacc4e10))
    - thanks clippy ([`5bf6b52`](https://github.com/Byron/gitoxide/commit/5bf6b52cd51bef19079e87230e5ac463f8f881c0))
    - thanks clippy ([`53f27e0`](https://github.com/Byron/gitoxide/commit/53f27e04dd186c32eaa8c03615a58a10938cab8d))
    - thanks clippy ([`1e2b239`](https://github.com/Byron/gitoxide/commit/1e2b239abee7e8889fe2060c79c00f2e506023e1))
    - onbranch uses wildmatch ([`8382df2`](https://github.com/Byron/gitoxide/commit/8382df2a7cb9cb12113085b9310560f63c51447f))
    - gitdir/i support. ([`5dd3f92`](https://github.com/Byron/gitoxide/commit/5dd3f92964cd530f435f2dfb81a10fb236dd5334))
    - Fail fast include condition if it's raw value contains backslash. ([`2c78e48`](https://github.com/Byron/gitoxide/commit/2c78e4866e10141b23980aa2db6405644ad92f34))
    - Replace \\ in pattern after expanding relative paths. ([`a955449`](https://github.com/Byron/gitoxide/commit/a9554497f81b2e2ecdd2ea7a14c347b1f136a688))
    - Replace \\ in pattern after interpolation. ([`d774485`](https://github.com/Byron/gitoxide/commit/d77448510bd276db83802bd7b183a757d6a48db3))
    - Handle relative path patterns. Update tests. ([`546ec2c`](https://github.com/Byron/gitoxide/commit/546ec2c30fd6d1ed1eea1f1497251513940e82ac))
    - Use `std::path::MAIN_SEPARATOR` when adding ** globbing. ([`b85e706`](https://github.com/Byron/gitoxide/commit/b85e7066fe86d822b6912101c7eb499998d2c4cd))
    - Use `std::path::MAIN_SEPARATOR` when adding ** globbing. ([`cc42edf`](https://github.com/Byron/gitoxide/commit/cc42edf1e5c11ad25fe3ffc9dbc170868748cf66))
    - Use `git-glob` for pattern matching. ([`6066701`](https://github.com/Byron/gitoxide/commit/6066701f1c852b61203aa46399bd7731834c79bf))
    - Refact. ([`35f955a`](https://github.com/Byron/gitoxide/commit/35f955a3cd881359b573f4abd92239e18701aa34))
    - Fix out of order whne reading includeIf sections. ([`e6ef931`](https://github.com/Byron/gitoxide/commit/e6ef931567888e2794d17f2e0fa598a04ac1ef49))
    - Fix out of order whne reading includeIf sections. ([`293e86e`](https://github.com/Byron/gitoxide/commit/293e86ec96864fcdf5f42ba0d5b4d3892574e7ec))
    - Test WIP. ([`7a59791`](https://github.com/Byron/gitoxide/commit/7a59791181c21927340137340b17cd9715755722))
    - PR feedback. ([`fd2b085`](https://github.com/Byron/gitoxide/commit/fd2b085a856ea4665976e5f662a4fad3d7cb5090))
    - Use new git-ref API. ([`32c5729`](https://github.com/Byron/gitoxide/commit/32c5729c5a42ade2e881de5d5575e670b5808f33))
    - IncludeIf condition and gitdir tests. ([`892b77a`](https://github.com/Byron/gitoxide/commit/892b77a8ae09c61391cb637051ea4576b66cf450))
    - Nop includeIf. ([`5d86a02`](https://github.com/Byron/gitoxide/commit/5d86a02dd7617488285b6d0bd43d13ebfa3fb67a))
    - Get values for a section across all subsections. ([`aff2777`](https://github.com/Byron/gitoxide/commit/aff2777baaffa08d1d8ad2e1da34f47e0fe01f7f))
    - Merge branch 'worktree-stack' ([`39046e9`](https://github.com/Byron/gitoxide/commit/39046e98098da7d490757477986479126a45b3e5))
    - Merge branch 'main' into repo-status ([`9679d6b`](https://github.com/Byron/gitoxide/commit/9679d6b0e68c28438e22cb65c554d0b31dfaf159))
    - Merge branch 'git-sec' ([`cd723b5`](https://github.com/Byron/gitoxide/commit/cd723b5ae11148e7e9fd07daf28bc04455d5c46f))
    - Merge branch 'main' into repo-status ([`4086335`](https://github.com/Byron/gitoxide/commit/40863353a739ec971b49410fbc2ba048b2762732))
    - add suppport for android ([`031bd2f`](https://github.com/Byron/gitoxide/commit/031bd2f401199a05d6465c0260ceed3cc849c7ac))
    - Merge branch 'worktree-stack' ([`e90d3fd`](https://github.com/Byron/gitoxide/commit/e90d3fd0a9764511e6280596f21d3a0494ed7021))
    - thanks clippy ([`273895a`](https://github.com/Byron/gitoxide/commit/273895a06ddfff33c6197799d7e63e8382b4b5e3))
    - Update doc comment. ([`322f825`](https://github.com/Byron/gitoxide/commit/322f82529c1b5fb22406a1392217af5d53dcdac4))
    - Handle overflow. ([`61c5285`](https://github.com/Byron/gitoxide/commit/61c52853e61a4cbb356cc607f970e150c827d679))
    - Add doc comment. ([`001862a`](https://github.com/Byron/gitoxide/commit/001862abde9cbb717c83fdf49a6ddf89a4db16e2))
    - Canonicalize ´git_config::values::Integer` values as simple decimal numbers. ([`03f360a`](https://github.com/Byron/gitoxide/commit/03f360a19d365f614e71948df7e8b0c62d13cff4))
    - Refactor values tests. ([`ee4ad7e`](https://github.com/Byron/gitoxide/commit/ee4ad7eadd6675959c9759bb43a08159e3e0daa9))
    - Merge branch 'mzr-fix_empty_values' ([`09c8628`](https://github.com/Byron/gitoxide/commit/09c8628d68ab82d234921b5c6cf80cda4d21802e))
    - do not treat empty values in sections like multi-line values ([`8b9432c`](https://github.com/Byron/gitoxide/commit/8b9432c22186a290fd05b6272490dad2bccb7f63))
</details>

## 0.2.1 (2022-04-05)

### Features

- New `values::String` data type which makes it easier to obtain string values to work with as
  binary string.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#298](https://github.com/Byron/gitoxide/issues/298)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - prepare changelog prior to release ([`fc8f52d`](https://github.com/Byron/gitoxide/commit/fc8f52d91c89fdc1130990e4392f151a30d1899c))
    - Support for simple BString powered string values ([`2381c5d`](https://github.com/Byron/gitoxide/commit/2381c5d3b91e3a071c887d9e1e166625977d5830))
 * **Uncategorized**
    - Release git-config v0.2.1, git-diff v0.15.0, git-traverse v0.14.0, git-pack v0.18.0, git-odb v0.28.0, git-ref v0.12.1, git-revision v0.1.0, git-repository v0.16.0, gitoxide-core v0.14.0, gitoxide v0.12.0, safety bump 6 crates ([`b612021`](https://github.com/Byron/gitoxide/commit/b612021683ba709b693bd48aef3e2e3c2f5b9ead))
    - thanks clippy ([`7887d8b`](https://github.com/Byron/gitoxide/commit/7887d8b5bedc49890bd73beb058a9828aa734729))
</details>

## 0.2.0 (2022-04-02)

<csr-id-55c00d880535a1f8c37cb7d4405d39ff5a7654a0/>

### New Features

 - <csr-id-e4d6685064ad2b433f8acd3a74b320bf0169a994/> Add `git_config::values::Path` for a typesafe git path
   Add a `Path` type to the `git_config::values` which
   can be interpolated according to gits own path interpolation
   rules.
 - <csr-id-61af06b905926849abce19677ff4b9ac05d625a3/> compatibility with Rust <1.53
 - Respect `include.path` when reading configuration files
 - Support for path interpolation

### Refactor

 - <csr-id-55c00d880535a1f8c37cb7d4405d39ff5a7654a0/> remove `git_config::values::Value`; use `Bytes` in its place.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 44 commits contributed to the release over the course of 39 calendar days.
 - 60 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#331](https://github.com/Byron/gitoxide/issues/331)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - minor refactor ([`2f0234c`](https://github.com/Byron/gitoxide/commit/2f0234c05d1a3e1e3b96dff9680189c67cb6c9ff))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - Update changelog prior to release ([`1d07934`](https://github.com/Byron/gitoxide/commit/1d079346e789b0acc9a4bdf7577b21c1c37b6106))
    - minor refactor of tests ([`ebe551f`](https://github.com/Byron/gitoxide/commit/ebe551fc16a98d4101799b0d657b42f445f5b16e))
    - refactor ([`845fe37`](https://github.com/Byron/gitoxide/commit/845fe373bf25de497f01209f4bed5132cc8eae65))
    - refactor include path recursion logic ([`2862a07`](https://github.com/Byron/gitoxide/commit/2862a0718e3bc03e4edda60f1b359dd6068e8d1d))
    - Ignore subsections when resolving include.path keys ([`baa300f`](https://github.com/Byron/gitoxide/commit/baa300f844eadf8db6ca5503a0b426235d4ee6fb))
    - assure `from_env()` include paths only use paths of the correct key ([`0d84ce8`](https://github.com/Byron/gitoxide/commit/0d84ce8b229bcbbaa7ee1b3682bd3f374e803dff))
    - remove unnecessary doc comments; remove unused field in Options ([`e94ded4`](https://github.com/Byron/gitoxide/commit/e94ded49d8913bd74aff556ee745e83c5dffc3ac))
    - add TODO to not forget reworking the 'fs' module ([`0b032e4`](https://github.com/Byron/gitoxide/commit/0b032e44c1c124c80039e48cdf539e2bda68607c))
    - use the same BOM bytes as in git-attributes ([`7204755`](https://github.com/Byron/gitoxide/commit/7204755a4e800dfc58cc667f4e751359badf548b))
    - refactor ([`85be984`](https://github.com/Byron/gitoxide/commit/85be98437be80d8f79fbfbc032972e4395f19ef0))
    - add from_paths::Options::default(); minor refactor ([`bcd038c`](https://github.com/Byron/gitoxide/commit/bcd038cccc197cca9012db268dd7502d05c88369))
    - implement include.path support ([`a392988`](https://github.com/Byron/gitoxide/commit/a3929880e1639eba448aec15333dfaf08ac2dd28))
    - fix docs ([`3e7ef3e`](https://github.com/Byron/gitoxide/commit/3e7ef3e6bb5915126da5486ef627e4edf6a727ff))
    - more descriptive test names ([`049b243`](https://github.com/Byron/gitoxide/commit/049b2434dfbc97fa5734d852ebc8d07b18265e8a))
    - turn PathError into path::interpolate::Error; refactor ([`27085e0`](https://github.com/Byron/gitoxide/commit/27085e0e7a1d5067cbc5a8083953446bc6926c5d))
    - Work with std::path::* during interpolation ([`f0ff687`](https://github.com/Byron/gitoxide/commit/f0ff6879d0453be2fa2700f5a2432c3a5c830c31))
    - Fix build ([`f6d9693`](https://github.com/Byron/gitoxide/commit/f6d969370b8ef05b3b29983dcd9f6fa11d6225f2))
    - Make `Path::interpolate()` more useful by returning an actual `PathBuf` ([`86aa7b3`](https://github.com/Byron/gitoxide/commit/86aa7b3a98f933d9eff377fc426f37a22bf473be))
    - Don't interpolate on Path creation due to missing context ([`a071ce8`](https://github.com/Byron/gitoxide/commit/a071ce8f49cd70802776effbd25777a4e65d036c))
    - Add AsRef and Deref for values::Path; additional assertions ([`0666a35`](https://github.com/Byron/gitoxide/commit/0666a358b3b7aadda504979e543cc2058b478bfe))
    - Add `git_config::values::Path` for a typesafe git path ([`e4d6685`](https://github.com/Byron/gitoxide/commit/e4d6685064ad2b433f8acd3a74b320bf0169a994))
 * **Uncategorized**
    - Release git-config v0.2.0 ([`ddfe833`](https://github.com/Byron/gitoxide/commit/ddfe833c13a9fd46aa96283bc3bb372e3f7d82ce))
    - Release git-features v0.20.0, git-config v0.2.0 ([`a6460db`](https://github.com/Byron/gitoxide/commit/a6460db80ba3c49ea37c712465c7cbdefa5c32b6))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - derive PartialEq, Eq and Debug on ResolvedGitConfig ([`b2a88a5`](https://github.com/Byron/gitoxide/commit/b2a88a5af259ec07c51d873cac172bb60d7575aa))
    - Merge branch 'svetli-n-refactor_git_config_tests' ([`babaa9f`](https://github.com/Byron/gitoxide/commit/babaa9f5725ab8cdf14e0c7e002c2e1de09de103))
    - Refactor git_config tests. ([`714ef5c`](https://github.com/Byron/gitoxide/commit/714ef5c2cdea2af4026dba91119845ff68298d8d))
    - make fmt ([`7cf3545`](https://github.com/Byron/gitoxide/commit/7cf354509b545f7e7c99e159b5989ddfbe86273d))
    - Merge branch 'main' into mailmap ([`b2df941`](https://github.com/Byron/gitoxide/commit/b2df941feaf5ae9fa170fa49270189f3527f2eab))
    - thanks clippy ([`a87844a`](https://github.com/Byron/gitoxide/commit/a87844ab8b03357a52bea6a36002c8f1f1c3a5bb))
    - Revert "Move tests out of git_config." ([`3cbe072`](https://github.com/Byron/gitoxide/commit/3cbe072b024848c0133b6800dc84e68f58eee621))
    - Move tests out of git_config. ([`7fd8369`](https://github.com/Byron/gitoxide/commit/7fd83692411d8c5d392875c877f3f25985123f00))
    - Relative include path from env is error. ([`e303466`](https://github.com/Byron/gitoxide/commit/e303466857484eab5110a11d90f482f32943f74a))
    - Refactor and add skip bom when reading config. ([`2d5768d`](https://github.com/Byron/gitoxide/commit/2d5768dc9b40e0b830e0cc5aefb77f5e030bb8f8))
    - Replace `GitConfigFromEnvError` with `from_env::Error`. ([`e1f8b52`](https://github.com/Byron/gitoxide/commit/e1f8b527067cb3dcc74a33c238ba4edfafa95789))
    - Add path include to `from_env`. Follow duplicate include paths until max include depth is exceeded. ([`2295dc5`](https://github.com/Byron/gitoxide/commit/2295dc5c8fbbd6e27292dfd7a489ad0567421155))
    - Return error when max allowed nested includes depth is passed. ([`9692694`](https://github.com/Byron/gitoxide/commit/969269475d76a3bad323a9cc6e5b9d0f436ddc37))
    - remove `git_config::values::Value`; use `Bytes` in its place. ([`55c00d8`](https://github.com/Byron/gitoxide/commit/55c00d880535a1f8c37cb7d4405d39ff5a7654a0))
    - Merge branch 'svetli-n-path_value' ([`e8383ca`](https://github.com/Byron/gitoxide/commit/e8383caf6db211beb57d70019fe4ad13ce9066ee))
    - Minor fixes ([`c72ca00`](https://github.com/Byron/gitoxide/commit/c72ca0098e4daa153186789143a192ed38e9656c))
    - make fmt; fix build ([`ae4f122`](https://github.com/Byron/gitoxide/commit/ae4f122d191f1e4ee63bd11971fd61dfdd60bc8f))
    - Use context in PathError. ([`3b55f25`](https://github.com/Byron/gitoxide/commit/3b55f257a7ff1b89eea6616d61dfd51d409c797b))
    - Small refactoring and documentation. ([`fefb01b`](https://github.com/Byron/gitoxide/commit/fefb01b84f954700b19e010282c4b413de8e03d2))
</details>

## 0.1.11 (2022-01-31)

### New Features

 - <csr-id-e822f566dcff3f6c784c206dff2fbc5f82d543be/> subsection iteration.
   
   introduce method `sections_by_name_with_header` to allow iterating over tuples of
   section header and section body.

### Bug Fixes

 - <csr-id-469406dc0d9fece4a06230ef0d8018846202f0ad/> fix usage example in README.md

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 6 calendar days.
 - 7 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#319](https://github.com/Byron/gitoxide/issues/319)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#319](https://github.com/Byron/gitoxide/issues/319)**
    - update changelog prior to release ([`858ec8c`](https://github.com/Byron/gitoxide/commit/858ec8cc25f18c435465baee762def3013743f0b))
    - Adjust docs ([`38c201c`](https://github.com/Byron/gitoxide/commit/38c201c505ce2ea4257cdd0255713154745a330c))
    - An example to illustrate the problem ([`c47e8f8`](https://github.com/Byron/gitoxide/commit/c47e8f8ee8ea79f8f654f6c28e54e0b0b1fff1b6))
 * **Uncategorized**
    - Release git-config v0.1.11 ([`a605b67`](https://github.com/Byron/gitoxide/commit/a605b67294773628590220600f5017c63911f620))
    - fix usage example in README.md ([`469406d`](https://github.com/Byron/gitoxide/commit/469406dc0d9fece4a06230ef0d8018846202f0ad))
    - Merge branch 'sassman-config-subsection-iter' ([`e348b2b`](https://github.com/Byron/gitoxide/commit/e348b2bc1c6dd33009709dee366d01c9dde38d1f))
    - implement a draft for subsection fetching ([`e822f56`](https://github.com/Byron/gitoxide/commit/e822f566dcff3f6c784c206dff2fbc5f82d543be))
</details>

## 0.1.10 (2022-01-23)

### New Features

 - <csr-id-61af06b905926849abce19677ff4b9ac05d625a3/> compatibility with Rust <1.53

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 51 calendar days.
 - 55 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#266](https://github.com/Byron/gitoxide/issues/266)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#266](https://github.com/Byron/gitoxide/issues/266)**
    - upgrade dependencies ([`8adf0d8`](https://github.com/Byron/gitoxide/commit/8adf0d80bbd5c4e81ccd0b5363dbce6609a6c90a))
 * **Uncategorized**
    - Release git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`8f57c29`](https://github.com/Byron/gitoxide/commit/8f57c297d7d6ed68cf51415ea7ede4bf9263326e))
    - Release git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`d78aab7`](https://github.com/Byron/gitoxide/commit/d78aab7b9c4b431d437ac70a0ef96263acb64e46))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - prepar changelogs for cargo-smart-release release ([`8900d69`](https://github.com/Byron/gitoxide/commit/8900d699226eb0995be70d66249827ce348261df))
    - compatibility with Rust <1.53 ([`61af06b`](https://github.com/Byron/gitoxide/commit/61af06b905926849abce19677ff4b9ac05d625a3))
    - thanks clippy ([`7dd2313`](https://github.com/Byron/gitoxide/commit/7dd2313d980fe7c058319ae66d313b3097e3ae5f))
</details>

## 0.1.9 (2021-11-29)

A maintenance release.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 9 calendar days.
 - 12 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0 ([`d3f9227`](https://github.com/Byron/gitoxide/commit/d3f922781a81e8fbb81aa47afdbe9afeb06d666b))
    - Release git-features v0.18.0, git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0, safety bump 12 crates ([`acd3737`](https://github.com/Byron/gitoxide/commit/acd37371dcd92ebac3d1f039224d02f2b4e9fa0b))
    - Adjust changelogs prior to release ([`ec38950`](https://github.com/Byron/gitoxide/commit/ec3895005d141abe79764eaff7c0f04153e38d73))
    - Fix build warnings related to pin-project lite ([`126aeec`](https://github.com/Byron/gitoxide/commit/126aeec1f4cb358c7d24fec4fb0a92e7ff9319e8))
    - thanks clippy ([`db1bb99`](https://github.com/Byron/gitoxide/commit/db1bb99101a9248b464b0df9f526067b8f2a184e))
    - Add `GitConfig::from_env_paths` with git-like sequence resolution ([`aec51a2`](https://github.com/Byron/gitoxide/commit/aec51a2240c548a0737e61aeaebc2997945af197))
</details>

## 0.1.8 (2021-11-16)

A maintenance release triggered by changes to git-pack and changelog rewrites.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 19 commits contributed to the release over the course of 25 calendar days.
 - 31 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#241](https://github.com/Byron/gitoxide/issues/241), [#254](https://github.com/Byron/gitoxide/issues/254)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#241](https://github.com/Byron/gitoxide/issues/241)**
    - Improve usability of the pack-cache environment variable ([`47d8162`](https://github.com/Byron/gitoxide/commit/47d81629a0bfa2eccf75cbe081de55d80d0abd59))
 * **[#254](https://github.com/Byron/gitoxide/issues/254)**
    - Adjust changelogs prior to git-pack release ([`6776a3f`](https://github.com/Byron/gitoxide/commit/6776a3ff9fa5a283da06c9ec5723d13023a0b267))
 * **Uncategorized**
    - Release git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0 ([`f606fa9`](https://github.com/Byron/gitoxide/commit/f606fa9a0ca338534252df8921cd5e9d3875bf94))
    - better changelog descriptions. ([`f69b2d6`](https://github.com/Byron/gitoxide/commit/f69b2d627099639bc144fd94fde678d84a10d6f7))
    - Adjusting changelogs prior to release of git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0, safety bump 5 crates ([`39b40c8`](https://github.com/Byron/gitoxide/commit/39b40c8c3691029cc146b893fa0d8d25d56d0819))
    - Remove stale clippy allow ([`8441e92`](https://github.com/Byron/gitoxide/commit/8441e9217def0c77cfb69a75d98644ec6a9b46d9))
    - Note, not zero-copy nor alloc ([`75879b0`](https://github.com/Byron/gitoxide/commit/75879b0997afe87af96ccdff44b2c1a696aa223e))
    - Comment ([`4b00d68`](https://github.com/Byron/gitoxide/commit/4b00d6898bd21a7bd924b39c0ddb90f7c36e014b))
    - Lint ([`e700284`](https://github.com/Byron/gitoxide/commit/e7002844fbab0d415b9656395450402f2de7539b))
    - Format ([`960dcdc`](https://github.com/Byron/gitoxide/commit/960dcdc6752685e19b97e56f3fae9bc45a9ced4c))
    - Add multi value test ([`f3bcefb`](https://github.com/Byron/gitoxide/commit/f3bcefbd83d5c6f78a710b031c93342658b4a3a1))
    - Assert error kind ([`763266d`](https://github.com/Byron/gitoxide/commit/763266d24746247dc333916761561a12a210a767))
    - Assert io error ([`03541c5`](https://github.com/Byron/gitoxide/commit/03541c579027dc4b00745f573bb41c043cea087a))
    - Not mutable ([`0cfe8a4`](https://github.com/Byron/gitoxide/commit/0cfe8a40c7087fa744cd0b51878c7369f89a3801))
    - Rename test ([`bfcad07`](https://github.com/Byron/gitoxide/commit/bfcad07650398e83bb27201b25e61342ad20a03e))
    - Assert invalid paths ([`be4a4ea`](https://github.com/Byron/gitoxide/commit/be4a4ea2735060aee59f23cc742b6d97a324cb79))
    - Assert config len ([`bbce210`](https://github.com/Byron/gitoxide/commit/bbce210326311ef1d10b12d19c35b39a7606412b))
    - Remove debug print ([`cdf88e6`](https://github.com/Byron/gitoxide/commit/cdf88e6f8a1dab3c7dbd24314232908ae4a7b8ad))
    - First pass ([`a424d5a`](https://github.com/Byron/gitoxide/commit/a424d5adff97adf421aa9b1a3da9c39148c12144))
</details>

## v0.1.7 (2021-10-15)

This is a maintenance release without functional changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 3 calendar days.
 - 38 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#198](https://github.com/Byron/gitoxide/issues/198), [#213](https://github.com/Byron/gitoxide/issues/213)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Maintenance release note to avoid being fully generated ([`56ef363`](https://github.com/Byron/gitoxide/commit/56ef363f0e7a8b9106765d96d0636e38b2bed550))
    - Changlog for git-config ([`abdfe58`](https://github.com/Byron/gitoxide/commit/abdfe588030b0fbdd4d69a73c5739ef4a83e3616))
    - Use correct title for github release to match name of tag ([`90f39ad`](https://github.com/Byron/gitoxide/commit/90f39ad693e0998bc3307bf553fccdc37c8dc0c8))
 * **[#213](https://github.com/Byron/gitoxide/issues/213)**
    - refactor ([`e906d37`](https://github.com/Byron/gitoxide/commit/e906d37e0b4e088b7973728db386a23ea7645fc9))
    - Remove environment variable after test passed ([`7a3ff29`](https://github.com/Byron/gitoxide/commit/7a3ff293048dd6bebec492bd79b12d7889fee3a1))
 * **Uncategorized**
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com/Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Remove after ([`cb72bef`](https://github.com/Byron/gitoxide/commit/cb72befbe08590f29489bde1b85c1582d729e0c4))
    - Mark tests as serial ([`0456142`](https://github.com/Byron/gitoxide/commit/0456142c2ee79c39fd738b5e0ef5a258e56d524f))
    - Add a test for multiple sections ([`0ad6438`](https://github.com/Byron/gitoxide/commit/0ad6438b0c19ef6cd7db469cb3f45f3f820665fd))
    - Format ([`d743ef8`](https://github.com/Byron/gitoxide/commit/d743ef8a4fb08511100650b7cbd027491ecb54de))
    - Add a test for a single key value pair ([`a64d312`](https://github.com/Byron/gitoxide/commit/a64d312313ae9f268747e400ba78cd6254d91426))
    - Add a test case for GIT_CONFIG_COUNT parse error ([`a864812`](https://github.com/Byron/gitoxide/commit/a86481207c592eef9abf3b382fe658370657d296))
    - Add test case for GIT_CONFIG_COUNT=0 ([`c33b498`](https://github.com/Byron/gitoxide/commit/c33b498cac29f04d260e361622a4ee86c035a9c1))
</details>

## v0.1.6 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 7 calendar days.
 - 8 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-config v0.1.6 ([`b1b6fe0`](https://github.com/Byron/gitoxide/commit/b1b6fe0af52d0ec133cd9ec9ffd5a173ba14a5d2))
    - [repository #185] rustfmt ([`dfbb015`](https://github.com/Byron/gitoxide/commit/dfbb015a89db47c79015135870013ecc384c4aea))
    - [config #185] refactor ([`509c938`](https://github.com/Byron/gitoxide/commit/509c938dd061060141756ee791cdcb6017934fe2))
    - [config #185] Count lines correctly on windows… ([`57203ce`](https://github.com/Byron/gitoxide/commit/57203ce5d5e3c481b69c3ca173e4b00f11aaf7d7))
    - [config #185] add test for handling windows formatted files… ([`2a2a89f`](https://github.com/Byron/gitoxide/commit/2a2a89f68cc45e27a1cf0d33fc644ebabc762302))
    - [config #185] flyby refactor ([`9b9ffa3`](https://github.com/Byron/gitoxide/commit/9b9ffa3c1d5ccbea22aa38b740daa8a349494395))
</details>

## v0.1.5 (2021-08-29)

- maintenance release

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 10 calendar days.
 - 12 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-config v0.1.5 ([`150ed76`](https://github.com/Byron/gitoxide/commit/150ed760c8b357e5c40ec0bd8d0cd849b39c34c0))
    - [various #184] configure docs.rs build features ([`cc50249`](https://github.com/Byron/gitoxide/commit/cc502492c512293e93e95610ca80a71896076ded))
    - [object #177] fix docs ([`2fd23ed`](https://github.com/Byron/gitoxide/commit/2fd23ed9ad556b8e46cf650e23f0c6726e304708))
    - [actor #173] rename immutable::Signature to SignatureRef! ([`96461ac`](https://github.com/Byron/gitoxide/commit/96461ace776d6b351b313d4f2697f2d95b9e196e))
    - Merge branch 'Byron:main' into main ([`dc58eca`](https://github.com/Byron/gitoxide/commit/dc58eca510e5a067acdeaad4b595a34b4598a0cd))
    - Upgrade to nom-7 ([`f0aa3e1`](https://github.com/Byron/gitoxide/commit/f0aa3e1b5b407b2afd187c9cb622676fcddaf706))
    - [smart-release #162] format everything ([`8ff83e5`](https://github.com/Byron/gitoxide/commit/8ff83e5c511ae29979348789bd6e7a2f72b16f1c))
</details>

## v0.1.4 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release.
 - 2 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-config v0.1.4 ([`535ff79`](https://github.com/Byron/gitoxide/commit/535ff79d6d28d3f08572f4353a8db4da2b658473))
    - [git-config] Resolved config construction ([`1ab44c0`](https://github.com/Byron/gitoxide/commit/1ab44c06b30b745711bda3711b5ce92dfae306be))
    - [config] Allow certain warnings during development, fix docs ([`1a2f408`](https://github.com/Byron/gitoxide/commit/1a2f408d045b48925062646bf014d419bd753086))
    - Don't enable resolved module yet ([`0bd05b2`](https://github.com/Byron/gitoxide/commit/0bd05b22c86b366bdd01be747ffd5207434ece0d))
    - disable all git-config lints ([`05687b4`](https://github.com/Byron/gitoxide/commit/05687b471cb1cbaa8785ec09177c949773dac05a))
    - disable lint ([`b4302cd`](https://github.com/Byron/gitoxide/commit/b4302cd257e6c76cd85c3af5f28457a1ed91f098))
    - rustfmt git-config for consistency ([`b559dd0`](https://github.com/Byron/gitoxide/commit/b559dd0eda1b210eb996b3e9518d6264e614035f))
    - Add todos ([`dbcd79a`](https://github.com/Byron/gitoxide/commit/dbcd79a0b9776ad2e9f5ca0ff2ed965d3d52c104))
    - Fix contains_key ([`50f9122`](https://github.com/Byron/gitoxide/commit/50f91225b903c8d45a7f3c4a3754b03bc80ccc45))
    - Add IntoIterator for SectionBody ([`d37b17c`](https://github.com/Byron/gitoxide/commit/d37b17c55d35b76cd831e51f18c3b0942bc53724))
    - Document GitConfigFromEnvError ([`eb44cf6`](https://github.com/Byron/gitoxide/commit/eb44cf675cb49a313220377b05f0eded422f7e09))
    - More git-config docs ([`f05a669`](https://github.com/Byron/gitoxide/commit/f05a66905c12844515860d0d5e5e113e05df54cb))
</details>

## v0.1.3 (2021-08-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 6 calendar days.
 - 8 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-config v0.1.3 ([`319a4ae`](https://github.com/Byron/gitoxide/commit/319a4ae2b71f4e847757aa46f1d9fcc4b4ee12ca))
    - [config] pacify clippy ([`ad41ba6`](https://github.com/Byron/gitoxide/commit/ad41ba6c96da28d704163a455c3185aec7050db5))
    - Fix bench path ([`70f9403`](https://github.com/Byron/gitoxide/commit/70f94032ca7fcad5eaa9cd0064720d72569f9c17))
</details>

## v0.1.2 (2021-08-06)

### Added

 - Added the following methods to `GitConfig`:
   - `is_empty`
   - `len`
   - `from_env`
   - `open`
- `len`
- `from_env`
- `open`

### Changed

 - `parse_from_path` now accepts a `AsRef<Path>` instead of a `&Path`.
 - `parse_from_path` now returns an `ParserOrIoError<'static>` instead, from
   `ParserFromIoError`

### Fixed

 - _None._

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 17 commits contributed to the release over the course of 86 calendar days.
 - 89 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Fix bench path ([`bf0004e`](https://github.com/Byron/gitoxide/commit/bf0004e05ede404921073755aadd8ab8f75273c9))
    - Bump git-config to 0.1.2 ([`9c275dc`](https://github.com/Byron/gitoxide/commit/9c275dc6f1a07ebd6c4cc8ae0edae382bd13c0cf))
    - Use newtyped Index and Size ([`15ae2d7`](https://github.com/Byron/gitoxide/commit/15ae2d76bb1b4bd64d3ee50021a359a777e95538))
    - Re-export everything in git-config::file::mod ([`392c131`](https://github.com/Byron/gitoxide/commit/392c13175892ecf6e543ee6a1fd47c62a38f09cb))
    - Fix rustdoc links ([`042eaf4`](https://github.com/Byron/gitoxide/commit/042eaf4b4625ed47b417a9012556ef6fc69aa2d6))
    - Use AsRef<Path> when opening from path ([`515d256`](https://github.com/Byron/gitoxide/commit/515d2564e430da77c092ceb9414a3b3e7071c158))
    - Add GitConfig::from_env ([`17e30a1`](https://github.com/Byron/gitoxide/commit/17e30a1ede39326cda6c64989ab37d979c9c4a29))
    - Add GitConfig::from_path ([`27df3d1`](https://github.com/Byron/gitoxide/commit/27df3d1d5de1e7660beaf599e4931c3cf7c1f99a))
    - Add is_empty and len to GitConfig ([`aa86594`](https://github.com/Byron/gitoxide/commit/aa865942559ee48d7998adb211a6a8f4e0760375))
    - split file.rs into module ([`da40593`](https://github.com/Byron/gitoxide/commit/da40593b3e4d35dcdf8003123cefc0e367367734))
    - clippy on tests and thanks clippy ([`a77a71c`](https://github.com/Byron/gitoxide/commit/a77a71cf02d328a2a964388928d6b2a235a0aa85))
    - thanks clippy ([`e1964e4`](https://github.com/Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - Merge branch 'parser-into-iter-without-alloc' ([`a799ca8`](https://github.com/Byron/gitoxide/commit/a799ca8d6c2e51303512160ddef7477e176ab01b))
    - Implement Parser::into_iter without extra allocation ([`aa79924`](https://github.com/Byron/gitoxide/commit/aa79924b36c0d717cc65d7471fedd27eb41e83a5))
    - clippy cleanup; fix CI build ([`3e943f2`](https://github.com/Byron/gitoxide/commit/3e943f2afd5f0cfe7294a21cca8e0344c7dd0216))
    - thanks clippy ([`6200ed9`](https://github.com/Byron/gitoxide/commit/6200ed9ac5609c74de4254ab663c19cfe3591402))
    - [git-config] Annotate more functions with inline ([`2006acb`](https://github.com/Byron/gitoxide/commit/2006acb381a3a9e807575991a8eeab1ea010af60))
</details>

<csr-unknown>
lenfrom_envopen<csr-unknown/>

## v0.1.1 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 56 calendar days.
 - 58 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.1.1 ([`e583f70`](https://github.com/Byron/gitoxide/commit/e583f70947803b5b6885a4eb22cd515263177b5b))
    - thanks clippy ([`17258cc`](https://github.com/Byron/gitoxide/commit/17258cc58767caa6e71227898decd160ad0cdf13))
    - TODO's about 'Iterator::reduce()' are probably not applicable ([`ac1a433`](https://github.com/Byron/gitoxide/commit/ac1a4333612c7ef238b84d15a194d4bc4685cd3a))
    - Thank cargo-doc ([`ca96be1`](https://github.com/Byron/gitoxide/commit/ca96be1654a175606a4af6032b2ace4875334231))
    - [git-config] Finish cleaning up 1.51 clippy lints ([`aec7240`](https://github.com/Byron/gitoxide/commit/aec7240036750c98796b8ef4075758f6b825d293))
    - [git-config] Fix various 1.51 clippy lints; inline ([`d899df0`](https://github.com/Byron/gitoxide/commit/d899df0d9feec1f38b60be73af80113958dfa7d1))
    - [git-config] Fix must_use lints ([`71aff75`](https://github.com/Byron/gitoxide/commit/71aff75d02329caf78c61d3c1dd8ab3c33b8597d))
    - Slim down git-config with cargo-diet ([`1c555e0`](https://github.com/Byron/gitoxide/commit/1c555e04d395eadb6b22639afd41c0892d48fa0d))
    - [git-config] add parse test from git remote ([`63bee9c`](https://github.com/Byron/gitoxide/commit/63bee9c3217689df5fbe36d79857db7cdd349d84))
    - [git-config] Add sections_by_name ([`1f7a533`](https://github.com/Byron/gitoxide/commit/1f7a53357d0f1f2f8164b59e8b276ae61fff552f))
    - [git-config] Add to_owned for parser::Error ([`e316c8c`](https://github.com/Byron/gitoxide/commit/e316c8c7a8864daf2ade0ec8fdf42aa20694805f))
    - [git-config] Add coercion into owned variants ([`6387aea`](https://github.com/Byron/gitoxide/commit/6387aeaefccb2c80f9a276f3a8978be28f23bdfb))
</details>

## v0.1.0 (2021-03-12)

<csr-id-949622e461eb2116393ec6f4633ec0cb2e1695b5/>
<csr-id-bcacfc9bcf19a0339541b24e84de68d95291c62b/>
<csr-id-41f118d2aa560188fd3399d2390aa43794b0af75/>

### Other

 - <csr-id-949622e461eb2116393ec6f4633ec0cb2e1695b5/> Include benches in crate to allow publishing to work
 - <csr-id-bcacfc9bcf19a0339541b24e84de68d95291c62b/> remove clippy-cargo lint until there are no warnings
   Please feel free to re-add once all other git-* crates have been
   adjusted. It's interesting to see how a crate lint spills into the
   workspace.
   
   Personally I am surprised that the keywords value is supposed
   to repeat the crate name as it seems redundant.
 - <csr-id-41f118d2aa560188fd3399d2390aa43794b0af75/> remove redundant lines from git-ignore file

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 125 commits contributed to the release over the course of 157 calendar days.
 - 158 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Include benches in crate to allow publishing to work ([`949622e`](https://github.com/Byron/gitoxide/commit/949622e461eb2116393ec6f4633ec0cb2e1695b5))
    - remove clippy-cargo lint until there are no warnings ([`bcacfc9`](https://github.com/Byron/gitoxide/commit/bcacfc9bcf19a0339541b24e84de68d95291c62b))
    - remove redundant lines from git-ignore file ([`41f118d`](https://github.com/Byron/gitoxide/commit/41f118d2aa560188fd3399d2390aa43794b0af75))
    - fix format ([`1655b56`](https://github.com/Byron/gitoxide/commit/1655b56b34618d28f67b1ef08b41c598eaf3530e))
    - remove release-profile ([`3d62449`](https://github.com/Byron/gitoxide/commit/3d62449feed68cef213df31268dfbb9fc54f2d62))
    - rename_section ([`4975fff`](https://github.com/Byron/gitoxide/commit/4975fff3edc67a39bd0046870ac8c572c09d0f78))
    - return key iterator ([`adfa460`](https://github.com/Byron/gitoxide/commit/adfa460f8ae0cdbbe8a3b91e0d0c55e46bb9b9ce))
    - add push_section ([`81271e2`](https://github.com/Byron/gitoxide/commit/81271e24bcf9fc9f7241bbd70d11e0cbbab789a1))
    - more work on sections ([`84e959d`](https://github.com/Byron/gitoxide/commit/84e959da3a19abb451be82d290fa8329ee1df015))
    - remove offset newtype ([`41da7ed`](https://github.com/Byron/gitoxide/commit/41da7edc1b8661e2e3f53b4b061f7e52c9604a06))
    - remove section for owned section type ([`11cf526`](https://github.com/Byron/gitoxide/commit/11cf526769998915e542a93d01023f67b3329fa6))
    - more functionality to mutablesection ([`b0cf849`](https://github.com/Byron/gitoxide/commit/b0cf849f32fb6b0e612efa12f279a6e0bb40d49d))
    - optimize section pushing ([`f409931`](https://github.com/Byron/gitoxide/commit/f4099310dfad85c087bae45a9af2a441569c58fa))
    - section API ([`0aad0f1`](https://github.com/Byron/gitoxide/commit/0aad0f12ed82caa784abde36a9dd4fe2f0cc83bf))
    - section stuct ([`21b4fe1`](https://github.com/Byron/gitoxide/commit/21b4fe11001fa4f09718bc5cacc140a0a97e8ab3))
    - fix example ([`357a761`](https://github.com/Byron/gitoxide/commit/357a76137c8c309b6c16809af32641d1f52a1222))
    - update readme ([`c2fa869`](https://github.com/Byron/gitoxide/commit/c2fa869ffb37180f67c37f2c229b7b26390d6957))
    - fix macro comment gen ([`a19c17f`](https://github.com/Byron/gitoxide/commit/a19c17f6d64bd87ec42ec975a9b4a8e641288816))
    - implement case insensitivity for names ([`c39ff33`](https://github.com/Byron/gitoxide/commit/c39ff332415a7c546af14bf925c9cc5c60b36622))
    - test MutableMultiValue ([`8cfe67d`](https://github.com/Byron/gitoxide/commit/8cfe67df4f322d7ebfee9f5c7de206cacf08f5ed))
    - more tests, fix mutablevalue ([`377532c`](https://github.com/Byron/gitoxide/commit/377532c4b6a3bf2ac8cc3ca7c3c661cfc954a16c))
    - fix lints ([`bb7a544`](https://github.com/Byron/gitoxide/commit/bb7a5445272e8abd825751212762a76e7876e9d1))
    - enable requiring docs ([`68320ca`](https://github.com/Byron/gitoxide/commit/68320ca5ba2d3665bc64580ff9ab12d86b719e56))
    - clippy fix ([`e7bad2e`](https://github.com/Byron/gitoxide/commit/e7bad2eb2633b60f1bf12b7f53dcda1f431172d4))
    - docs ([`a1f833c`](https://github.com/Byron/gitoxide/commit/a1f833ccc7c07b37b5285c109aefaabf9a97f202))
    - add into bytes for gitconfig ([`9b54a5b`](https://github.com/Byron/gitoxide/commit/9b54a5b9df699f2508cf47411293f74252a02473))
    - docs ([`4d7da4e`](https://github.com/Byron/gitoxide/commit/4d7da4e9ef60a90360e51d5425580d9b309db151))
    - mutableevent interface ([`014776f`](https://github.com/Byron/gitoxide/commit/014776f8be482d2d61ef532d1cea84e26a42d893))
    - remove serde code for now ([`fc4ee85`](https://github.com/Byron/gitoxide/commit/fc4ee8585372c023b47c0f28d0c746ff9f8eac5b))
    - disable serde ([`7c01808`](https://github.com/Byron/gitoxide/commit/7c01808c4fdfee35e339c2d939ef6b013b430f38))
    - multablemultivalue ([`faa1b93`](https://github.com/Byron/gitoxide/commit/faa1b9368b8dbcfbef10a3bbb027bdef81a377a0))
    - better test formatting ([`635e5c1`](https://github.com/Byron/gitoxide/commit/635e5c15e39b24767e793e6ab4f230b8913a0760))
    - Add get_multi_value ([`79eeca1`](https://github.com/Byron/gitoxide/commit/79eeca128a3b146e2900cc90f34877cef4fa6e52))
    - check all sections for lookup before failing ([`01b617d`](https://github.com/Byron/gitoxide/commit/01b617d74f10eab8f87e1032aebeeb6f56a2ae10))
    - misc improvements ([`87057c9`](https://github.com/Byron/gitoxide/commit/87057c9f03516d6659cb2cc54f330ba4072d6563))
    - benchmarks ([`b0ff69e`](https://github.com/Byron/gitoxide/commit/b0ff69e9f5260c30ecfd2879b3346437b38cec83))
    - crate level docs ([`a909bcf`](https://github.com/Byron/gitoxide/commit/a909bcf90fa7ad3995fa15f00417257f66ccb6d0))
    - integration tests for value extraction ([`d45af63`](https://github.com/Byron/gitoxide/commit/d45af630ab25cfc3fc9b4b10c5038132f23b5c95))
    - fix drain ([`4f425a7`](https://github.com/Byron/gitoxide/commit/4f425a72cad6b0159e3747ce6a42c4d4343b8e61))
    - use memrchr ([`ccadf89`](https://github.com/Byron/gitoxide/commit/ccadf89aa13051c55debd6aa7c709b138bab6167))
    - use drain instead ([`6e5b67b`](https://github.com/Byron/gitoxide/commit/6e5b67b0d5dec90f22cf05b1597201c8a8b9ab80))
    - use mutablevalue for mut entries ([`13fdda5`](https://github.com/Byron/gitoxide/commit/13fdda5e01cb494f5992be0bb82464392016d4ed))
    - normalize get_raw_value ([`5952cab`](https://github.com/Byron/gitoxide/commit/5952cab59c3aab7f35683f959737a5c8bef29fb1))
    - fix get_raw_value, fix returning refs to cows ([`ba982b9`](https://github.com/Byron/gitoxide/commit/ba982b971aa97eaa17d58d7b00f20923171d89eb))
    - cleanup docs ([`e0a8b8d`](https://github.com/Byron/gitoxide/commit/e0a8b8d808cbed2d7ca902304e6819bfba8f6715))
    - add tests and docs ([`7caf012`](https://github.com/Byron/gitoxide/commit/7caf012b05c6819f5a897918fc50eb61d76517de))
    - rename config mod to file ([`a965ebc`](https://github.com/Byron/gitoxide/commit/a965ebcfc08c71255389c62fe43ff479960e7921))
    - pendantic clippy lints ([`18c9dff`](https://github.com/Byron/gitoxide/commit/18c9dff7c0bde99d4c1d4a7263f86fd4a656d1c0))
    - remove unnecessarily lifetimes ([`7d0e6b4`](https://github.com/Byron/gitoxide/commit/7d0e6b4fe1cb8e3d752cd5b42fa9167b552d6320))
    - use str in most cases ([`9fc8993`](https://github.com/Byron/gitoxide/commit/9fc8993a54950e88aa05b0fe85962ee124a86891))
    - fully comment values ([`8e32d56`](https://github.com/Byron/gitoxide/commit/8e32d5609d81087e17a5dcd15dbe7ed22594aa50))
    - more normalize docs ([`9767b5b`](https://github.com/Byron/gitoxide/commit/9767b5be5a4d45c44e2aba6c2164c800e17ec437))
    - collaspe if block ([`1cf1f3b`](https://github.com/Byron/gitoxide/commit/1cf1f3be284afb1b0ed3d8f06439a5effc553be0))
    - better doc ([`ec63ce6`](https://github.com/Byron/gitoxide/commit/ec63ce633578af6adf6239d76f5802aaa842941c))
    - implement unquoting in normalize ([`7e8ae93`](https://github.com/Byron/gitoxide/commit/7e8ae932f888707fe7466e93ef6c8289749d04d2))
    - add normalize ([`6c245dc`](https://github.com/Byron/gitoxide/commit/6c245dc654bfd7bc6b82226abd900ba2e8a312cd))
    - dedup multivar docs ([`236d37b`](https://github.com/Byron/gitoxide/commit/236d37b608b8a912fc005c956c87d251172fff0f))
    - add todo ([`bc63005`](https://github.com/Byron/gitoxide/commit/bc630057c6f138647f2f8a2d93f8a09e4f4494c3))
    - Implement get_value for GitConfig ([`ca7c1dc`](https://github.com/Byron/gitoxide/commit/ca7c1dca5539bf71e524ba0ee4b40c60bd80f0ad))
    - Use traits instead of from_str ([`ce9b7bf`](https://github.com/Byron/gitoxide/commit/ce9b7bfbb9abe5e74dbef1d3c637876e7d996e52))
    - Use traits instead of shadowing from_str ([`a4ce9b0`](https://github.com/Byron/gitoxide/commit/a4ce9b04012060ab06dabd6658f3e518f994831b))
    - remove falsevariant ([`e10a4a2`](https://github.com/Byron/gitoxide/commit/e10a4a298371b6641e6184fa8d61bebe8c783923))
    - more tests ([`24a2dfd`](https://github.com/Byron/gitoxide/commit/24a2dfd2d7dd0c1b28b62b7dbebeb539e086016e))
    - remove unreachable variants ([`93b85e3`](https://github.com/Byron/gitoxide/commit/93b85e38ff46f0f8fdaabcf2c7210bb9e0421254))
    - use mut vec reference ([`8b68fdb`](https://github.com/Byron/gitoxide/commit/8b68fdb2aeac6dd7211f31333489012647e65f55))
    - Don't use mutex ([`4027daf`](https://github.com/Byron/gitoxide/commit/4027daf93bb931d9b839057b88afcfa849a4ed8c))
    - documented parsererror ([`0c226ad`](https://github.com/Byron/gitoxide/commit/0c226ad112f7ff70dee20669419ba8f7eae3f0c7))
    - clippy fixes ([`8618c22`](https://github.com/Byron/gitoxide/commit/8618c2233abada5fb101258cffc8c046b155134b))
    - don't use stack for error handling ([`819a1d3`](https://github.com/Byron/gitoxide/commit/819a1d3a0bf47c95dc469c7bbf80b3452ef9918d))
    - very rough error handling ([`45d5250`](https://github.com/Byron/gitoxide/commit/45d52502fb91635f3db3d4b09f69f5d7b2a29e09))
    - add error trait impl for ParserError ([`d173b4b`](https://github.com/Byron/gitoxide/commit/d173b4bcc855365640d8931460d0bed748264817))
    - Basic error reporting ([`f293334`](https://github.com/Byron/gitoxide/commit/f293334d984755053b532706134df88a5c57a43f))
    - move fully_sumed to test_util ([`41245eb`](https://github.com/Byron/gitoxide/commit/41245ebd74a0ee40bbc91a2e2f9bca670da02b16))
    - Don't immediately drop fuzzer values ([`568d360`](https://github.com/Byron/gitoxide/commit/568d36084f2c4f94a4757588ca17078ce523a6ac))
    - Add more fields to cargo.toml ([`89791fd`](https://github.com/Byron/gitoxide/commit/89791fd0f3bba9d22418302fe862896a6ccacdc4))
    - Add basic fuzzer ([`cf41bb3`](https://github.com/Byron/gitoxide/commit/cf41bb300fe8213a7b4b02329cd987fd55d2ac9c))
    - add from_bytes variants for parser ([`954f433`](https://github.com/Byron/gitoxide/commit/954f4338e7496208b1e8b13a105eec9f5c07ba76))
    - exclude fuzz folder from cargo ([`afe4ac7`](https://github.com/Byron/gitoxide/commit/afe4ac754f0843d8350504200a2a833bf158c335))
    - add tests for boolean ([`0353033`](https://github.com/Byron/gitoxide/commit/03530334a956e4337d3e92d2bb1a2be5c0278014))
    - Use lto and single codegen unit for release ([`53077bd`](https://github.com/Byron/gitoxide/commit/53077bda06aeed84b3985f941e7f4660b5e0d5e8))
    - select nom features ([`79dc19f`](https://github.com/Byron/gitoxide/commit/79dc19f7a7517f0953a1b14b180e54cfe110bab8))
    - make serde optional, clippy lints ([`5defc4a`](https://github.com/Byron/gitoxide/commit/5defc4a64a9ee37910c2caa9f23253adead1ab6f))
    - Add ColorValue tests ([`5b7cc13`](https://github.com/Byron/gitoxide/commit/5b7cc13e62aff1014369aa7a8d64dc9eaad1f0cd))
    - Add tests for ColorAttribute ([`0cc9cd6`](https://github.com/Byron/gitoxide/commit/0cc9cd6bd1d383bbca97610f6a1a67119a90ab56))
    - Fix docs ([`b0fc08b`](https://github.com/Byron/gitoxide/commit/b0fc08b940dbca8d6b78f29f5d4e391c8b535121))
    - Use BStr instead ([`ec2602c`](https://github.com/Byron/gitoxide/commit/ec2602cfce2867ed341a1bff5e26ca82785a4434))
    - Add key-value delimination event ([`df0da82`](https://github.com/Byron/gitoxide/commit/df0da822d23708cc488027c0830895f0274ad9ce))
    - gitconfig writing to string ([`b59a51a`](https://github.com/Byron/gitoxide/commit/b59a51af3896ba510c30db1044a11472df7d3998))
    - document multivar behavior better ([`e43518e`](https://github.com/Byron/gitoxide/commit/e43518ebdadd739bd4edc90cd4ef279000b4f94e))
    - Use Cow instead of strs ([`ff8ee4a`](https://github.com/Byron/gitoxide/commit/ff8ee4a0352ad7fb8a2c93c0e09f5b3b2c15d3a9))
    - Implement get_mut for gitconfig ([`1d8e58b`](https://github.com/Byron/gitoxide/commit/1d8e58b6008c36141ba38fd37bfbbbdb458b35ef))
    - remove meme comment ([`897450c`](https://github.com/Byron/gitoxide/commit/897450c2e7d89388072d806d4646e1bbac4df422))
    - test get_raw_values ([`0ea6210`](https://github.com/Byron/gitoxide/commit/0ea62105abd84b3b73996f8809772b6856fab6fa))
    - finish raw value queries for gitconfig ([`4b7f218`](https://github.com/Byron/gitoxide/commit/4b7f21875ee12a86c56c2942f2981a79993a3a2a))
    - Handle empty git-config file for parser ([`a516885`](https://github.com/Byron/gitoxide/commit/a5168857071db604f3b2e7191ed749bcdb0354af))
    - fully document parser ([`f66e0be`](https://github.com/Byron/gitoxide/commit/f66e0bedff4c7ba9d455bd7e4d24d299b21fb109))
    - completely refactor config ([`b820d6c`](https://github.com/Byron/gitoxide/commit/b820d6c987263251575844547298e69ddb52d8c4))
    - Booleans now retain original value ([`8ea467e`](https://github.com/Byron/gitoxide/commit/8ea467e01e7e231827041ada0c531f7a3e66715f))
    - parser is now perfect ([`3f708ec`](https://github.com/Byron/gitoxide/commit/3f708ecadee910aebc007fdba1aae004e3344104))
    - more work on parser ([`2691756`](https://github.com/Byron/gitoxide/commit/26917564cf007949ee7f71ee48e10be1efb6f6a0))
    - more work ([`eb07890`](https://github.com/Byron/gitoxide/commit/eb07890347fda7c24721ea91fca5a2eaa519d5b3))
    - Complete initial parser ([`d721625`](https://github.com/Byron/gitoxide/commit/d72162555cc677c4ff143d01f338fc4508a7b11e))
    - Deny rust-2018-idioms ([`a4d2a4b`](https://github.com/Byron/gitoxide/commit/a4d2a4b248f0f22ffd6c8c567d780a152831347a))
    - Add remaining docs for all types in 'git-config' crate ([`b7790b4`](https://github.com/Byron/gitoxide/commit/b7790b4ce3884daaff198890f4a8fb36c38f2230))
    - more planning for config parser implementation ([`9676db9`](https://github.com/Byron/gitoxide/commit/9676db9f58b5776986cfd7185a0ade93f89cb080))
    - Add missing '.' at end of doc comments ([`7136854`](https://github.com/Byron/gitoxide/commit/71368544f97369a4d371d43513607c4805bd0fd0))
    - Signal the compiler that configuration edits must be used ([`14b17e4`](https://github.com/Byron/gitoxide/commit/14b17e4d202fae2eeabfe46552a1c0b17e30ac9c))
    - better docs for git-config; name method for Entry ([`5ab4bdb`](https://github.com/Byron/gitoxide/commit/5ab4bdbc3f1760bcb667d9a1b26eb069084581eb))
    - refactor; more comments ([`8d933cb`](https://github.com/Byron/gitoxide/commit/8d933cbd08977ad0dc70ed18b37e7e06ab24c4fb))
    - time-constrained write-down of some high-level concepts of git-config structures ([`157fa2a`](https://github.com/Byron/gitoxide/commit/157fa2a31e0382ee2c8524ff7862873787f5f648))
    - cargo clippy Rust 1.48 ([`475a68c`](https://github.com/Byron/gitoxide/commit/475a68ce33b895de911939c51afa159df534f7b8))
    - a path towards making config Files editable ([`bc008c3`](https://github.com/Byron/gitoxide/commit/bc008c32a16849a212eced783aa14727765004c3))
    - additional setters for more fluid edits ([`5a54dae`](https://github.com/Byron/gitoxide/commit/5a54dae6470c5dcf48bf96c16c5bbe2a8951be6a))
    - sketch out editing lossless of Files ([`8f00063`](https://github.com/Byron/gitoxide/commit/8f00063bc9b6a63ffe44e58945be55acca40a714))
    - Skip comments as well ([`32cc684`](https://github.com/Byron/gitoxide/commit/32cc6849444c16a3d2917c6de62e47597c9979da))
    - Stop entry iteration when next section is encountered ([`83a1b83`](https://github.com/Byron/gitoxide/commit/83a1b83a1f7a0ff22850efc7b5b460f0c1ed8230))
    - sketch of iteration over sections and entries ([`acb8947`](https://github.com/Byron/gitoxide/commit/acb894762b38f77d21e6d70936727cf0daeaff6f))
    - sketch out section and entries access ([`06679d9`](https://github.com/Byron/gitoxide/commit/06679d9b69575183231ddb22edd89ab29357632d))
    - refactor ([`b5fa727`](https://github.com/Byron/gitoxide/commit/b5fa727403a78e5f9238dd36d8b071eec425d731))
    - Turn off 'unused' warnings for experimental git-config crate ([`0b52eb0`](https://github.com/Byron/gitoxide/commit/0b52eb0e75a268c5c7b6475677fd20acace3435b))
    - Revert "remove git-config from workspace while it's so fresh…" ([`99214f4`](https://github.com/Byron/gitoxide/commit/99214f4c1097fa8da8f14f1279caf00db78fa822))
    - remove git-config from workspace while it's so fresh… ([`84e0d19`](https://github.com/Byron/gitoxide/commit/84e0d19ab2285916cb6a6b941ec2206aef485d56))
    - Plan how to deal with whitespace and comments to be lossless ([`eb5a534`](https://github.com/Byron/gitoxide/commit/eb5a534340396429d7c2c95e71b0a23457d954f4))
    - refactor ([`3846bab`](https://github.com/Byron/gitoxide/commit/3846bab8c7ae53e5528388522bf4571260ec4ae6))
    - very first sketch of types for read-only git config ([`e2a39c9`](https://github.com/Byron/gitoxide/commit/e2a39c96a96b3ec9de519c685fe9caddeb89342c))
</details>

## v0.0.0 (2020-10-05)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - stub for git-config crate ([`3539531`](https://github.com/Byron/gitoxide/commit/3539531adb06e8f59609f0a83e8ed94d0864c0a1))
</details>

