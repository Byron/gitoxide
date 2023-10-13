# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.55.2 (2023-10-13)

### Bug Fixes

 - <csr-id-8011c73ee401bfca03811a249c46a4dd468af1b8/> bump `gix-transport` version to prevent it from being picked up.
   `gix-transport` v0.37.1 could accidentally be picked up by older, incompatible,
   `gix` versions which now fail to build.
   
   Thus v0.37.1 is now yanked and replaced with v0.38.0 along with a new
   release of `gix` to go with it.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Prepare changelogs prior to release ([`12b5caf`](https://github.com/Byron/gitoxide/commit/12b5cafc49baf07d00313de468970a2db33ac1f8))
    - Bump `gix-transport` version to prevent it from being picked up. ([`8011c73`](https://github.com/Byron/gitoxide/commit/8011c73ee401bfca03811a249c46a4dd468af1b8))
</details>

## 0.55.1 (2023-10-12)

### New Features

 - <csr-id-5732303180d26374016b70bdd7fa0278dd84cff3/> Add `take_data()` to all primitive object types.
   That is the new, most direct way to obtain its data which otherwise
   is immovable.
 - <csr-id-88f2e6c4c540b9c8032e6eee9c5da65a9bcfeef8/> Add `detach()` and `detached()` too all object types.
   That way, the detachment API is symmetric.
   It's required to overcome the `Drop` implementation of each of these types
   which prevents moving data out of the object (easily).

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix v0.55.1 ([`4642c0c`](https://github.com/Byron/gitoxide/commit/4642c0c78f45b1956837bc874f6757fc302bee4a))
    - Add `take_data()` to all primitive object types. ([`5732303`](https://github.com/Byron/gitoxide/commit/5732303180d26374016b70bdd7fa0278dd84cff3))
    - Add `detach()` and `detached()` too all object types. ([`88f2e6c`](https://github.com/Byron/gitoxide/commit/88f2e6c4c540b9c8032e6eee9c5da65a9bcfeef8))
</details>

## 0.55.0 (2023-10-12)

<csr-id-f478a3722f0be35c109ea60b79cd4ac6e607480b/>

This release contains a complete rewrite of the internal url parsing logic, the public interface stays mostly the same however. Gitoxide will now be
more correct, interpreting more urls the same way Git does. Improvements include the added support for ssh aliases (`github:byron/gitoxide` has previously
been parsed as local path), adjustments around the interpretation of colons in file names (previously we disallowed colons that were not followed up
with a slash character) and some smaller changes that bring the interpretation of file urls more in line with Git's implementation. Additionally, the
error types have been adjusted to print a more comprehensive message by default, making sure they stay helpful even when bubbled up through multiple abstraction
layers.

There are still many (edge) cases in Git's url parsing implementation which are not handled correctly by Gitoxide. If you notice any such deviation please
open a new issue to help us making Gitoxide even more correct.

### Other

 - <csr-id-f478a3722f0be35c109ea60b79cd4ac6e607480b/> inform about the absence of strict hash verification and strict object creation.
   Those are present in `git2` and enabled by default, and `gitoxde` definitely
   wants to do the same at some point.

### New Features

 - <csr-id-c79a7daa30fe90d14d8e3387ec48116b37faf460/> add `Repository::head_tree()` to more easily obtain the current tree.
 - <csr-id-787a9aa91c1abaa7572f5d19f8a2acbb7ecc0732/> Add `Repository::has_object()` as a high-level alternative.
   Previously, one would have to call `repo.objects.contains()`, which
   is fine, but this method is necessary for symmetry of the API
   and one shouldn't have to drop down a level to do this.
   
   This method also knows empty trees as special case.
 - <csr-id-3cec935e692eeb33ffcac98988e34a390f755bf3/> add `Object::try_into_blob()` and `into_blob()` and `Repository::empty_blob()`
   This way it's easier to assert that an object is actually a blob.
 - <csr-id-7d9ecdd1c230204468a965f703d5efd00fa7fb79/> add `Repository::index_or_empty()`.
   This is useful if a missing index should mean it's empty.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 16 calendar days.
 - 17 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-transport v0.37.1, gix-protocol v0.41.0, gix-revision v0.23.0, gix-refspec v0.19.0, gix-worktree v0.27.0, gix-status v0.2.0, gix-submodule v0.5.0, gix-worktree-state v0.4.0, gix v0.55.0 ([`14ddbd4`](https://github.com/Byron/gitoxide/commit/14ddbd4c15128b1d5631a2388a00e024842b7b83))
    - Release gix-hash v0.13.1, gix-features v0.36.0, gix-actor v0.28.0, gix-object v0.38.0, gix-glob v0.14.0, gix-attributes v0.20.0, gix-command v0.2.10, gix-filter v0.6.0, gix-fs v0.8.0, gix-commitgraph v0.22.0, gix-revwalk v0.9.0, gix-traverse v0.34.0, gix-worktree-stream v0.6.0, gix-archive v0.6.0, gix-tempfile v11.0.0, gix-lock v11.0.0, gix-ref v0.38.0, gix-config v0.31.0, gix-url v0.25.0, gix-credentials v0.21.0, gix-diff v0.37.0, gix-discover v0.26.0, gix-ignore v0.9.0, gix-index v0.26.0, gix-mailmap v0.20.0, gix-negotiate v0.9.0, gix-pack v0.44.0, gix-odb v0.54.0, gix-pathspec v0.4.0, gix-packetline v0.16.7, gix-transport v0.37.0, gix-protocol v0.41.0, gix-revision v0.23.0, gix-refspec v0.19.0, gix-worktree v0.27.0, gix-status v0.2.0, gix-submodule v0.5.0, gix-worktree-state v0.4.0, gix v0.55.0, safety bump 37 crates ([`68e5432`](https://github.com/Byron/gitoxide/commit/68e54326e527a55dd5b5079921fc251615833040))
    - Prepare changelogs prior to release ([`1347a54`](https://github.com/Byron/gitoxide/commit/1347a54f84599d8f0aa935d6e64b16c2298d25cf))
    - Merge branch 'improvements' ([`429e7b2`](https://github.com/Byron/gitoxide/commit/429e7b25f93c8a7947db19bafa74babf199a1aa6))
    - Inform about the absence of strict hash verification and strict object creation. ([`f478a37`](https://github.com/Byron/gitoxide/commit/f478a3722f0be35c109ea60b79cd4ac6e607480b))
    - Add `Repository::head_tree()` to more easily obtain the current tree. ([`c79a7da`](https://github.com/Byron/gitoxide/commit/c79a7daa30fe90d14d8e3387ec48116b37faf460))
    - Add `Repository::has_object()` as a high-level alternative. ([`787a9aa`](https://github.com/Byron/gitoxide/commit/787a9aa91c1abaa7572f5d19f8a2acbb7ecc0732))
    - Add `Object::try_into_blob()` and `into_blob()` and `Repository::empty_blob()` ([`3cec935`](https://github.com/Byron/gitoxide/commit/3cec935e692eeb33ffcac98988e34a390f755bf3))
    - Thanks clippy ([`345712d`](https://github.com/Byron/gitoxide/commit/345712dcdfddcccc630bbfef2ed4f461b21550d3))
    - Merge branch 'reset' ([`b842691`](https://github.com/Byron/gitoxide/commit/b8426919a491dc3a7df01ee3f258fc0d8a3a327c))
    - Trust Ctime again ([`f929d42`](https://github.com/Byron/gitoxide/commit/f929d420cb768f2df1d7886564ca03b3c3254a82))
    - Add `Repository::index_or_empty()`. ([`7d9ecdd`](https://github.com/Byron/gitoxide/commit/7d9ecdd1c230204468a965f703d5efd00fa7fb79))
    - Adapt to changes in `gix-status` ([`54fb7c2`](https://github.com/Byron/gitoxide/commit/54fb7c24a97cb2339a67ad269344ce65166a545d))
    - Merge branch 'gix-url-parse-rewrite' ([`a12e4a8`](https://github.com/Byron/gitoxide/commit/a12e4a88d5f5636cd694c72ce45a8b75aa754d28))
    - Update changelogs ([`4349353`](https://github.com/Byron/gitoxide/commit/43493531bbf3049bee3d7b14b7a6dbe874e37ebc))
</details>

## 0.54.1 (2023-09-25)

### Bug Fixes

 - <csr-id-300a83821358f2a43649515606ebb84741e82780/> local refs created during fetching will now always be valid.
   Previously it could create symbolic refs that were effectively unborn, i.e.
   point to a reference which doesn't exist.
   
   Now these will always point to the peeled object instead.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix v0.54.1 ([`f603fd7`](https://github.com/Byron/gitoxide/commit/f603fd7a68206a6989a9f959216eba6cca0a6733))
    - Local refs created during fetching will now always be valid. ([`300a838`](https://github.com/Byron/gitoxide/commit/300a83821358f2a43649515606ebb84741e82780))
</details>

## 0.54.0 (2023-09-24)

<csr-id-e022096aa495f55a05f83860243f49552be501f7/>
<csr-id-79e47a512507c7fd7acbdff624a5249e24505e0d/>

### New Features

 - <csr-id-f9d14d86a6578cf0f9a0c4a2256ad227b9264340/> Add `PathspecDetached` as pathspec that can more easily be used across threads.
 - <csr-id-f066f9889b57a4ffaebc0ed1442d77999498db42/> `PathSpec` implements `gix_status::PathSpec` to allow it to be used there.
   The reason we need a trait and can't do with simply a function is that multiple calls
   are needed to test for inclusion *and* allow the common-prefix optimization.
 - <csr-id-a8333f1137df51d237f6debf056ac075b0a2cd94/> add `Repository::stat_options()` to learn how an index would compare filesystem stats.
 - <csr-id-2734e84b74b761bff27fc1eb27f57d9d839c9240/> add `parallel` feature toggle
   Make certain data structure threadsafe (or `Sync`) to facilitate multithreading.
   Further, many algorithms will now use multiple threads by default.
   If unset, most of `gix` can only be used in a single thread
   as data structures won't be `Send` anymore.

### Bug Fixes

 - <csr-id-e22893c1c95a76d9a5f3b2f2a4e2a30f815ee7e5/> do not trust ctime by default.
   On MacOS it seems to be off by two seconds right from the source, which
   seems to be an issue `stat` isn't having.
 - <csr-id-334281c8771790df7a022daa4a700c96b99acbc0/> ignore empty `core.askpass` settings
   This is the same as what `git` does, it's explicit per value, which
   means that other paths might be flagged as empty automatically.

### Other

 - <csr-id-e022096aa495f55a05f83860243f49552be501f7/> add note about the trust-model.
   It should explain why `gix` is happy to open repositories that won't
   be handled by `git` unless overrides are set.

### Test

 - <csr-id-79e47a512507c7fd7acbdff624a5249e24505e0d/> add assertion to assure `ThreadSafeRepository` is sync.
   If it doesn't appear to be sync, be sure to use the `max-performance-safe` feature.

### Bug Fixes (BREAKING)

 - <csr-id-ee9276f2a7789c20d88d40624ad648e44b604a27/> `PrepareCheckout::main_worktree()` now takes `Progress` as geric argument.
   This makes it more flexible and convenient, but is technically a breaking change.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 31 commits contributed to the release over the course of 15 calendar days.
 - 15 days passed between releases.
 - 9 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-features v0.35.0, gix-actor v0.27.0, gix-object v0.37.0, gix-glob v0.13.0, gix-attributes v0.19.0, gix-filter v0.5.0, gix-fs v0.7.0, gix-commitgraph v0.21.0, gix-revwalk v0.8.0, gix-traverse v0.33.0, gix-worktree-stream v0.5.0, gix-archive v0.5.0, gix-tempfile v10.0.0, gix-lock v10.0.0, gix-ref v0.37.0, gix-config v0.30.0, gix-url v0.24.0, gix-credentials v0.20.0, gix-diff v0.36.0, gix-discover v0.25.0, gix-ignore v0.8.0, gix-index v0.25.0, gix-mailmap v0.19.0, gix-negotiate v0.8.0, gix-pack v0.43.0, gix-odb v0.53.0, gix-pathspec v0.3.0, gix-transport v0.37.0, gix-protocol v0.40.0, gix-revision v0.22.0, gix-refspec v0.18.0, gix-status v0.1.0, gix-submodule v0.4.0, gix-worktree v0.26.0, gix-worktree-state v0.3.0, gix v0.54.0, gitoxide-core v0.32.0, gitoxide v0.30.0, safety bump 37 crates ([`7891fb1`](https://github.com/Byron/gitoxide/commit/7891fb17348ec2f4c997665f9a25be36e2713da4))
    - Prepare changelogs prior to release ([`8a60d5b`](https://github.com/Byron/gitoxide/commit/8a60d5b80877c213c3b646d3061e8a33e0e433ec))
    - Merge branch 'reset' ([`54a8495`](https://github.com/Byron/gitoxide/commit/54a849545140f7f1c0c7564c418071c0a76a34e7))
    - Add `PathspecDetached` as pathspec that can more easily be used across threads. ([`f9d14d8`](https://github.com/Byron/gitoxide/commit/f9d14d86a6578cf0f9a0c4a2256ad227b9264340))
    - Do not trust ctime by default. ([`e22893c`](https://github.com/Byron/gitoxide/commit/e22893c1c95a76d9a5f3b2f2a4e2a30f815ee7e5))
    - `PathSpec` implements `gix_status::PathSpec` to allow it to be used there. ([`f066f98`](https://github.com/Byron/gitoxide/commit/f066f9889b57a4ffaebc0ed1442d77999498db42))
    - Add `Repository::stat_options()` to learn how an index would compare filesystem stats. ([`a8333f1`](https://github.com/Byron/gitoxide/commit/a8333f1137df51d237f6debf056ac075b0a2cd94))
    - Fix compile time warning ([`4ce7f7c`](https://github.com/Byron/gitoxide/commit/4ce7f7c2c8bb66f0c093bf6e4d20f5568ca04f6a))
    - Merge branch 'parallel-feature' ([`c270f78`](https://github.com/Byron/gitoxide/commit/c270f7883e1ea8156d521b12d161a47b2144425c))
    - Add `parallel` feature toggle ([`2734e84`](https://github.com/Byron/gitoxide/commit/2734e84b74b761bff27fc1eb27f57d9d839c9240))
    - Add assertion to assure `ThreadSafeRepository` is sync. ([`79e47a5`](https://github.com/Byron/gitoxide/commit/79e47a512507c7fd7acbdff624a5249e24505e0d))
    - Merge pull request #1015 from NobodyXu/optimize/prepare-checkout ([`14312b6`](https://github.com/Byron/gitoxide/commit/14312b6c21d6382f7e536db7a1d8b519b97b8300))
    - Merge branch 'path-config' ([`9c528dc`](https://github.com/Byron/gitoxide/commit/9c528dc8282c8b2f3a023e523dccdd0f7a711e61))
    - Merge pull request #1012 from NobodyXu/optimization/try-into-de-momo ([`afb1960`](https://github.com/Byron/gitoxide/commit/afb1960e3e13bc9fc7cc5f3a3a244945f00966ad))
    - Ignore empty `core.askpass` settings ([`334281c`](https://github.com/Byron/gitoxide/commit/334281c8771790df7a022daa4a700c96b99acbc0))
    - Merge branch 'optimize/progress-use' ([`1f2ffb6`](https://github.com/Byron/gitoxide/commit/1f2ffb6d86ef073caf43a2f7a77fe712a1aa495e))
    - `PrepareCheckout::main_worktree()` now takes `Progress` as geric argument. ([`ee9276f`](https://github.com/Byron/gitoxide/commit/ee9276f2a7789c20d88d40624ad648e44b604a27))
    - Add note about the trust-model. ([`e022096`](https://github.com/Byron/gitoxide/commit/e022096aa495f55a05f83860243f49552be501f7))
    - Optimize `clone::PrepareCheckout::main_worktree`` ([`938f518`](https://github.com/Byron/gitoxide/commit/938f5187f0ff51561971ca463584ec0db93f3455))
    - Fix `maybe_async` ([`c80e809`](https://github.com/Byron/gitoxide/commit/c80e809c2655d15d7f22170782dacd64fe2e01bd))
    - Rm unused clippy lint ([`d82f84b`](https://github.com/Byron/gitoxide/commit/d82f84b23a3def8e237e2b2511874c6045032c04))
    - Fixed error by also using trait object in `remote::fetch::Prepare::receive` ([`44faa01`](https://github.com/Byron/gitoxide/commit/44faa01cf0612df5685922710d4a0adf6715ef77))
    - Revert changes to binary files ([`3eb8653`](https://github.com/Byron/gitoxide/commit/3eb8653b78f2a0ca654fbebb185f3d6416d779d5))
    - Rm binary files ([`6a33594`](https://github.com/Byron/gitoxide/commit/6a335940332b8d5069cb0c310d2d8e43fbeee01e))
    - Use trait object for `progress` in `PrepareFetch::fetch_only` ([`70989b3`](https://github.com/Byron/gitoxide/commit/70989b3965077ae00ec6cf344f31627a804a8225))
    - Fix clippy warnings ([`d5aa2ba`](https://github.com/Byron/gitoxide/commit/d5aa2ba030f57d65021d84efa99c0abc9d61f575))
    - Optimize `Repository::write_blob_stream`: Avoid dup codegen ([`ca8a373`](https://github.com/Byron/gitoxide/commit/ca8a373b1a3de44d2bef3e4908d6f5269b6cdd1f))
    - Apply `gix_macros::momo` to `Repository::write_blob` ([`bae928d`](https://github.com/Byron/gitoxide/commit/bae928d9668bbb4ba0dadb4605d77fc773362e3f))
    - Optimize `Repository::write_object`: Avoid dup momo ([`32f1c7d`](https://github.com/Byron/gitoxide/commit/32f1c7d2bc2e91cb346c8b379dce41293f88b222))
    - Rm unnecessary lifetime annotation in `Repository::commit_as_inner` ([`cf70a2e`](https://github.com/Byron/gitoxide/commit/cf70a2e0f08dd323c0713b4e23b21f54668a99a2))
    - Optimize `gix`: de-momo `impl TryInto` by hand ([`b19c140`](https://github.com/Byron/gitoxide/commit/b19c140ce3a6e5d9ddf65684361223a2f9fa7e73))
</details>

## 0.53.1 (2023-09-08)

### Bug Fixes

 - <csr-id-902639b9b72ead72b5355e0a1a4da5afd7fed46d/> `interrupt` feature only gates signal-handling, but leaves the `interrupt` module alone.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix v0.53.1 ([`1b1fc25`](https://github.com/Byron/gitoxide/commit/1b1fc257d5748c7c41e899bf2d1447ffd9f22d19))
    - `interrupt` feature only gates signal-handling, but leaves the `interrupt` module alone. ([`902639b`](https://github.com/Byron/gitoxide/commit/902639b9b72ead72b5355e0a1a4da5afd7fed46d))
</details>

## 0.53.0 (2023-09-08)

<csr-id-ed327f6163f54756e58c20f86a563a97efb256ca/>

This release adds feature toggles which help to reduce compile time. Please [see the library](https://docs.rs/gix/0.53.0/gix/) documentation for all the details.

### New Features

 - <csr-id-2b8d09f785f471aa12fc6793f0ea40c1f8d9ea4a/> remove `log` dependency in favor of `gix-trace`
 - <csr-id-36d34bd7e8cd944c009cb7acbe39c1dc445b4adc/> add `interrupt` feature to reduce dependencies
 - <csr-id-721c37722ca8b12a5f9c060061040c79f9da6aa9/> Allow index access to be toggled with the `index` feature.
 - <csr-id-92dd18154b526b4c5132d770960a36ccf739dec8/> add `excludes` feature to make exclude-checks possible.
 - <csr-id-c4ffde013a62a38a6f63c4a160bc3cdb6aafd65a/> add `mailmap` feature
 - <csr-id-c42064d8ca382513c944f3df5c08e4ff66d5f804/> add `revision` component behind a feature toggle.
 - <csr-id-147528ff647dc74473ef8dd4ceac6fedebc0b15c/> `gix` without connection support includes less code
 - <csr-id-fea044e5d09282a5772c8fe9a534d9ebf7f11bbc/> allow disabling the `blob-diff` capability
   This also removes all diff capabilities.
 - <csr-id-c5ec244979b7e6baf9a8237e4f12cb87809131ae/> improve feature documentation.
   This should make optimizing compile time and performance easier, while
   assuring these options aren't pre-determined by library providers.
 - <csr-id-c79991cde8216271ab854b7574e7d97efd79d07c/> `Clone` for `ThreadSafeRepository`
   It is `Sync` and can easily be passed by reference, but sometimes it's nice
   to be cloning it as well.
 - <csr-id-d22b7fb1304cce3b2aabac42cc58fe7c5911f276/> provide `Repository::find_fetch_remote()` to obtain a remote just like git would.

### Bug Fixes

 - <csr-id-a957478e0f623803bc6358d08b9ffaa2305e24d4/> put `gix-credentials` and `gix-prompt` behind the 'credentials' feature toggle.
   They are also available when using https transports.
 - <csr-id-4971a4837ff5ac6654aa75214bdd2243d4d864a5/> handle submodules whose entry in the index is a file.

### Chore (BREAKING)

 - <csr-id-ed327f6163f54756e58c20f86a563a97efb256ca/> update to the latest `prodash`
   It makes proper usage of `Progress` types easier and allows them to be used
   as `dyn` traits as well.

### New Features (BREAKING)

 - <csr-id-58b0e6f860d4d3b5548c18d3eae97141bc6dc377/> Use `stack` abstraction in `Repository::excludes()`.
   This makes it easier to use.
 - <csr-id-24dd870919ba444aa8099c63a78ea120d47ec28e/> use `prodash::Count` to indicate that nothing more than counting is performed, in place of `prodash::Progress`
 - <csr-id-54291fdfc62c7d8a31bc5564713c23eab3865dc5/> Provide a wrapper for `gix_worktree::Stack` for simpler attribute queries.

### Bug Fixes (BREAKING)

 - <csr-id-741b41e6e6c6f283c1632a7de0da44a5e7842817/> remove `regex` feature in favor of `revparse-regex`.
   `revparse-regex` is only used when parsing revspecs that use a special syntax.
   This feature is also enabled by default.
 - <csr-id-072ee32f693a31161cd6a843da6582d13efbb20b/> use `dyn` trait where possible.
   This reduces compile time due to avoiding duplication.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 57 commits contributed to the release over the course of 17 calendar days.
 - 17 days passed between releases.
 - 19 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-transport v0.36.0, gix-protocol v0.39.0, gix-revision v0.21.0, gix-refspec v0.17.0, gix-submodule v0.3.0, gix-worktree v0.25.0, gix-worktree-state v0.2.0, gix v0.53.0 ([`1ff3064`](https://github.com/Byron/gitoxide/commit/1ff30641b8724efd6699d8bef5c71d28454e98b9))
    - Release gix-date v0.8.0, gix-hash v0.13.0, gix-features v0.34.0, gix-actor v0.26.0, gix-object v0.36.0, gix-path v0.10.0, gix-glob v0.12.0, gix-attributes v0.18.0, gix-packetline-blocking v0.16.6, gix-filter v0.4.0, gix-fs v0.6.0, gix-commitgraph v0.20.0, gix-hashtable v0.4.0, gix-revwalk v0.7.0, gix-traverse v0.32.0, gix-worktree-stream v0.4.0, gix-archive v0.4.0, gix-config-value v0.14.0, gix-tempfile v9.0.0, gix-lock v9.0.0, gix-ref v0.36.0, gix-sec v0.10.0, gix-config v0.29.0, gix-prompt v0.7.0, gix-url v0.23.0, gix-credentials v0.19.0, gix-diff v0.35.0, gix-discover v0.24.0, gix-ignore v0.7.0, gix-index v0.24.0, gix-macros v0.1.0, gix-mailmap v0.18.0, gix-negotiate v0.7.0, gix-pack v0.42.0, gix-odb v0.52.0, gix-pathspec v0.2.0, gix-packetline v0.16.6, gix-transport v0.36.0, gix-protocol v0.39.0, gix-revision v0.21.0, gix-refspec v0.17.0, gix-submodule v0.3.0, gix-worktree v0.25.0, gix-worktree-state v0.2.0, gix v0.53.0, safety bump 39 crates ([`8bd0456`](https://github.com/Byron/gitoxide/commit/8bd045676bb2cdc02624ab93e73ff8518064ca38))
    - Prepare changelogs for release ([`375db06`](https://github.com/Byron/gitoxide/commit/375db06a8442378c3f7a922fae38e2a6694d9d04))
    - Merge branch 'optimizations' ([`6135a5e`](https://github.com/Byron/gitoxide/commit/6135a5ea8709646f01da62939a59dd3a9750e007))
    - Adapt to changes in `gix` ([`805b8aa`](https://github.com/Byron/gitoxide/commit/805b8aa74b064b7aa08d87094a994bb8c7aae6ed))
    - Remove `log` dependency in favor of `gix-trace` ([`2b8d09f`](https://github.com/Byron/gitoxide/commit/2b8d09f785f471aa12fc6793f0ea40c1f8d9ea4a))
    - Add `interrupt` feature to reduce dependencies ([`36d34bd`](https://github.com/Byron/gitoxide/commit/36d34bd7e8cd944c009cb7acbe39c1dc445b4adc))
    - Allow index access to be toggled with the `index` feature. ([`721c377`](https://github.com/Byron/gitoxide/commit/721c37722ca8b12a5f9c060061040c79f9da6aa9))
    - Put `gix-credentials` and `gix-prompt` behind the 'credentials' feature toggle. ([`a957478`](https://github.com/Byron/gitoxide/commit/a957478e0f623803bc6358d08b9ffaa2305e24d4))
    - Add `excludes` feature to make exclude-checks possible. ([`92dd181`](https://github.com/Byron/gitoxide/commit/92dd18154b526b4c5132d770960a36ccf739dec8))
    - Use `stack` abstraction in `Repository::excludes()`. ([`58b0e6f`](https://github.com/Byron/gitoxide/commit/58b0e6f860d4d3b5548c18d3eae97141bc6dc377))
    - Add `mailmap` feature ([`c4ffde0`](https://github.com/Byron/gitoxide/commit/c4ffde013a62a38a6f63c4a160bc3cdb6aafd65a))
    - Simplify test-suite ([`799a515`](https://github.com/Byron/gitoxide/commit/799a5152c7ca444a8240a022e049c14b0b61d22d))
    - Remove `regex` feature in favor of `revparse-regex`. ([`741b41e`](https://github.com/Byron/gitoxide/commit/741b41e6e6c6f283c1632a7de0da44a5e7842817))
    - Add `revision` component behind a feature toggle. ([`c42064d`](https://github.com/Byron/gitoxide/commit/c42064d8ca382513c944f3df5c08e4ff66d5f804))
    - `gix` without connection support includes less code ([`147528f`](https://github.com/Byron/gitoxide/commit/147528ff647dc74473ef8dd4ceac6fedebc0b15c))
    - Allow disabling the `blob-diff` capability ([`fea044e`](https://github.com/Byron/gitoxide/commit/fea044e5d09282a5772c8fe9a534d9ebf7f11bbc))
    - Improve feature documentation. ([`c5ec244`](https://github.com/Byron/gitoxide/commit/c5ec244979b7e6baf9a8237e4f12cb87809131ae))
    - Merge branch 'feat/gix-momo' ([`a1ed6a1`](https://github.com/Byron/gitoxide/commit/a1ed6a1aacae02a167b7ec44e1a47411a2194ff7))
    - Handle submodules whose entry in the index is a file. ([`4971a48`](https://github.com/Byron/gitoxide/commit/4971a4837ff5ac6654aa75214bdd2243d4d864a5))
    - Merge branch `dyn`ification ([`f658fcc`](https://github.com/Byron/gitoxide/commit/f658fcc52dc2200ae34ca53dc10be97fb9012057))
    - Use `dyn` trait where possible. ([`072ee32`](https://github.com/Byron/gitoxide/commit/072ee32f693a31161cd6a843da6582d13efbb20b))
    - Use `prodash::Count` to indicate that nothing more than counting is performed, in place of `prodash::Progress` ([`24dd870`](https://github.com/Byron/gitoxide/commit/24dd870919ba444aa8099c63a78ea120d47ec28e))
    - Update to the latest `prodash` ([`ed327f6`](https://github.com/Byron/gitoxide/commit/ed327f6163f54756e58c20f86a563a97efb256ca))
    - Merge branch 'improvements' ([`8a7c2af`](https://github.com/Byron/gitoxide/commit/8a7c2af0d302d5acc87ef2d432bd6870017af63e))
    - Provide a wrapper for `gix_worktree::Stack` for simpler attribute queries. ([`54291fd`](https://github.com/Byron/gitoxide/commit/54291fdfc62c7d8a31bc5564713c23eab3865dc5))
    - `Clone` for `ThreadSafeRepository` ([`c79991c`](https://github.com/Byron/gitoxide/commit/c79991cde8216271ab854b7574e7d97efd79d07c))
    - Merge branch 'adjustments-for-cargo' ([`b7560a2`](https://github.com/Byron/gitoxide/commit/b7560a2445b62f888bf5aa2ba4c5a47ae037cb23))
    - Adapt to changes in `gix-submodule` ([`f8471b1`](https://github.com/Byron/gitoxide/commit/f8471b11e0d65fdb2617b927a8a207659a161439))
    - Release gix-index v0.23.1 ([`11b9c71`](https://github.com/Byron/gitoxide/commit/11b9c71311df978ebb20cca0d765cf249c8eedcf))
    - Release gix-date v0.7.4, gix-index v0.23.0, safety bump 5 crates ([`3be2b1c`](https://github.com/Byron/gitoxide/commit/3be2b1ccfe30eeae45711c64b88efc522a2b51b7))
    - Apply `momo` to fn `gix::Remote::save_as_to` ([`875c287`](https://github.com/Byron/gitoxide/commit/875c28757e4a91cf314ec59dd1a0dde779698e53))
    - Apply `momo` to fn `gix::revision::Spec::from_bstr` ([`1d90301`](https://github.com/Byron/gitoxide/commit/1d9030112b54699db9bd8d1125116e46c4a6f71e))
    - Apply `momo` to mod `config::snapshot::access` ([`25912fe`](https://github.com/Byron/gitoxide/commit/25912fe1e5d60765458a2e90c0fa487657b0831c))
    - Apply `momo` to mod `gix::create::into` ([`cd3c289`](https://github.com/Byron/gitoxide/commit/cd3c2893b095d38d36f0c969549f0aaadfcef2ee))
    - Rm unnecessary `#[allow(unused_mut)]` put on `momo`ed functions ([`89ae797`](https://github.com/Byron/gitoxide/commit/89ae797c7f1f4d26c48ed54c5d8b31f39599f063))
    - Remove unnecessary change in `repository/config/transport.rs` ([`86b8e50`](https://github.com/Byron/gitoxide/commit/86b8e50fafa7e5d57989acb9e8b848fd95d271a9))
    - Remove unnecessary `#[allow(clippy::needless_lifetimes)]` ([`e1b9d51`](https://github.com/Byron/gitoxide/commit/e1b9d51acd137cdea7680584451702e52aab775f))
    - Dramatically simplify `gix_macros::momo` ([`c72eaa0`](https://github.com/Byron/gitoxide/commit/c72eaa05697a3e34adaa3ee90584dce4b5c00120))
    - Manually de-`momo` `Repository::try_find_remote_{without_url_rewrite}` ([`e760225`](https://github.com/Byron/gitoxide/commit/e7602257662cd9ddb4cfe41ef26cdf28cc007be7))
    - Merge branch 'fixes' ([`4bfd1cc`](https://github.com/Byron/gitoxide/commit/4bfd1cc8f7922a8c4de6b9d078d54b93e78f51ff))
    - Thanks clippy ([`0d6d4ec`](https://github.com/Byron/gitoxide/commit/0d6d4ec8030d2e8f4c7a9d6f421d54776c4b67fb))
    - Adapt to changes in `gix-index` and pass skip-hash through for performance.. ([`713cd59`](https://github.com/Byron/gitoxide/commit/713cd59f0b1eff6397b80f1e1fceec278532fd59))
    - Use new `gix` method to obtain the fetch remote (instead of implementing it by hand) ([`e2c0912`](https://github.com/Byron/gitoxide/commit/e2c0912cfede044431c17ae81ddae02746650ae4))
    - Provide `Repository::find_fetch_remote()` to obtain a remote just like git would. ([`d22b7fb`](https://github.com/Byron/gitoxide/commit/d22b7fb1304cce3b2aabac42cc58fe7c5911f276))
    - Fix clippy lints in `gix/src/repository/remote.rs` ([`ff210d8`](https://github.com/Byron/gitoxide/commit/ff210d82573cebfdf4edbfb39beaef08979c058f))
    - Apply `momo` to mod `gix::repository` ([`5a50537`](https://github.com/Byron/gitoxide/commit/5a505377199730354c2a6b6b7b060184558bb9c4))
    - Apply `momo` to mod `remote::connection::fetch::receive_pack` ([`ea5c2db`](https://github.com/Byron/gitoxide/commit/ea5c2dbabe9d3c1eb1ab5f15a578ec9f9c36a5d8))
    - Apply `momo` to `gix::reference` ([`3c205ab`](https://github.com/Byron/gitoxide/commit/3c205abbdc0a80090b9f0f5681ce0949497e770f))
    - Apply `momo` to `gix::pathspec` ([`767ec2d`](https://github.com/Byron/gitoxide/commit/767ec2dcfd1fadaca93390770604494d03f88ab3))
    - Apply `momo` to mod `gix::open::repository` ([`3ce0144`](https://github.com/Byron/gitoxide/commit/3ce014499a86e4e8bb57ffe7caa540792c1c0a47))
    - Apply `momo` to `gix::object::tree` ([`d835526`](https://github.com/Byron/gitoxide/commit/d8355267fd64dbcf22a01a11cb29d93e75f0fb4c))
    - Apply `momo` to mod `gix::init` ([`46a9dfe`](https://github.com/Byron/gitoxide/commit/46a9dfe12dedc1cbf997ea408d1f1d2c5d673ba5))
    - Apply `momo` to mod `gix::discover` ([`58fbb08`](https://github.com/Byron/gitoxide/commit/58fbb08461064d96dd9816e2fb6911cf76b6badc))
    - Thanks clippy ([`5044c3b`](https://github.com/Byron/gitoxide/commit/5044c3b87456cf58ebfbbd00f23c9ba671cb290c))
    - Imrpove git2 mapping by using aliases. ([`6194ebe`](https://github.com/Byron/gitoxide/commit/6194ebe1fb10dedc22f1937b91df858dffc50db3))
    - Merge branch 'gix-submodule' ([`363ee77`](https://github.com/Byron/gitoxide/commit/363ee77400805f473c9ad66eadad9214e7ab66f4))
</details>

## 0.52.0 (2023-08-22)

### New Features

 - <csr-id-28249bda58b56af340c7d6af883496c3bb2d6804/> add `Worktree::pathspec()` to easily get worktree-scoped pathspec searches.
 - <csr-id-59bb3c4109c4e7f1977cea602293be85b7d14a8a/> add `Submodule` type to represent a declared submodule.
 - <csr-id-a7d0e441b2326520ae467e83e045302792e6bcd0/> `pathspec_search([specs])` to instantiate a search using pathspecs.
   It can be used to for filtering input paths.
   This type also makes filtering index entries easy.
 - <csr-id-77da01456118227a59b654f32c15eeb1e5e19cd9/> make `gix-pathspec` crate available
 - <csr-id-5c13459721eabb9d0746899a2498a104ddbdae59/> add `Commit::signature()` to yield the PGP sigature of a commit, if present.

### Bug Fixes

 - <csr-id-c51c8daee1ab54130ae3ed83ce67d08f01c4881a/> fix incorrect s/git-config/gix-config/
   3a861c8f049f6502d3bcbdac752659aa1aeda46a just blindly replaced any
   occurence of "git-config" or "git_config" with "gix-config"/"gix_config".
   
   There is no such thing as a gix-config file.
   gix-config is a git-config file parser.

### New Features (BREAKING)

 - <csr-id-b1e55d6f9e4c0d78f0cdeb7b85e09c2eb7032ced/> `Repository::prefix()` turns `Option<Result` into `Result<Option`.
   This makes it easier for the caller as they won't have to call transpose anymore.
 - <csr-id-46225c2bc399e6db5a56b522c978e0d1fac163df/> improve `interrupt::init_handler()` to be usable from multiple threads
   Previously it was geared towards applications which would initialize handlers
   only from the main thread.
   
   Now the API supports multiple threads.

### Bug Fixes (BREAKING)

 - <csr-id-430e58cd1efef0044fc36b23de019156a21f947c/> `Repository::prefix()` is now side-effect free and won't error if CWD is outside of working tree dir.
   This makes it more usable, especially in contexts where many repositories are held, possibly with
   changing current working dirs.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 32 commits contributed to the release over the course of 18 calendar days.
 - 19 days passed between releases.
 - 9 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-url v0.22.0, gix-credentials v0.18.0, gix-diff v0.34.0, gix-discover v0.23.0, gix-ignore v0.6.0, gix-bitmap v0.2.7, gix-index v0.22.0, gix-mailmap v0.17.0, gix-negotiate v0.6.0, gix-pack v0.41.0, gix-odb v0.51.0, gix-pathspec v0.1.0, gix-packetline v0.16.5, gix-transport v0.35.0, gix-protocol v0.38.0, gix-revision v0.20.0, gix-refspec v0.16.0, gix-submodule v0.2.0, gix-worktree v0.24.0, gix-worktree-state v0.1.0, gix v0.52.0, gitoxide-core v0.31.0, gitoxide v0.29.0 ([`6c62e74`](https://github.com/Byron/gitoxide/commit/6c62e748240ac0980fc23fdf30f8477dea8b9bc3))
    - Release gix-date v0.7.3, gix-hash v0.12.0, gix-features v0.33.0, gix-actor v0.25.0, gix-object v0.35.0, gix-path v0.9.0, gix-glob v0.11.0, gix-quote v0.4.7, gix-attributes v0.17.0, gix-command v0.2.9, gix-packetline-blocking v0.16.5, gix-filter v0.3.0, gix-fs v0.5.0, gix-commitgraph v0.19.0, gix-hashtable v0.3.0, gix-revwalk v0.6.0, gix-traverse v0.31.0, gix-worktree-stream v0.3.0, gix-archive v0.3.0, gix-config-value v0.13.0, gix-tempfile v8.0.0, gix-lock v8.0.0, gix-ref v0.35.0, gix-sec v0.9.0, gix-config v0.28.0, gix-prompt v0.6.0, gix-url v0.22.0, gix-credentials v0.18.0, gix-diff v0.34.0, gix-discover v0.23.0, gix-ignore v0.6.0, gix-bitmap v0.2.7, gix-index v0.22.0, gix-mailmap v0.17.0, gix-negotiate v0.6.0, gix-pack v0.41.0, gix-odb v0.51.0, gix-pathspec v0.1.0, gix-packetline v0.16.5, gix-transport v0.35.0, gix-protocol v0.38.0, gix-revision v0.20.0, gix-refspec v0.16.0, gix-submodule v0.2.0, gix-worktree v0.24.0, gix-worktree-state v0.1.0, gix v0.52.0, gitoxide-core v0.31.0, gitoxide v0.29.0, safety bump 41 crates ([`30b2761`](https://github.com/Byron/gitoxide/commit/30b27615047692d3ced1b2d9c2ac15a80f79fbee))
    - Update changelogs prior to release ([`f23ea88`](https://github.com/Byron/gitoxide/commit/f23ea8828f2d9ba7559973daca388c9591bcc5fc))
    - Merge branch 'gix-submodule' ([`8f3f358`](https://github.com/Byron/gitoxide/commit/8f3f358800f1fe77d7ba7ebd396a90b692d3c0c1))
    - Make sure that submodule hashes aren't attached as the parent repo is the wrong one here. ([`c96f26b`](https://github.com/Byron/gitoxide/commit/c96f26b5c13581812753638a24d261c3f75dddcf))
    - Properly isolate environment variable based tests into their own binary ([`c35ddab`](https://github.com/Byron/gitoxide/commit/c35ddab41ff6f18ad9cd11df44cfffee91563433))
    - Just fmt ([`0d258f4`](https://github.com/Byron/gitoxide/commit/0d258f40afcd848509e2b0c7c264e9f346ed1726))
    - Merge branch 'submodule-in-gix' ([`36f7b78`](https://github.com/Byron/gitoxide/commit/36f7b783c67b8a087076a130f5ee9b90b23bc3cc))
    - Adapt to changes in `gix` ([`9fe3052`](https://github.com/Byron/gitoxide/commit/9fe305291cf8ba908eaf38235f54abfa1d0ddeed))
    - Add `Worktree::pathspec()` to easily get worktree-scoped pathspec searches. ([`28249bd`](https://github.com/Byron/gitoxide/commit/28249bda58b56af340c7d6af883496c3bb2d6804))
    - Add `Submodule` type to represent a declared submodule. ([`59bb3c4`](https://github.com/Byron/gitoxide/commit/59bb3c4109c4e7f1977cea602293be85b7d14a8a))
    - Merge branch 'worktree-organization' ([`8d0d8e0`](https://github.com/Byron/gitoxide/commit/8d0d8e005d7f11924a6717954d892aae5cec45e7))
    - Adapt to changes in `gix-worktree` ([`e5717e1`](https://github.com/Byron/gitoxide/commit/e5717e1d12c49285d31a90b03b7f8e9cbc6c1108))
    - Merge pull request #988 from not-my-profile/fix-gix-config-sub ([`7735047`](https://github.com/Byron/gitoxide/commit/7735047198bd7cc5059ca338f5c2147dd273f711))
    - Fix incorrect s/git-config/gix-config/ ([`c51c8da`](https://github.com/Byron/gitoxide/commit/c51c8daee1ab54130ae3ed83ce67d08f01c4881a))
    - Merge branch 'submodule-active' ([`a3afaa4`](https://github.com/Byron/gitoxide/commit/a3afaa42741616a0f1abeef9b54557e7c2b800cb))
    - Adapt to changes in `gix-url` ([`f8fc662`](https://github.com/Byron/gitoxide/commit/f8fc6625d8c22f43e7ab5f1cdf1e8eb9a6ea34de))
    - `pathspec_search([specs])` to instantiate a search using pathspecs. ([`a7d0e44`](https://github.com/Byron/gitoxide/commit/a7d0e441b2326520ae467e83e045302792e6bcd0))
    - `Repository::prefix()` is now side-effect free and won't error if CWD is outside of working tree dir. ([`430e58c`](https://github.com/Byron/gitoxide/commit/430e58cd1efef0044fc36b23de019156a21f947c))
    - Merge branch 'pathspec-matching' ([`9f4dfe0`](https://github.com/Byron/gitoxide/commit/9f4dfe0f0b948280692916b596923959ea2fd9da))
    - `Repository::prefix()` turns `Option<Result` into `Result<Option`. ([`b1e55d6`](https://github.com/Byron/gitoxide/commit/b1e55d6f9e4c0d78f0cdeb7b85e09c2eb7032ced))
    - Make `gix-pathspec` crate available ([`77da014`](https://github.com/Byron/gitoxide/commit/77da01456118227a59b654f32c15eeb1e5e19cd9))
    - Merge branch 'handlers-mt' ([`f584d76`](https://github.com/Byron/gitoxide/commit/f584d7698d93836daef2000fd369034de46037f0))
    - Improve `interrupt::init_handler()` to be usable from multiple threads ([`46225c2`](https://github.com/Byron/gitoxide/commit/46225c2bc399e6db5a56b522c978e0d1fac163df))
    - Merge branch 'extract-signatures' ([`b37affe`](https://github.com/Byron/gitoxide/commit/b37affefecfb30a94431cd21dae6659004ca6244))
    - Add `Commit::signature()` to yield the PGP sigature of a commit, if present. ([`5c13459`](https://github.com/Byron/gitoxide/commit/5c13459721eabb9d0746899a2498a104ddbdae59))
    - Release gix-glob v0.10.2, gix-date v0.7.2, gix-validate v0.8.0, gix-object v0.34.0, gix-ref v0.34.0, gix-config v0.27.0, gix-commitgraph v0.18.2, gix-revwalk v0.5.0, gix-revision v0.19.0, gix-refspec v0.15.0, gix-submodule v0.1.0, safety bump 18 crates ([`4604f83`](https://github.com/Byron/gitoxide/commit/4604f83ef238dc07c85aaeae097399b67f3cfd0c))
    - Merge branch 'submodules' ([`b629f8a`](https://github.com/Byron/gitoxide/commit/b629f8a774931d58c0a9b124fa75f85807c6c5d1))
    - More idiomatic use of `config.section_by_name()` ([`0a584ee`](https://github.com/Byron/gitoxide/commit/0a584eeb5c756ec4b0d54c4fd9ea3cc1497f4ba9))
    - Adjust to changes in `gix-validate` ([`a8bc0de`](https://github.com/Byron/gitoxide/commit/a8bc0de6d071be82364434b6e27afecc02f3be51))
    - Merge branch 'dev-on-linux' ([`6b4a303`](https://github.com/Byron/gitoxide/commit/6b4a30330fe49fc97daa73f55bf56580cc0597aa))
    - Fix various tests to run properly on linux ([`ef8ccd9`](https://github.com/Byron/gitoxide/commit/ef8ccd9d16143d37155d063747c69cade80f162d))
</details>

## 0.51.0 (2023-08-02)

This is mostly a bug-fix release with many improvements for fetching, along with more forgiving commit parsing.

### New Features

 - <csr-id-d9e551b44aa3e84109660328de7637d465d59578/> Add `Reference::follow()` as a way to peel symbolic refs step by step.

### Bug Fixes (BREAKING)

 - <csr-id-74ce8639e88db5107691e9279df2bbfd38d26de3/> handle symbolic ref updates far more gracefully and with more logical consistency.
   Previously, refspecs couldn't be used to update sybolic references locally, particularly because the logic
   to do so correctly isn't trivial and `git` itself also seems to cover only the most common cases.
   
   However, the logic now changed so that remote updates will only be rejected if
   
   * fast-forward rules are violated
* the local ref is currently checked out
* existing refs would not become 'unborn', i.e. point to a reference that doesn't exist and won't be created due to ref-specs

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 2 calendar days.
 - 9 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-actor v0.24.2, gix-object v0.33.2, gix-ref v0.33.3, gix-config v0.26.2, gix-prompt v0.5.5, gix-odb v0.50.2, gix-transport v0.34.2, gix-protocol v0.37.0, gix-worktree v0.23.1, gix v0.51.0, safety bump 3 crates ([`231ac1c`](https://github.com/Byron/gitoxide/commit/231ac1c6ad5ca9a84dbeb0dee14bfbf2fef1ae1e))
    - Prepare additional changelogs ([`db63815`](https://github.com/Byron/gitoxide/commit/db6381522395a0de047118e81df5cd3cbeb862b9))
    - Prepare changelogs ([`e4d2890`](https://github.com/Byron/gitoxide/commit/e4d2890a85bf60e9cdb4016dddfab3c4dccbe75e))
    - Merge branch 'fixes-and-improvements' ([`f8b1f55`](https://github.com/Byron/gitoxide/commit/f8b1f553371f25b1bea6bce7cbb2ff1f01194856))
    - Handle symbolic ref updates far more gracefully and with more logical consistency. ([`74ce863`](https://github.com/Byron/gitoxide/commit/74ce8639e88db5107691e9279df2bbfd38d26de3))
    - Adapt to changes in `gix-protocol` ([`df81076`](https://github.com/Byron/gitoxide/commit/df810766dfeaaad7474339358a3d844b2c3368cd))
    - Add `Reference::follow()` as a way to peel symbolic refs step by step. ([`d9e551b`](https://github.com/Byron/gitoxide/commit/d9e551b44aa3e84109660328de7637d465d59578))
</details>

## 0.50.1 (2023-07-24)

### Bug Fixes

 - <csr-id-145f8658a32c46db1f54d3098cf9371fe6eeec5e/> `Tree::lookup_entry(_by_path)()` now actually works
   Previously it was lacking a test and that showed.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-archive v0.2.1, gix-ref v0.33.2, gix-pack v0.40.2, gix v0.50.1 ([`13883e5`](https://github.com/Byron/gitoxide/commit/13883e5528385f892ee402e911298121e0c297c0))
    - Prepare changelogs ([`735c206`](https://github.com/Byron/gitoxide/commit/735c2062625aaeffbdbca3c1395dbcf075661e3a))
    - `Tree::lookup_entry(_by_path)()` now actually works ([`145f865`](https://github.com/Byron/gitoxide/commit/145f8658a32c46db1f54d3098cf9371fe6eeec5e))
</details>

## 0.50.0 (2023-07-22)

### New Features

 - <csr-id-caa8fb9502906fa47546c26bbeb3c546664ad944/> `TreeEntryRefExt` and `TreeEntryExt` to be able to easily attach a repo to it.
   Also, add `detach()` to types that were missing it.
 - <csr-id-62cacd4b7a9fc0c0e4c5049f6d0aa7011c8ef923/> `Tree::find_entry()` to easily find an entry in a tree's entries.
 - <csr-id-c4a1fb1ba461c28ac3ea2482adf5f75721d14706/> add `Repository::archive()` as extra
   It implements a high-level interface to achieve `git archive` like functionality.
 - <csr-id-4ee285741e6e1cde3a967980fbf48bab20ddbf68/> optionally make `gix-workspace-stream` available via `Repository::worktree_stream()`
   That way it's easy to obtain a representation of the worktree
   in a fully streaming fashion, which is also the basis for
   `archive`-like functionality.

### New Features (BREAKING)

 - <csr-id-d5e4ee0e6e26ff3feeed1f5aee5bdd0cdc03d1f8/> unify API between `object::tree::Entry` and `object::tree::EntryRef<'_>`

### Bug Fixes (BREAKING)

 - <csr-id-8cad009eafe8e1054a715dc99bb9a884325d5ea5/> `Tree::lookup_entry(_by_path))()` are not mutating anymore; add `Tree::peel_to_entry()` and `peel_to_entry_by_path()`
   The previous implementation was a crutch that could now be circumvented.
   
   The new methods allow to reuse a buffer in case the object isn't used or needed further,
   possibly saving allocations.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 1 calendar day.
 - 3 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-config v0.26.1, gix v0.50.0 ([`d34a4ea`](https://github.com/Byron/gitoxide/commit/d34a4ea27cd83b916c84cf15e1c05da35576db5e))
    - Release gix-diff v0.33.1, gix-discover v0.22.1, gix-ignore v0.5.1, gix-bitmap v0.2.6, gix-index v0.21.1, gix-mailmap v0.16.1, gix-negotiate v0.5.1, gix-pack v0.40.1, gix-odb v0.50.1, gix-packetline v0.16.4, gix-transport v0.34.1, gix-protocol v0.36.1, gix-revision v0.18.1, gix-refspec v0.14.1, gix-worktree v0.23.0, gix v0.50.0 ([`0062971`](https://github.com/Byron/gitoxide/commit/00629710dffeb10fda340665530353703cf5d129))
    - Release gix-tempfile v7.0.2, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.1, gix-sec v0.8.4, gix-prompt v0.5.4, gix-url v0.21.1, gix-credentials v0.17.1, gix-diff v0.33.1, gix-discover v0.22.1, gix-ignore v0.5.1, gix-bitmap v0.2.6, gix-index v0.21.1, gix-mailmap v0.16.1, gix-negotiate v0.5.1, gix-pack v0.40.1, gix-odb v0.50.1, gix-packetline v0.16.4, gix-transport v0.34.1, gix-protocol v0.36.1, gix-revision v0.18.1, gix-refspec v0.14.1, gix-worktree v0.23.0, gix v0.50.0 ([`107a64e`](https://github.com/Byron/gitoxide/commit/107a64e734580ad9e2c4142db96394529d8072df))
    - Release gix-features v0.32.1, gix-actor v0.24.1, gix-validate v0.7.7, gix-object v0.33.1, gix-path v0.8.4, gix-glob v0.10.1, gix-quote v0.4.6, gix-attributes v0.16.0, gix-command v0.2.8, gix-packetline-blocking v0.16.4, gix-filter v0.2.0, gix-fs v0.4.1, gix-chunk v0.4.4, gix-commitgraph v0.18.1, gix-hashtable v0.2.4, gix-revwalk v0.4.1, gix-traverse v0.30.1, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.5, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.1, gix-sec v0.8.4, gix-prompt v0.5.4, gix-url v0.21.1, gix-credentials v0.17.1, gix-diff v0.33.1, gix-discover v0.22.1, gix-ignore v0.5.1, gix-bitmap v0.2.6, gix-index v0.21.1, gix-mailmap v0.16.1, gix-negotiate v0.5.1, gix-pack v0.40.1, gix-odb v0.50.1, gix-packetline v0.16.4, gix-transport v0.34.1, gix-protocol v0.36.1, gix-revision v0.18.1, gix-refspec v0.14.1, gix-worktree v0.23.0, gix v0.50.0, safety bump 5 crates ([`16295b5`](https://github.com/Byron/gitoxide/commit/16295b58e2581d2e8b8b762816f52baabe871c75))
    - Prepare more changelogs ([`c4cc5f2`](https://github.com/Byron/gitoxide/commit/c4cc5f261d29f712a101033a18293a97a9d4ae85))
    - Release gix-date v0.7.1, gix-hash v0.11.4, gix-trace v0.1.3, gix-features v0.32.0, gix-actor v0.24.0, gix-validate v0.7.7, gix-object v0.33.0, gix-path v0.8.4, gix-glob v0.10.0, gix-quote v0.4.6, gix-attributes v0.15.0, gix-command v0.2.7, gix-packetline-blocking v0.16.3, gix-filter v0.1.0, gix-fs v0.4.0, gix-chunk v0.4.4, gix-commitgraph v0.18.0, gix-hashtable v0.2.4, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.4, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.0, gix-sec v0.8.4, gix-prompt v0.5.3, gix-url v0.21.0, gix-credentials v0.17.0, gix-diff v0.33.0, gix-discover v0.22.0, gix-ignore v0.5.0, gix-bitmap v0.2.6, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-packetline v0.16.4, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.1 ([`5cb3589`](https://github.com/Byron/gitoxide/commit/5cb3589b74fc5376e02cbfe151e71344e1c417fe))
    - Update changelogs prior to release ([`2fc66b5`](https://github.com/Byron/gitoxide/commit/2fc66b55097ed494b72d1af939ba5561f71fde97))
    - Merge branch 'improvements-for-crates-index' ([`3f914e8`](https://github.com/Byron/gitoxide/commit/3f914e8840afc59441c3c463bdc89c53136d583e))
    - `TreeEntryRefExt` and `TreeEntryExt` to be able to easily attach a repo to it. ([`caa8fb9`](https://github.com/Byron/gitoxide/commit/caa8fb9502906fa47546c26bbeb3c546664ad944))
    - `Tree::find_entry()` to easily find an entry in a tree's entries. ([`62cacd4`](https://github.com/Byron/gitoxide/commit/62cacd4b7a9fc0c0e4c5049f6d0aa7011c8ef923))
    - `Tree::lookup_entry(_by_path))()` are not mutating anymore; add `Tree::peel_to_entry()` and `peel_to_entry_by_path()` ([`8cad009`](https://github.com/Byron/gitoxide/commit/8cad009eafe8e1054a715dc99bb9a884325d5ea5))
    - Unify API between `object::tree::Entry` and `object::tree::EntryRef<'_>` ([`d5e4ee0`](https://github.com/Byron/gitoxide/commit/d5e4ee0e6e26ff3feeed1f5aee5bdd0cdc03d1f8))
    - J fmt ([`57cab40`](https://github.com/Byron/gitoxide/commit/57cab40f5cb437cc5b0a2c1fc5ae0f91f98bbbcc))
    - Merge branch 'gix-archive' ([`1dda48b`](https://github.com/Byron/gitoxide/commit/1dda48ba2fccb93ebac00fe3460e923af43c86ce))
    - Change archive implementation to require the seek bound. ([`61aed0e`](https://github.com/Byron/gitoxide/commit/61aed0e955974f65f4fea042cbae68ea8a2cc2f5))
    - Add `Repository::archive()` as extra ([`c4a1fb1`](https://github.com/Byron/gitoxide/commit/c4a1fb1ba461c28ac3ea2482adf5f75721d14706))
    - Optionally make `gix-workspace-stream` available via `Repository::worktree_stream()` ([`4ee2857`](https://github.com/Byron/gitoxide/commit/4ee285741e6e1cde3a967980fbf48bab20ddbf68))
    - Update license field following SPDX 2.1 license expression standard ([`9064ea3`](https://github.com/Byron/gitoxide/commit/9064ea31fae4dc59a56bdd3a06c0ddc990ee689e))
</details>

## 0.49.1 (2023-07-19)

A maintenance release without user-facing changes.

### Bug Fixes (BREAKING)

 - <csr-id-8cad009eafe8e1054a715dc99bb9a884325d5ea5/> `Tree::lookup_entry(_by_path))()` are not mutating anymore; add `Tree::peel_to_entry()` and `peel_to_entry_by_path()`
   The previous implementation was a crutch that could now be circumvented.
   
   The new methods allow to reuse a buffer in case the object isn't used or needed further,
   possibly saving allocations.

### New Features (BREAKING)

 - <csr-id-d5e4ee0e6e26ff3feeed1f5aee5bdd0cdc03d1f8/> unify API between `object::tree::Entry` and `object::tree::EntryRef<'_>`

### New Features

 - <csr-id-caa8fb9502906fa47546c26bbeb3c546664ad944/> `TreeEntryRefExt` and `TreeEntryExt` to be able to easily attach a repo to it.
   Also, add `detach()` to types that were missing it.
 - <csr-id-62cacd4b7a9fc0c0e4c5049f6d0aa7011c8ef923/> `Tree::find_entry()` to easily find an entry in a tree's entries.
 - <csr-id-c4a1fb1ba461c28ac3ea2482adf5f75721d14706/> add `Repository::archive()` as extra
   It implements a high-level interface to achieve `git archive` like functionality.
 - <csr-id-4ee285741e6e1cde3a967980fbf48bab20ddbf68/> optionally make `gix-workspace-stream` available via `Repository::worktree_stream()`
   That way it's easy to obtain a representation of the worktree
   in a fully streaming fashion, which is also the basis for
   `archive`-like functionality.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-prompt v0.5.3, gix v0.49.1, cargo-smart-release v0.20.0 ([`f069852`](https://github.com/Byron/gitoxide/commit/f0698522940c9ba4d45db5a44dce9f21ca29cb4e))
    - Prepare changelogs prior to release ([`849f508`](https://github.com/Byron/gitoxide/commit/849f5081313c4a44bdaef6848758d0d9a5d1598b))
    - Merge branch 'smart-release-stability' ([`8629f56`](https://github.com/Byron/gitoxide/commit/8629f569cd5917b6c0c3fd928fde021e7976ee85))
    - Update git2 API mapping and be clear what stability means as well. ([`64cd396`](https://github.com/Byron/gitoxide/commit/64cd396ab05959e1f843f7ccd53ac5d4585584ad))
</details>

## 0.49.0 (2023-07-19)

<csr-id-c548780e6ea49453ecdb45b11bf4c5781b105e6b/>

### New Features

 - <csr-id-980c2ba591dce7fc787c418aed85078c19e2d6d4/> Make `EntryMode` available from `gix::object::tree`.
   Previously one had to go through `gix::objs::tree` which wasn't symmetric
   with `gix::object::Kind`.
 - <csr-id-d4a8f8cf6d8b059978719ea314fc8a4bfe26c60d/> Add `Id::header()` and `Id::try_header()` as syblings to `::object()` and `::try_object()`.
   With the new header related functions one can obtain information about an object more quickly.
 - <csr-id-b73435b3bf334d5be2931c2ea6a597a9dd51b783/> `Repository::header()` and `::try_header()` to learn about objects, quickly
   Accessing just the headers of an object is much faster than accessing the entire
   object. Previously, this method was only available on the `objects` field, now it's
   available through `Repository` directly.
 - <csr-id-c05eb2204620a5ff5e04b766009c873a14ae0f9e/> top-level examples that represent fully-fledged command-line applications.
   Please note that these are just examples, which aren't necessarily
   production ready in terms of quality or performance.
 - <csr-id-8cc106aa430d39ac9967dcfb3d293725fc76cb79/> checkouts when cloning now respect attributes and use filters.
 - <csr-id-8993b777cd0331e7260d7d7d1f820afc79a34b19/> add `Repository::filter_pipeline()` to obtain a primitive to handle data conversions.
   It's fully configured as git would, and can be used to convert data from git or to git.

### Bug Fixes

 - <csr-id-47ca8465e04bdd13fe2ebfc6f012f8191e3f7896/> properly re-initialize object caches after their configuration changes.

### Refactor (BREAKING)

 - <csr-id-c548780e6ea49453ecdb45b11bf4c5781b105e6b/> move error structs into `repository` module where appropriate.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 28 commits contributed to the release over the course of 11 calendar days.
 - 19 days passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#925](https://github.com/Byron/gitoxide/issues/925)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#925](https://github.com/Byron/gitoxide/issues/925)**
    - Remove all copies of repo-initialization files and rework them to be our own. ([`5ac2269`](https://github.com/Byron/gitoxide/commit/5ac22699936dbc5c09c5eefd28b75d48271b286b))
 * **Uncategorized**
    - Release gix-filter v0.1.0, gix-ignore v0.5.0, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.0 ([`4aca8c2`](https://github.com/Byron/gitoxide/commit/4aca8c2ae2ec588fb65ec4faa0c07c19d219569f))
    - Release gix-features v0.32.0, gix-actor v0.24.0, gix-glob v0.10.0, gix-attributes v0.15.0, gix-commitgraph v0.18.0, gix-config-value v0.12.4, gix-fs v0.4.0, gix-object v0.33.0, gix-ref v0.33.0, gix-config v0.26.0, gix-command v0.2.7, gix-url v0.21.0, gix-credentials v0.17.0, gix-diff v0.33.0, gix-discover v0.22.0, gix-filter v0.1.0, gix-ignore v0.5.0, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.0 ([`68ae3ff`](https://github.com/Byron/gitoxide/commit/68ae3ff9d642ec56f088a6a682a073dc16f4e8ca))
    - Adjust package versions (by cargo-smart-release) ([`c70e54f`](https://github.com/Byron/gitoxide/commit/c70e54f163c312c87753a506eeaad462e8579bfb))
    - Prepare changelogs prior to release ([`e4dded0`](https://github.com/Byron/gitoxide/commit/e4dded05138562f9737a7dcfb60570c55769486d))
    - Just fmt ([`a063c62`](https://github.com/Byron/gitoxide/commit/a063c62e3a30006d837b267e2ce74e70e48b4fb6))
    - Merge branch 'adjustments-for-crates-index' ([`b82868d`](https://github.com/Byron/gitoxide/commit/b82868d5688d8d4849c47ed3d209a96ee59e69b3))
    - Make `EntryMode` available from `gix::object::tree`. ([`980c2ba`](https://github.com/Byron/gitoxide/commit/980c2ba591dce7fc787c418aed85078c19e2d6d4))
    - Add `Id::header()` and `Id::try_header()` as syblings to `::object()` and `::try_object()`. ([`d4a8f8c`](https://github.com/Byron/gitoxide/commit/d4a8f8cf6d8b059978719ea314fc8a4bfe26c60d))
    - `Repository::header()` and `::try_header()` to learn about objects, quickly ([`b73435b`](https://github.com/Byron/gitoxide/commit/b73435b3bf334d5be2931c2ea6a597a9dd51b783))
    - Properly re-initialize object caches after their configuration changes. ([`47ca846`](https://github.com/Byron/gitoxide/commit/47ca8465e04bdd13fe2ebfc6f012f8191e3f7896))
    - Top-level examples that represent fully-fledged command-line applications. ([`c05eb22`](https://github.com/Byron/gitoxide/commit/c05eb2204620a5ff5e04b766009c873a14ae0f9e))
    - Cargo fmt ([`6121b8f`](https://github.com/Byron/gitoxide/commit/6121b8f6a7da7f263c6e066155f053a1d7c81477))
    - `git log` example include empty parents and paths ([`bd59bbe`](https://github.com/Byron/gitoxide/commit/bd59bbebddf804a4dd0872127dcc31b5c3b29c2f))
    - `git log` example now accepts multiple paths. ([`0df9f70`](https://github.com/Byron/gitoxide/commit/0df9f707987c8001c4ca81faf69033c679a75fd5))
    - `git log` example filter for min/max parents ([`01e9c29`](https://github.com/Byron/gitoxide/commit/01e9c29bf7106b30d8e3e8c71b37eff77bcc38b5))
    - `git log` example iterator now properly lazy ([`8a6f1e8`](https://github.com/Byron/gitoxide/commit/8a6f1e89fa4d736a2c902be55413887e14885957))
    - `git log` example now shows merge parents ([`5cbb6a7`](https://github.com/Byron/gitoxide/commit/5cbb6a72c34d926a2782569d8370e54d4c63ab34))
    - A `git log` example ([`03b3423`](https://github.com/Byron/gitoxide/commit/03b342306c5effac5e8aa92a349385e59785c0b7))
    - A `git ls-tree` example ([`6f4b431`](https://github.com/Byron/gitoxide/commit/6f4b43101f7b46e38c2f61c2f859347085d8214f))
    - Thanks clippy ([`3ef32af`](https://github.com/Byron/gitoxide/commit/3ef32af9bf477cbc60d24da8bb3f15d20976e9e0))
    - Merge branch 'unique-templates' ([`cbb0db8`](https://github.com/Byron/gitoxide/commit/cbb0db80ccc5c29c92f7abd8af2a03c67d86fc2b))
    - Adapt journey tests to changes in init templates ([`6297d22`](https://github.com/Byron/gitoxide/commit/6297d2201abb97ec999986a7a19b9ddb02114e24))
    - Merge branch 'integrate-filtering' ([`b19a56d`](https://github.com/Byron/gitoxide/commit/b19a56dcfa9bea86332a84aa4e8fad445e7d1724))
    - Checkouts when cloning now respect attributes and use filters. ([`8cc106a`](https://github.com/Byron/gitoxide/commit/8cc106aa430d39ac9967dcfb3d293725fc76cb79))
    - Add `Repository::filter_pipeline()` to obtain a primitive to handle data conversions. ([`8993b77`](https://github.com/Byron/gitoxide/commit/8993b777cd0331e7260d7d7d1f820afc79a34b19))
    - Move error structs into `repository` module where appropriate. ([`c548780`](https://github.com/Byron/gitoxide/commit/c548780e6ea49453ecdb45b11bf4c5781b105e6b))
    - Add keys required to deal with worktree conversions and filters. ([`3fbd7b0`](https://github.com/Byron/gitoxide/commit/3fbd7b0c864cf2f1a38ae24e85d47b0b26b271a7))
</details>

## 0.48.0 (2023-06-29)

<csr-id-fb63f3f07f0f9545be5942bcb66b06040fbc7fe9/>
<csr-id-3c8e3c1e88d36657d4e6eeaf0819be7fd9341ae1/>

The main feature of this release is support dates prior to the UNIX epoch. Note that this is a feature that isn't supported by `git`, but only by `libgit2`.

### Bug Fixes

 - <csr-id-9cfc4aa318bc44c9e4310db7d3764b015472e1af/> use type for time consistently.
   This will allow it to be changed more easily later.

### Other

 - <csr-id-fb63f3f07f0f9545be5942bcb66b06040fbc7fe9/> Add incomplete mapping of typical `git2` functions and their counterpart in `gix`.
   That way the ground-work is laid for making the usage of `gix` easier for those who used
   `git2` before.
 - <csr-id-3c8e3c1e88d36657d4e6eeaf0819be7fd9341ae1/> make clear what can happen if rewrite-tracking isn't disabled if it is not desired.
   Triggered by this `onefetch` PR: https://github.com/o2sh/onefetch/pull/1093

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 6 calendar days.
 - 6 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-transport v0.33.1, gix v0.48.0 ([`f27ca12`](https://github.com/Byron/gitoxide/commit/f27ca128c5f109ad02e4e1a12dc14e93b07bbfcf))
    - Release gix-lock v7.0.1, gix v0.48.0 ([`5ce81ef`](https://github.com/Byron/gitoxide/commit/5ce81ef16210f0b0b72dfd5710a064ccda96ac1c))
    - Release gix-glob v0.9.1, gix-attributes v0.14.1, gix-config-value v0.12.3, gix-ref v0.32.1, gix-sec v0.8.3, gix-config v0.25.1, gix-url v0.20.1, gix-credentials v0.16.1, gix-discover v0.21.1, gix-ignore v0.4.1, gix-pack v0.39.1, gix-odb v0.49.1, gix-worktree v0.21.1, gix v0.48.0 ([`69c6a36`](https://github.com/Byron/gitoxide/commit/69c6a36ba14cbef129deebda9fd8870005fefa17))
    - Release gix-features v0.31.1, gix-path v0.8.3, gix v0.48.0 ([`9ca3464`](https://github.com/Byron/gitoxide/commit/9ca346462806671fbc49643a87cea25ab0da3be8))
    - Release gix-date v0.7.0, gix-trace v0.1.2, gix-actor v0.23.0, gix-commitgraph v0.17.1, gix-utils v0.1.4, gix-object v0.32.0, gix-ref v0.32.0, gix-config v0.25.0, gix-diff v0.32.0, gix-discover v0.21.0, gix-hashtable v0.2.3, gix-revwalk v0.3.0, gix-traverse v0.29.0, gix-index v0.20.0, gix-mailmap v0.15.0, gix-negotiate v0.4.0, gix-pack v0.39.0, gix-odb v0.49.0, gix-protocol v0.35.0, gix-revision v0.17.0, gix-refspec v0.13.0, gix-worktree v0.21.0, gix v0.48.0, safety bump 20 crates ([`27e8c18`](https://github.com/Byron/gitoxide/commit/27e8c18db5a9a21843381c116a8ed6d9f681b3f8))
    - Prepare changelogs prior to release ([`00f96fb`](https://github.com/Byron/gitoxide/commit/00f96fb3110a8f81a1bd0d74c757c15b8773c6f6))
    - Merge branch 'i64-times' ([`b407461`](https://github.com/Byron/gitoxide/commit/b407461d8991db67a5bdb2ab13f518f78a85ed40))
    - Add incomplete mapping of typical `git2` functions and their counterpart in `gix`. ([`fb63f3f`](https://github.com/Byron/gitoxide/commit/fb63f3f07f0f9545be5942bcb66b06040fbc7fe9))
    - Adapt to changes in `gix-date` ([`fba45c6`](https://github.com/Byron/gitoxide/commit/fba45c68d57d5f73070a6949556a04187d42e427))
    - Use type for time consistently. ([`9cfc4aa`](https://github.com/Byron/gitoxide/commit/9cfc4aa318bc44c9e4310db7d3764b015472e1af))
    - Add a test to see what happens if negative dates are used in commits ([`57a5cd1`](https://github.com/Byron/gitoxide/commit/57a5cd1ca2f8153568c366cd1709be7d4ebec972))
    - Make clear what can happen if rewrite-tracking isn't disabled if it is not desired. ([`3c8e3c1`](https://github.com/Byron/gitoxide/commit/3c8e3c1e88d36657d4e6eeaf0819be7fd9341ae1))
    - More tracing information when updating refs ([`6906e0d`](https://github.com/Byron/gitoxide/commit/6906e0d717bb04efb0744eb20afb4c53ebe360c9))
    - Add more details for negotation phases ([`8341d08`](https://github.com/Byron/gitoxide/commit/8341d0882697b02275ec2bb0f05229ba3b60df3b))
    - Add a span for each negotiation round ([`ec73479`](https://github.com/Byron/gitoxide/commit/ec7347971163e838e0fdedb0bc6bc88d32d30d8e))
</details>

## 0.47.0 (2023-06-22)

<csr-id-bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d/>

### Chore

 - <csr-id-bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d/> Add `clippy::redundant-closure-for-method-calls` lint

### New Features

 - <csr-id-3cffa268460eb2d41bd6a30d45778b88db4ec602/> provide basic `tracing` spans for common operations.
   This is just the beginning and more crates will integrate with it over time.
 - <csr-id-47c7b0dff9ca82e7c0c60b1dcf1d120f7bec7955/> expose `hashtable` in root for access to optimized-for-object-ids sets and maps.

### Bug Fixes

 - <csr-id-67c06d99dc8387ce566c2fe436c28cdaa041bf07/> make sure empty packs in shallow clones are working as well.
 - <csr-id-db69e31d8bb73aba886a9a323bfa154a23deacf8/> no-want detection for negotiation phase is now consistent.
   It being inconsistent was a reason for 'failing to parse server response' which
   was empty as we didn't provide any wants to the server, but didn't detect that case
   in the initial negotiation-preparation phase.
   
   Turns out we didn't detect it as our special handling of implicit tags was not done
   in the negotiation-preparation phase.
   
   The fix consists of unifying the filtering phase to all places that needed, so
   the preparation phase outcome is now consistent with what would have come later.

### New Features (BREAKING)

 - <csr-id-682def03ce6ec93d7bcd2f79eedea52021b77f03/> provide `fetch::outcome::Negotiate` for details on what happened during negotiation.
   We also remove the `negotiation_rounds` field in favor of a far more detailed `fetch::outcome::Negotiate` struct.
 - <csr-id-574e0f4786719bd56da2fa218772f879fda282bf/> respect the `core.commitGraph` option.
   Previously, we would always use the commitgraph when available, but now we only do so
   if the `core.commitGraph` option is set.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 22 commits contributed to the release over the course of 11 calendar days.
 - 12 days passed between releases.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.6.0, gix-hash v0.11.3, gix-trace v0.1.1, gix-features v0.31.0, gix-actor v0.22.0, gix-path v0.8.2, gix-glob v0.9.0, gix-quote v0.4.5, gix-attributes v0.14.0, gix-chunk v0.4.3, gix-commitgraph v0.17.0, gix-config-value v0.12.2, gix-fs v0.3.0, gix-tempfile v7.0.0, gix-utils v0.1.3, gix-lock v7.0.0, gix-validate v0.7.6, gix-object v0.31.0, gix-ref v0.31.0, gix-sec v0.8.2, gix-config v0.24.0, gix-command v0.2.6, gix-prompt v0.5.2, gix-url v0.20.0, gix-credentials v0.16.0, gix-diff v0.31.0, gix-discover v0.20.0, gix-hashtable v0.2.2, gix-ignore v0.4.0, gix-bitmap v0.2.5, gix-revwalk v0.2.0, gix-traverse v0.28.0, gix-index v0.19.0, gix-mailmap v0.14.0, gix-negotiate v0.3.0, gix-pack v0.38.0, gix-odb v0.48.0, gix-packetline v0.16.3, gix-transport v0.33.0, gix-protocol v0.34.0, gix-revision v0.16.0, gix-refspec v0.12.0, gix-worktree v0.20.0, gix v0.47.0, gitoxide-core v0.29.0, gitoxide v0.27.0, safety bump 30 crates ([`ea9f942`](https://github.com/Byron/gitoxide/commit/ea9f9424e777f10da0e33bb9ffbbefd01c4c5a74))
    - Prepare changelogs prior to release ([`18b0a37`](https://github.com/Byron/gitoxide/commit/18b0a371941aa2d4d62512437d5daa351ba99ffd))
    - Add a span for another potentially expensive portion of the negotiation ([`f2e7ec4`](https://github.com/Byron/gitoxide/commit/f2e7ec4d80299933327ebdc932a758d5d9c7f218))
    - `just fmt` ([`871dd0b`](https://github.com/Byron/gitoxide/commit/871dd0b977caf17159092a4739ba5408403cdb2c))
    - Merge branch 'corpus' ([`aa16c8c`](https://github.com/Byron/gitoxide/commit/aa16c8ce91452a3e3063cf1cf0240b6014c4743f))
    - Change MSRV to 1.65 ([`4f635fc`](https://github.com/Byron/gitoxide/commit/4f635fc4429350bae2582d25de86429969d28f30))
    - Provide basic `tracing` spans for common operations. ([`3cffa26`](https://github.com/Byron/gitoxide/commit/3cffa268460eb2d41bd6a30d45778b88db4ec602))
    - Merge branch 'gix-revision-graph' ([`036e60a`](https://github.com/Byron/gitoxide/commit/036e60a3ad39ba9b018c0b56454f12fad455c7bb))
    - Expose `hashtable` in root for access to optimized-for-object-ids sets and maps. ([`47c7b0d`](https://github.com/Byron/gitoxide/commit/47c7b0dff9ca82e7c0c60b1dcf1d120f7bec7955))
    - Provide `fetch::outcome::Negotiate` for details on what happened during negotiation. ([`682def0`](https://github.com/Byron/gitoxide/commit/682def03ce6ec93d7bcd2f79eedea52021b77f03))
    - Merge branch 'fix-no-want-detection' ([`71efcbb`](https://github.com/Byron/gitoxide/commit/71efcbba112376b4acaf37d662cdb38d369462be))
    - Make sure empty packs in shallow clones are working as well. ([`67c06d9`](https://github.com/Byron/gitoxide/commit/67c06d99dc8387ce566c2fe436c28cdaa041bf07))
    - No-want detection for negotiation phase is now consistent. ([`db69e31`](https://github.com/Byron/gitoxide/commit/db69e31d8bb73aba886a9a323bfa154a23deacf8))
    - Merge branch 'help-874-redundant-closures' ([`fe59956`](https://github.com/Byron/gitoxide/commit/fe59956ad667303a923d7cfd9ffd72283df41d78))
    - Add `clippy::redundant-closure-for-method-calls` lint ([`bcad5c2`](https://github.com/Byron/gitoxide/commit/bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d))
    - Merge branch 'future-dates' ([`8d2e6a9`](https://github.com/Byron/gitoxide/commit/8d2e6a91ac92a033e9e3daad5cffa90263075536))
    - Respect the `core.commitGraph` option. ([`574e0f4`](https://github.com/Byron/gitoxide/commit/574e0f4786719bd56da2fa218772f879fda282bf))
    - Adapt to changes in `gix-revision`/`gix-revwalk` ([`1fdaf71`](https://github.com/Byron/gitoxide/commit/1fdaf71d32eb60ad056376d27837ff37d4d314cd))
    - Adapt to changes in `gix-protocol` ([`b785e81`](https://github.com/Byron/gitoxide/commit/b785e811232d645ad72bfb87459efbd80cb0a399))
    - Adapt to changes in `gix-traverse` ([`b447f47`](https://github.com/Byron/gitoxide/commit/b447f478b8a5a28c659cef178b2a06b666f89ec3))
    - Adapt to changes in `gix-actor` ([`4a80e86`](https://github.com/Byron/gitoxide/commit/4a80e868f9530896616e649838e9be64b6d10036))
    - Adapt to changes in `gix-date` ([`d575336`](https://github.com/Byron/gitoxide/commit/d575336c26e6026e463cd06d88266bb2bdd3e162))
</details>

## 0.46.0 (2023-06-10)

<csr-id-f0ddc3b9c5a34b7930b965dfb1438f95279a8bde/>

### New Features (BREAKING)

 - <csr-id-7e9f202746d4376332f9779c6e4bd67933d618c7/> `revision::Walk` yields `revision::Info` structs instead of `Id`s.
   This enables re-use of information that was already obtained, like the parents of
   a commit and possibly its commit-time.

### Changed (BREAKING)

 - <csr-id-068603a4b7a52eaa397d61212e7aec5a0195ac29/> rename `Repository::commit_graph()` to `::revision_graph()`.
   THat's a better fix given its locaion in `gix-revision`, while differentiating
   it further from the lower-level `commit-graph`.
   
   Also rename `Repository::commit_cache()` to `::commit_graph()` now that the name is free.

### Other

 - <csr-id-f0ddc3b9c5a34b7930b965dfb1438f95279a8bde/> `gix::revision::walk::Platform` now informas about the commitgraph.
   In short, one should use the `Graph` to obtain the tools necessary for potentially
   accelerated, custom commit walks.

### New Features

 - <csr-id-cc72e497868636b0e7c943f675bda82860c2b53e/> make it possible to use `config::tree::Key` to more conveniently set values via `config::SnapshotMut::set()`
 - <csr-id-b2b88dc11d1b745e787596e9b94122238ccaf34c/> use the `commitgraph` if possible and allow its usage to be controlled via `revision::walk::Platform::use_commit_graph(toggle)`.
   The commitgraph is a data structure to greatly accelerate commit walks. It is now supported and
   used by default, but can be deactivated if desired.
   
   Further, add `Repository::commit_cache()` for direct access to just the commit-graph datastructure,
   without the extras provided by `gix_revision::Graph`.
 - <csr-id-5d320121533d60fc594792da7838a4f9c661dea0/> add `Repository::index_or_load_from_head()`.
   That way it's possible to either open the existing worktree index, or create one
   in-memory by turning our HEAD tree into an index on the fly.
 - <csr-id-2a698fab7323fd5befd14926bcb9cebf09afc312/> make it possible to use `config::tree::Key` to more conveniently set values via `config::SnapshotMut::set()`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 2 calendar days.
 - 3 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-attributes v0.13.1, gix-diff v0.30.1, gix-revwalk v0.1.0, gix-traverse v0.27.0, gix-index v0.18.0, gix-revision v0.15.2, gix-negotiate v0.2.1, gix-pack v0.37.0, gix-odb v0.47.0, gix-protocol v0.33.2, gix-worktree v0.19.0, gix v0.46.0, safety bump 7 crates ([`2560a2c`](https://github.com/Byron/gitoxide/commit/2560a2cc3e1d8c60cd812e15696fa4761d036e19))
    - Prepare changelogs prior to release ([`298f3d7`](https://github.com/Byron/gitoxide/commit/298f3d7359c5b183314d8c584e45dcdd559d88b3))
    - Improve tests related to the handling of shallow repos ([`d50bfa9`](https://github.com/Byron/gitoxide/commit/d50bfa97f528141e0183558f21a364d969911ef4))
    - Merge branch 'walk-with-commitgraph' ([`fdee9a2`](https://github.com/Byron/gitoxide/commit/fdee9a22873a13ae644d3dc92f8fe93f8f0266c0))
    - Adapt to changes in `gix` ([`20f73c8`](https://github.com/Byron/gitoxide/commit/20f73c8224ead1b423a1b6331c9cab65f769d46a))
    - `revision::Walk` yields `revision::Info` structs instead of `Id`s. ([`7e9f202`](https://github.com/Byron/gitoxide/commit/7e9f202746d4376332f9779c6e4bd67933d618c7))
    - Rename `Repository::commit_graph()` to `::revision_graph()`. ([`068603a`](https://github.com/Byron/gitoxide/commit/068603a4b7a52eaa397d61212e7aec5a0195ac29))
    - Use the `commitgraph` if possible and allow its usage to be controlled via `revision::walk::Platform::use_commit_graph(toggle)`. ([`b2b88dc`](https://github.com/Byron/gitoxide/commit/b2b88dc11d1b745e787596e9b94122238ccaf34c))
    - Adapt to changes in `gix-traverse` ([`1f682fd`](https://github.com/Byron/gitoxide/commit/1f682fd991b9b76a8d37e6852567ff239c0ac0db))
    - Adapt to changes in `gix-revwalk` ([`f7d95d1`](https://github.com/Byron/gitoxide/commit/f7d95d189af1422a7ba48db1857452e32e1d9db9))
    - Add `Repository::index_or_load_from_head()`. ([`5d32012`](https://github.com/Byron/gitoxide/commit/5d320121533d60fc594792da7838a4f9c661dea0))
    - `gix::revision::walk::Platform` now informas about the commitgraph. ([`f0ddc3b`](https://github.com/Byron/gitoxide/commit/f0ddc3b9c5a34b7930b965dfb1438f95279a8bde))
    - Update changelog with information for the `gix` CLI. ([`4e081f2`](https://github.com/Byron/gitoxide/commit/4e081f2141dcb9919597c53dfd5706cc9439d541))
    - Make it possible to use `config::tree::Key` to more conveniently set values via `config::SnapshotMut::set()` ([`2a698fa`](https://github.com/Byron/gitoxide/commit/2a698fab7323fd5befd14926bcb9cebf09afc312))
    - Release gix-protocol v0.33.1 ([`9c99ed3`](https://github.com/Byron/gitoxide/commit/9c99ed30162081a7f26d72e0ed26966ff62d2b1c))
</details>

## 0.45.1 (2023-06-06)

### Bug Fixes

 - <csr-id-9010f586ac46fcea5b8abba8f30a5639ed6b9225/> `gix::env::fetch::collate::Error` now considers negotiation errors a sign of corrupt git repos.
   Indeed, all of these negotiation errors are due to failures reading references or objects that ought
   to be there.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-revision v0.15.1, gix v0.45.1 ([`11766a0`](https://github.com/Byron/gitoxide/commit/11766a0a82754fee9918ccdb8eaf92af6d2561ba))
    - Merge branch 'adjustments-for-cargo' ([`04f011c`](https://github.com/Byron/gitoxide/commit/04f011c3c3e49e87a3b868d4bf6e77a361b96da8))
    - `gix::env::fetch::collate::Error` now considers negotiation errors a sign of corrupt git repos. ([`9010f58`](https://github.com/Byron/gitoxide/commit/9010f586ac46fcea5b8abba8f30a5639ed6b9225))
</details>

## 0.45.0 (2023-06-06)

<csr-id-dbc6cbb4363c2532f81b0bd6e351c4577bb9e9a3/>
<csr-id-9689a08d00e9d54f6bb581660ee99077bd214cb4/>

The reason for this release is the ability to properly negotiate packs, also across multiple rounds, and with `protocol.version` 1 or 2, across
stateless or stateful transports.

### Chore

 - <csr-id-dbc6cbb4363c2532f81b0bd6e351c4577bb9e9a3/> inline format args

### New Features

 - <csr-id-af0ef2f36736e3805f769d8cd59c9fa7eb6a22a0/> use `gix-negotiate` in fetch machinery.
   Thanks to it we are finally able to do pack negotiations just like git can,
   as many rounds as it takes and with all available algorithms.
   
   Works for V1 and V2 and for stateless and stateful transports.
 - <csr-id-020ff4e383fc76a255eabf099bb9cf5116a95afa/> Add `gitoxide.core.defaultPackCacheMemoryLimit` to control memory limits.
   Previously the 64 slot LRU cache didn't have any limit, now one is implemented that
   defaults to about 96MB.

### New Features (BREAKING)

 - <csr-id-e011e360fb2db0288f718cb3bb2b28b4652bc8ae/> respect `core.useReplaceRefs` and remove `gitoxide.objects.noReplace`.
   The gitoxide specific variable wasn't needed in the first place.

### Refactor (BREAKING)

 - <csr-id-9689a08d00e9d54f6bb581660ee99077bd214cb4/> Move `Kind` into `repository::Kind`.
   This type was from old times where `gix` was called `gix-repository`.
   Also remote `ThreadSafeRepository::kind()` in favor of leaving only
   `Repository::kind()`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 33 commits contributed to the release over the course of 40 calendar days.
 - 40 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#851](https://github.com/Byron/gitoxide/issues/851)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#851](https://github.com/Byron/gitoxide/issues/851)**
    - Add `gitoxide.core.defaultPackCacheMemoryLimit` to control memory limits. ([`020ff4e`](https://github.com/Byron/gitoxide/commit/020ff4e383fc76a255eabf099bb9cf5116a95afa))
 * **Uncategorized**
    - Release gix-date v0.5.1, gix-hash v0.11.2, gix-features v0.30.0, gix-actor v0.21.0, gix-path v0.8.1, gix-glob v0.8.0, gix-quote v0.4.4, gix-attributes v0.13.0, gix-chunk v0.4.2, gix-commitgraph v0.16.0, gix-config-value v0.12.1, gix-fs v0.2.0, gix-tempfile v6.0.0, gix-utils v0.1.2, gix-lock v6.0.0, gix-validate v0.7.5, gix-object v0.30.0, gix-ref v0.30.0, gix-sec v0.8.1, gix-config v0.23.0, gix-command v0.2.5, gix-prompt v0.5.1, gix-url v0.19.0, gix-credentials v0.15.0, gix-diff v0.30.0, gix-discover v0.19.0, gix-hashtable v0.2.1, gix-ignore v0.3.0, gix-bitmap v0.2.4, gix-traverse v0.26.0, gix-index v0.17.0, gix-mailmap v0.13.0, gix-revision v0.15.0, gix-negotiate v0.2.0, gix-pack v0.36.0, gix-odb v0.46.0, gix-packetline v0.16.2, gix-transport v0.32.0, gix-protocol v0.33.0, gix-refspec v0.11.0, gix-worktree v0.18.0, gix v0.45.0, safety bump 29 crates ([`9a9fa96`](https://github.com/Byron/gitoxide/commit/9a9fa96fa8a722bddc5c3b2270b0edf8f6615141))
    - `just fmt` ([`ffc1276`](https://github.com/Byron/gitoxide/commit/ffc1276e0c991ac33ce842f5dca0b45ac69680c0))
    - Prepare changelogs prior to release ([`8f15cec`](https://github.com/Byron/gitoxide/commit/8f15cec1ec7d5a9d56bb158f155011ef2bb3539b))
    - Merge branch 'integrate-gix-negotiate' ([`ae845de`](https://github.com/Byron/gitoxide/commit/ae845dea6cee6523c88a23d7a14293589cf8092f))
    - Add test to validate alternates in the context of fetching ([`ae1bc41`](https://github.com/Byron/gitoxide/commit/ae1bc41817bec3b83fe65104e7e3efe4bd798a78))
    - Use `gix-negotiate` in fetch machinery. ([`af0ef2f`](https://github.com/Byron/gitoxide/commit/af0ef2f36736e3805f769d8cd59c9fa7eb6a22a0))
    - Respect `core.useReplaceRefs` and remove `gitoxide.objects.noReplace`. ([`e011e36`](https://github.com/Byron/gitoxide/commit/e011e360fb2db0288f718cb3bb2b28b4652bc8ae))
    - Thanks clippy ([`9525ac8`](https://github.com/Byron/gitoxide/commit/9525ac822aa902f5325f17e7b08ffb60b683e0e7))
    - Merge branch 'fix-docs' ([`420553a`](https://github.com/Byron/gitoxide/commit/420553a10d780e0b2dc466cac120989298a5f187))
    - Minor fixes ([`89a8cfe`](https://github.com/Byron/gitoxide/commit/89a8cfe40e5c3a9d4a4181fa055e3f4a529a8081))
    - Cleaning up documentation ([`2578e57`](https://github.com/Byron/gitoxide/commit/2578e576bfa365d194a23a1fb0bf09be230873de))
    - Move `Kind` into `repository::Kind`. ([`9689a08`](https://github.com/Byron/gitoxide/commit/9689a08d00e9d54f6bb581660ee99077bd214cb4))
    - Merge branch 'auto-clippy' ([`dbf8aa1`](https://github.com/Byron/gitoxide/commit/dbf8aa19d19109195d0274928eae4b94f248cd88))
    - Autofix map-or-unwrap clippy lint (and manual fix what was left) ([`2087032`](https://github.com/Byron/gitoxide/commit/2087032b5956dcd82bce6ac57e530e8724b57f17))
    - Merge branch 'main' into auto-clippy ([`3ef5c90`](https://github.com/Byron/gitoxide/commit/3ef5c90aebce23385815f1df674c1d28d58b4b0d))
    - Merge pull request #864 from nyurik/lint-fmt ([`279dc09`](https://github.com/Byron/gitoxide/commit/279dc09446f41d7f1d76350fbfafb444e53cd7da))
    - Merge branch 'blinxen/main' ([`9375cd7`](https://github.com/Byron/gitoxide/commit/9375cd75b01aa22a0e2eed6305fe45fabfd6c1ac))
    - Inline format args ([`dbc6cbb`](https://github.com/Byron/gitoxide/commit/dbc6cbb4363c2532f81b0bd6e351c4577bb9e9a3))
    - Include license files in all crates ([`facaaf6`](https://github.com/Byron/gitoxide/commit/facaaf633f01c857dcf2572c6dbe0a92b7105c1c))
    - Merge branch 'consecutive-negotiation' ([`97b3f7e`](https://github.com/Byron/gitoxide/commit/97b3f7e2eaddea20c98f2f7ab6a0d2e2117b0793))
    - Release gix-commitgraph v0.15.0, gix-revision v0.14.0, gix-negotiate v0.1.0, safety bump 7 crates ([`92832ca`](https://github.com/Byron/gitoxide/commit/92832ca2899cd2f222f4c7b1cc9e766178f55806))
    - Merge branch 'consecutive-negotiation' ([`4507f94`](https://github.com/Byron/gitoxide/commit/4507f94984c811ea098e43472e5f54ec4dbb90c1))
    - Adapt to changes in `gix-revision` ([`56f4d30`](https://github.com/Byron/gitoxide/commit/56f4d30960de4afc8c53136af45149cf880547c5))
    - Refactor ([`f4245f4`](https://github.com/Byron/gitoxide/commit/f4245f4bb0921610456dde2c56068e7c5e4f1d27))
    - Merge branch 'fix-851' ([`2f275d5`](https://github.com/Byron/gitoxide/commit/2f275d5d3cb49b3b8ba53b30e4b4386fac32662b))
    - Adjust to changes in `gix-pack` ([`215889c`](https://github.com/Byron/gitoxide/commit/215889ceb976a59368c132aabfffb71a6a2ac9f8))
    - Support reading the fetch algorithm from configuration ([`33b7770`](https://github.com/Byron/gitoxide/commit/33b777074db21db8cd060ecf8cfdac0409a7e10c))
    - Release gix-object v0.29.2 ([`4f879bf`](https://github.com/Byron/gitoxide/commit/4f879bf35653bdc8f9729d524c6e8e1fb3c6886b))
    - Release gix-discover v0.18.1, gix-worktree v0.17.1, gix-testtools v0.12.0 ([`f7b6c6f`](https://github.com/Byron/gitoxide/commit/f7b6c6f27c090cbc584fbd3f5403da5ac1a9ff02))
    - Release gix-index v0.16.1 ([`08c6f9d`](https://github.com/Byron/gitoxide/commit/08c6f9de95c65ff05db4ce6a5593127c4280b2ef))
    - Release gix-ref v0.29.1 ([`13e01f5`](https://github.com/Byron/gitoxide/commit/13e01f5742ed2121f00f4b16c1df0cce5e7708ef))
    - Improve docs for `Shallow` ([`3d95bb7`](https://github.com/Byron/gitoxide/commit/3d95bb76746a56b6e9060245f6c190c3836a0102))
</details>

## 0.44.1 (2023-04-27)

A maintenance release without user-facing changes. It's meant to fix breakage that occurred when publishing a breaking change in `gix-path` by accident.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-path v0.8.0, gix-glob v0.7.0, gix-attributes v0.12.0, gix-config-value v0.12.0, gix-ref v0.29.0, gix-sec v0.8.0, gix-config v0.22.0, gix-prompt v0.5.0, gix-url v0.18.0, gix-credentials v0.14.0, gix-discover v0.18.0, gix-ignore v0.2.0, gix-pack v0.35.0, gix-odb v0.45.0, gix-transport v0.31.0, gix-protocol v0.32.0, gix-refspec v0.10.1, gix-worktree v0.17.0, gix v0.44.1 ([`7ebc9f7`](https://github.com/Byron/gitoxide/commit/7ebc9f734ec4371dd27daa568c0244185bb49eb5))
    - Prepare changelogs prior to release ([`0135158`](https://github.com/Byron/gitoxide/commit/013515897215400539bfd53c25548bd054186ba6))
    - Bump gix-path v0.8.0, safety bump 20 crates (gix set to 0.44.1 manually) ([`43ebaf2`](https://github.com/Byron/gitoxide/commit/43ebaf267557218865862538ffc7bdf00558492f))
</details>

## 0.44.0 (2023-04-26)

### New Features

 - <csr-id-08e8fc2152794652ba1c986df493c2ac915af9e7/> `gix index entries` also prints attributes.
 - <csr-id-bc28443e452c4de81368739a11a2482ae0a93485/> add `Repository::attributes()` and `Worktree::attributes()`.
 - <csr-id-40a1b7444ba9d9b61a1c22a7f25662eec3c25a1b/> add `index.threads` configuration to `gix::config::tree`
 - <csr-id-afe7faa14afb2ec4934f204e01ed12bcd0b3e786/> Before writing new objects, check if they exist.
   That way we safe expensive IO at the cost of some CPU.
 - <csr-id-037f52d4099e239c28210476ad7ab57d22aa3626/> add `Object::into_tag()` and `Tag::decode()` methods.
   This makes the API more symmetric as similar methods exist for commits
   and trees.
 - <csr-id-35cb6b42bd8071e5e5c16ed6d37884deea524330/> Allow `USE_NSEC` and `USE_STDEV` compile time flags to configured at runtime.
   Right now git may be compiled without these capabilities, even though on some platforms
   it might make perfect sense to enable them by default or enable them on a per repository
   basis. This is now possible thanks to added gitoxide specific functions.
 - <csr-id-358500f0efaec7c67b307a6a1aa27ecad7502eb7/> `open::Options` now allow controlling where gitattributes files are loaded from.
   That way it's possible to, for example, isolate all operations that rely on the `gitattribute`
   system, like checkouts or additions to the index.
 - <csr-id-ec93f75cfdf6cbd617c4a92eefae97f2c7736d65/> `revision::walk::Platform::selected(filter)` to selectively prune parts of the commit graph.

### Bug Fixes

 - <csr-id-2cd5054b0a1994571a25a49193449904cfd30b50/> When removing all shallow commits from shallow file, delete it.
   Previously it would leave an empty file, which will be ignored by the implementation
   but might be confusing to users.
 - <csr-id-43f695a9607f1f85f859f2ef944b785b5b6dd238/> `gix::open()` can handle bare repositories with index.
   These are mis-classified as non-bare repository, which previosuly
   caused it to get off-track.

### New Features (BREAKING)

 - <csr-id-26e6a661ed5827151708b9fcc3d7468aa60cf4e3/> add `Repository::excludes()` and simplify signature of `Worktree::excludes()`.
   Further, this change removes the `permission` module without replacement,
   and moves `permissions` into `open`.
   
   This corrects an artifact of this crate previously being name `gix-repository` and brings
   these types semantically closer to where they are actually used.
 - <csr-id-cb3437632fe7ff0ce4efd11c08a8d684d7e7e430/> support configuring the connection (i.e. for auth) during clone.
   This change also removes the generic type for Progress from `Connection`
   which forces it to be passed to every potentially long-running method.
 - <csr-id-b83ee366a3c65c717beb587ad809268f1c54b8ad/> Rename `serde1` cargo feature to `serde` and use the weak-deps cargo capability.
   With it it's possible to not automatically declare all optional dependencies externally visible
   features, and thus re-use feature names that oterwise are also a crate name.
   
   Previously I thought that `serde1` is for future-proofing and supporting multiple serde versions
   at the same time. However, it's most definitely a burden I wouldn't want anyway, so using
   `serde` seems to be the way to go into the future.
 - <csr-id-b645d28f9641c6b4022e1e37ad9fe528922ec747/> remove types that are now available in `gix-os`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 48 commits contributed to the release over the course of 26 calendar days.
 - 27 days passed between releases.
 - 14 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#801](https://github.com/Byron/gitoxide/issues/801), [#814](https://github.com/Byron/gitoxide/issues/814)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#801](https://github.com/Byron/gitoxide/issues/801)**
    - `revision::walk::Platform::selected(filter)` to selectively prune parts of the commit graph. ([`ec93f75`](https://github.com/Byron/gitoxide/commit/ec93f75cfdf6cbd617c4a92eefae97f2c7736d65))
 * **[#814](https://github.com/Byron/gitoxide/issues/814)**
    - Rename `serde1` cargo feature to `serde` and use the weak-deps cargo capability. ([`b83ee36`](https://github.com/Byron/gitoxide/commit/b83ee366a3c65c717beb587ad809268f1c54b8ad))
 * **Uncategorized**
    - Release gix-worktree v0.16.0, gix v0.44.0 ([`4527fb8`](https://github.com/Byron/gitoxide/commit/4527fb8e0fe0786a70ccfd8c3f5c5e79e8867944))
    - Release gix-index v0.16.0, gix-mailmap v0.12.0, gix-pack v0.34.0, gix-odb v0.44.0, gix-packetline v0.16.0, gix-transport v0.30.0, gix-protocol v0.31.0, gix-revision v0.13.0, gix-refspec v0.10.0, gix-worktree v0.16.0, gix v0.44.0 ([`d7173b2`](https://github.com/Byron/gitoxide/commit/d7173b2d2cb79685fdf7f618c31c576db24fa648))
    - Release gix-index v0.16.0, gix-mailmap v0.12.0, gix-pack v0.34.0, gix-odb v0.44.0, gix-packetline v0.16.0, gix-transport v0.30.0, gix-protocol v0.31.0, gix-revision v0.13.0, gix-refspec v0.10.0, gix-worktree v0.16.0, gix v0.44.0 ([`e4df557`](https://github.com/Byron/gitoxide/commit/e4df5574c0813a0236319fa6e8b3b41bab179fc8))
    - Release gix-hash v0.11.1, gix-path v0.7.4, gix-glob v0.6.0, gix-attributes v0.11.0, gix-config-value v0.11.0, gix-fs v0.1.1, gix-tempfile v5.0.3, gix-utils v0.1.1, gix-lock v5.0.1, gix-object v0.29.1, gix-ref v0.28.0, gix-sec v0.7.0, gix-config v0.21.0, gix-prompt v0.4.0, gix-url v0.17.0, gix-credentials v0.13.0, gix-diff v0.29.0, gix-discover v0.17.0, gix-hashtable v0.2.0, gix-ignore v0.1.0, gix-bitmap v0.2.3, gix-traverse v0.25.0, gix-index v0.16.0, gix-mailmap v0.12.0, gix-pack v0.34.0, gix-odb v0.44.0, gix-packetline v0.16.0, gix-transport v0.30.0, gix-protocol v0.31.0, gix-revision v0.13.0, gix-refspec v0.10.0, gix-worktree v0.16.0, gix v0.44.0, safety bump 7 crates ([`91134a1`](https://github.com/Byron/gitoxide/commit/91134a11c8ba0e942f692488ec9bce9fa1086324))
    - Prepare changelogs prior to release ([`30a1a71`](https://github.com/Byron/gitoxide/commit/30a1a71f36f24faac0e0b362ffdfedea7f9cdbf1))
    - When removing all shallow commits from shallow file, delete it. ([`2cd5054`](https://github.com/Byron/gitoxide/commit/2cd5054b0a1994571a25a49193449904cfd30b50))
    - Merge branch 'index-entries-attrs' ([`f37a930`](https://github.com/Byron/gitoxide/commit/f37a930aefa27e67f0b693ba9669cc26d49044fa))
    - `gix index entries` also prints attributes. ([`08e8fc2`](https://github.com/Byron/gitoxide/commit/08e8fc2152794652ba1c986df493c2ac915af9e7))
    - Adjust to changes in `gix-worktree` ([`27a39ca`](https://github.com/Byron/gitoxide/commit/27a39cad498ca8b2c9cba05790284e2b68ba7636))
    - Add `Repository::attributes()` and `Worktree::attributes()`. ([`bc28443`](https://github.com/Byron/gitoxide/commit/bc28443e452c4de81368739a11a2482ae0a93485))
    - Add `Repository::excludes()` and simplify signature of `Worktree::excludes()`. ([`26e6a66`](https://github.com/Byron/gitoxide/commit/26e6a661ed5827151708b9fcc3d7468aa60cf4e3))
    - Add `index.threads` configuration to `gix::config::tree` ([`40a1b74`](https://github.com/Byron/gitoxide/commit/40a1b7444ba9d9b61a1c22a7f25662eec3c25a1b))
    - Adjust to changes in `gix-worktree` ([`f722d6b`](https://github.com/Byron/gitoxide/commit/f722d6bebcd215b6e270261a3ed032a5f7e7b72f))
    - Merge branch 'attributes-cache' ([`3456c84`](https://github.com/Byron/gitoxide/commit/3456c845dfeedd2fa96b4313b1a84c8cbe9433c5))
    - Adjust to changes in `gix-worktree` ([`13a070f`](https://github.com/Byron/gitoxide/commit/13a070f405230d52e4377e18f6bdc5c673b718a0))
    - Merge branch 'fix-823' ([`6ebd61e`](https://github.com/Byron/gitoxide/commit/6ebd61e548a36a04e413ac725a03e607a3588334))
    - `gix::open()` can handle bare repositories with index. ([`43f695a`](https://github.com/Byron/gitoxide/commit/43f695a9607f1f85f859f2ef944b785b5b6dd238))
    - Thanks clippy ([`14e64e7`](https://github.com/Byron/gitoxide/commit/14e64e74649cfb1f2f99da87015939af98fae5c8))
    - Merge branch 'clone-auth' ([`1a65308`](https://github.com/Byron/gitoxide/commit/1a653083bf0a3a01ee116535e65202392a2c676c))
    - Support configuring the connection (i.e. for auth) during clone. ([`cb34376`](https://github.com/Byron/gitoxide/commit/cb3437632fe7ff0ce4efd11c08a8d684d7e7e430))
    - Merge branch 'fix-819' ([`69faad0`](https://github.com/Byron/gitoxide/commit/69faad0d7cc100de54d757d42acc152a22edc022))
    - Before writing new objects, check if they exist. ([`afe7faa`](https://github.com/Byron/gitoxide/commit/afe7faa14afb2ec4934f204e01ed12bcd0b3e786))
    - Add `Object::into_tag()` and `Tag::decode()` methods. ([`037f52d`](https://github.com/Byron/gitoxide/commit/037f52d4099e239c28210476ad7ab57d22aa3626))
    - Release gix-utils v0.1.0, gix-hash v0.11.0, gix-date v0.5.0, gix-features v0.29.0, gix-actor v0.20.0, gix-object v0.29.0, gix-archive v0.1.0, gix-fs v0.1.0, safety bump 25 crates ([`8dbd0a6`](https://github.com/Byron/gitoxide/commit/8dbd0a60557a85acfa231800a058cbac0271a8cf))
    - Support native zlib-ng via flate2's zlib-ng feature ([`9a6e0d7`](https://github.com/Byron/gitoxide/commit/9a6e0d7b418ea721da6a7e4bc48c47b47d4dfa79))
    - Make fmt ([`5d2b5d0`](https://github.com/Byron/gitoxide/commit/5d2b5d02c3869e07dc2507a8f2519ee1df633df7))
    - Merge branch 'main' into dev ([`cdef398`](https://github.com/Byron/gitoxide/commit/cdef398c4a3bd01baf0be2c27a3f77a400172b0d))
    - Minor adjustments to the worktree structure. ([`8920229`](https://github.com/Byron/gitoxide/commit/89202296f63dacedfd396aefe25e686b4d426b2a))
    - Rename the serde1 feature to serde ([`19338d9`](https://github.com/Byron/gitoxide/commit/19338d934b6712b7d6bd3fa3b2e4189bf7e6c8a1))
    - Create new `gix-fs` crate to consolidate all filesystem utilities ([`f8cc33c`](https://github.com/Byron/gitoxide/commit/f8cc33cb372dd2b4bbe4a09cf4f64916681ab1dd))
    - Allow `USE_NSEC` and `USE_STDEV` compile time flags to configured at runtime. ([`35cb6b4`](https://github.com/Byron/gitoxide/commit/35cb6b42bd8071e5e5c16ed6d37884deea524330))
    - Merge branch 'main' into dev ([`23ee47f`](https://github.com/Byron/gitoxide/commit/23ee47fb24c197f8437bd426544b2aa74e005bdc))
    - Merge branch 'worktree-stack' ([`3d47919`](https://github.com/Byron/gitoxide/commit/3d47919c1a2f83fc7c1fd7ae590d098057a22626))
    - `open::Options` now allow controlling where gitattributes files are loaded from. ([`358500f`](https://github.com/Byron/gitoxide/commit/358500f0efaec7c67b307a6a1aa27ecad7502eb7))
    - Adjust to changes in `gix-attributes` ([`1755c81`](https://github.com/Byron/gitoxide/commit/1755c81f64ce8a68807c2026eeae13dc46021db1))
    - Remove types that are now available in `gix-os` ([`b645d28`](https://github.com/Byron/gitoxide/commit/b645d28f9641c6b4022e1e37ad9fe528922ec747))
    - Refactor ([`0677406`](https://github.com/Byron/gitoxide/commit/067740636b3ca24ce90db91923dfd4ee592fa7f6))
    - Centralize index entry Stat creation/comparison ([`870bdb2`](https://github.com/Byron/gitoxide/commit/870bdb2f3957e0f5690679e2aeb6752cd0b8d93e))
    - Release gix-hash v0.10.4, gix-hashtable v0.1.3 ([`b574a39`](https://github.com/Byron/gitoxide/commit/b574a3904203762a6b9e475e16a7c358d7616599))
    - Merge branch 'patch-1' ([`b02bf24`](https://github.com/Byron/gitoxide/commit/b02bf247890c873184e58f734e0912eac6c6bbae))
    - Add test to run tests on 32 bit systems ([`fb31ee8`](https://github.com/Byron/gitoxide/commit/fb31ee8bbcfc72fa0e7e38bc84d02f6f7d2f0fff))
    - Merge branch 'patch-1' ([`d0052c1`](https://github.com/Byron/gitoxide/commit/d0052c13cabcde8058177d2439053b50ea5adbfc))
    - Upgrade serial-test to v2 ([`6932017`](https://github.com/Byron/gitoxide/commit/69320174685e72940cd0fe600c94abb948a62bdd))
    - Release gix-revision v0.12.2 ([`ec64a88`](https://github.com/Byron/gitoxide/commit/ec64a88690243a210efee6d5ae5164723e13f734))
    - Merge branch 'fix-801' ([`a884121`](https://github.com/Byron/gitoxide/commit/a88412194ff8960cd69a3794042d9c6c29428ea6))
    - Prevent env-altering tests to affect shallow tests ([`61eec5a`](https://github.com/Byron/gitoxide/commit/61eec5ae48006b4f0a6ac5c7b9549811dfa9431d))
</details>

## 0.43.1 (2023-03-30)

### Documentation

 - <csr-id-02c4659984fa6423bc76cc4980a143edaba8ace0/> fix minor typos
 - <csr-id-cc48c35d0ecf35824910c5b6ecc62fe9b2aff1b5/> fix minor typos

### New Features

 - <csr-id-7c2e5c8d08e4dd1ec115ae06f20f9c8f93d6d616/> add `Tree::decode()` and `TryFrom<Tree> for gix::objs::Tree`.
   This makes it possible to obtain mutable trees for creating trees by hand
   for the purpose of making commits.

### Bug Fixes

 - <csr-id-d1bd513f27e17787eb223f7b0521f954c518153e/> $HOME detection on windows

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 3 calendar days.
 - 4 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-path v0.7.3, gix-config-value v0.10.2, gix-config v0.20.1, gix-discover v0.16.2, gix-index v0.15.1, gix-odb v0.43.1, gix-packetline v0.15.1, gix-protocol v0.30.2, gix-worktree v0.15.2, gix v0.43.1 ([`38eed1d`](https://github.com/Byron/gitoxide/commit/38eed1d06e7cbb8fbcd54b2cad3163ca45e0baf1))
    - Merge branch 'pascalkuthe/main' ([`d47cebe`](https://github.com/Byron/gitoxide/commit/d47cebe3b23080c45829cb307b867220e3af20db))
    - Refactor ([`d1e5e12`](https://github.com/Byron/gitoxide/commit/d1e5e12d54f79c030325860838c1cfadac1a7ac5))
    - $HOME detection on windows ([`d1bd513`](https://github.com/Byron/gitoxide/commit/d1bd513f27e17787eb223f7b0521f954c518153e))
    - Fix minor typos ([`02c4659`](https://github.com/Byron/gitoxide/commit/02c4659984fa6423bc76cc4980a143edaba8ace0))
    - Fix minor typos ([`cc48c35`](https://github.com/Byron/gitoxide/commit/cc48c35d0ecf35824910c5b6ecc62fe9b2aff1b5))
    - Add `Tree::decode()` and `TryFrom<Tree> for gix::objs::Tree`. ([`7c2e5c8`](https://github.com/Byron/gitoxide/commit/7c2e5c8d08e4dd1ec115ae06f20f9c8f93d6d616))
    - Release gix-ref v0.27.2 ([`e965b18`](https://github.com/Byron/gitoxide/commit/e965b18aedcf13ec4538bc7bc700269a56ca615e))
    - Be sure to clear the buffer after an intermediate read error happened and we ignore it. ([`877951a`](https://github.com/Byron/gitoxide/commit/877951aa0009ab5e2a814c95f4c5d3662305cb27))
</details>

## 0.43.0 (2023-03-26)

<csr-id-87f5621d941b5af40abd59a26164a09d0dde2649/>

### Bug Fixes

 - <csr-id-7bd8823ab4241d6d0401f03aec8c0d34f68c347c/> opening repositories without 'strict' mode also ignores IO errors.
   These will instead be logged, but won't make it impossible to open an
   otherwise fine repository.

### Other

 - <csr-id-87f5621d941b5af40abd59a26164a09d0dde2649/> make clear that `gix::discover()` isn't suited for authentication remote operations.
   We also provide information on how to accomplish this.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 5 calendar days.
 - 11 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#787](https://github.com/Byron/gitoxide/issues/787), [#790](https://github.com/Byron/gitoxide/issues/790)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#787](https://github.com/Byron/gitoxide/issues/787)**
    - Make clear that `gix::discover()` isn't suited for authentication remote operations. ([`87f5621`](https://github.com/Byron/gitoxide/commit/87f5621d941b5af40abd59a26164a09d0dde2649))
 * **[#790](https://github.com/Byron/gitoxide/issues/790)**
    - Opening repositories without 'strict' mode also ignores IO errors. ([`7bd8823`](https://github.com/Byron/gitoxide/commit/7bd8823ab4241d6d0401f03aec8c0d34f68c347c))
 * **Uncategorized**
    - Release gix-tempfile v5.0.2, gix-validate v0.7.4, gix-config v0.20.0, gix-prompt v0.3.3, gix-diff v0.28.1, gix-discover v0.16.1, gix-pack v0.33.2, gix-transport v0.29.1, gix-protocol v0.30.1, gix-revision v0.12.1, gix-worktree v0.15.1, gix v0.43.0, safety bump gix v0.43.0 ([`5dc1f9f`](https://github.com/Byron/gitoxide/commit/5dc1f9f2bcb8b3e147115fcb6f76558e8f48ffef))
    - Prepare changelogs prior to release ([`3016a28`](https://github.com/Byron/gitoxide/commit/3016a285f566bdfe7de2774fa6f2254c1b1a2c51))
    - Merge branch 'fix-790' ([`ee36e5b`](https://github.com/Byron/gitoxide/commit/ee36e5bb985e9ad90bc382cdd051a8b5295ca18c))
    - Less dependencies for tests (via `serial_test` no default features) ([`8f2accd`](https://github.com/Byron/gitoxide/commit/8f2accdf738def9aa4abdf08fc299d0e9807bc3e))
</details>

## 0.42.0 (2023-03-14)

### New Features

 - <csr-id-93d412c54833d822e5369644226c6fd3b888c89c/> shallow support for `fetch` operations.
 - <csr-id-4e89c19d7656a96bd512dafbc9669011487671f5/> shallow support for `clone` operations.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-features v0.28.1, gix-tempfile v5.0.1, gix-ref v0.27.1, gix-pack v0.33.1, gix-packetline v0.15.0, gix-transport v0.29.0, gix-protocol v0.30.0, gix v0.42.0, safety bump 3 crates ([`c1f1bfb`](https://github.com/Byron/gitoxide/commit/c1f1bfb8dc0e73993678353e4492d0614b642ed1))
    - Prepare changelogs prior to release ([`c66e298`](https://github.com/Byron/gitoxide/commit/c66e2982577e4cd9faef63798986b8cf8ece93a2))
    - Make fmt ([`3836cc0`](https://github.com/Byron/gitoxide/commit/3836cc0c9c3e1158b56142b924483c8a77217d53))
    - Merge branch 'various-fixes' ([`cc0f506`](https://github.com/Byron/gitoxide/commit/cc0f5061fba27d57022dc616c941034b98fd4875))
    - Improve fetchspec handling to be closer to what git does. ([`a22621d`](https://github.com/Byron/gitoxide/commit/a22621d1b92a6155b83a09e68ed1de3a4860e766))
    - Assure that --deepen 0 (despite allowed) doesn't actually confuse the server. ([`b43ea6b`](https://github.com/Byron/gitoxide/commit/b43ea6bdc873da2facdb0fe8369ab1644a6702ef))
    - Adjust to changes in `gix-packetline` ([`4f45814`](https://github.com/Byron/gitoxide/commit/4f45814eea9c20b449effd9b29d31623943ff853))
    - Merge branch 'shallow-protocol' ([`531dd19`](https://github.com/Byron/gitoxide/commit/531dd19502b8b635fb1a60f747eb381fd12e00ca))
    - Shallow support for `fetch` operations. ([`93d412c`](https://github.com/Byron/gitoxide/commit/93d412c54833d822e5369644226c6fd3b888c89c))
    - Shallow support for `clone` operations. ([`4e89c19`](https://github.com/Byron/gitoxide/commit/4e89c19d7656a96bd512dafbc9669011487671f5))
    - Merge branch 'fix-cred-helper' ([`01277a6`](https://github.com/Byron/gitoxide/commit/01277a681e4997896e04567490c572b5af606f35))
</details>

## 0.41.0 (2023-03-10)

A maintenance release without user-facing changes, but with some fixes in the dependency chain, namely:

- `gix-credentials` allows credential helpers to ignore `stdin`, making it robust when facing helpers that don't read from `stdin`.
- `gix-tempfile` refers to the most recent version of `tempfile` without pinning it, which removes a security vulnerability.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 1 day passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-tempfile v5.0.0, gix-lock v5.0.0, gix-ref v0.27.0, gix-config v0.19.0, gix-url v0.16.0, gix-credentials v0.12.0, gix-discover v0.16.0, gix-index v0.15.0, gix-pack v0.33.0, gix-odb v0.43.0, gix-transport v0.28.0, gix-protocol v0.29.0, gix-worktree v0.15.0, gix v0.41.0, safety bump 12 crates ([`29a0870`](https://github.com/Byron/gitoxide/commit/29a087043d1feb2f127b065341c8028d0bd0301e))
    - Prepare changelogs prior to release ([`e06f5f5`](https://github.com/Byron/gitoxide/commit/e06f5f523e83f4da390eddbebcb9a2d58674587b))
    - Merge branch 'password-in-urls' ([`85f8b28`](https://github.com/Byron/gitoxide/commit/85f8b283a1671e2631cda437ca8da93f9a2a4ebd))
    - Adjust to changes in `gix-url` ([`66602bb`](https://github.com/Byron/gitoxide/commit/66602bbb7fe62f7425c8289902a1d2fce121e87c))
</details>

## 0.40.0 (2023-03-09)

### New Features

 - <csr-id-5bfbb9a32f8edb8bfb71ae00167277b9109de35a/> `Repository::shallow_commits()` returns an uptodate list of shallow boundary commits.
 - <csr-id-3e69535630714205904fe64f511da28a3f2d7fb6/> `Repository::is_shallow()` to test if a repository is shallow.

### Bug Fixes (BREAKING)

 - <csr-id-1046ea2b3312838169aa08f30b598bf4ce2338d9/> allow to traverse the entire commit graph of shallow repos
   Previously, when traversing commits, we would assume to be in a
   shallow repository if a commit's parent could not be found in the
   repository.
   
   Now we validate that assumption by reading the 'shallow' file to
   check if the last seen commit is on the commit boundary.
   
   This removes `is_shallow` and `error_on_missing_commit()` on the
   `revision::walk::Platform` as shallow commits are now known and handled
   without any guesswork.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 3 calendar days.
 - 4 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix v0.40.0 ([`18e72c9`](https://github.com/Byron/gitoxide/commit/18e72c988a58415080d4555bc869ae04df8d04fa))
    - Merge branch 'shallow-support' ([`6d88fd2`](https://github.com/Byron/gitoxide/commit/6d88fd208bcdec0603d57785bdbfe2f286a65384))
    - Allow to traverse the entire commit graph of shallow repos ([`1046ea2`](https://github.com/Byron/gitoxide/commit/1046ea2b3312838169aa08f30b598bf4ce2338d9))
    - `Repository::shallow_commits()` returns an uptodate list of shallow boundary commits. ([`5bfbb9a`](https://github.com/Byron/gitoxide/commit/5bfbb9a32f8edb8bfb71ae00167277b9109de35a))
    - `Repository::is_shallow()` to test if a repository is shallow. ([`3e69535`](https://github.com/Byron/gitoxide/commit/3e69535630714205904fe64f511da28a3f2d7fb6))
</details>

## 0.39.0 (2023-03-04)

A maintenance release without user-facing changes, primarily for getting the progress-bar updates into `cargo`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-attributes v0.10.0, gix-ref v0.26.0, gix-config v0.18.0, gix-url v0.15.0, gix-credentials v0.11.0, gix-discover v0.15.0, gix-index v0.14.0, gix-mailmap v0.11.0, gix-odb v0.42.0, gix-transport v0.27.0, gix-protocol v0.28.0, gix-revision v0.12.0, gix-refspec v0.9.0, gix-worktree v0.14.0, gix v0.39.0 ([`93e75fe`](https://github.com/Byron/gitoxide/commit/93e75fed454ed8b342231bde4638db90e407ce52))
    - Prepare changelogs prior to release ([`895e482`](https://github.com/Byron/gitoxide/commit/895e482badf01e953bb9144001eebd5e1b1c4d84))
    - Release gix-features v0.28.0, gix-actor v0.19.0, gix-object v0.28.0, gix-diff v0.28.0, gix-traverse v0.24.0, gix-pack v0.32.0, safety bump 20 crates ([`0f411e9`](https://github.com/Byron/gitoxide/commit/0f411e93ec812592bb9d3a52b751399dd86f76f7))
</details>

## 0.38.0 (2023-03-01)

### New Features

 - <csr-id-256f7d46ed88067aa96f47be2a97a6f9f5b98075/> the `hp-tempfile-registry` feature toggle to control the `dashmap` dependency.
   And also, probably provide a better performance in certain cases.
 - <csr-id-fd7eebcd922f98c1aed9e3177b9a48ff1415ffd8/> make `gix-pack` feature toggles related to pack caches available.
   Previously they would have to be configured by pulling in `gix-pack`, which
   isn't desirable as the only crate we want to expose like that is `gix-features`.
 - <csr-id-5b0ebd272c3d98e26c9249ed27b4ea9a8ad80746/> Add `comfort` feature toggle (default enabled) to make better progress units available.
   This could be a breaking change for those who turned default-features off, as you may now
   have to re-add the `comfort` feature to get nicer progress messages.

### Bug Fixes

 - <csr-id-b2375e3dbe1f87ee3ac6e814fc8f4898143c438d/> `gix-tempfile` is now configured to not use the high-performance hashmap anymore.
   It was hard to justify as tests actually seemed to be faster without it.

### New Features (BREAKING)

 - <csr-id-fea8c56089e5b354669396853c5bd0f31bdf0d33/> Put `progress::tree` behind the `progress-tree` feature toggle.
   It's a convenience export that implies pulling in more dependencies, so it
   should be gated.
 - <csr-id-441f5ac4dd2f0636ec07065f8095e8bae5ce6985/> gate all signal handling behind the `signals` feature toggle.
   This change also consolidates all signal handling into its own module called
   `signal` to provide reusable handlers and as well as well as signal initialization.
   
   Note that the functions to cleanup tempfiles don't interact with the signal registry,
   hence they still can be called without the `signals` feature enabled.
   
   Note that this change sneakily fixes a bug that could have caused a `write_all()`
   on a tempfile that was removed by a signal to enter an infinite loop.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 17 commits contributed to the release over the course of 4 calendar days.
 - 4 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#339](https://github.com/Byron/gitoxide/issues/339)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#339](https://github.com/Byron/gitoxide/issues/339)**
    - Gate all signal handling behind the `signals` feature toggle. ([`441f5ac`](https://github.com/Byron/gitoxide/commit/441f5ac4dd2f0636ec07065f8095e8bae5ce6985))
 * **Uncategorized**
    - Release gix-tempfile v4.1.0, gix-lock v4.0.0, gix-ref v0.25.0, gix-config v0.17.0, gix-url v0.14.0, gix-credentials v0.10.0, gix-diff v0.27.0, gix-discover v0.14.0, gix-hashtable v0.1.2, gix-bitmap v0.2.2, gix-traverse v0.23.0, gix-index v0.13.0, gix-mailmap v0.10.0, gix-pack v0.31.0, gix-odb v0.41.0, gix-transport v0.26.0, gix-protocol v0.27.0, gix-revision v0.11.0, gix-refspec v0.8.0, gix-worktree v0.13.0, gix v0.38.0, safety bump 6 crates ([`ea9fd1d`](https://github.com/Byron/gitoxide/commit/ea9fd1d9b60e1e9e17042e9e37c06525823c40a5))
    - Release gix-features v0.27.0, gix-actor v0.18.0, gix-quote v0.4.3, gix-attributes v0.9.0, gix-object v0.27.0, gix-ref v0.25.0, gix-config v0.17.0, gix-url v0.14.0, gix-credentials v0.10.0, gix-diff v0.27.0, gix-discover v0.14.0, gix-hashtable v0.1.2, gix-bitmap v0.2.2, gix-traverse v0.23.0, gix-index v0.13.0, gix-mailmap v0.10.0, gix-pack v0.31.0, gix-odb v0.41.0, gix-transport v0.26.0, gix-protocol v0.27.0, gix-revision v0.11.0, gix-refspec v0.8.0, gix-worktree v0.13.0, gix v0.38.0 ([`e6cc618`](https://github.com/Byron/gitoxide/commit/e6cc6184a7a49dbc2503c1c1bdd3688ca5cec5fe))
    - Adjust manifests prior to release ([`addd789`](https://github.com/Byron/gitoxide/commit/addd78958fdd1e54eb702854e96079539d01965a))
    - Prepare changelogs prior to release ([`94c99c7`](https://github.com/Byron/gitoxide/commit/94c99c71520f33269cc8dbc26f82a74747cc7e16))
    - Merge branch 'adjustments-for-cargo' ([`d686d94`](https://github.com/Byron/gitoxide/commit/d686d94e1030a8591ba074757d56927a346c8351))
    - `gix-tempfile` is now configured to not use the high-performance hashmap anymore. ([`b2375e3`](https://github.com/Byron/gitoxide/commit/b2375e3dbe1f87ee3ac6e814fc8f4898143c438d))
    - Depend on latest version of `prodash` for performance improvements. ([`5d00324`](https://github.com/Byron/gitoxide/commit/5d003242abe82b1604e2188d49dec9690ebb2a6a))
    - The `hp-tempfile-registry` feature toggle to control the `dashmap` dependency. ([`256f7d4`](https://github.com/Byron/gitoxide/commit/256f7d46ed88067aa96f47be2a97a6f9f5b98075))
    - Make `gix-pack` feature toggles related to pack caches available. ([`fd7eebc`](https://github.com/Byron/gitoxide/commit/fd7eebcd922f98c1aed9e3177b9a48ff1415ffd8))
    - Put `progress::tree` behind the `progress-tree` feature toggle. ([`fea8c56`](https://github.com/Byron/gitoxide/commit/fea8c56089e5b354669396853c5bd0f31bdf0d33))
    - Add `comfort` feature toggle (default enabled) to make better progress units available. ([`5b0ebd2`](https://github.com/Byron/gitoxide/commit/5b0ebd272c3d98e26c9249ed27b4ea9a8ad80746))
    - Prepare for git-tempfile release ([`56c005b`](https://github.com/Byron/gitoxide/commit/56c005b13c44376f71e61781e73c0bf93416d0e4))
    - Merge branch 'tempfile-upgrades' ([`3522cba`](https://github.com/Byron/gitoxide/commit/3522cbaac721c8079605be51b9053014bc5e863a))
    - Adjust to changes in `gix-tempfile` ([`c6785fc`](https://github.com/Byron/gitoxide/commit/c6785fc7082b90c8a27cef6a0f5cc5acd8cb8951))
    - Make fmt ([`8ef1cb2`](https://github.com/Byron/gitoxide/commit/8ef1cb293434c7b9e1fda4a6963368e0435920a9))
    - Fix diff-tests on windows ([`441a64b`](https://github.com/Byron/gitoxide/commit/441a64b6b703f7f97cfcefe4d3db31bc7427b48c))
</details>

## 0.37.2 (2023-02-24)

### Bug Fixes

 - <csr-id-1d3d22d45e70222c12fcf5a82063ceb9321a0129/> reproduce a diff issue and fix it
   Diffs could be quite wrong and this is a small repro along with the fix.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-object v0.26.4, gix-diff v0.26.3, gix v0.37.2, gix-commitgraph v0.13.1, gitoxide-core v0.25.0, gitoxide v0.23.0 ([`9982949`](https://github.com/Byron/gitoxide/commit/9982949cab401501d5ce3cba4e2ba900bc249c53))
    - Fix new diff tests on windows ([`b1ec1b7`](https://github.com/Byron/gitoxide/commit/b1ec1b776696b4b1d73e3dd26cbaf33260367855))
    - Prepare changelog for release ([`13a1ec1`](https://github.com/Byron/gitoxide/commit/13a1ec1803d677c2e94f3ea0461118c2426f8071))
    - Merge branch 'rename-tracking' ([`550144a`](https://github.com/Byron/gitoxide/commit/550144a5fd37d501d86f4b1c4db2948d951d1c93))
    - Reproduce a diff issue and fix it ([`1d3d22d`](https://github.com/Byron/gitoxide/commit/1d3d22d45e70222c12fcf5a82063ceb9321a0129))
</details>

## 0.37.1 (2023-02-21)

A maintenance release to restore MSRV (1.64) support.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-config v0.16.3, gix v0.37.1 ([`a3c283f`](https://github.com/Byron/gitoxide/commit/a3c283ff0e3f21cedb3ba7cd464fdfa0f5133af0))
    - Prepare changelogs prior to release ([`362d659`](https://github.com/Byron/gitoxide/commit/362d659f946ca1ff2cbf915766113a34a9df97b3))
    - Restore msrv compatibility by removing sole `if let ... else` ([`9160659`](https://github.com/Byron/gitoxide/commit/91606597b714a6e9b3a2c071bdb08baeacd6056b))
</details>

## 0.37.0 (2023-02-20)

### Bug Fixes

 - <csr-id-d3b974000133caa0ea107cb4724b950eda91d69b/> `Repository::object_cache_size()` now unsets the cache if `Some(0)` is passed.
   Previously it would fail.

### New Features (BREAKING)

 - <csr-id-ed87f4c7c2799625bc6c7109368687908f0fb6f0/> `object::tree::diff::Platform::track_rewrites(...)`
   The invocation of `object::tree::diff::Platform::track_rewrites(Rewrites { percentage: None, ..Default::default() })`
   is now able to explicitly configure perfect rename tracking without percentage of equivalence.
   
   By setting `percentage = Some(<fraction>)` one can set how similar both files should be to be considered related.
   
   The same can be configured for copy-tracking, which also includes something like `--find-copies-harder`.
   
   Note that by default, renames are considered if a file looks 50% similar, and copies tracking is
   using the same convention.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-object v0.26.3, gix-diff v0.26.2, gix-traverse v0.22.2, gix v0.37.0, safety bump 3 crates ([`8b3e42f`](https://github.com/Byron/gitoxide/commit/8b3e42f69fe97fe5083eb845c323f10d7ac087b2))
    - `Repository::object_cache_size()` now unsets the cache if `Some(0)` is passed. ([`d3b9740`](https://github.com/Byron/gitoxide/commit/d3b974000133caa0ea107cb4724b950eda91d69b))
    - Merge branch 'rename-tracking' ([`35415c5`](https://github.com/Byron/gitoxide/commit/35415c5061bf5ea90a04db80d06ac3622d0b0f1a))
    - `object::tree::diff::Platform::track_rewrites(...)` ([`ed87f4c`](https://github.com/Byron/gitoxide/commit/ed87f4c7c2799625bc6c7109368687908f0fb6f0))
</details>

## 0.36.1 (2023-02-20)

### Bug Fixes

 - <csr-id-135d317065aae87af302beb6c26bb6ca8e30b6aa/> compatibility with `bstr` v1.3, use `*.as_bytes()` instead of `.as_ref()`.
   `as_ref()` relies on a known target type which isn't always present. However, once
   there is only one implementation, that's no problem, but when that changes compilation
   fails due to ambiguity.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 2 calendar days.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix v0.36.1 ([`fac6bce`](https://github.com/Byron/gitoxide/commit/fac6bce2f9942d7f333f66a92374d5400a00b0a5))
    - Release gix-date v0.4.3, gix-hash v0.10.3, gix-features v0.26.5, gix-actor v0.17.2, gix-glob v0.5.5, gix-path v0.7.2, gix-quote v0.4.2, gix-attributes v0.8.3, gix-validate v0.7.3, gix-object v0.26.2, gix-ref v0.24.1, gix-config v0.16.2, gix-command v0.2.4, gix-url v0.13.3, gix-credentials v0.9.2, gix-discover v0.13.1, gix-index v0.12.4, gix-mailmap v0.9.3, gix-pack v0.30.3, gix-packetline v0.14.3, gix-transport v0.25.6, gix-protocol v0.26.4, gix-revision v0.10.4, gix-refspec v0.7.3, gix-worktree v0.12.3, gix v0.36.1 ([`9604783`](https://github.com/Byron/gitoxide/commit/96047839a20a657a559376b0b14c65aeab96acbd))
    - Compatibility with `bstr` v1.3, use `*.as_bytes()` instead of `.as_ref()`. ([`135d317`](https://github.com/Byron/gitoxide/commit/135d317065aae87af302beb6c26bb6ca8e30b6aa))
    - Release gix-glob v0.5.4 ([`c56d336`](https://github.com/Byron/gitoxide/commit/c56d3365fde21120cf6101cf34f8b5669804977c))
    - Release gix-transport v0.25.5 ([`f872ba8`](https://github.com/Byron/gitoxide/commit/f872ba8271a5d632acc071e7a857ef19f7cf5610))
</details>

## 0.36.0 (2023-02-17)

### New Features

 - <csr-id-4f49992fae2bc60b22644e86808d61afe557f192/> cloning repositories doesn't require a committer anymore.
   This is similar to what git does and probably a decent convenience to have.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 45 commits contributed to the release over the course of 3 calendar days.
 - 4 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#737](https://github.com/Byron/gitoxide/issues/737)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#737](https://github.com/Byron/gitoxide/issues/737)**
    - Cloning repositories doesn't require a committer anymore. ([`4f49992`](https://github.com/Byron/gitoxide/commit/4f49992fae2bc60b22644e86808d61afe557f192))
 * **Uncategorized**
    - Release gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`7fc00f8`](https://github.com/Byron/gitoxide/commit/7fc00f87d74aedf631ce4032be1cdfe1804c7e7d))
    - Release gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`59e9fac`](https://github.com/Byron/gitoxide/commit/59e9fac67d1b353e124300435b55f6b5468d7deb))
    - Release gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`48f5bd2`](https://github.com/Byron/gitoxide/commit/48f5bd2014fa3dda6fbd60d091065c5537f69453))
    - Release gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`a5869e0`](https://github.com/Byron/gitoxide/commit/a5869e0b223406820bca836e3e3a7fae2bfd9b04))
    - Release gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`41d57b9`](https://github.com/Byron/gitoxide/commit/41d57b98964094fc1528adb09f69ca824229bf25))
    - Release gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`e313112`](https://github.com/Byron/gitoxide/commit/e31311257bd138b52042dea5fc40c3abab7f269b))
    - Release gix-features v0.26.4, gix-actor v0.17.1, gix-glob v0.5.3, gix-path v0.7.1, gix-quote v0.4.1, gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`6efd0d3`](https://github.com/Byron/gitoxide/commit/6efd0d31fbeca31ab7319aa2ac97bb31dc4ce055))
    - Release gix-date v0.4.2, gix-hash v0.10.2, gix-features v0.26.4, gix-actor v0.17.1, gix-glob v0.5.3, gix-path v0.7.1, gix-quote v0.4.1, gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`6ccc88a`](https://github.com/Byron/gitoxide/commit/6ccc88a8e4a56973b1a358cf72dc012ee3c75d56))
    - Merge branch 'rename-crates' into inform-about-gix-rename ([`c9275b9`](https://github.com/Byron/gitoxide/commit/c9275b99ea43949306d93775d9d78c98fb86cfb1))
    - Release git-date v0.4.3, git-hash v0.10.3, git-features v0.26.5, git-actor v0.17.2, git-glob v0.5.4, git-path v0.7.2, git-quote v0.4.2, git-attributes v0.8.3, git-bitmap v0.2.2, git-chunk v0.4.2, git-command v0.2.4, git-commitgraph v0.13.1, git-config-value v0.10.2, git-tempfile v3.0.3, git-lock v3.0.3, git-validate v0.7.3, git-object v0.26.2, git-ref v0.24.1, git-sec v0.6.3, git-config v0.16.2, git-prompt v0.3.3, git-url v0.13.3, git-credentials v0.9.2, git-diff v0.26.2, git-discover v0.13.1, git-fetchhead v0.1.0, git-filter v0.1.0, git-hashtable v0.1.2, git-traverse v0.22.2, git-index v0.12.4, git-lfs v0.1.0, git-mailmap v0.9.3, git-note v0.1.0, git-pack v0.31.0, git-odb v0.41.0, git-packetline v0.14.3, git-pathspec v0.1.0, git-transport v0.25.5, git-protocol v0.26.4, git-rebase v0.1.0, git-revision v0.10.4, git-refspec v0.7.3, git-sequencer v0.1.0, git-submodule v0.1.0, git-tix v0.1.0, git-tui v0.1.0, git-worktree v0.12.3, safety bump 2 crates ([`90035a3`](https://github.com/Byron/gitoxide/commit/90035a332d0ba67584558db3605500fbcb424ddd))
    - Rename `git-testtools` to `gix-testtools` ([`b65c33d`](https://github.com/Byron/gitoxide/commit/b65c33d256cfed65d11adeff41132e3e58754089))
    - Adjust to renaming of `git-pack` to `gix-pack` ([`1ee81ad`](https://github.com/Byron/gitoxide/commit/1ee81ad310285ee4aa118118a2be3810dbace574))
    - Adjust to renaming of `git-odb` to `gix-odb` ([`476e2ad`](https://github.com/Byron/gitoxide/commit/476e2ad1a64e9e3f0d7c8651d5bcbee36cd78241))
    - Adjust to renaming of `git-index` to `gix-index` ([`86db5e0`](https://github.com/Byron/gitoxide/commit/86db5e09fc58ce66b252dc13b8d7e2c48e4d5062))
    - Adjust to renaming of `git-diff` to `gix-diff` ([`49a163e`](https://github.com/Byron/gitoxide/commit/49a163ec8b18f0e5fcd05a315de16d5d8be7650e))
    - Adjust to renaming of `git-mailmap` to `gix-mailmap` ([`2e28c56`](https://github.com/Byron/gitoxide/commit/2e28c56bb9f70de6f97439818118d3a25859698f))
    - Adjust to renaming of `git-discover` to `gix-discover` ([`53adfe1`](https://github.com/Byron/gitoxide/commit/53adfe1c34e9ea3b27067a97b5e7ac80b351c441))
    - Adjust to renaming for `git-protocol` to `gix-protocol` ([`823795a`](https://github.com/Byron/gitoxide/commit/823795addea3810243cab7936cd8ec0137cbc224))
    - Adjust to renaming of `git-refspec` to `gix-refspec` ([`c958802`](https://github.com/Byron/gitoxide/commit/c9588020561577736faa065e7e5b5bb486ca8fe1))
    - Adjust to renaming of `git-revision` to `gix-revision` ([`ee0ee84`](https://github.com/Byron/gitoxide/commit/ee0ee84607c2ffe11ee75f27a31903db68afed02))
    - Adjust to renaming of `git-transport` to `gix-transport` ([`b2ccf71`](https://github.com/Byron/gitoxide/commit/b2ccf716dc4425bb96651d4d58806a3cc2da219e))
    - Adjust to renaming of `git-credentials` to `gix-credentials` ([`6b18abc`](https://github.com/Byron/gitoxide/commit/6b18abcf2856f02ab938d535a65e51ac282bf94a))
    - Adjust to renaming of `git-prompt` to `gix-prompt` ([`6a4654e`](https://github.com/Byron/gitoxide/commit/6a4654e0d10ab773dd219cb4b731c0fc1471c36d))
    - Adjust to renaming of `git-worktree` to `gix-worktree` ([`73a1282`](https://github.com/Byron/gitoxide/commit/73a12821b3d9b66ec1714d07dd27eb7a73e3a544))
    - Adjust to renamining of `git-hashtable` to `gix-hashtable` ([`26a0c98`](https://github.com/Byron/gitoxide/commit/26a0c98d0a389b03e3dc7bfc758b37155e285244))
    - Adjust to renamining of `git-worktree` to `gix-worktree` ([`108bb1a`](https://github.com/Byron/gitoxide/commit/108bb1a634f4828853fb590e9fc125f79441dd38))
    - Adjust to renaming of `git-url` to `gix-url` ([`b50817a`](https://github.com/Byron/gitoxide/commit/b50817aadb143e19f61f64e19b19ec1107d980c6))
    - Adjust to renaming of `git-date` to `gix-date` ([`9a79ff2`](https://github.com/Byron/gitoxide/commit/9a79ff2d5cc74c1efad9f41e21095ae498cce00b))
    - Adjust to renamining of `git-attributes` to `gix-attributes` ([`4a8b3b8`](https://github.com/Byron/gitoxide/commit/4a8b3b812ac26f2a2aee8ce8ca81591273383c84))
    - Adjust to renaming of `git-config` to `gix-config` ([`3a861c8`](https://github.com/Byron/gitoxide/commit/3a861c8f049f6502d3bcbdac752659aa1aeda46a))
    - Adjust to renaming of `git-ref` to `gix-ref` ([`1f5f695`](https://github.com/Byron/gitoxide/commit/1f5f695407b034377d94b172465ff573562b3fc3))
    - Adjust to renaming of `git-lock` to `gix-lock` ([`2028e78`](https://github.com/Byron/gitoxide/commit/2028e7884ae1821edeec81612f501e88e4722b17))
    - Adjust to renaming of `git-tempfile` to `gix-tempfile` ([`b6cc3eb`](https://github.com/Byron/gitoxide/commit/b6cc3ebb5137084a6327af16a7d9364d8f092cc9))
    - Adjust to renaming of `git-object` to `gix-object` ([`fc86a1e`](https://github.com/Byron/gitoxide/commit/fc86a1e710ad7bf076c25cc6f028ddcf1a5a4311))
    - Adjust to renaming of `git-actor` to `gix-actor` ([`4dc9b44`](https://github.com/Byron/gitoxide/commit/4dc9b44dc52f2486ffa2040585c6897c1bf55df4))
    - Adjust to renaming of `git-validate` to `gix-validate` ([`5e40ad0`](https://github.com/Byron/gitoxide/commit/5e40ad078af3d08cbc2ca81ce755c0ed8a065b4f))
    - Adjust to renaming of `git-hash` to `gix-hash` ([`4a9d025`](https://github.com/Byron/gitoxide/commit/4a9d0257110c3efa61d08c8457c4545b200226d1))
    - Adjust to renaming of `git-features` to `gix-features` ([`e2dd68a`](https://github.com/Byron/gitoxide/commit/e2dd68a417aad229e194ff20dbbfd77668096ec6))
    - Adjust to renaming of `git-glob` to `gix-glob` ([`35b2a3a`](https://github.com/Byron/gitoxide/commit/35b2a3acbc8f2a03f151bc0a3863163844e0ca86))
    - Adjust to renaming of `git-sec` to `gix-sec` ([`eabbb92`](https://github.com/Byron/gitoxide/commit/eabbb923bd5a32fc80fa80f96cfdc2ab7bb2ed17))
    - Adapt to renaming of `git-path` to `gix-path` ([`d3bbcfc`](https://github.com/Byron/gitoxide/commit/d3bbcfccad80fc44ea8e7bf819f23adaca06ba2d))
    - Fix `gix` changelog (find-replace issue) to indicate renaming from `git-repository` ([`f86b780`](https://github.com/Byron/gitoxide/commit/f86b7803e85839450ed2eeef57fe738da1e3ec87))
    - Release git-features v0.26.4 ([`109f434`](https://github.com/Byron/gitoxide/commit/109f434e66559a791d541f86876ded8df10766f1))
    - Release git-features v0.26.3 ([`1ecfb7f`](https://github.com/Byron/gitoxide/commit/1ecfb7f8bfb24432690d8f31367488f2e59a642a))
</details>

## 0.35.0 (2023-02-13)

This is the last release under this name and merely a notice to inform that `git-repository` from now on is `gix`.

Furthermore, all `git-*` crates belonging to the `gitoxide` project will be renamed to `gix-*`.

### Changed (BREAKING)

 - <csr-id-1408482fd21be7487b46753bb54a018c7a164f34/> a note of the pending rename of `git-repository` to `gix`

### New Features

 - <csr-id-069eb6c3f0844b43873ae1bd536e2bca53ff5c8a/> tree diffs with simple rename and copy tracking in cases where there is no additional modification.
   As the fastest way of rename tracking, we now offer support for tracking renames and copies,
   that is a file was renamed or copied without modification.
 - <csr-id-f6ed34aa254d34e596ad027c33f78404a630ff76/> Add `diff.renames` and `diff.renameLimit` keys to config tree.
   In preparation for the implementation.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 1 calendar day.
 - 2 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-config v0.16.1, git-revision v0.10.3, gix v0.35.0 ([`74390ba`](https://github.com/Byron/gitoxide/commit/74390baf9d177a1abe3c7c35f1d9bc67faba1e97))
    - Show more debugging information if unreachable code is reached. ([`66f5341`](https://github.com/Byron/gitoxide/commit/66f53414efef6cfd6d03f830520964c9bdd23634))
    - Prepare changelogs prior to release ([`446f866`](https://github.com/Byron/gitoxide/commit/446f866d146e255ab8302b89f87bf28f2c5f3733))
    - Merge branch 'rename-crates' ([`6461c3d`](https://github.com/Byron/gitoxide/commit/6461c3da4d6daee857606d94294c3f87fc36965a))
    - Rename `git-repository` to `gix` ([`7bed2a9`](https://github.com/Byron/gitoxide/commit/7bed2a96604397fa990f427b1a970ddeb6f09f1c))
</details>

## 0.34.0 (2023-02-09)

<csr-id-a01f5d72346c36fdcb77af095273da6f4ab86e21/>

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### New Features

 - <csr-id-297d59e8396fbe2e5a2224a8524fa0778e786773/> add `env::collate::fetch::Error` - a combined error type with its own API.
   This error API allows to look at all the steps it takes to perform an operation and
   gather insights from it which require understanding a lot about the semantics of
   the contained errors.
 - <csr-id-d792ea543246632bf1ca8d0e1d239bbe7f07e219/> use enumerations to advertise progress ids publicly.
   Previously these were an implementation detail which also means they
   couldn't be relied upon.
   
   Thanks to an intermediate enumeration, they become part of the public API
   and their actual value is not exposed.
 - <csr-id-5dc408f726d6f0f480438348eb5d713776329710/> read shared indices by dissolving them into the current one.
   This allows the 'link' extension to be processed correctly, even though it
   won't be maintained. When written back, the 'link' extension will be removed
   automatically.

### Bug Fixes

 - <csr-id-5d3a3a245968d5ad8c29ea11a99b4896d1b41191/> don't panic, but error when parsing the rev-specs `^`, `..`, `...`.

### Chore (BREAKING)

 - <csr-id-a01f5d72346c36fdcb77af095273da6f4ab86e21/> adjust to changes in `gitoxide` for clap upgrade to 4.1

### New Features (BREAKING)

 - <csr-id-2faad43d11283ff06381c51d2466307cfb8736ff/> transfer knowledge about configuration and its usage into the type system.
   That way it's possible to use configuration overrides, even though ultimately being strings,
   in a type-safe manner and leverage code-completion while at it.
   
   In that process, we also change `Repository::(committer|Author)()` to return
   `Option<Result<...>>` to be able to account for date parse errors.

## 0.33.0 (2023-01-10)

<csr-id-dd7f3bf19cce0d214924fa86aeb4c5823f5bcc02/>

### Chore (BREAKING)

 - <csr-id-dd7f3bf19cce0d214924fa86aeb4c5823f5bcc02/> upgrade MSRV to v1.64 (possible due to `windows` upgrade)

## 0.32.0 (2023-01-09)

<csr-id-80dcb406c5f588122531da115398094de3c3af79/>

### Bug Fixes

 - <csr-id-a05b1c4d82bc6c7758989a3bbe326ea610903820/> default author and committer time
   When needing to fallback to a default author or committer signature, the
   time from GIT_AUTHOR_DATE should only be used for the author and
   GIT_COMMITTER_DATE should only be used for the committer and not
   intermixed. This change enforces that constraint.
 - <csr-id-ec7bf71b60f8c1e7529d610557c0305d624c1253/> signature name and email resolution
   The name and email for the author and/or committer may come from different
   config files. For example, user.name may be set in the global config and
   user.email may come from the repository local config.
   
   This case was broken due to Personas.from_config_and_env() only looking in
   the last config section containing, for example, a "user" section. Thus if
   the user.name and user.email are split across multiple sections (i.e.
   originating from separate config files), the fallback name and email
   ("gitoxide" and "gitoxide@localhost") would be used.
   
   The solution is to use gix_config::File::string() to lookup the name and
   email separately. The string() method correctly resolves the value by
   looking through all sections from all files in the correct order.

### Other

 - <csr-id-80dcb406c5f588122531da115398094de3c3af79/> name and email from different config sections
   The user.name, user.email, author.name, author.email, committer.name, and
   committer.email configuration may come from different sections from
   different config files. This new test exercises a couple of scenarios that
   are currently broken.

### Reverted (BREAKING)

 - <csr-id-87abb51596bd0a5a6b552a5de98a920d6c797e3c/> `committer_or_default()`, `author_or_default()` and `user_default()`.
   This means that all methods that previously succeeded by adding a default
   will now fail.
   
   This is preferable over 'doing something' and also admits that gits
   guesswork that tries to find user information by querying the system
   is nothing we want to repeat.

## 0.31.0 (2022-12-30)

<csr-id-9fabfc50007603f9c1f7e70b5bb79a39726b12af/>
<csr-id-91720798889ee7eb26da03a9e732caedda83b3e3/>

### New Features

 - <csr-id-d48b9a7ae2d51676c7549377bcb0b9d3baa83681/> fetching `ssh` urls can ask for a different username.
   If authentication fails, the user will be queried for a different username
   to try authentication via ssh mechanisms again.
 - <csr-id-61d89f586a0ad913fc2f502520282520a5e1fd15/> collect ssh-specific options to control how the ssh program is invoked.
   These are passed through when creating the ssh transport.

### Other

 - <csr-id-9fabfc50007603f9c1f7e70b5bb79a39726b12af/> explain how it's possible to deal with the first commit when comparing trees
   The reason the other tree isn't an option is that it's a special case that can more easily be handled
   with an `.unwrap_or_else(|| repo.empty_tree())` (or similar) for those who need it.
   
   Making the empty tree explicit also helps to deal with diffs from the empty tree (which can't be `Option<Tree>`)
   to the first tree of the first commit.

### Chore (BREAKING)

 - <csr-id-91720798889ee7eb26da03a9e732caedda83b3e3/> upgrade to prodash v23

## 0.30.2 (2022-12-26)

<csr-id-114f184855b6177aa1f0dbf6e6589f23deb5ffe6/>

### New Features

 - <csr-id-38ae61a805bd8cca5df8d1c1dcf3a8a0f9c85f5a/> make more HTTP options available
   - `http.schannelCheckRevoke`

### Other

 - <csr-id-114f184855b6177aa1f0dbf6e6589f23deb5ffe6/> provide a repository clone example

## 0.30.1 (2022-12-22)

### New Features

 - <csr-id-ca84c87734804cbfc65e311b89ff6ccfc236149c/> `open::Options::open_path_as_is()` allows to avoid 'smart opening' to try the path verbatim.
   The path to git repositories is well-known as they either end in `.git` or `.../.git`.
   If this is not the case, by default we append `/.git` to the path.
   
   With this new option enabled, no path transformations apply to open the given path as is,
   which is preferable if you know it's a non-standard git repository folder name.

## 0.30.0 (2022-12-19)

<csr-id-fceee748c114b2d0760074e911e533cd020f6996/>

### Changed

 - <csr-id-a4ac9cf3e667a3059e33aac8188150529578622d/> represent `GIT_(COMMITTER|AUTHOR)_(NAME|EMAIL|DATE)` with git configuration.
   That way it becomes more obvious where values are coming from.

### New Features

 - <csr-id-1683a848459cae2b9182b365e3e22b0e8ba73534/> expose `gix-features` crate at root under `features`.
   That way application developers can use more of the utilities
   that power most of the `gitoxide` plumbing crates.
 - <csr-id-90ef6fc36b440cc4baf3fde4a30060f1b4a0c8cf/> `Remote` knows about its `tagOpt` configuration.
   That way it's clear if it should or shouldn't fetch included/reachable
   tags automatically.
   
   The default setting for this is to include tags, similar to `git`.
   
   The `fetch_tags()` accessor allows to query this information, and the
   `with_fetch_tags()` builder method allows to set the value comfortably
   right after creating the `Remote` instance.
   
   The `tagOpt` key will also be written as part of the remote's git
   configuration.
   
   Clone operations can set the `Tags` setting when configuring the
   remote in a callback.
   
   This also comes with a fix to assure that ref-updates aren't skipped
   just because there was no pack to receive. That way, locally missing
   refs or tags will automatically be put back.
 - <csr-id-28e48083052216ddf1fd1f187cc22d506d3d9f86/> network related Error type support `is_spurious()` method.
   That way the caller can determine more easily if it makes sense
   to try again.
 - <csr-id-457c2e081b1aa5dfaab3f663b9aba66c16369939/> Make `prodash::tree` available as `progress::tree`.
 - <csr-id-d1b7ec605f8016c52c088477b6b0c5adf7ea0ab2/> read worktree specific configuration to override the one from the shared repository.
   This is intensively used when space checkouts are created, along with
   Cone mode. Thus it's the basis for properly interpreting sparse checkout
   options which are set on a per-worktree basis.
 - <csr-id-fc64693d5af0fda402c560d10d15652c24d14219/> add `permissions::Environment::http_transport`.
   That way it's possible to deny using environment variables that affect
   the HTTP transport, like setting the proxy.
 - <csr-id-0ce29a965cf16273cf74bd22e40f464e322e2f62/> `open::Options::modify()` as general pattern to allow builder methods usage in `&mut self`.
   That way it's easier to configure both the `full` and the `partial` trust instances
   of discovery options.
 - <csr-id-8482f90d0a2b61259cd51ca4f40ce322e388cb34/> Add `Repository::commit_as(committer, author, …)` convenience method.
   That way it's, very much beyond convenience, possible to set the time
   of a commit.
   
   Many thanks to @epage for the suggestion.
 - <csr-id-c8835c6edae784c9ffcb69a674c0a6545dbb2af3/> upgrade to `prodash 21.1` and add `Ids` to all progress instances.
   That way callers can identify progress they are interested in, say, for
   selective visualizations.

### Bug Fixes

 - <csr-id-d659bda2e1b0fcab529df7af6467f063ae5d0dd7/> provide a clearer error message when trying to open a git repository that isn't one.
 - <csr-id-ff0332e815c228cc5cdfe58c3598ad261bb2879e/> http transports can now reuse a connection.
   This makes connections more efficient generally and `cargo` relies
   on that behaviour in their tests as well.
 - <csr-id-9079b9d2e5f7cc133c6f2b2c2e64245b150c7d74/> allow to open a `Repository` from if 'config' file is missing.
   In this case, treat it similar to having an empty repository configuration
   file and assume defaults everywhere.
 - <csr-id-40f7379b7a89f7fe6f916801384e9e65e5b85c57/> improve error verbosity when fetching and cloning
 - <csr-id-b77fc86ab580dd81b08022996f07cc7925104e77/> `tree::diff::Platform::for_each_to_obtain_tree()` now properly surfaces user provided errors.
   Previously it would squelch them unintentionally.
   
   First discovered via https://github.com/Byron/crates-index-diff-rs/issues/35.
 - <csr-id-5386eed6a13a32a850c59706b15d8988c67733ce/> when fetching from file://, don't upset windows by trying `d:/foo`, use `d:\\foo` instead.
 - <csr-id-363ac7a74ec841505b5fc7cc1b8fae11c0a63ea9/> `config::CommitAutoRollback` now implements `DerefMut`.

### Changed (BREAKING)

 - <csr-id-3c84cebc5997356ff5f531c6cc9567bdd9b83eb5/> default features are set to `max-performance-safe` to assure compatibility.
   Previously the `max-performance` setting might have caused issues during compilation
   or issues at runtime if libraries like `git2` are used in the same binary, and the
   new default feature settings maximizes compatibility so this won't happen.
   
   For best performance, however, one will have to activate the `max-performance`
   feature on top of that.
 - <csr-id-5fe6aa3f3f2f81d84f0d96e874e68a8bf4de1db1/> environment variable permissions are per topic.
   Now `Permissions` for environment variables are so that they
   are by topic instead of by prefix, by default. That way
   it's easier to allow or deny particular sets of related
   environment variables.
   
   The catch-all permissions by prefix are still present for all
   other variables that aren't contained in one particular topic.
 - <csr-id-49f39d6bb487c0254176a5082f2c7851b83952a1/> `open::ReplacementObjects` is removed in favor of two custom git-configuration flags.
   Now it's possible to map the environment variables `GIT_REPLACE_REF_BASE` and `GIT_NO_REPLACE_OBJECTS`
   to custom git configuration keys which can also be set, namely `gitoxide.odb.replaceObjectsRefBase`
   and `gitoxide.odb.noReplaceObjects`.
   
   Along with the possibility of disabling the usage of `GIT_` prefixed environment variables one
   reaches the previous level of control without making object replacement a special case.

### New Features (BREAKING)

 - <csr-id-f8a2bfb93dadbc64393135e0a447f3d76628509c/> `interrupts::init_handler()` can now be undone.
   This can be done by calling `deregister()` or `auto_deregister()` on the return value
   of `interrupts::init_handler(…)`.
   
   That way it's possible to temporarily do interrupt handling only while some methods
   that require it are running.
 - <csr-id-becbd8d896a1663f1607be4e86e632773e926f1f/> represent object cache configuration like `GITOXIDE_PACK_CACHE_MEMORY` in git-configuration.
   That way there is a unified system for how to set values, which may be overridable by configuration
   variables or not.
   
   With this changes, the explicit application of environment variables for setting the cache
   isn't required anymore as everything happens using git-configuration, and automatically,
   while providing full control like before.
 - <csr-id-f16e36168cc93768ba5d53c9848ff2e8432d06b1/> remove `SnapshotMut::apply_cli_overrides()` in favor of `open::Options::cli_overrides()`.
 - <csr-id-84d594caf3f04f1ce337e455343278a6f4674957/> more type-safety for remote names by making clear they can be named after URLs.

### Other (BREAKING)

 - <csr-id-fceee748c114b2d0760074e911e533cd020f6996/> `Remote::with_refspec()` to `Remote::with_refspecs()` to allow adding more than one refspec as part of the builder.

## 0.29.0 (2022-11-21)

<csr-id-f302fc1bcd06fadccd126f4f5f9c0165afabedda/>

### New Features

<csr-id-ff9e1571b558475e727dc6ba11dab24ef15fb6f4/>

 - <csr-id-3ddbd2de369b521fa3f21935f10fe9c248840893/> Make `reqwest` TLS backend configuration easy.
   We provide the choice of `native-tls` or `rust-tls`. If none is
   provided, the user can configure on their on similar to how it's done
   in `gix`.
   
   Please note that a choice now has to be made or HTTPS will not be
   available, so use one of…
   
   * blocking-http-transport-reqwest-rust-tls
* blocking-http-transport-reqwest-native-tls

### Bug Fixes

 - <csr-id-c6a690219915b2b401d2d11f61db35b2931e5b3a/> `gix::Commit::describe()` chooses tag names (more) correctly.
   Previously, if there were multiple choices for tags on the same commit,
   `git describe` would disagree with `gitoxide` due to different
   prioritization of names.
   
   This has now been fixed.
 - <csr-id-84ed89c3bf6692f18c4bb97173527de1bcba7ac6/> also sort entries lexicographically

### Other

 - <csr-id-f302fc1bcd06fadccd126f4f5f9c0165afabedda/> Set GIT_EDITOR in make_rebase_i_repo.sh
   If the user has core.editor set in their global git config, then that value
   takes precedence over the EDITOR environment variable. The GIT_EDITOR
   environment variable, however, has higher precedence than core.editor. For
   this test, using GIT_EDITOR ensures that the desired sed command line is
   used.

### New Features (BREAKING)

 - <csr-id-bc2a399f2fbb69d23b0b05e8dfb95f3c64ff93b9/> rename `blocking-http-transport` feature to `blocking-http-transport-curl`; add `blocking-http-transport-reqwest`.
   With the new and relatively immature second tier http backend we pave
   the way to support builds without the use of open-ssl and probably many
   other C libraries.
   
   Note that it's early and not `http` configuration option is implemented
   yet.
 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `gix-features` and `gix-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

## 0.28.0 (2022-11-17)

<csr-id-6beb6f263fd40884b440092f39034dd43d3a95de/>

### New Features

 - <csr-id-58e14884b1d025651f874d899cb2d627c4a2afbf/> `Id` implements `std::fmt::Display`
 - <csr-id-25f7aabe38267b6b6c0547806028b2becb806416/> `Remote::repo()` to obtain the underlying repository.
   For convenience.
 - <csr-id-709a73229b7cde56ddffa099158661632c606468/> Support for user-customizable user agent strings.
   Doable by setting the `gitoxide.userAgent` variable.
 - <csr-id-e60d07997989993216c2bd93efeb6f1b48da0a87/> add `env::agent()` for obtaining the default client agent string.

### Other

 - <csr-id-6beb6f263fd40884b440092f39034dd43d3a95de/> try to apply maybe-async in a place where it's probably not possible.
   The goal is to re-use the existing tests, but right now they only
   compile in async mode as the `maybe-async` crates needs
   a feature to be set. Doing so is hard(er) if it's not already used
   in the main crate, which we do not and will do our best to avoid.

### New Features (BREAKING)

 - <csr-id-db9040f0bb3a16879c8da0252a77df80bd417387/> add `remote::Connection::with_transport_config()`, change the way `*::transport_mut()` is used.
   Previously `transport_mut()` was supposed to be used for calling
   `configure()`, but that doesn't work anymore as `configure()` can
   only effectively be called once the initialization of the Connection
   is complete, as it may depend on the Remote name AND the credential
   provider for proxy auth credential acquisition.
   
   Thus we allow callers to set the transport options they need in advance
   for it to be used when needed.

## 0.27.0 (2022-11-08)

### Changed (BREAKING)

 - <csr-id-c50868c7ed7309515b4f0a188213d332d57dd146/> Move `object::tree::diff::change::DiffPlatform` to `object::blob::diff::Platform`.
 - <csr-id-4ee32713093c2e41a12d148c6030add1df6aa966/> new `DiffPlatform::counts()`, open `DiffPlatform` for use of `gix-diff::blob::*`.

## 0.26.0 (2022-11-06)

<csr-id-c6f92c15529ddff7539667b74bafa2348f3040e3/>

### New Features

 - <csr-id-b1edb9e3537df86669714f03666f4a88e0ac3709/> diff algorithm is controlled by git configuration `diff.algorithm`
 - <csr-id-072f5bc9c91c4c09bd6a73f9d7ac672805cae533/> Query of `core.logAllRefUpdates` is now fallible.
   This is the same behaviour as shown by `git`, which requires valid
   values or aborts.
 - <csr-id-2eaf69e5f8f8da10e5af85cb9f0c39577ad1707f/> automatically handle `.keep` files after writing a pack bundle to disk.
   The logic implemented here tries to do the right thing, that is when
   we have reason to believe that the objects in the pack are linked up
   with a ref, we delete the keep file automatically.
   
   However, if there was no local ref edit as the ref specs didn't contain
   local destinations, or if the pack was empty, then keep the .keep file
   and leave it to the caller to handle.
 - <csr-id-8b9fbd4e9ed7be37976c7203cd9a89c6116a6d3d/> Use `core.askpass` when building the credential helper.
   Previously it would only consider the environment variable, which can
   still override the provided git-configuration at core.askpass .
 - <csr-id-a9d14492322785a14f4ecb5b0d3dbdc87e56f8c5/> `remote::fetch::Prepare::handshake_outcome()` to obtain server information right after listing refs.
 - <csr-id-0b5c53ec43bdb58b2b7cf46e453ddf858770a95a/> `open::Options::config_overrides()` for early configuration; support for `init.defaultBranch`.

### Bug Fixes

 - <csr-id-f869b224170b0c49a0e4d89e88bfbf5caedaa725/> don't allow non-bare repositories to be initialized into non-empty directories.
 - <csr-id-91baefad02a0d52c745106359da3693d06aace46/> `init_bare()` now creates the destination directory if it doesn't exist.
 - <csr-id-5c11b84f4e74e3eefdd0f5804976ebfc505e0f2f/> build correct path for `$HOME/.config/…` files.
   The special per-user `ignore` and `attributes` files can also be
   defaulted if some environment variables are set and may be accessed.
   
   Previously the default for `$HOME` was incorrect, as it was missing the
   intermediate `.config/` directory. This is now present to build paths
   exactly like git.
 - <csr-id-275e80f3d602b63ef91efe31a92b4aafb2eeca44/> ref-map filtering now uses correct prefixes.
   Previously specs could get filtered out server-side as a matching prefix
   was entirely missing.

### Changed (BREAKING)

 - <csr-id-449ff066d2b5dd423c639618193dd9e54d03c1f8/> `Repository::branch_remote_name()` returns `reference::remote::Name`.
   That way it's made clear the remote can also be a URL, while rejecting
   illformed UTF8. The latter isn't valid for remote names anyway as these
   only support a very limited character set.
   
   Note that this error currently is degenerated, making it appear if the
   remote name doesn't exists if illformed UTF-8 is found in what appears
   to be a symbolic ref.
 - <csr-id-71f15fc46fbaea455cf84a2b4cfe3e680047d790/> be specific about the kind of `diff::Error`, moving it to `diff::for_each::Error`.

### New Features (BREAKING)

 - <csr-id-7413a284eb7754e63ba45d0f526347b9f79b557d/> `Tree::lookup_entry*()` returns attached `Entry` type.
   That way chaining gets even easier.

### Bug Fixes (BREAKING)

 - <csr-id-2bece79285e244a7029f9393dafc990e39420e2d/> `create::into(…)` takes `create::Kind` to determine if it's bare or not.
   First of all, `bare` is not an option anymore, but a parameter as
   it can't be defaulted.
   Other public signatures change as well to accommodate for it.

### Other (BREAKING)

 - <csr-id-c6f92c15529ddff7539667b74bafa2348f3040e3/> `DiffPlatform::text()` to `*::lines()`.
   This is more specific as one could also do character changes in a single
   line, and it adapts the signature to the new `imra-diff` powered
   mechanism, for a 2x speed boost.

## 0.25.0 (2022-10-10)

<csr-id-5bef0a00e8d01110c054a517f6d9696f981a7efc/>

### New Features

 - <csr-id-22d3b37ea6239170a478b859361a7d1d7ba01a9a/> `Url::try_from(path: &std::path::Path)` for more convenient instantiation.
 - <csr-id-31a7089f2583832727e2175ada6fb5c30c3beebe/> make some private methods public to give callers more flexibility.
   This allows to implement the fetch-negotiation part oneself and break
   free from constraints of the delegate.
 - <csr-id-4367994a8a7476eb44e1309e833e345fdb78f246/> add `config::SnapshotMut::commit()` to make clear it's transactional.
 - <csr-id-d2bea003230078ffb4e6cd80d1b01c3995435a34/> add `config::SnapshotMut::forget()` to forget all changes before applying them.
   The documentation was update to make clear when the changes are applied.
 - <csr-id-4b1e3b3d91c51da3dbea9191e60f959a1266cf47/> add `Repository::find_default_remote()` which works on detached heads as well.
 - <csr-id-25f06400c49ddd1688fd76f9285542b121b223b4/> `Remote::rem_map()` now specifies ref-prefixes to the remote.
   This can greatly reduce the amount of refs sent.

### Other

 - <csr-id-5bef0a00e8d01110c054a517f6d9696f981a7efc/> try to make the transport configurable after being boxed, but…
   …that would force it to be 'static, which is something we explicitly
   cannot have. We need references to be contained within, if I remember
   correctly.

### Changed (BREAKING)

 - <csr-id-e88de0f948325773db1925b07aa878e1dbb76bad/> All methods editing references don't take the author as parameter anymore.
   Instead, these are taken from the git configuration and can be
   configured on the fly with temporarily altered configuration.
 - <csr-id-3a0fb1b45c757add49677450836c0aaf6179a2b5/> remote `lock_mode` from all methods dealing with reference edits.
   It is now read from `core.filesRefLockTimeout` accordingly.

### New Features (BREAKING)

 - <csr-id-3b29fc18672c0176684c797a0f16f85d09369bf8/> make jwalk fully optional
 - <csr-id-78ad3df64f2c016ba17b158bd9ab1d2341aab399/> add `fetch::Transport::configure` to generically configure any transport.

## 0.24.0 (2022-09-20)

<csr-id-f5959edc1477573278afcfe23e9e52ddaacb37db/>
<csr-id-79c22557ce0aea1ee8f3a58192c2c76087ccd3d8/>

### New Features

 - <csr-id-0871a96b9cc84d7a496d39393e081999c0a3fe17/> `Object::peel_to_tree()` as convenience method.
   It's very common to try to work with trees, so let's make that easier.
 - <csr-id-1027be960852618915014f9ba6e6632bd4999b0e/> `interrupt::Iter` now allows accessing the inner iterator without consumption.
   This is useful if these provide additional out-of-band information.
 - <csr-id-8c2e5c60f9f5f8d0859ecd84c17af20e88812512/> Once a change is obtained, it's easy to obtain changes line by line.
 - <csr-id-7e96d1841989b37133cddf334724a2d6df557e69/> obtain a refmap after listing refs via `remote::Connection::list_refs_to_map()`.
   With it it's possible to establish a relationship between what's about
   to be fetched to local tracking branches as established by refspecs for
   fetching.
 - <csr-id-d51e7c901fe5ed60d5dd56006c5faedb71cad537/> Add `permissions::Config::git_binary` field
   When true, default false, inject the git installation configuration file
   if present at the cost of one `git config` invocation.
   
   Note that we rely on the underlying `gix-config` crate to not load
   duplicate files.
   
   We also currently lie about the scope which is actually unclear - have
   seen 'unknown' or normal scopes like `system`.
 - <csr-id-1c13f1125664fbcc276a1ca440d168d07d0bf493/> add `prompt` to top level forwarding #450)

### Bug Fixes

 - <csr-id-ae3866065c9c3c6d01709f8dde1cea1ae1345779/> rev-spec parsing can now handle the empty tree as full hex hash.
   Even though the empty-tree object can be found when searched via
   `Repository::find_object()`, previously it was not locatable when
   used during rev-spec parsing.
 - <csr-id-74ede2031d1beedf11f1cdf006fff71e597a2cb5/> `Reference::remote()` can produce remotes for URLs

### Refactor

 - <csr-id-f5959edc1477573278afcfe23e9e52ddaacb37db/> use specific error type for `rev_parse_single()`

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

### New Features (BREAKING)

 - <csr-id-2992b1ba4e7bbeab26f41175cd31fd664abf2a11/> Add reference remote name type to make usage of `remote_name()` result clear

### Other (BREAKING)

 - <csr-id-79c22557ce0aea1ee8f3a58192c2c76087ccd3d8/> `Tree::lookup_path()` -> `Tree::lookup_entry()`.

## 0.23.1 (2022-09-01)

### Changed (BREAKING)

 - <csr-id-36d8c57824a2b921012439105e49261fac66c955/> Remove 'unstable' feature.
   It's not worth maintaining it especially since everything is in
   pre-release mode right now.
   
   It might be something to re-introduce after go-live.

### Bug Fixes

 - <csr-id-d18e76cfb512ef7fe5bfee6e87726372c6a4a8b6/> `max-performance-safe` mode does not include zlib-ng adjustments anymore.
   git2 cannot handle this and fails to fetch packs after a couple of
   seconds.
   
   It's unclear what is causing this except that git2 doesn't like libz
   with zlibng support enabled, which happens if git2 in the
   same tree is with us.
 - Transitively through a kindly contributed fix in the `gix-discover` crate, `Repository` can now be opened on `exFat` volumes.

## 0.23.0 (2022-08-28)

### New Features

 - <csr-id-70aa850591de268488ae9bf2d3839a5c9c543c35/> The empty tree can always be returned by `Repository::(try_)find_object()`
   This matches the behaviour of git and libgit2.
   We consciously chose to only do this on the highest level, allowing lower
   levels to determine if the object exists or not.
 - <csr-id-8d0786646e17a82d20ca6b2799b54f6349d389f4/> Make `find::object::*::Error` publicly available.
 - <csr-id-2d0b63997b276a53b3cf8f09fac51f8e3f044bcd/> Add `Reference::delete()` for simple reference deletion
 - <csr-id-9170562059c3eaa529850025ef35ac5ffffc0fdf/> `Reference::set_target_id()` to easily set the target id of a reference
 - <csr-id-950da602925e6376b08640ed3ebfdf407394db34/> `Reference::head_ref()` to quickly access the reference the head points to.

### Bug Fixes

 - <csr-id-2834311b4f262c57e76627addaa4932196fd26b3/> `Commit::tree_id()` now returns a connected id

### New Features (BREAKING)

 - <csr-id-e090f843f5cffc8e8e47a8cac2e6fb98e4c47771/> `gix-diff` is now included by default as part of core functionality

## 0.22.1 (2022-08-24)

A maintenance release without user facing changes.

## 0.22.0 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes

### New Features

 - <csr-id-c28bcec19b5526acf888f928e6ddc4671873368e/> support avoiding usage of `fast-sha1` in gix-features separately.
   That way one has an angle on compile failures in client libraries,
   see https://github.com/o2sh/onefetch/pull/752 for motivation.
 - <csr-id-4f87a0672f7288486a9b6403c0bb07a6279d2cfe/> `Repository::write_blob[_stream]()` to more easily write blobs.
   That way, one won't have to use the underlying `objects` database but
   can remain in the land of `Repository` enabled types for longer.
 - <csr-id-d35cd2a12c6db3d655ce10cad5c027bde99e19b4/> `SnapshotMut::apply_cli_overrides()` to make it easy to support things like `-c`
 - <csr-id-2a839f3209f3bd35e0c0f7edff664cc953059f65/> `Repository::config_snapshot_mut()` to mutate configuration values in memory.
   It's a first step towards writing changes back to disk, which can work
   already, but probably wouldn't as we currently don't localize changes
   to only one section type, i.e. Api, but instead may change values
   from other sections.
 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs

### Bug Fixes

 - <csr-id-ff71730b4e3635533d9969d9dd44c0f3c75c6648/> Don't fail worktree tests if the system's git version is not supporting required features

## 0.21.1 (2022-08-19)

A maintenance release that speeds up `commit.describe()` performance if `max_candidates()` is 0.

## 0.21.0 (2022-08-17)

<csr-id-b38a212459e2646ab97ad7b5c24e54d962aae960/>

### Changed

 - <csr-id-0235111a4fcc40c7b57d973bfce27a66eddea901/> Invert behaviour to `open::Options::strict_config()`, with lenient being the default.
   This means API users will get libgit2 behaviour but commands like `gix` can
   change options to emulate `git` behaviour.

### New Features

 - <csr-id-a01525d159a33d6ad60a5324f2e9abbbe17fcfad/> `Kind` can now represent submodules.
   This should complete the list of git repository types and flavors.
 - <csr-id-5dac021bbbc5621167e7f49d62b11f68f76e42b6/> `open()` and `discover()` support opening submodules.
   This includes submodule checkouts as well as their original module git
   directories.
 - <csr-id-067c3342f3564dd7f152a720e93e3aa590ae6524/> `open::Options::lenient_config(…)` to default otherwise invalid configuration values where possible
   Originally required by https://github.com/starship/starship/issues/4266 .
 - <csr-id-0bf8371706d319681c3f794af5cd13f2f50a27d0/> support core.worktree option
 - <csr-id-b47bbb787ef2e31dd2612a56f9e7759ef8a188b8/> display for `object::tree::EntryRef`
 - <csr-id-727768a49c41165b03ddcdbc71ca88b66c330f32/> `Head::prior_checked_out_branches()`
 - <csr-id-ffe72918baf5c4c9f0f3709f75f068a663778588/> `Repository::index()` and `Worktree::index()`.
   These methods provide a possibly updated shared index.
 - <csr-id-47619f7c06a49dcf60a30e1a43a5352914183092/> add `Repository::object_cache_size_if_unset()`
 - <csr-id-d2611cee61841bc7bd978bef5af7dc66154248a2/> `Commit::message_raw_sloppy()` to provide yet another way to obtain a commit message.
 - <csr-id-906c95845fa4aa2d4390c522bb566a188b8c0e78/> add `rev_spec::parse::ObjectKindHint` to support `core.disambiguate`.
   The latter is seemingly undocumented in the typical place, gix-config.
 - <csr-id-ef187f0180d89544d9015cbc2bc03d8cb51f4615/> `Remote::with_refspec()` to add new unique refspecs
 - <csr-id-d51ba42c643d8ee03a3c6b648f8524ff04827170/> `Remote::push_url()` to set it after the fact
 - <csr-id-9b07b91ad065836e7473df6635025659af2865ee/> `Repository::remote_at(…)` to create an unnamed remote
 - <csr-id-a67fc26b80e5d1183ddc5c6598396214f3e19945/> more conversions for `TryFrom`: `String` and `&str`
 - <csr-id-7a512ecdf236afc0b3d562d60fa81ab62c00cd9d/> `Head::into_remote()` to try really hard to find the correct remote
 - <csr-id-f392f26bec6069ac43ecd461b4f50e0def8b8972/> `Repository::remote_default_name()` to obtain the repo-wide remote for a a direction.
 - <csr-id-f47464f64f7c21500a24f563b25a8fc070c41778/> `Repository::branch_names()` to obtain branch names for which configuration exists.

### Bug Fixes

 - <csr-id-be6114e7c4ac48467db6acb2180b443dc9f59f32/> assure permissions per trust level are properly inherited into `open::Options`.
 - <csr-id-270242c707bd086b7746ee45b55791587f1484b1/> provide additional explanation about when to use `open::Options::with()`

### Refactor

 - <csr-id-b38a212459e2646ab97ad7b5c24e54d962aae960/> embrace `revision` module and move `rev_walk` there.
   Let's embrace the idea of structured modules and platforms in the right
   spot in the module hierarchy instead of forcing known names on it that
   over-simplify.

### Changed (BREAKING)

 - <csr-id-0deda0df55c11647f51374ed5a8bf11c932e2bae/> remove `permissions::Config::strict()` as they were unused internally.
   Furthermore, they were allowing everything as before so better not to
   have it.
 - <csr-id-1c12d49eefa6d79ef50b2960f41b29de680ac8eb/> rename `Repository::load_mailmap*` to `Repository::open_mailmap*`.
   For consistency with other similar methods.
 - <csr-id-ea35183b53f2ff71bdf2270ac4f7470a85d7756f/> remove `Repository::load_index()` in favor of `repo.worktree().open_index()`.
 - <csr-id-4fd096840ba27da6ce86678a85ede33e3be974ff/> `gix_revision` is now available in `revision::plumbing`.
   That way it won't clash with the higher-level constructs on top of it
   which use the same names.
 - <csr-id-2424957cff75daacf6f6f14f74b9639f6875c4fb/> Turn `id::Ancestors` into general-purpose `RevWalk`.
 - <csr-id-1df379ab0046887a330c0a670ad0414e79cfae7b/> remove `Permissions::git_dir` field entirely.
   It was meant to help dealing with bailing out if the git dir isn't
   fully trusted, but the way this was done was over-engineered especially
   since the read-only permission level wasn't implemented at all.
   
   That function is now performed by a new flag, the `bail_on_untrusted`
   which is off by default.
 - <csr-id-5ab81ece15ec802ad4328ce31304233bd25b2929/> rename `Repository::remote_ref()` to `::branch_remote_ref()`

### New Features (BREAKING)

 - <csr-id-e2aff28e818951785d933f4b55b2f1b882729cb6/> `Repository::rev_parse()` returns a `RevSpec`.
   This lays the foundation for actually handling rev-specs faithfully.
   Previous users should use `rev_parse().single()` to obtain a single
   object id which was the only supported usecase previously.

### Bug Fixes (BREAKING)

 - <csr-id-c68b125a46f666700cdbda6f8cd39a044f4feb1b/> Don't panic for `@{1}` in new repos; rename `Head::into_referent()` to `::try_into_referent()`
   The signature change will prevent such issues in the future as one
   cannot simply ignore new repositories.

## 0.20.0 (2022-07-22)

### New Features

 - <csr-id-1b765ec6ae70d1f4cc5a885b3c68d6f3335ba827/> respect `safe.directory`.
   In practice, this code will rarely be hit as it would require very
   strict settings that forbid any operation within a non-owned git
   directory.
 - <csr-id-840d9a3018d11146bb8e80fc92693c65eb534d91/> permissions for configuration.
   It provides fine-grained control over what sources to load.
 - <csr-id-657080829867d9dcb0c9b9cb6c1c8126c4df3783/> `gix-config` is now accessible in `git-repository::config`.
 - <csr-id-d99453ebeb970ed493be236def299d1e82b01f83/> `gix config` lists all entries of all configuration files git considers.
   Filters allow to narrow down the output.
 - <csr-id-ebedd03e119aa5d46da07e577bfccad621eaecb5/> repository now initializes global configuration files and resolves includes
 - <csr-id-de8572ff2ced9422832e1ba433955c33f0994675/> resolve includes in local repository configuration
 - <csr-id-d5a48b82230b047434610550aacd2dc741b4b5f0/> `config::Snapshot::trusted_path()` to obtain trustworthy paths.
   We also apply trust-based config query during initialization to assure
   we don't use paths which aren't owned by the current user.
 - <csr-id-5f9bfa89ceb61f484be80575b0379bbf9d7a36b3/> `Repository::config_snapshot()` to access configuration values.
 - <csr-id-7f67b23b9462b805591b1fe5a8406f8d7404f372/> Use `gix-config` to write config file on initialization, including `logallrefupdates` and `precomposeunicode`.
 - <csr-id-e263e13d312e41aa1481d104fa79ede509fbe1c5/> respect `core.logallrefupdates` configuration setting.

### Changed (BREAKING)

 - <csr-id-68f4bc2570d455c762da7e3d675b9b507cec69bb/> Make `SignatureRef<'_>` mandatory for editing reference changelogs.
   If defaults are desired, these can be set by the caller.
 - <csr-id-f932cea68ece997f711add3368db53aeb8cdf064/> `Repository::committer()` now returns an `Option`, see `::committer_or_default()` for a method that doesn't.
 - <csr-id-89a41bf2b37db29b9983b4e5492cfd67ed490b23/> remove local-time-support feature toggle.
   We treat local time as default feature without a lot of fuzz, and
   will eventually document that definitive support needs a compile
   time switch in the compiler (`--cfg unsound_local_offset` or something).
   
   One day it will perish. Failure is possible anyway and we will write
   code to deal with it while minimizing the amount of system time
   fetches when asking for the current local time.
 - <csr-id-6f4eea936d64fb9827277c160f989168e7b1dba2/> Associate `file::Metadata` with each `File`.
   This is the first step towards knowing more about the source of each
   value to filter them based on some properties.
   
   This breaks various methods handling the instantiation of configuration
   files as `file::Metadata` typically has to be provided by the caller
   now or be associated with each path to read configuration from.

### New Features (BREAKING)

 - <csr-id-d003c0f139d61e3bd998a0283a9c7af25a60db02/> Support for `lossy` load mode.
   There is a lot of breaking changes as `file::from_paths::Options` now
   became `file::init::Options`, and the same goes for the error type.
 - <csr-id-311d4b447daf8d4364670382a20901468748d34d/> change mostly internal uses of [u8] to BString/BStr

## 0.19.0 (2022-06-13)

### New Features (BREAKING)

 - <csr-id-266d4379e9132fd7dd21e6c8fccb36e125069d6e/> Make `realpath()` easier to use by introducing `realpath_opt()`.
   That way there is consistency about how many symlinks to follow.

## 0.18.1 (2022-05-23)

### New Features

 - <csr-id-c78baecbb37fd92a0a86231810c9e35e9a4c21cd/> `Debug` for `Reference`.

## 0.18.0 (2022-05-21)

<csr-id-e63e722791a7795cd99048bed834459595c60abc/>

### Other

 - <csr-id-e63e722791a7795cd99048bed834459595c60abc/> add ceiling_dirs option to upwards discovery

## 0.17.0 (2022-05-18)

<csr-id-53c06c7e6a3003b34edaab10db1f158e2fb57403/>
<csr-id-e4f4c4b2c75a63a40a174e3a006ea64ef8d78809/>
<csr-id-da8059ce26343c8cd275f43c879d98c92f77fa51/>

### New Features

 - <csr-id-45920da7c8c5618c6e7258de08dbd633a637d017/> Add `Repository::head_name()`.
   A convenient way to obtain the name of a head, if not detached.

### Bug Fixes

 - <csr-id-a1680b44ef568317465d2971da6e0930f9885530/> `Commit::describe()` now returns annotated tags before lightweight ones and prefers more recent ones as well
 - <csr-id-99365f221065ebc315ac80940ad72cae253743bc/> Support for in truncated history in git-describe
   This allows `describe()` to work on shallow clones.

### Other

 - <csr-id-53c06c7e6a3003b34edaab10db1f158e2fb57403/> allow reading information about remote branch
 - <csr-id-e4f4c4b2c75a63a40a174e3a006ea64ef8d78809/> `path::discover()` now returns the shortest path.
   If and only if it canonicalized the source path. That way, users will
   still get a familiar path. This is due to `parent()` not operating
   in the file system, which otherwise would be equivalent to `..`,
   but that's not how we work.
   
   Maybe we should overhaul the way this works to use `../` instead
   and just 'absolutize' the path later (std::path::absolute()) is
   on the way for that.
 - <csr-id-da8059ce26343c8cd275f43c879d98c92f77fa51/> remove unused variant

### Changed (BREAKING)

 - <csr-id-80e8fd4a5944890f43f3d888b7a73bb26351b195/> integrate trust model into repository discovery
   That way it's possible to ignore repositories which effectively
   aren't owned by the current user, or to not ignore them (default)
   but assign tighter permissions to the repository.
 - <csr-id-2e39b0ede98826e6f85c56fef77ac65a5b7e7ac2/> `path::discover::existing()` -> `path::discover()`
 - <csr-id-38dfdcf80f9b7368ccaa10f4b78b2129849848d0/> remove `values::*Error` in favor of `value::parse::Error`.
   This makes it easier to work with errors in practice, we are either
   interested in the value that failed to parse to try something else
   or want a nice user message.
   
   Having one decode error type facilitates that.

### New Features (BREAKING)

 - <csr-id-32dc1829a5661f66396d109c8d0a8eaae6b1f532/> use `gix-credentials` in `gix-protocol`

## 0.16.0 (2022-04-05)

### New Features

 - <csr-id-47556f6815148ed960a727fd122f7162345544c3/> auto-calculation of a good hex-len, like what git does
   If the `core.abbrev` value isn't set or is set to `auto`.
 - <csr-id-654f4afb794a370b7cd9d9502ff6d0c3378ec417/> `Commit::describe()`
   A way to fluidly configure a `git describe` operation and run it.
   
   Along that, a new `Tag` top-level object was added as well to provide
   convenient access to otherwise lower-level objects. It's not strictly
   required for our implementation here but it's needed for a symmetric
   API.

## 0.15.0 (2022-04-03)

<csr-id-5f7595305efc85d6ca3c541e9f9adac3915cbd84/>
<csr-id-c10f07c50f6dde4b39bf1e3ff26c239c5f202912/>
<csr-id-bbc6efeceb26050973e1425e68a52e51b9df4572/>

### New Features

 - <csr-id-1322dbf6827ea5cc1d71175afb669e01fb1242ef/> support for object replacement
   The Repository now respects replacement refs created by `git replace`
   and picks up environment variables for its configuration as well.
   
   Which environment variables are used is fully configurable.
 - <csr-id-a39bf71531ee0a6c8db082758d3212c805ce2bf0/> support for trimming of whitespace around name and email
   It's separated from parsing to assure we can round-trip, but it's
   made easy to obtain trimmed results using new methods.
   
   This high-level git-repository will also trim by default now.
 - <csr-id-00578040a699e1939b3d3813616d3cc4e1d8669e/> `Repository::head_commit()`
   A shortcut to get to the commit much faster.
 - <csr-id-def80df2e165b74f4b053e4030f563902b7d34a4/> `ein tool estimate-hours` now supports mailmaps
 - <csr-id-f0d8a49587c08713350252e1701a45bb308b6f9d/> `Repository::head_id()`
   A long-needed shortcut.
 - <csr-id-d2388d8d80f379eccc9ee84ebe07acd67d154630/> `gix repository mailmap entries`
 - <csr-id-e3bc1b410409a9e27894a5cac48b06d8c3295e36/> unstable mailmap module
 - <csr-id-1be00cf9e00ce9428ffddb2c79b2373926069b13/> `Commit::short_id()`
 - <csr-id-c7dff9e8b695d298a3fb21f19f51752a885a5ce3/> in-manifest and in-lib documentation of feature toggles
 - <csr-id-9f5663ed83d83c7335b346313837d4cada9cd846/> `easy::Commit::time()` to access the committers time conveniently.
 - <csr-id-7c88b62e439af7a60ddb68fb6737cb3b1cebf00d/> easy::Head::name() to learn about the name of the HEAD ref
   It's mainly for completeness to provide people with with a `FullNameRef`
   of HEAD.
 - <csr-id-3b0913a2e6695e4e9e94341ef48d2ba3b4a518e6/> `easy::Head::peel_to_commit_in_place()`
   It allows to quickly get a commit from the head, something most people
   want when getting started with any kind of tool.
 - <csr-id-1c22d76c26464db4a185e19bb6c1f9a17fa19bc9/> `Repository::load_index()`
   This method makes the index of the default workspace available.

### Bug Fixes

 - <csr-id-c329dd75420f82d506fd415cd377f7df6c6ccbad/> Properly classify worktrees as non-bare, helps with `ein t find`
   They use git-files which point to the actual repository data.

### Changed (BREAKING)

 - <csr-id-a8b6589a7c645f323f95da6cb94321fc967e9b06/> Easier access to local and remote branches

### New Features (BREAKING)

 - <csr-id-8945d95f7fa88562d37ff67ac6e38bead73dd2df/> `interrupt::Iter`, rename `interrupt::Iter` -> `interrupt::IterWithError`
 - <csr-id-813a3bea88cdbe1fd9b0a8070efeee2a44f7823e/> Let 'easy::Object::try_into_…()` return `try_into::Error`.
   That way, the typical usage of `try_into_commit()?` will not result
   in a strange error about `Object` not being convertible into some
   error. We think having a real error there is the least surprising.

### Bug Fixes (BREAKING)

 - <csr-id-c863ea5b34fa9ee3dac21c1f85587da16045f8d8/> do not install signal handlers by default
   The previous behaviour is meant to be convenient for the casual
   user even though it
   ends up being surprising when used in applications that install
   their own signal handlers and need more control over how the program
   shuts down.
   
   This is now fixed by **requiring an explicit `setup()`** call before
   the first tempfile is created, which makes it a breaking change.

### Other (BREAKING)

 - <csr-id-5f7595305efc85d6ca3c541e9f9adac3915cbd84/> `Id::prefix` -> `Id::shorten()`
   It's definitely more intuitive that way.

### Refactor (BREAKING)

 - <csr-id-c10f07c50f6dde4b39bf1e3ff26c239c5f202912/> dissolve 'easy' module by moving everything one level up
 - <csr-id-bbc6efeceb26050973e1425e68a52e51b9df4572/> clarify different repository types much better

## 0.14.0 (2022-01-23)

<csr-id-7a91212631219e94b9454d2874b53f3ecc1db77e/>
<csr-id-b2cc0c63570d45de032d63e62d94c3344783440e/>
<csr-id-ebc7f47708a63c3df4415ba0e702660d976dfb3e/>
<csr-id-2290d006705ff47ad780b009fe58ee422b3285af/>

### New Features

 - <csr-id-667485e133ca29fcc6914a7142cf953564b5fce3/> Add `easy::Tree::traverse()` platform
 - <csr-id-8f650c089c88698483f778aa5c0070f606b94f09/> Add `easy::Commit` object
   It allows to more conveniently access commit information.
 - <csr-id-0ae2a8da010d848d98bef47ac923ae1d770091ff/> `easy::Oid::ancestors()` now supports `sorting()` and iteration by first commit only
   Especially the sorting is useful to avoid having to sort commits by
   hand after collecting them.
 - <csr-id-bc77534f9c385046f6c9adb994b1443307afda46/> Use GITOXIDE_OBJECT_CACHE_MEMORY to control how much object cache is used
   Note that this is mostly for debugging or quickly seeing if object
   caches help with certain operations.
   
   Ideally the implementation knows themselves and sets up caches
   accordingly, probably after trying it with these environment variables.

### Changed (BREAKING)

 - <csr-id-6e3a745dfada66a2fcac256dae0ac63959e74d08/> rename `easy::Object` methods returning `Ref` objects to have `ref` in their name
   That way, it's more clear that the `Ref` versions are low-level ones
   whereas the `into_` ones are higher-level ones that are part of the
   `easy` suite.
 - <csr-id-b6730979808ce28b98c65888a349f1e3d0ea1b9a/> Rename `OwnedObject` to `DetachedObject`
   The latter more clearly indicates what the difference is to
   `Object` (which is attached and carries a lifetime)
 - <csr-id-c4184f3c31ffc4597bd089e8140653906a6594d8/> Remove easy::borrow::Error entirely; support for multiple objects per handle
   This massive simplification finally allows any amounts of objects to be
   created while adding support for reusing their data buffers thanks
   to a simple free-list stored with the handle.
 - <csr-id-880b56426859306aa30038ff35e2ad14607e9e90/> rename `easy::Object` to `OwnedObject`; remove `Ref` suffix from `ObjectRef` and `TreeRef`
 - <csr-id-f9c0493460ab7c664aaa231ffcf7dfd56076c920/> use `gix_odb::Find*` traits in prelude, instead of `gix_pack::Find*`
   These are higher-level and generally more desirable.
   The Find traits in `gix-pack` are more useful internally when packs
   have to be handled directly, for example when generating packs.
 - <csr-id-83d7b31e7dd6d09eea79fc3c68620d099459132f/> rename easy::State to easy::Handle
   As the first step to remove the 'Easy' abstraction.
 - <csr-id-5e7aa1689f5d7ea5b510611a3ca0868828226291/> fully rely on OdbHandle in repository State
 - <csr-id-57de915886b76f80b3641def0ccf4fd79e334fc8/> Rename `Repository::odb` to` Repository::objects`
   This way it's more inline with `Repository::refs`.
 - <csr-id-93db4a5e70456d2c33ea010e3c86e5f26eb1bcc0/> remove Repository::refresh_object_database()
   With the linked DB this is simply not possible anymore and we expect
   these updates to happen automatically in future for greater convenience.
   
   For now, in order to refresh a repository, one has to reopen it.
 - <csr-id-580e96c1b2d9782a2e8cf9d1123f6d53a5376a3d/> Rename `Handle` to `Cache`
   Because this is exactly what it is effectively.
   Also add some basic instantiation for the new object store.
 - remove borrowing Repo as possible failure cause
   The `easy::Handle` is now a full (but shared) clone of the original
   Repository with additional thread-local state, hence there is no more
   need for a way to access the original repository.
 - remove Easy… abstraction in favor of Handle
   This great reduction of complexity allows for being multi-threading
   capable by default with the option to turn that off at compile time.
   
   All `to|into_easy…()` methods are removed in favor of `to_easy()`
   along with the removal of all `Easy` types in favor of the single
 - remove pack-cache from `Find::try_find(…)`
   With the new architecture this can be an implementation detail without
   forcing it to be Sync.
 - move gix_pack::data::Object to gix_object::Data, massively alter gix_odb::Find trait
   This will break a lot, but has to happen to prepare these traits for the
   next generation of object databases.

## 0.13.0 (2021-11-29)

<csr-id-951c050ecbb70c9de216603e55c7cfbc89a067e3/>
<csr-id-0e1875363fea09452789d7a90fc6860a7996d6d3/>

With changes to `gix-ref`, what follows is all the adjustments made to simplify the `gix` implementation.

### Changed (BREAKING)

 - <csr-id-5d498a33236391d8e456f267b1bf6af24de66f11/> file::Store::iter() is now a platform, with `.all()` and `.prefixed(…)` respectively
   This way, it's possible to keep shared ownership of the packed buffer
   while allowing the exact same iterator machinery to work as before.
 - <csr-id-15d429bb50602363292453606902bdce5042d9a5/> file::Store::(try_)find(…, packed) was removed
   The packed buffer is now handled internally while loading it on demand.
   When compiled with `gix-features/parallel` the `file::Store` remains
   send and sync.
   
   The packed refs buffer is shared across clones and it's recommended
   to clone one `file::Store` instance per thread, each of which can
   use its own namespace.
 - <csr-id-95247322a8191edfa7fac9c5aa72b40239f3aa88/> move `gix_ref::file::WriteRefLog` to `gix_ref::store::WriteRefLog`

### Bug Fixes (BREAKING)

 - <csr-id-fc8e85cd71d4f16bc8daad0b790d875045faefff/> ref namespaces are now thread-local
   Previously these were shared in the shared Repo instance, which makes
   threaded applications impossible to remain deterministic across multiple
   connections.
   
   Now they are local to the thread, which allowed some methods to remove
   their Result<> as they cannot fail anymore, the reason for this being
   a breaking change.

### Other (BREAKING)

 - <csr-id-951c050ecbb70c9de216603e55c7cfbc89a067e3/> Reference::logs() -> Reference::log_iter()
   The latter now returns a standard Platform to iterate over all
   reflog entries from oldest to newest or vice versa.

### Refactor (BREAKING)

 - <csr-id-0e1875363fea09452789d7a90fc6860a7996d6d3/> `file::Store::base` is now `file::Store::base()` and read-only
   That way, file databases can't be repositioned anymore, it's recommended
   to recreate it if that's desired.

## 0.12.0 (2021-11-16)

### New Features

 - <csr-id-b7aab9efd42975e8f2dcb5c97e51495996175702/> Allow `PartialNameRef` to be created from owned items

### Changed (BREAKING)

 - <csr-id-e8b091943f0c9a26317da0003f7fcdf5a56ef21a/> Rename gix->ein and gixp->gix

## v0.11.0 (2021-10-19)

A maintenance release to properly dealing with previously breaking changes in `gix-hash`.

## v0.10.0 (2021-10-15)

<csr-id-1cb41f81cffe19c75aadf49a5cc7ec390ec6cae7/>
<csr-id-2f2d856efe733d3cf81110c0e0607d2e7c40d968/>
<csr-id-a19567eceab0dd7f5478b83c2ff9ce79754db308/>
<csr-id-61793ff42f5c2f9ddf302901adea2dac6149eac8/>
<csr-id-0cd585e20a5abd323a34ec32d92fbd48531b3b18/>
<csr-id-89f15051763a03627f332c46beedfc53b8b9b15b/>
<csr-id-f644d0ede7a2e8d344a81c7003c3877eed64a6b0/>
<csr-id-ac3b9efb7b90958274ce55800959d930f8641115/>
<csr-id-03fe8a7ebd34608d725d4585da5c1630123762ec/>
<csr-id-8fe461281842b58aa11437445637c6e587bedd63/>
<csr-id-b209da29f361512ba757febf56bc1aca039f2a41/>
<csr-id-741558dd8194590c5cc8566aa22f96e73df38edf/>
<csr-id-e16603b15b5488b81563c583cd8f5292ab9d24a2/>
<csr-id-54a64a588ff72515451a3d0343306ac4abe1cb35/>
<csr-id-1f4e45a26a3d2727f00c3f248452dd41fc8a95be/>
<csr-id-1958e8aa65eb97f9755f065d713f0a48c5e41b1b/>
<csr-id-066f59b23a125b1ce9a015437a3f4468e5791da0/>
<csr-id-329d183ad4e256a4f9cdeb34589b5f3432495f79/>
<csr-id-1a1959f487d69ffdd5394775b707139c44dbd11d/>
<csr-id-5e091fb2b4fd33879c176e6dadd3c9805d99af50/>
<csr-id-e3760679547e0dc1bf31761acdb6e63b04a50919/>
<csr-id-de004b318fdc6923711dd001bff5f4bcbba4270e/>
<csr-id-41afad3386461b658ee859225785b6de86d13cfb/>
<csr-id-f582439a3efe5c234f54c488792395e9de09a032/>
<csr-id-42080aefe3b286afb58235c1c22491579ab73919/>
<csr-id-d422b9a31a37a03551bec4382039aaf3a7e49902/>
<csr-id-e7c061b10c263001eb4abf03098d6694b770f828/>
<csr-id-66292fd1076c2c9db4694c5ded09799a0be11a03/>
<csr-id-5aadf75a0d93d1a990ad0305c38366c5c22bdcb2/>
<csr-id-d79a1b75304e397c16b5af7055906591a187ddfd/>
<csr-id-7d2b6b66e09ff39727fccd68d190679b52d90126/>
<csr-id-06996e032b1e451a674395ebaca94434fac46f05/>
<csr-id-daec7167df524b329daad7dabb1b9920b6ef8936/>
<csr-id-4fe4786797d240a59d29dbf2c6310490a381c8b6/>
<csr-id-debe0094826f83839f907523715def929133fd58/>
<csr-id-56e39fac54bfa3871c42bbf76a9f7c49486b85be/>
<csr-id-293bfc0278c5983c0beaec93253fb51f00d81156/>
<csr-id-650241251a420602f74037babfc24c9f64df78d8/>
<csr-id-2b4a61589a7cba3f7600710e21304e731ae3b36a/>
<csr-id-8b82f7d44c7eb63b7922ddc31ada9cefdce776b0/>

### New Features

 - <csr-id-11b64fce4630371633b6415f227eecdc6b42b20b/> Make `gix_url::Url` available under `gix::Url`
 - <csr-id-80b8331092f4856f52afa1d85fa375ae688bdd28/> add easy::ext::ObjectAccessExt::tag(…) to create tag objects
   It's a quick sketch on how tag object creation could work.
   
   Note the duplication the method name using traits, which seems like a good solution
   to the problem of differentiating tag objects and tag references while
   keeping the method name short.
   
   Most will only ever need one, right?
   
   Even in my example that's not the case, so maybe we have to rename it.
 - <csr-id-0ebfeb614264ca06ab763189e55e6c016c9997af/> Make `gix_url::Url` available under `gix::Url`

### BREAKING Changes

 - Use 'to_*' when converting `easy::Object` to specific object kind
   This also makes the API more consistent while being more idiomatic.
 - Avoid duplicate module paths in 'tree' and 'commit'
 - rename ObjectIdExt::ancestors_iter() to *::ancestors()
 - rename `easy::Object::to_(commit|tag)_iter()`…
   …to  `easy::Object::try_to_(commit|tag)_iter()` for consistency.
 - rename `*::State` into `*::Platform`
 - various small API changes
 - move easy::head::peel::Error -> easy::head::peel::to_id::Error
 - rename path::is_git to path::is
 - rename easy::reference::log::State to easy::reference::Logs

### Other

 - <csr-id-293bfc0278c5983c0beaec93253fb51f00d81156/> loose reference iteration with non-dir prefixes…
   Previously it was expected for the prefix `Path` to always exist for
   the prefix to be valid. This, however, is not similar to packed
   prefixes, which allow non-dir prefixes as well.
   
   Now we will check if the prefix is actually a directory, and if not
   split it into its parent directory and the filename portion. The latter
   is then used for prefix matching file names within that directory.
 - <csr-id-650241251a420602f74037babfc24c9f64df78d8/> Add 'references().all().peeled().'…
   …to not only make typical usage of iterated references more convenient
   but also work around a double-borrow error one would see otherwise.
 - <csr-id-2b4a61589a7cba3f7600710e21304e731ae3b36a/> filter refs correctly, but…
   …it needs a way to peel references right away without trying
   to double-borrow. This means the Iterator needs to implement this.
 - <csr-id-8b82f7d44c7eb63b7922ddc31ada9cefdce776b0/> improved changelog…
   …akin to 'Keep a changelog'.

### Refactor

 - <csr-id-8fe461281842b58aa11437445637c6e587bedd63/> split data::output::count::objects into files
 - <csr-id-b209da29f361512ba757febf56bc1aca039f2a41/> use new gix_pack::cache::Object trait
 - <csr-id-741558dd8194590c5cc8566aa22f96e73df38edf/> remove object cache impl which now lives in gix-pack

### Other

 - <csr-id-e16603b15b5488b81563c583cd8f5292ab9d24a2/> :remote_url() is now optional
   Otherwise it wouldn't work on repos that don't have a remote set yet.
   Instead of failing, we don't create links.
 - <csr-id-54a64a588ff72515451a3d0343306ac4abe1cb35/> try to create persistent Easy iterator, but can't make it Send…
   …which is fair as it contains borrowed RefCells, which really would have
   to be owned to work for this, which would in turn require the Ancestor's
   struct to be kind of self-referential
 - <csr-id-1f4e45a26a3d2727f00c3f248452dd41fc8a95be/> path::is
 - <csr-id-1958e8aa65eb97f9755f065d713f0a48c5e41b1b/> path::discover
 - <csr-id-066f59b23a125b1ce9a015437a3f4468e5791da0/> top-level of 'path' module
 - <csr-id-329d183ad4e256a4f9cdeb34589b5f3432495f79/> object_id
 - <csr-id-1a1959f487d69ffdd5394775b707139c44dbd11d/> repository
 - <csr-id-5e091fb2b4fd33879c176e6dadd3c9805d99af50/> ext::tree
 - <csr-id-e3760679547e0dc1bf31761acdb6e63b04a50919/> easy::object::peel
 - <csr-id-de004b318fdc6923711dd001bff5f4bcbba4270e/> easy::object::errors
 - <csr-id-41afad3386461b658ee859225785b6de86d13cfb/> a seemingly slow version of path lookup, but…
   …in debug mode it's faster than the fast path, despite doing more
   and being the same when it comes to searching path components.
 - <csr-id-f582439a3efe5c234f54c488792395e9de09a032/> easy::object, sans a few child-modules
 - <csr-id-42080aefe3b286afb58235c1c22491579ab73919/> update 'platform' information to reflect the current usage
 - <csr-id-d422b9a31a37a03551bec4382039aaf3a7e49902/> configure caches with env vars using `apply_environment()`
 - <csr-id-e7c061b10c263001eb4abf03098d6694b770f828/> refactor
 - <csr-id-66292fd1076c2c9db4694c5ded09799a0be11a03/> set package cache via RepositoryAccessExt
 - <csr-id-5aadf75a0d93d1a990ad0305c38366c5c22bdcb2/> Add GITOXIDE_PACK_CACHE_MEMORY_IN_BYTES=536870912 to control pack-cache size…
   …which can mean another considerable speed-up for many workloads, but
   usually needs some knowledge about the application, repos, and should
   thus be with the user.
 - <csr-id-d79a1b75304e397c16b5af7055906591a187ddfd/> allow disabling the pack cache with GITOXIDE_DISABLE_PACK_CACHE
 - <csr-id-7d2b6b66e09ff39727fccd68d190679b52d90126/> prepare for configurable pack cache
 - <csr-id-06996e032b1e451a674395ebaca94434fac46f05/> object-cache to allow for a speed boost…
   …by avoiding duplicate accesses to hit the object database.
   However, the cost for the cache are relatively high and involve some
   memory copying, so hit rates of about 50% is certainly what is needed
   to get any speed boost at all.
 - <csr-id-daec7167df524b329daad7dabb1b9920b6ef8936/> build commit history for later use in changelog generation
 - <csr-id-4fe4786797d240a59d29dbf2c6310490a381c8b6/> Allow object access during commit ancestor traversal…
   …by getting only a temporary handle to the pack-cache. The cost of this
   should be negligible compared to the cost of object decoding.
 - <csr-id-debe0094826f83839f907523715def929133fd58/> sketch history acquisition
 - <csr-id-56e39fac54bfa3871c42bbf76a9f7c49486b85be/> add 'Head::peeled()' method

### Changed (BREAKING)

 - <csr-id-c3385cd144298eb9f06d7751d180e26da7b4d338/> `easy::Object::try_to_commit()` now returns `Result<CommitRef>`…
   …without the nested `Option`, folding the type mismatch into a specific
   `conversion::Error` instead.
 - <csr-id-e59f901f47fb0180211494a1591aed62b856406a/> rename `ObjectAccessExt::tag(…)` to `*::tag_reference(…)`, add `easy::Object::try_to_tag()`
   This one also contains the first and probably only test for tag object
   creation.

## v0.9.1 (2021-09-10)

<csr-id-293bfc0278c5983c0beaec93253fb51f00d81156/>
<csr-id-650241251a420602f74037babfc24c9f64df78d8/>
<csr-id-2b4a61589a7cba3f7600710e21304e731ae3b36a/>
<csr-id-8b82f7d44c7eb63b7922ddc31ada9cefdce776b0/>

- Remove `max-performance` feature from default set until the `msvc` build issue is fixed. Otherwise it will surprisingly break windows builds.

### Other

 - <csr-id-293bfc0278c5983c0beaec93253fb51f00d81156/> loose reference iteration with non-dir prefixes…
   Previously it was expected for the prefix `Path` to always exist for
   the prefix to be valid. This, however, is not similar to packed
   prefixes, which allow non-dir prefixes as well.
   
   Now we will check if the prefix is actually a directory, and if not
   split it into its parent directory and the filename portion. The latter
   is then used for prefix matching file names within that directory.
 - <csr-id-650241251a420602f74037babfc24c9f64df78d8/> Add 'references().all().peeled().'…
   …to not only make typical usage of iterated references more convenient
   but also work around a double-borrow error one would see otherwise.
 - <csr-id-2b4a61589a7cba3f7600710e21304e731ae3b36a/> filter refs correctly, but…
   …it needs a way to peel references right away without trying
   to double-borrow. This means the Iterator needs to implement this.
 - <csr-id-8b82f7d44c7eb63b7922ddc31ada9cefdce776b0/> improved changelog…
   …akin to 'Keep a changelog'.

## v0.9.0 (2021-09-08)

- rename `prelude::ConfigAccessExt` to `prelude::RepositoryAccessExt`
- `prelude::ObjectAccessExt::commit()` signature change
- cargo feature changed in incompatible ways. `network` was replaced by more finegrained options for _blocking_ and _async_ networking, as well as optional http transport

### New

- `init()`
- `init_bare()`
- `Repository::init(Kind)`
- `open()`
- `Repository::open()`
- `easy::Head`
- `easy::ext::ReferenceAccessExt::head()`
- `ext::ReferenceExt` trait

### Breaking
- **renames / moves / Signature Changes**
    - `path::Path` to `Path`
    - `init::repository(dir)` -> `path::create::into(dir, **Kind**)`

## v0.8.2 (2021-09-07)

## v0.8.1 (2021-08-28)

- Introduce `EasyArcExclusive` type, now available thanks to `parking_lot` 0.11.2

## v0.8.0 (2021-08-27)

- Rename `object` to `objs` to be equivalent to `refs` and make space for the new `object` module
- various minor version updates of pre-release dependencies

## v0.7.2 (2021-08-17)

## v0.7.1 (2021-08-13)

## v0.7.0 (2021-08-10)

## v0.6.0 (2021-05-28)

## v0.5.0 (2021-04-08)

## v0.4.0 (2020-09-12)

## v0.3.0 (2020-08-12)

## v0.1.0 (2020-07-12)

## 0.0.0 (2023-02-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix v0.0.0 ([`8bce6d5`](https://github.com/Byron/gitoxide/commit/8bce6d5cba12630bf4d12ed92f572a379d945329))
    - Add `gix` crate to reserve name ([`5104a78`](https://github.com/Byron/gitoxide/commit/5104a787127bf0b1f9b65f371b7c5b79f491e396))
</details>

