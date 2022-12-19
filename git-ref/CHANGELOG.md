# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.21.0 (2022-12-19)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 27 calendar days.
 - 27 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
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
    - thanks clippy ([`f1160fb`](https://github.com/Byron/gitoxide/commit/f1160fb42acf59b37cbeda546a7079af3c9bc050))
    - adapt to changes in `git-features::fs`. ([`35f7d59`](https://github.com/Byron/gitoxide/commit/35f7d5960210738d88d35aef9c1ed3480681c481))
    - adjust to changes in `git-testtools` ([`4eb842c`](https://github.com/Byron/gitoxide/commit/4eb842c7150b980e1c2637217e1f9657a671cea7))
    - Release git-hash v0.10.1, git-hashtable v0.1.0 ([`7717170`](https://github.com/Byron/gitoxide/commit/771717095d9a67b0625021eb0928828ab686e772))
    - Merge branch 'main' into http-config ([`6b9632e`](https://github.com/Byron/gitoxide/commit/6b9632e16c416841ffff1b767ee7a6c89b421220))
    - Release git-features v0.24.1, git-actor v0.14.1, git-index v0.9.1 ([`7893502`](https://github.com/Byron/gitoxide/commit/789350208efc9d5fc6f9bc4f113f77f9cb445156))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - make fmt ([`0abab7d`](https://github.com/Byron/gitoxide/commit/0abab7da2ec1b8560e6c1eb009f534c9fc7814fe))
</details>

## 0.20.0 (2022-11-21)

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

 - 4 commits contributed to the release over the course of 2 calendar days.
 - 4 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
</details>

## 0.19.0 (2022-11-17)

### New Features

 - <csr-id-e86e159e00c9b54803abbfa09809707be7ac8aee/> `file::Transaction::rollback()` allows to explicitly roll back a pending change.
   As opposed to dropping the Transaction, this method allows to obtain all
   edits that would have been applied.
 - <csr-id-bbdb4804d8c3bd6a1fb8bea97adce509c90c5ca8/> higher performance for edits which would write the same value.
   Instead of moving them into place, we just drop them, without ever
   writing into them.

### Bug Fixes

 - <csr-id-584b705cee8be3fb68c67dcb8535b981d1efc5f4/> assure symrefs don't get deleted when moving refs to packed-refs.
   Previously it was possible for symbolic refs to be deleted right after
   they have been created or updated as they were included in the set of
   refs that was assumed to be part of packed-refs, which isn't the case
   for symbolic refs.
 - <csr-id-9f848506f5a42abc954612ea375f845e3b23ae5a/> case-insentively conflicting references can be created even on case-insensitie filesystems*.
   The asterisk indicates that this only works if packed-refs are present
   and these references are written straight to packed references without
   ever trying to handle the otherwise conflicting loose reference files.
   
   This is done by leveraging the fact that in presence of packed-refs
   or a pending creation of packed-refs, there is no need to create
   per-file locks as concurrent transactions also have to obtain the
   packed-refs lock and fail (or wait) until it's done.
 - <csr-id-e9853dd640cf4545134aa6e0d093e560af090a2b/> instead of erroring if loose iteration is performed on missing base, correctly yield zero references.
   Previously it reported an error, now it does not and instead performs no
   iteration, which is more helpful to the user of the API I believe as
   they won't randomly fail just because somebody deleted the `refs`
   folder.
 - <csr-id-27386a96ddc022ba75730901f8bb098b9d5ff9d4/> loose ref iteration on a repo with missing 'ref/' fails when creating the iterator.
   Previously, it would fail on first iteration, making it seem like there
   is one reference even though it's just an error stating that the base
   cannot be read.
   
   This is clearly worse than making a metadata check on the filesystem,
   no matter how unlikely the case.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 19 commits contributed to the release over the course of 1 calendar day.
 - 10 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#595](https://github.com/Byron/gitoxide/issues/595)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#595](https://github.com/Byron/gitoxide/issues/595)**
    - assure symrefs don't get deleted when moving refs to packed-refs. ([`584b705`](https://github.com/Byron/gitoxide/commit/584b705cee8be3fb68c67dcb8535b981d1efc5f4))
    - Avoid lock-acquisition for refs which are to be deleted if a global lock is helt. ([`66b053d`](https://github.com/Byron/gitoxide/commit/66b053dd070cc05dbcec9b251bfab32a00f75b68))
    - case-insentively conflicting references can be created even on case-insensitie filesystems*. ([`9f84850`](https://github.com/Byron/gitoxide/commit/9f848506f5a42abc954612ea375f845e3b23ae5a))
    - instead of erroring if loose iteration is performed on missing base, correctly yield zero references. ([`e9853dd`](https://github.com/Byron/gitoxide/commit/e9853dd640cf4545134aa6e0d093e560af090a2b))
    - loose ref iteration on a repo with missing 'ref/' fails when creating the iterator. ([`27386a9`](https://github.com/Byron/gitoxide/commit/27386a96ddc022ba75730901f8bb098b9d5ff9d4))
    - First test to validate how collisions are expressed. ([`3f54ade`](https://github.com/Byron/gitoxide/commit/3f54ade216cfdfbba8d4a74f544ccf0436225d46))
    - Attempt to add the first case-sensitive test… ([`063ab73`](https://github.com/Byron/gitoxide/commit/063ab73d77a480191d10338964f4a6209aec3cb6))
    - `file::Transaction::rollback()` allows to explicitly roll back a pending change. ([`e86e159`](https://github.com/Byron/gitoxide/commit/e86e159e00c9b54803abbfa09809707be7ac8aee))
    - higher performance for edits which would write the same value. ([`bbdb480`](https://github.com/Byron/gitoxide/commit/bbdb4804d8c3bd6a1fb8bea97adce509c90c5ca8))
 * **Uncategorized**
    - Release git-glob v0.4.2, git-config-value v0.8.2, git-lock v2.2.0, git-ref v0.19.0, git-config v0.11.0, git-discover v0.8.0, git-index v0.8.0, git-transport v0.22.0, git-protocol v0.23.0, git-worktree v0.8.0, git-repository v0.28.0, gitoxide-core v0.20.0, gitoxide v0.18.0, safety bump 9 crates ([`0c253b1`](https://github.com/Byron/gitoxide/commit/0c253b15143dcedfe4c66d64ab1ea6e097030651))
    - prepare changelogs prior to release ([`fe5721f`](https://github.com/Byron/gitoxide/commit/fe5721f888c64c79fe9a734a9e33b94a282f8d97))
    - Merge branch 'http-config' ([`665b53e`](https://github.com/Byron/gitoxide/commit/665b53e1c2e1de65fafa28b669f58977868bbc81))
    - Don't assert on state that is based on a transaction that isn't committed ([`00f6f7a`](https://github.com/Byron/gitoxide/commit/00f6f7a2d056d150306817b3563470173a091b4c))
    - thanks clippy ([`fe7d6f9`](https://github.com/Byron/gitoxide/commit/fe7d6f91ad6c8a0b0beca9faa8230537d2fd6a3c))
    - Assure reflogs aren't skipped just because there is no per-loose lock file. ([`130d13b`](https://github.com/Byron/gitoxide/commit/130d13bbf1b4b2da8f688a440f3e2f3b1a51519f))
    - refactor ([`f17c6b6`](https://github.com/Byron/gitoxide/commit/f17c6b649d9e0bed59c4e6d8380c3dcdfd73a2f9))
    - refactor ([`c1d2aea`](https://github.com/Byron/gitoxide/commit/c1d2aea68a2c57f5d498987c51fe2806f669eaaa))
    - refactor ([`b0a231a`](https://github.com/Byron/gitoxide/commit/b0a231aaca5cf371e2a204bf3b3100a4a7cc913e))
    - thanks clippy ([`5f7fe69`](https://github.com/Byron/gitoxide/commit/5f7fe698e0ea322a731f8e86e724be327e9d3420))
</details>

## 0.18.0 (2022-11-06)

### Bug Fixes

 - <csr-id-6e5c0ae63deed181419232c61896e22404e4c84a/> allow symref updates to receive reflogs if these are new…
   …and well-known on top of that, that means having an exact expectation
   on what should be present.
   
   This allows symrefs to be created with reflog.

### Bug Fixes (BREAKING)

 - <csr-id-328900add089f40d3bf9f1019c9f31663da387e2/> support for non-'static PackedRefs transactions.
   When configuring for packed-refs updates, previously one needed to
   provide a function to find objects that could not borrow data due
   to implicit 'static requirement. This has been lifted to allow
   it to access references to data on the stack.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 6 calendar days.
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
    - support for non-'static PackedRefs transactions. ([`328900a`](https://github.com/Byron/gitoxide/commit/328900add089f40d3bf9f1019c9f31663da387e2))
    - add test to validate new symref reflog behaviour ([`8b5cf38`](https://github.com/Byron/gitoxide/commit/8b5cf385cb6cf31ad438398ddcc5668a01b6fb3d))
    - allow symref updates to receive reflogs if these are new… ([`6e5c0ae`](https://github.com/Byron/gitoxide/commit/6e5c0ae63deed181419232c61896e22404e4c84a))
 * **Uncategorized**
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - prepare changelogs prior to release ([`423af90`](https://github.com/Byron/gitoxide/commit/423af90c8202d62dc1ea4a76a0df6421d1f0aa06))
    - Merge branch 'main' into write-sparse-index (upgrade to Rust 1.65) ([`5406630`](https://github.com/Byron/gitoxide/commit/5406630466145990b5adbdadb59151036993060d))
    - thanks clippy ([`04cfa63`](https://github.com/Byron/gitoxide/commit/04cfa635a65ae34ad6d22391f2febd2ca7eabca9))
    - Merge branch 'main' into write-sparse-index ([`c4e6849`](https://github.com/Byron/gitoxide/commit/c4e68496c368611ebe17c6693d06c8147c28c717))
    - Merge branch 'gix-clone' ([`def53b3`](https://github.com/Byron/gitoxide/commit/def53b36c3dec26fa78939ab0584fe4ff930909c))
    - thanks clippy ([`3eaedda`](https://github.com/Byron/gitoxide/commit/3eaedda0af1c1a495e1b98474ade02edfa75b66a))
</details>

## 0.17.0 (2022-10-10)

### New Features

 - <csr-id-370ed3dcc393eca7a393ea0150f698a9fc844320/> `transaction::Change::new_value()` to get easy access to new values of references.
   That's more convenient than matching on the enum.
 - <csr-id-658c1257c073507327d9a50c1c89b49d17e9ccbc/> `FullName::try_from(&BString)` for convenience.
   Sometimes when matching one only has a `&BString`, and it's hard to
   convert it to `&BStr` without an extra line of code, it's cumbersome,
   so we workaround by adding another conversion.

### Changed (BREAKING)

 - <csr-id-e699291097cec346374a30c325848f787ca9d736/> `file::Transaction::prepare()` now takes two `git_lock::acquisition::Fail` instances.
   This allows to configure the file-ref lock failure mode differently from
   the packed-refs lock failure mode, which is exactly what `git` does as
   well defaulting them to 100ms and 1000ms till lock acquisition gives up.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 20 calendar days.
 - 20 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - `transaction::Change::new_value()` to get easy access to new values of references. ([`370ed3d`](https://github.com/Byron/gitoxide/commit/370ed3dcc393eca7a393ea0150f698a9fc844320))
    - `file::Transaction::prepare()` now takes two `git_lock::acquisition::Fail` instances. ([`e699291`](https://github.com/Byron/gitoxide/commit/e699291097cec346374a30c325848f787ca9d736))
    - `FullName::try_from(&BString)` for convenience. ([`658c125`](https://github.com/Byron/gitoxide/commit/658c1257c073507327d9a50c1c89b49d17e9ccbc))
    - Improve docs slightly ([`4850202`](https://github.com/Byron/gitoxide/commit/485020252da95b1369326156ebd8ff6052f591ec))
 * **Uncategorized**
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - Merge branch 'main' into new-http-impl ([`702a161`](https://github.com/Byron/gitoxide/commit/702a161ef11fc959611bf44b70e9ffe04561c7ad))
    - make fmt ([`53acf25`](https://github.com/Byron/gitoxide/commit/53acf2565743eff7cead7a42011107b2fc8d7e0e))
    - Merge branch 'fetch-pack' ([`3c49400`](https://github.com/Byron/gitoxide/commit/3c49400809c7c2120f4ce704c19a0421545b5acd))
    - Merge branch 'fix-git-features' ([`82fd251`](https://github.com/Byron/gitoxide/commit/82fd251ac80d07bc9da8a4d36e517aa35580d188))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
</details>

## 0.16.0 (2022-09-20)

<csr-id-725210dc401406fe9450eae9d375b0238d645027/>

### Chore (BREAKING)

 - <csr-id-725210dc401406fe9450eae9d375b0238d645027/> replace `quick-error` with `thiserror`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 19 commits contributed to the release over the course of 22 calendar days.
 - 22 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - better refmap printing ([`6f60a79`](https://github.com/Byron/gitoxide/commit/6f60a793297e2a29cf835591add6669c067da3e5))
    - fix windows tests ([`140e690`](https://github.com/Byron/gitoxide/commit/140e6903d9bd9d9b393b717988f00e42c52a4d36))
    - fix docs ([`dad9cbe`](https://github.com/Byron/gitoxide/commit/dad9cbeb853c0cc5128360b05c04b5a3da7ec75e))
    - replace `quick-error` with `thiserror` ([`725210d`](https://github.com/Byron/gitoxide/commit/725210dc401406fe9450eae9d375b0238d645027))
    - adjust to changes in `git-validate` ([`4eac45f`](https://github.com/Byron/gitoxide/commit/4eac45f89d4581a7be8eedcc931512cd52c255a9))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **Uncategorized**
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`3773b92`](https://github.com/Byron/gitoxide/commit/3773b92b8372c9a40a74d281149ca65b057a7da9))
    - Release git-features v0.22.6 ([`c9eda72`](https://github.com/Byron/gitoxide/commit/c9eda729d8f8bc266c7516c613d38acfb83a4743))
    - Merge branch 'filter-refs' ([`e10554d`](https://github.com/Byron/gitoxide/commit/e10554d2a3b9c027353a432b0c84f7d3797b7cae))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - Merge branch 'main' into filter-refs-by-spec ([`9aa1d3d`](https://github.com/Byron/gitoxide/commit/9aa1d3dc46d4b1c76af257f573aff3aeef2d3fa8))
    - Release git-features v0.22.4, git-url v0.8.0, safety bump 4 crates ([`1d4600a`](https://github.com/Byron/gitoxide/commit/1d4600ae51475c2e225f96c16c41e2c4a2b3f2aa))
    - Merge branch 'main' into filter-refs-by-spec ([`1f6e5ab`](https://github.com/Byron/gitoxide/commit/1f6e5ab15f5fd8d23719b13e6aea59cd231ac0fe))
    - Merge branch 'fix-522' ([`5869e9f`](https://github.com/Byron/gitoxide/commit/5869e9ff2508d5a93c07635277af8764fcb57713))
    - Release git-hash v0.9.9 ([`da0716f`](https://github.com/Byron/gitoxide/commit/da0716f8c27b4f29cfff0e5ce7fcb3d7240f4aeb))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Release git-path v0.4.2, git-config-value v0.7.0 ([`c48fb31`](https://github.com/Byron/gitoxide/commit/c48fb3107d29f9a06868b0c6de40567063a656d1))
    - Merge branch 'main' into filter-refs-by-spec ([`cef0b51`](https://github.com/Byron/gitoxide/commit/cef0b51ade2a3301fa09ede7a425aa1fe3527e78))
</details>

## 0.15.4 (2022-08-28)

### New Features

 - <csr-id-2d0b63997b276a53b3cf8f09fac51f8e3f044bcd/> Add `Reference::delete()` for simple reference deletion

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#XXX](https://github.com/Byron/gitoxide/issues/XXX)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#XXX](https://github.com/Byron/gitoxide/issues/XXX)**
    - prepare changelogs prior to release ([`8c0bca3`](https://github.com/Byron/gitoxide/commit/8c0bca37ff9fbaadbe55561fb2b0d649980c95b1))
 * **Uncategorized**
    - Release git-object v0.20.3, git-ref v0.15.4, git-config v0.7.1, git-diff v0.18.0, git-traverse v0.16.3, git-pack v0.22.0, git-odb v0.32.0, git-url v0.7.3, git-transport v0.19.3, git-protocol v0.19.1, git-refspec v0.1.1, git-repository v0.23.0, safety bump 6 crates ([`85a3bed`](https://github.com/Byron/gitoxide/commit/85a3bedd68d2e5f36592a2f691c977dc55298279))
    - Add `Reference::delete()` for simple reference deletion ([`2d0b639`](https://github.com/Byron/gitoxide/commit/2d0b63997b276a53b3cf8f09fac51f8e3f044bcd))
</details>

## 0.15.3 (2022-08-27)

Maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-attributes v0.3.3, git-ref v0.15.3, git-index v0.4.3, git-worktree v0.4.3, git-testtools v0.8.0 ([`baad4ce`](https://github.com/Byron/gitoxide/commit/baad4ce51fe0e8c0c1de1b08148d8303878ca37b))
    - prepare changelogs prior to release of git-testtools ([`7668e38`](https://github.com/Byron/gitoxide/commit/7668e38fab8891ed7e73fae3a6f5a8772e0f0d0b))
    - Release git-features v0.22.3, git-revision v0.4.4 ([`c2660e2`](https://github.com/Byron/gitoxide/commit/c2660e2503323531ba02519eaa51124ee22fec51))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
</details>

## 0.15.2 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### New Features

 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 5 calendar days.
 - 6 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Release git-path v0.4.1 ([`5e82346`](https://github.com/Byron/gitoxide/commit/5e823462b3deb904f5d6154a7bf114cef1988224))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - use docsrs feature in code to show what is feature-gated automatically on docs.rs ([`b1c40b0`](https://github.com/Byron/gitoxide/commit/b1c40b0364ef092cd52d03b34f491b254816b18d))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - pass --cfg docsrs when compiling for https://docs.rs ([`5176771`](https://github.com/Byron/gitoxide/commit/517677147f1c17304c62cf97a1dd09f232ebf5db))
    - remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
</details>

## 0.15.1 (2022-08-17)

### Bug Fixes

 - <csr-id-1d2003519e3b0f745af1524a32f2816475024b31/> always update modification date when packed refs are forcefully reloaded.
   The latter happens right after the file was written, which means we
   definitely have to update our modification date or else the buffer
   will be read again next time somebody asks for an up-to-date version.
   
   The ordering of operations is such that at worst, the buffer is loaded
   again instead of keeping an outdated version of it, which would happen
   if the metadata is read afterwards.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 21 commits contributed to the release over the course of 26 calendar days.
 - 26 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#427](https://github.com/Byron/gitoxide/issues/427)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - make fmt ([`4b320e7`](https://github.com/Byron/gitoxide/commit/4b320e773368ac5e8c38dd8a779ef3d6d2d024ec))
    - adapt to changes in git-features ([`777b40c`](https://github.com/Byron/gitoxide/commit/777b40cc33f0cc952bc2a7cbb01c0c3c1b261e5b))
    - Adjust to changes in `git-features` ([`07168c7`](https://github.com/Byron/gitoxide/commit/07168c704c1f8ba1eeb4dcec65b7d34ddca3e147))
    - Use generalized reload-on-demand in `git-ref` ([`8d0cce7`](https://github.com/Byron/gitoxide/commit/8d0cce7d1521374d5199552fc69a417a957519bc))
    - Now it's possible to update packed refs using the shared code ([`78222c2`](https://github.com/Byron/gitoxide/commit/78222c2e39aa24c84852e999448c042f2fd37db4))
    - The first step towards using the generalized `ReloadIfChanged` in git-ref ([`e8de0ef`](https://github.com/Byron/gitoxide/commit/e8de0ef38db2f2d83cb277ed101464f23c0e98e4))
    - always update modification date when packed refs are forcefully reloaded. ([`1d20035`](https://github.com/Byron/gitoxide/commit/1d2003519e3b0f745af1524a32f2816475024b31))
 * **Uncategorized**
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge branch 'main' into write-index-v2 ([`a938986`](https://github.com/Byron/gitoxide/commit/a938986877302c197d1aed087594c5605416fe5f))
    - Merge branch 'main' into remote-ls-refs ([`de61c4d`](https://github.com/Byron/gitoxide/commit/de61c4db7855d6925d66961f62ae3d12cc4acf78))
    - thanks clippy ([`4bd747c`](https://github.com/Byron/gitoxide/commit/4bd747cb3e126fe5b1d540270cfbd731cffd42ef))
    - Merge branch 'rev-parse-delegate' ([`2f506c7`](https://github.com/Byron/gitoxide/commit/2f506c7c2988477b0f97d272a9ac9ed47b236457))
    - thanks clippy ([`763cec8`](https://github.com/Byron/gitoxide/commit/763cec829da1e49830a9a52d25a45c07d3d0dba5))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Merge branch 'kianmeng-fix-typos' ([`4e7b343`](https://github.com/Byron/gitoxide/commit/4e7b34349c0a01ad8686bbb4eb987e9338259d9c))
    - Fix typos ([`e9fcb70`](https://github.com/Byron/gitoxide/commit/e9fcb70e429edb2974afa3f58d181f3ef14c3da3))
</details>

## 0.15.0 (2022-07-22)

### New Features

 - <csr-id-4607a18e24b8270c182663a434b79dff8761db0e/> Add `store::WriteRefLog::Always` to unconditionally write reflogs.
 - <csr-id-f3c609f0af00aa280649975b0705a3cfaad8a0e5/> Allow `Reference` to be serialized/deserialized with `serde`.

### Changed (BREAKING)

 - <csr-id-0f753e922e313f735ed267f913366771e9de1111/> `Target(Ref)?::try_name()` now returns `Option<&FullNameRef>`.
   That way, the name is actually directly usable in most methods that
   require a validated name as input.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 36 calendar days.
 - 39 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#331](https://github.com/Byron/gitoxide/issues/331), [#427](https://github.com/Byron/gitoxide/issues/427)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - `Target(Ref)?::try_name()` now returns `Option<&FullNameRef>`. ([`0f753e9`](https://github.com/Byron/gitoxide/commit/0f753e922e313f735ed267f913366771e9de1111))
    - Add `store::WriteRefLog::Always` to unconditionally write reflogs. ([`4607a18`](https://github.com/Byron/gitoxide/commit/4607a18e24b8270c182663a434b79dff8761db0e))
 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - Allow `Reference` to be serialized/deserialized with `serde`. ([`f3c609f`](https://github.com/Byron/gitoxide/commit/f3c609f0af00aa280649975b0705a3cfaad8a0e5))
 * **Uncategorized**
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge branch 'config-cascade' ([`f144eaf`](https://github.com/Byron/gitoxide/commit/f144eaf5863ae5cac63103f0db51c35fcf03a948))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Merge branch 'config-comfort' ([`84b98d9`](https://github.com/Byron/gitoxide/commit/84b98d94177ceaf931aaa521e44eca0fa484d2d3))
    - assure document-features are available in all 'usable' and 'early' crates ([`238581c`](https://github.com/Byron/gitoxide/commit/238581cc46c7288691eed37dc7de5069e3d86721))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`daa71c3`](https://github.com/Byron/gitoxide/commit/daa71c3b753c6d76a3d652c29237906b3e28728f))
    - thanks clippy ([`e1003d5`](https://github.com/Byron/gitoxide/commit/e1003d5fdee5d4439c0cf0286c67dec9b5e34f53))
    - generally avoid using `target_os = "windows"` in favor of `cfg(windows)` and negations ([`91d5402`](https://github.com/Byron/gitoxide/commit/91d54026a61c2aae5e3e1341d271acf16478cd83))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-path v0.3.0, safety bump 14 crates ([`400c9be`](https://github.com/Byron/gitoxide/commit/400c9bec49e4ec5351dc9357b246e7677a63ea35))
</details>

## 0.14.0 (2022-06-13)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 25 calendar days.
 - 25 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - make fmt ([`c665aef`](https://github.com/Byron/gitoxide/commit/c665aef4270c5ee54da89ee015cc0affd6337608))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
</details>

## 0.13.0 (2022-05-18)

### New Features

 - <csr-id-4a5176a78b64981ce5612b88c7736b7323aa8cdd/> ref iteration for worktrees.
   It merges the iteration result of private worktree refs along with
   all shared common references references.
 - <csr-id-9cccce35e527cdda58e01b03cd335a527418cf14/> `Category::LinkedRef`
   With it one can access all refs as advertised.
 - <csr-id-eada5dfdd4981dbb032e8f155e3829eb5d1f380f/> `Category::MainRef`
   With it it will be possible to compute all paths correctly and actually
   perform certain operations as documented.
 - <csr-id-0304b7fad3ae5e3ac1fea71e30658474a770184b/> `Category::(WorktreePrivate|Bisect|Rewritten)`
 - <csr-id-612a2dbf7f70a8eb44b7278fccd4f1589749968a/> `Category::LinkedPseudoRef`
 - <csr-id-613b5844e74535d391339bd8e7c106e18257b917/> Add `Category::MainPseudoRef`
 - <csr-id-6ccfea97d30dbfa013e1633c77c5a26ab5cbaf8f/> `Category::PseudoRef`

### Changed (BREAKING)

 - <csr-id-3d6299f47f41397c1c72035a86b94d1c263b5b98/> `Transaction::commit(…)` takes `git-actor::SignatureRef` instead of `…::Signature`.
   This makes the API more versatile and corrects a shortcoming that was
   caused by `SignatureRef` previously being unable to serialize itself.
 - <csr-id-f1dbb6bd4534527b6f1f2aba2a562dd4e64cf55d/> remove `FullName::to_ref()` in favor of `•::as_ref()`.
   This became possible now that `FullNameRef` is a proper ref, not just
   ref-like.
 - <csr-id-1611c3ddff6c930deaa4c2440383f5684c029b28/> rename `PartialNameRef` to `PartialNameCow`
   Because this is what it is, which also implies that it's not `Copy`
   anymore which a `Ref` would definitely be.
   
   The reason we need this to be a `Cow` is to support passing ownership.
 - <csr-id-8a92ec9834b6d5aa3057c5509f6c13b6a6cd6e1b/> remove `Store` from public API
   It is unclear if ref-tables, which are the reason for it to exist in
   the first place, will fit into the concept as they might not support
   worktrees. It's entirely unclear how this works.
   
   Maybe there can be a non-worktree version of the store with work-trees
   only being supported by the file based ref database, and ref-tables
   remaining a server-side feature.
 - <csr-id-2becffc85ff6225522fe38482739fb1406ae1060/> rename `file::Store::base()` to `git_dir()`.
   That way it is clearer what it actually is especially in presence
   of the newly added `file::Store::common_dir()` method.
   
   That way, work-trees can eventually be properly supported.

### New Features (BREAKING)

 - <csr-id-0ace957c595c8a38afb7de1462cdc73b617d2a76/> Turn `FullNameRef` into an actual reference type.
   That way `Cow<'_, FullNameRef>` works as expected.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 116 commits contributed to the release over the course of 42 calendar days.
 - 43 days passed between releases.
 - 13 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#384](https://github.com/Byron/gitoxide/issues/384), [#393](https://github.com/Byron/gitoxide/issues/393)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 8 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - Enforce path conversion on windows gnu, it doesn't seem to like slashes ([`4d55a8f`](https://github.com/Byron/gitoxide/commit/4d55a8f99f2a0b7c0c4ed70a615b7e58b5bee04b))
    - mention that failing path when a ref-file couldn't be read ([`ecb539a`](https://github.com/Byron/gitoxide/commit/ecb539a4c12696ecb4384af2a16c381fb7980d95))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - ref iteration for worktrees. ([`4a5176a`](https://github.com/Byron/gitoxide/commit/4a5176a78b64981ce5612b88c7736b7323aa8cdd))
    - down to one borrowcheck error ([`7571be5`](https://github.com/Byron/gitoxide/commit/7571be5388d766afd88f224a6eab652659d38bfe))
    - possibly a step towards solving this traversal… ([`c64a77f`](https://github.com/Byron/gitoxide/commit/c64a77f09e2d8a1904704aaab4018a66a7e8417b))
    - prepare for dual-iteration ([`200384c`](https://github.com/Byron/gitoxide/commit/200384c4efbfb7438d192c7060b594f01df6b189))
    - make a common-dir iterator available ([`78658c7`](https://github.com/Byron/gitoxide/commit/78658c75c02fc9fc1b31fe2e24dd19ec16a8d477))
    - refactor ([`5521391`](https://github.com/Byron/gitoxide/commit/5521391d8b2369ffa5f78617b97793276480a96b))
    - try all prefixes when producing a relative path for error purposes ([`8cf0302`](https://github.com/Byron/gitoxide/commit/8cf0302bc75aed6e78adf7b855b0a28f2fa15616))
    - restore previous non-namespace test ([`ba17b40`](https://github.com/Byron/gitoxide/commit/ba17b40d7e22f1607ee7d8d206a136d307289591))
    - refactor ([`1b3a6da`](https://github.com/Byron/gitoxide/commit/1b3a6dacd55894f12dfa58ae5726f3b231dbe165))
    - refactor ([`7777f7e`](https://github.com/Byron/gitoxide/commit/7777f7e62e54d80e183d6bfc831950ca9c40e40d))
    - represent loose iteration in terms of the overlay iterator ([`ad37a64`](https://github.com/Byron/gitoxide/commit/ad37a64550e9e14bbdb88c04d2f1a367af01ca62))
    - start preparing for multi-loose ref iteration, but… ([`014459b`](https://github.com/Byron/gitoxide/commit/014459bf200204f69fc63481cd4e50caf70f21da))
    - validate traversal of main repository refs in presence of worktrees ([`f067286`](https://github.com/Byron/gitoxide/commit/f06728662834b89449cc4ec379a80ab778544e3b))
    - refactor ([`c2ac110`](https://github.com/Byron/gitoxide/commit/c2ac1107e840cf64d9ac33bc66d2a56586e0d06a))
    - simplify worktree test to not include superfluous refs ([`c46d838`](https://github.com/Byron/gitoxide/commit/c46d8383db0c6c2c57a3bf0544520d8dc05adf5d))
    - Assure proper handling of transactions in linked worktrees ([`07dc555`](https://github.com/Byron/gitoxide/commit/07dc5558cb872112d7c47494390b3e0db110edb6))
    - Humble beginnings of transaction testing within worktrees ([`6f52606`](https://github.com/Byron/gitoxide/commit/6f52606c395eee0b3cc98c11c84a17dd2636295d))
    - many more tests around transactions and prefix handling ([`8f3a4b5`](https://github.com/Byron/gitoxide/commit/8f3a4b5587f37c1a9894b13617133c50bd3d2a1d))
    - assure that private worktree refs have their reflogs written ([`96efbb1`](https://github.com/Byron/gitoxide/commit/96efbb10ab6c50606bc7fbacb9fdace77b5859e9))
    - Properly adjust names for prefixed refs for use in packed-refs and filter them ([`797af14`](https://github.com/Byron/gitoxide/commit/797af14a44d1208848253ad27c6d4ec8e4e45831))
    - prepare for filtering soon to be packed refs by name ([`3cea2ad`](https://github.com/Byron/gitoxide/commit/3cea2ad03de875a1613580eeac1b5aeca9934ef0))
    - uncover issue naming of refs written to packed-refs ([`1acf875`](https://github.com/Byron/gitoxide/commit/1acf87531fb41ebc7b6200417b05cbd9cfc58313))
    - a first test of main worktree transactions incl. changelog ([`9c5ab38`](https://github.com/Byron/gitoxide/commit/9c5ab38a1fbe45c95b6073949ddec4c3f42e9dbe))
    - adjusts to changes in git-ref ([`b362fb5`](https://github.com/Byron/gitoxide/commit/b362fb594546400e6c42688103df438954df7eeb))
    - `Transaction::commit(…)` takes `git-actor::SignatureRef` instead of `…::Signature`. ([`3d6299f`](https://github.com/Byron/gitoxide/commit/3d6299f47f41397c1c72035a86b94d1c263b5b98))
    - first rough step towards support iteration ([`b987575`](https://github.com/Byron/gitoxide/commit/b98757562527b703740e96086204577287351e81))
    - prepare for tests that can write into our test repositories ([`096ee5f`](https://github.com/Byron/gitoxide/commit/096ee5fc9b226b26b808d9aa14dc3c53e125b356))
    - improve reflog base generation in transactions so it could possibly work ([`d69b56e`](https://github.com/Byron/gitoxide/commit/d69b56e7f0104a0d8981777f1d3947b8a6a0b39e))
    - Improve conversions as much as possible ([`befe949`](https://github.com/Byron/gitoxide/commit/befe949b40d6201d35e4622725f5ad3c6d0315d4))
    - validate that owned values indeed can't be passed anymore ([`c78008e`](https://github.com/Byron/gitoxide/commit/c78008e4d8b7cc68c2c04d344ec3388fe42d9d9f))
    - Avoid using `Cow` at all in favor of a simple `&PartialNameref` ([`1bc9a87`](https://github.com/Byron/gitoxide/commit/1bc9a875d2b09b906f40db9e2c031c99e4fd9928))
    - This didn't get better, but probably will stay that way ([`7da239f`](https://github.com/Byron/gitoxide/commit/7da239fb0ee42fd5148fa6524e5bfecf51d19cee))
    - See what it means to use `Cow<PartialNameRef>` ([`2ae129a`](https://github.com/Byron/gitoxide/commit/2ae129ad6183f36179031ea905d8974705e70da8))
    - All custom TryFrom implementations for Cow<PartialNameRef> ([`15afec1`](https://github.com/Byron/gitoxide/commit/15afec1efbc01fbb52af8f88720c874e6aa2d2b1))
    - Cow support for PartialNameRef ([`3e470da`](https://github.com/Byron/gitoxide/commit/3e470da2d26f9b2c6bec9b28a9a108b8c032f007))
    - adapt to changes in git-ref ([`21109ca`](https://github.com/Byron/gitoxide/commit/21109ca9ab21df0ab45f3be552e83114817e98d0))
    - remove `FullName::to_ref()` in favor of `•::as_ref()`. ([`f1dbb6b`](https://github.com/Byron/gitoxide/commit/f1dbb6bd4534527b6f1f2aba2a562dd4e64cf55d))
    - reflog handling for worktree/ ref-names ([`77877e0`](https://github.com/Byron/gitoxide/commit/77877e09de324c7c32f5a170e853760f7b192aa4))
    - parse the worktree name as part of the cateogry ([`7666034`](https://github.com/Byron/gitoxide/commit/7666034a9b998eef7a2edaff0a8e6904babb5a13))
    - refactor ([`cebf736`](https://github.com/Byron/gitoxide/commit/cebf73655c887d57bbe597938cfa376fac96b44c))
    - more fixes for worktree ref handling ([`1aa546d`](https://github.com/Byron/gitoxide/commit/1aa546dea7f1aa8f6baf348f4398aa100fbd18a2))
    - better packed refs lookup for worktree refs ([`2b6982e`](https://github.com/Byron/gitoxide/commit/2b6982e89db0f2229bdafa314eadbfb7ee637195))
    - fix bugs with name resolution ([`3031f99`](https://github.com/Byron/gitoxide/commit/3031f996fdc2312f847da82f251f0a3f7875d40a))
    - Assure `main-worktree` is also resolved in the main worktree ([`bc3c05e`](https://github.com/Byron/gitoxide/commit/bc3c05e811b25be93294bebae96a0bba0afcf0f9))
    - assure nobody can bypass the namespace anymore ([`0079ab6`](https://github.com/Byron/gitoxide/commit/0079ab61c8f2818632949ed227e7251dd37a2a33))
    - frame for tests in worktree and main repositories ([`b12884a`](https://github.com/Byron/gitoxide/commit/b12884ad9596805521fe8c556c48d78669dd7ea5))
    - Assure namespaces are respected when calculating bases ([`4aac3c2`](https://github.com/Byron/gitoxide/commit/4aac3c248ed46963166defa46e80740d135906ed))
    - fix docs ([`e189914`](https://github.com/Byron/gitoxide/commit/e1899143f12c16360998ac27bce326343254a638))
    - transactions fully rely on proper base ([`d3a12e3`](https://github.com/Byron/gitoxide/commit/d3a12e31c8eae9209489d35b4b2e5d4835e85bde))
    - packed-refs now use correct common dir ([`dd7be18`](https://github.com/Byron/gitoxide/commit/dd7be189ea1d5c15385a7eee54368b2b2510ed2d))
    - reflog paths are now computed according to their ref name ([`c4d2493`](https://github.com/Byron/gitoxide/commit/c4d2493dc5f3e0d2c7fca71ae672af167b8ff1a5))
    - refactor ([`e93c222`](https://github.com/Byron/gitoxide/commit/e93c222c947f57d9df523372613265d6286f64ce))
    - first draft of fixture  setup for worktrees ([`a25b0bc`](https://github.com/Byron/gitoxide/commit/a25b0bccfa001273057089fecca637c2d1fc591a))
    - use suitable full names for store based packed-refs lookup too ([`d25b3ca`](https://github.com/Byron/gitoxide/commit/d25b3ca01450c308e08e060c3959aaca09b9415f))
    - packed-refs can now possibly transform names for lookup correctly ([`6478736`](https://github.com/Byron/gitoxide/commit/6478736b5a2f86fd369a40f55b3697089e4ee93b))
    - Offer a way to search packed refs by full name only ([`ae57e27`](https://github.com/Byron/gitoxide/commit/ae57e2766c4594b8336b36d4cfecb3c41d83e391))
    - Also build what should be a valid path for loose-ref lookup ([`37deca5`](https://github.com/Byron/gitoxide/commit/37deca572eeeada79bfcbb4f815d50d4d5be2dc7))
    - refactor ([`19d53fb`](https://github.com/Byron/gitoxide/commit/19d53fb1b8c13ca13d2111a840de1b6ef91780ec))
    - refactor ([`32b8fde`](https://github.com/Byron/gitoxide/commit/32b8fde5f8941d3799d461e1d258f6627c064891))
    - refactor ([`09eb432`](https://github.com/Byron/gitoxide/commit/09eb432f21e6999ed565437aba1d3550fe33ed8d))
    - refactor ([`f83910f`](https://github.com/Byron/gitoxide/commit/f83910fe56b07b8dfabedeeaefd02070c15dd6f3))
    - Loose ref lookup uses categories for refanames ([`61b449c`](https://github.com/Byron/gitoxide/commit/61b449ce8bf0d281839ba333af0dc33bf18556a1))
    - prepare for switch to category based base-path lookup ([`14c6f48`](https://github.com/Byron/gitoxide/commit/14c6f48d28d6f2e0a0dd778ec8205d04c731aeb2))
    - Unify pseudo-ref check for find reference ([`8c52fe4`](https://github.com/Byron/gitoxide/commit/8c52fe42ead34de18de3b649ca91bc5616da5e57))
    - Admit that we need PartialPathCow and can't work around it ([`202bafc`](https://github.com/Byron/gitoxide/commit/202bafcaeb122f9d61b0ad4855e1aaf90c69107c))
    - rename `PartialNameRef` to `PartialNameCow` ([`1611c3d`](https://github.com/Byron/gitoxide/commit/1611c3ddff6c930deaa4c2440383f5684c029b28))
    - refactor ([`4e26e62`](https://github.com/Byron/gitoxide/commit/4e26e62f62bffbc5744650d173061a8270e1cd68))
    - refactor ([`a611224`](https://github.com/Byron/gitoxide/commit/a6112249689438058724d859a87f0a4a64ed02e4))
    - refactor ([`467d583`](https://github.com/Byron/gitoxide/commit/467d5839af2098911bf5551b6dfa6db0a2b20300))
    - refactor ([`3574a4e`](https://github.com/Byron/gitoxide/commit/3574a4eaee78dc41c22d85af47cfd27b2c7c2303))
    - `Category::LinkedRef` ([`9cccce3`](https://github.com/Byron/gitoxide/commit/9cccce35e527cdda58e01b03cd335a527418cf14))
    - `Category::MainRef` ([`eada5df`](https://github.com/Byron/gitoxide/commit/eada5dfdd4981dbb032e8f155e3829eb5d1f380f))
    - `Category::(WorktreePrivate|Bisect|Rewritten)` ([`0304b7f`](https://github.com/Byron/gitoxide/commit/0304b7fad3ae5e3ac1fea71e30658474a770184b))
    - `Category::LinkedPseudoRef` ([`612a2db`](https://github.com/Byron/gitoxide/commit/612a2dbf7f70a8eb44b7278fccd4f1589749968a))
    - Add `Category::MainPseudoRef` ([`613b584`](https://github.com/Byron/gitoxide/commit/613b5844e74535d391339bd8e7c106e18257b917))
    - `Category::PseudoRef` ([`6ccfea9`](https://github.com/Byron/gitoxide/commit/6ccfea97d30dbfa013e1633c77c5a26ab5cbaf8f))
    - remove `Store` from public API ([`8a92ec9`](https://github.com/Byron/gitoxide/commit/8a92ec9834b6d5aa3057c5509f6c13b6a6cd6e1b))
    - rename `file::Store::base()` to `git_dir()`. ([`2becffc`](https://github.com/Byron/gitoxide/commit/2becffc85ff6225522fe38482739fb1406ae1060))
    - adapt to changes in git-path ([`cc2d810`](https://github.com/Byron/gitoxide/commit/cc2d81012d107da7a61bf4de5b28342dea5083b7))
    - adapt to all changes in git-path with bstr support ([`f158648`](https://github.com/Byron/gitoxide/commit/f158648aef8ad94d86550ceb2eeb20efb3df7596))
    - Use `git-path` crate instead of `git_features::path` ([`47e607d`](https://github.com/Byron/gitoxide/commit/47e607dc256a43a3411406c645eb7ff04239dd3a))
    - adjustments to go along with changes in git-features ([`c55cac6`](https://github.com/Byron/gitoxide/commit/c55cac6a1ada77619bb5723717a5a6d757499fa9))
    - make fmt ([`50ff7aa`](https://github.com/Byron/gitoxide/commit/50ff7aa7fa86e5e2a94fb15aab86470532ac3f51))
 * **[#384](https://github.com/Byron/gitoxide/issues/384)**
    - No need to isolate archives by crate name ([`19d46f3`](https://github.com/Byron/gitoxide/commit/19d46f35440419b9911b6e2bca2cfc975865dce9))
    - add archive files via git-lfs ([`7202a1c`](https://github.com/Byron/gitoxide/commit/7202a1c4734ad904c026ee3e4e2143c0461d51a2))
    - auto-set commit.gpgsign=false when executing git ([`c23feb6`](https://github.com/Byron/gitoxide/commit/c23feb64ad157180cfba8a11c882b829733ea8f6))
 * **[#393](https://github.com/Byron/gitoxide/issues/393)**
    - Add support for disabling archive usage ([`624ad2e`](https://github.com/Byron/gitoxide/commit/624ad2ef42172556efe942129f6f46dd627250d5))
 * **Uncategorized**
    - Release git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0 ([`349c590`](https://github.com/Byron/gitoxide/commit/349c5904b0dac350838a896759d51576b66880a7))
    - fix git-ref dependencies ([`633e571`](https://github.com/Byron/gitoxide/commit/633e571da33767072176ebb2e44caf6d85975982))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - make fmt ([`e043807`](https://github.com/Byron/gitoxide/commit/e043807abf364ca46d00760e2f281528efe20c75))
    - Merge branch 'refs-and-worktrees' ([`8131227`](https://github.com/Byron/gitoxide/commit/8131227ddff6f36919b6a0f7b33792ebde0f8ae9))
    - thanks clippy ([`4cff7a8`](https://github.com/Byron/gitoxide/commit/4cff7a82ea3e924a8eba58cf45061b5afc5b250f))
    - thanks clippy ([`a20d282`](https://github.com/Byron/gitoxide/commit/a20d282d608d1f8145951ac3ad26de583a60513c))
    - thanks clippy ([`c208912`](https://github.com/Byron/gitoxide/commit/c20891281f8222db68d0888a47f14822a03efac9))
    - Merge branch 'main' into refs-and-worktrees ([`9cf0c7b`](https://github.com/Byron/gitoxide/commit/9cf0c7bd0cc5419137db5796f3a5b91bdf3dcc94))
    - Merge branch 'davidkna-remote-branch-name' ([`068a2de`](https://github.com/Byron/gitoxide/commit/068a2de764fabff949ff49a50594563cc625e343))
    - Turn `FullNameRef` into an actual reference type. ([`0ace957`](https://github.com/Byron/gitoxide/commit/0ace957c595c8a38afb7de1462cdc73b617d2a76))
    - A sketch to show how Cow<FullNameRef> could work ([`c6b2705`](https://github.com/Byron/gitoxide/commit/c6b27058884b23cd3ee35ddc3aa2aaf69012cbff))
    - thanks clippy ([`61d6d93`](https://github.com/Byron/gitoxide/commit/61d6d93262cf2f1d13037542e4107ff558ea7c2c))
    - thanks clippy ([`6291015`](https://github.com/Byron/gitoxide/commit/6291015df786eaebb2339629adc4685ffb555d01))
    - thanks clippy ([`22ee920`](https://github.com/Byron/gitoxide/commit/22ee920fc93b88ed6d035915e9cab2d7501c92b9))
    - refactor ([`6149978`](https://github.com/Byron/gitoxide/commit/61499786b9b0743d949d5a54639c54d76c4d2a44))
    - thanks clippy ([`405d94d`](https://github.com/Byron/gitoxide/commit/405d94d8eb1ca07568bae5c51a7efd43dc9e2808))
    - thanks clippy ([`a7ac64c`](https://github.com/Byron/gitoxide/commit/a7ac64cd801b985790b5717be1a5dc722b2ae3a9))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - Merge branch 'worktree-stack' ([`98da8ba`](https://github.com/Byron/gitoxide/commit/98da8ba52cef8ec27f705fcbc84773e5bacc4e10))
    - Merge branch 'main' into repo-status ([`0eb2372`](https://github.com/Byron/gitoxide/commit/0eb23721dca78f6e6bf864c5c3a3e44df8b419f0))
    - Merge branch 'test-archive-support' ([`350df01`](https://github.com/Byron/gitoxide/commit/350df01042d6ca8b93f8737fa101e69b50535a0f))
    - Merge branch 'main' into repo-status ([`4086335`](https://github.com/Byron/gitoxide/commit/40863353a739ec971b49410fbc2ba048b2762732))
    - Merge branch 'worktree-stack' ([`e90d3fd`](https://github.com/Byron/gitoxide/commit/e90d3fd0a9764511e6280596f21d3a0494ed7021))
    - fix clippy - many false positives this time ([`045e6fa`](https://github.com/Byron/gitoxide/commit/045e6fae17077555c3e115992905c8046f2c5d0b))
    - fix clippy - many false positives this time ([`099bd5b`](https://github.com/Byron/gitoxide/commit/099bd5b86fb80b26a73863b80ad60a0394458b6d))
</details>

## 0.12.1 (2022-04-05)

### Changed (BREAKING)

 - <csr-id-1611c3ddff6c930deaa4c2440383f5684c029b28/> rename `PartialNameRef` to `PartialNameCow`
   Because this is what it is, which also implies that it's not `Copy`
   anymore which a `Ref` would definitely be.
   
   The reason we need this to be a `Cow` is to support passing ownership.
 - <csr-id-8a92ec9834b6d5aa3057c5509f6c13b6a6cd6e1b/> remove `Store` from public API
   It is unclear if ref-tables, which are the reason for it to exist in
   the first place, will fit into the concept as they might not support
   worktrees. It's entirely unclear how this works.
   
   Maybe there can be a non-worktree version of the store with work-trees
   only being supported by the file based ref database, and ref-tables
   remaining a server-side feature.
 - <csr-id-2becffc85ff6225522fe38482739fb1406ae1060/> rename `file::Store::base()` to `git_dir()`.
   That way it is clearer what it actually is especially in presence
   of the newly added `file::Store::common_dir()` method.
   
   That way, work-trees can eventually be properly supported.

### New Features

 - <csr-id-ecd60d7a75d4aef7c37cd0b28b57a8aea1166858/> add `FullName(Ref)::category_and_shortname()`
   It's a combination of `shorten()` and `category()` for convenience.
 - <csr-id-9cccce35e527cdda58e01b03cd335a527418cf14/> `Category::LinkedRef`
   With it one can access all refs as advertised.
 - <csr-id-eada5dfdd4981dbb032e8f155e3829eb5d1f380f/> `Category::MainRef`
   With it it will be possible to compute all paths correctly and actually
   perform certain operations as documented.
 - <csr-id-0304b7fad3ae5e3ac1fea71e30658474a770184b/> `Category::(WorktreePrivate|Bisect|Rewritten)`
 - <csr-id-612a2dbf7f70a8eb44b7278fccd4f1589749968a/> `Category::LinkedPseudoRef`
 - <csr-id-613b5844e74535d391339bd8e7c106e18257b917/> Add `Category::MainPseudoRef`
 - <csr-id-6ccfea97d30dbfa013e1633c77c5a26ab5cbaf8f/> `Category::PseudoRef`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 2 calendar days.
 - 2 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#364](https://github.com/Byron/gitoxide/issues/364)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - add `FullName(Ref)::category_and_shortname()` ([`ecd60d7`](https://github.com/Byron/gitoxide/commit/ecd60d7a75d4aef7c37cd0b28b57a8aea1166858))
 * **Uncategorized**
    - Release git-config v0.2.1, git-diff v0.15.0, git-traverse v0.14.0, git-pack v0.18.0, git-odb v0.28.0, git-ref v0.12.1, git-revision v0.1.0, git-repository v0.16.0, gitoxide-core v0.14.0, gitoxide v0.12.0, safety bump 6 crates ([`b612021`](https://github.com/Byron/gitoxide/commit/b612021683ba709b693bd48aef3e2e3c2f5b9ead))
    - refactor ([`2abedb8`](https://github.com/Byron/gitoxide/commit/2abedb851ba383f502c0c9f9f9a3a26349c1664d))
</details>

## 0.12.0 (2022-04-03)

### New Features

 - <csr-id-e7e4ba2739e5a15e826b5b4d9ef2b45a1cb016cc/> `Fullname(Ref)::category()` and `Category`
   A way to classify references.
 - <csr-id-af2d399261e4131299c2279904a1f224c116db66/> FullName(Ref)::strip_prefix()
   Get a short-hand for any reference name.
 - <csr-id-e4d6685064ad2b433f8acd3a74b320bf0169a994/> Add `git_config::values::Path` for a typesafe git path
   Add a `Path` type to the `git_config::values` which
   can be interpolated according to gits own path interpolation
   rules.
 - <csr-id-28e3251ee1996f638eaa0bc7b39b06be436f6eb7/> FullNameRef::file_name()
 - <csr-id-cf50ae2b33da0161d801b4c0a9cd8b5d24bb3510/> Display implementation for `FullName`.

### Bug Fixes

 - <csr-id-42e0487286c1f745837c0ce337ed7c9d86b14516/> support Rust 1.52

### Changed (BREAKING)

 - <csr-id-7984e19417dff13445f3ef2e15dea96bbd194ce5/> `Target::as_(id|name)` -> `Target::try_(id|name)`
   Conform to naming conventions, whenever something returns an option
   or Result while there is a more direct name that panics, prefix `try_`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 33 commits contributed to the release over the course of 66 calendar days.
 - 69 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#329](https://github.com/Byron/gitoxide/issues/329), [#331](https://github.com/Byron/gitoxide/issues/331), [#333](https://github.com/Byron/gitoxide/issues/333), [#364](https://github.com/Byron/gitoxide/issues/364)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - Use hash_hasher based hash state for better keys/less collisions ([`814de07`](https://github.com/Byron/gitoxide/commit/814de079f4226f42efa49ad334a348bce67184e4))
    - Display implementation for `FullName`. ([`cf50ae2`](https://github.com/Byron/gitoxide/commit/cf50ae2b33da0161d801b4c0a9cd8b5d24bb3510))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - performance issue on windows is due to slow process execution speed ([`60bcffc`](https://github.com/Byron/gitoxide/commit/60bcffc2d2921f01a0c8f42da9b43cd731eaf55d))
    - speed up git-pack testing on windows ([`8ca400c`](https://github.com/Byron/gitoxide/commit/8ca400c8647e0e59a96a936d41c2dc2d07c6bf2d))
 * **[#329](https://github.com/Byron/gitoxide/issues/329)**
    - Document all features related to serde1 ([`72b97f2`](https://github.com/Byron/gitoxide/commit/72b97f2ae4dc7642b160f183c6d5df4502dc186f))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - Add `git_config::values::Path` for a typesafe git path ([`e4d6685`](https://github.com/Byron/gitoxide/commit/e4d6685064ad2b433f8acd3a74b320bf0169a994))
 * **[#333](https://github.com/Byron/gitoxide/issues/333)**
    - Use git_features::path everywhere where there is a path conversion ([`2e1437c`](https://github.com/Byron/gitoxide/commit/2e1437cb0b5dc77f2317881767f71eaf9b009ebf))
    - remove os-str-bytes everywhere ([`71a086a`](https://github.com/Byron/gitoxide/commit/71a086aaf0835c31c834aa32d968552de490f2e7))
    - Found one valid case for using os-str-bytes ([`9c294bf`](https://github.com/Byron/gitoxide/commit/9c294bff45f8f04affd690327559e0e2c2415fa9))
    - Make real clear panics are only possible on windows ([`6b283dc`](https://github.com/Byron/gitoxide/commit/6b283dc7b9339fd65ea35f56eb29f121f571caf7))
    - one usage of os_str_bytes down, along with some custom conversion code ([`1cc95ce`](https://github.com/Byron/gitoxide/commit/1cc95cefbd132a4277ec52c2147f7c81fea92d48))
    - gitoxide-core without os-str-bytes ([`909aa14`](https://github.com/Byron/gitoxide/commit/909aa1402c82c3128052023613a297b213716e3d))
    - Don't use os_str_ext in git-features; adapt git-ref ([`9258b7b`](https://github.com/Byron/gitoxide/commit/9258b7baf0895593c13a152ff9e6f52e036cebe1))
    - Use new git-features::path module ([`7c53b27`](https://github.com/Byron/gitoxide/commit/7c53b275a71b9d2ee477bd19464d49a97e031e0c))
 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - `Target::as_(id|name)` -> `Target::try_(id|name)` ([`7984e19`](https://github.com/Byron/gitoxide/commit/7984e19417dff13445f3ef2e15dea96bbd194ce5))
    - refactor ([`b1b9871`](https://github.com/Byron/gitoxide/commit/b1b9871e8b0c2bcbdee0c3ea4c060b4a7c32bc15))
    - `Fullname(Ref)::category()` and `Category` ([`e7e4ba2`](https://github.com/Byron/gitoxide/commit/e7e4ba2739e5a15e826b5b4d9ef2b45a1cb016cc))
    - FullName(Ref)::strip_prefix() ([`af2d399`](https://github.com/Byron/gitoxide/commit/af2d399261e4131299c2279904a1f224c116db66))
    - Full error handling for CommitRefIter ([`b94471a`](https://github.com/Byron/gitoxide/commit/b94471a0ced50204156cf5d4126c676f0258a5eb))
 * **Uncategorized**
    - Release git-diff v0.14.0, git-bitmap v0.1.0, git-index v0.2.0, git-tempfile v2.0.1, git-lock v2.0.0, git-mailmap v0.1.0, git-traverse v0.13.0, git-pack v0.17.0, git-quote v0.2.0, git-odb v0.27.0, git-packetline v0.12.4, git-url v0.4.0, git-transport v0.16.0, git-protocol v0.15.0, git-ref v0.12.0, git-worktree v0.1.0, git-repository v0.15.0, cargo-smart-release v0.9.0, safety bump 5 crates ([`e58dc30`](https://github.com/Byron/gitoxide/commit/e58dc3084cf17a9f618ae3a6554a7323e44428bf))
    - Merge branch 'for-onefetch' ([`8e5cb65`](https://github.com/Byron/gitoxide/commit/8e5cb65da75036a13ed469334e7ae6c527d9fff6))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - thanks clippy ([`2066a80`](https://github.com/Byron/gitoxide/commit/2066a80664e239436685d10ba7bab92916661a56))
    - Merge branch 'main' into mailmap ([`b2df941`](https://github.com/Byron/gitoxide/commit/b2df941feaf5ae9fa170fa49270189f3527f2eab))
    - Merge branch 'describe-rev' ([`77b7cd9`](https://github.com/Byron/gitoxide/commit/77b7cd9a7813aaa1a15d035ea42c1e3fe4eef8dd))
    - adapt to breaking changes in git-actor ([`40c48c3`](https://github.com/Byron/gitoxide/commit/40c48c390eb796b427ebd516dde92e9538ce5fb7))
    - Merge branch 'svetli-n-path_value' ([`e8383ca`](https://github.com/Byron/gitoxide/commit/e8383caf6db211beb57d70019fe4ad13ce9066ee))
    - Release git-tempfile v2.0.0, safety bump 6 crates ([`90b1c42`](https://github.com/Byron/gitoxide/commit/90b1c42d5487904a9f329362d185b035d0ddb975))
    - Merge branch 'unify-path-encoding' ([`566ff8a`](https://github.com/Byron/gitoxide/commit/566ff8a3597b889899d41ca15e5b9af7e05f1a4b))
    - thanks clippy ([`a8e9497`](https://github.com/Byron/gitoxide/commit/a8e9497caebf1c0e9faac537717cd86378f1acf6))
    - thanks clippy ([`f242a24`](https://github.com/Byron/gitoxide/commit/f242a248909c61953030c112b34af565f851ac0d))
    - Release git-actor v0.8.1 ([`08fe550`](https://github.com/Byron/gitoxide/commit/08fe5508472f2eb209db8a5fc4e4942a9d7db93d))
    - Release git-hash v0.9.2, git-object v0.17.1, git-pack v0.16.1 ([`0db19b8`](https://github.com/Byron/gitoxide/commit/0db19b8deaf11a4d4cbc03fa3ae40eea104bc302))
</details>

## 0.11.0 (2022-01-23)

<csr-id-c46dec311c76a83dba136be5dba6b70d739c354d/>
<csr-id-ebc7f47708a63c3df4415ba0e702660d976dfb3e/>
<csr-id-2290d006705ff47ad780b009fe58ee422b3285af/>

### New Features

 - <csr-id-28e3251ee1996f638eaa0bc7b39b06be436f6eb7/> FullNameRef::file_name()
 - <csr-id-f6181afef285b32968b852467235edf3d7d99a66/> Add file::Store::find_packed(…, buffer)
   This makes the API complete as now there is a methods that uses the
   internal buffer, or the provided one, where both can have its benefits.
 - <csr-id-03bcac2763076bfd720b99d60fd036b4d5d00995/> add Reference::peel_to_id_in_place_packed() and Reference::follow_packed()
   This allows a stable/non-changing buffer to be used.

### Bug Fixes

 - <csr-id-42e0487286c1f745837c0ce337ed7c9d86b14516/> support Rust 1.52

### Changed (BREAKING)

 - <csr-id-dea6659a404bf8b2ad1290549653c776ec04f964/> Required `object_hash` as parameter when instantiating any Store
   This is needed to choose the correct kind of object hash in a couple of
   situations, and future-proofs it for when ref-table arrives.
 - remove `Target::must_exist()`
   It was a remainder of an old API which since has been replaced
   with a more explicit approach.
 - remove pack-cache from `Find::try_find(…)`
   With the new architecture this can be an implementation detail without
   forcing it to be Sync.
 - move git_pack::data::Object to git_object::Data, massively alter git_odb::Find trait
   This will break a lot, but has to happen to prepare these traits for the
   next generation of object databases.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 28 commits contributed to the release over the course of 55 calendar days.
 - 55 days passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 5 unique issues were worked on: [#263](https://github.com/Byron/gitoxide/issues/263), [#266](https://github.com/Byron/gitoxide/issues/266), [#279](https://github.com/Byron/gitoxide/issues/279), [#293](https://github.com/Byron/gitoxide/issues/293), [#298](https://github.com/Byron/gitoxide/issues/298)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#263](https://github.com/Byron/gitoxide/issues/263)**
    - Don't unnecessarily reload the packed-refs buffer under contention ([`3ab990e`](https://github.com/Byron/gitoxide/commit/3ab990ebbaf7feb999340e86933bbdc823166669))
    - Use a read-lock until mutation is needed, instead of upgradable rw locks ([`58294dd`](https://github.com/Byron/gitoxide/commit/58294ddf13c17e64dc224cc01ec70c3006cb14bc))
    - refactor ([`e41ab82`](https://github.com/Byron/gitoxide/commit/e41ab820a32177e54b9cd53856cc2cdd4a72271f))
    - Add file::Store::find_packed(…, buffer) ([`f6181af`](https://github.com/Byron/gitoxide/commit/f6181afef285b32968b852467235edf3d7d99a66))
    - add Reference::peel_to_id_in_place_packed() and Reference::follow_packed() ([`03bcac2`](https://github.com/Byron/gitoxide/commit/03bcac2763076bfd720b99d60fd036b4d5d00995))
 * **[#266](https://github.com/Byron/gitoxide/issues/266)**
    - upgrade git-ref's os_str_bytes crate to 6.0.0 ([`0cfba57`](https://github.com/Byron/gitoxide/commit/0cfba573eb85c5e990635363ac169b0114bba671))
    - adapt to changes in git-odb ([`a44dd4b`](https://github.com/Byron/gitoxide/commit/a44dd4b5d1910856d7a21e156e7bca3138c04484))
    - remove pack-cache from `Find::try_find(…)` ([`ebc7f47`](https://github.com/Byron/gitoxide/commit/ebc7f47708a63c3df4415ba0e702660d976dfb3e))
    - Clarify that we really need stable pack ids ([`cefc0fa`](https://github.com/Byron/gitoxide/commit/cefc0faf8c28939449f5df57e9f4fff14da08ea1))
    - move git_pack::data::Object to git_object::Data, massively alter git_odb::Find trait ([`2290d00`](https://github.com/Byron/gitoxide/commit/2290d006705ff47ad780b009fe58ee422b3285af))
 * **[#279](https://github.com/Byron/gitoxide/issues/279)**
    - Adapt to changes in git-hash ([`754a663`](https://github.com/Byron/gitoxide/commit/754a66344ff2cfcfc4a7a3d72f1240e939c48055))
    - adjust to changes in git-hash ([`9bf25cc`](https://github.com/Byron/gitoxide/commit/9bf25cc4f2e44821f93e85997677bc4e86a67bd4))
    - Required `object_hash` as parameter when instantiating any Store ([`dea6659`](https://github.com/Byron/gitoxide/commit/dea6659a404bf8b2ad1290549653c776ec04f964))
    - remove `Target::must_exist()` ([`c46dec3`](https://github.com/Byron/gitoxide/commit/c46dec311c76a83dba136be5dba6b70d739c354d))
    - adapt to changes in git-hash ([`3e75e8c`](https://github.com/Byron/gitoxide/commit/3e75e8cd4c55f339525914fdc69e7a7da1fb06d4))
 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - git-ref uses memmap2 ([`4dec3ea`](https://github.com/Byron/gitoxide/commit/4dec3ead7c28e88de1eb8e1576b9b29b1c0953c7))
 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - FullNameRef::file_name() ([`28e3251`](https://github.com/Byron/gitoxide/commit/28e3251ee1996f638eaa0bc7b39b06be436f6eb7))
 * **Uncategorized**
    - Release git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`b286b24`](https://github.com/Byron/gitoxide/commit/b286b24a51878be7d2e0fd77ff0c5c99b439a6a0))
    - Release git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`42ebb53`](https://github.com/Byron/gitoxide/commit/42ebb536cd6086f096b8422291776c9720fa0948))
    - Release git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`1b76119`](https://github.com/Byron/gitoxide/commit/1b76119259b8168aeb99cbbec233f7ddaa2d7d2c))
    - Release git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`8f57c29`](https://github.com/Byron/gitoxide/commit/8f57c297d7d6ed68cf51415ea7ede4bf9263326e))
    - Release git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`d78aab7`](https://github.com/Byron/gitoxide/commit/d78aab7b9c4b431d437ac70a0ef96263acb64e46))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - prepar changelogs for cargo-smart-release release ([`8900d69`](https://github.com/Byron/gitoxide/commit/8900d699226eb0995be70d66249827ce348261df))
    - support Rust 1.52 ([`42e0487`](https://github.com/Byron/gitoxide/commit/42e0487286c1f745837c0ce337ed7c9d86b14516))
    - Release git-bitmap v0.0.1, git-hash v0.9.0, git-features v0.19.0, git-index v0.1.0, safety bump 9 crates ([`4624725`](https://github.com/Byron/gitoxide/commit/4624725f54a34dd6b35d3632fb3516965922f60a))
    - Merge branch 'sync-db-draft' ([`7d2e20c`](https://github.com/Byron/gitoxide/commit/7d2e20c6fedc2c7e71a307d8d072412fa847a4aa))
    - thanks clippy ([`7dd2313`](https://github.com/Byron/gitoxide/commit/7dd2313d980fe7c058319ae66d313b3097e3ae5f))
</details>

## 0.10.0 (2021-11-29)

<csr-id-951c050ecbb70c9de216603e55c7cfbc89a067e3/>
<csr-id-0e1875363fea09452789d7a90fc6860a7996d6d3/>

With this release, `file::Store` is easier to use due to thread-safe handling of an internal and shared
packed-buffer instance. An API for passing it as parameter like before is still present, allowing to use
a 'frozen' version of the packed buffer for any amount of operations.

### New Features

 - <csr-id-6a17416557112a6464e548c5de1c46e563b3a187/> Add `file::Store::iter_(prefixed_)packed()`.
   
   These methods allow using an own packed buffer, usually obtained through
   `cached_packed_buffer()`.
 - <csr-id-9eb3a31d1f9f519e153e8df3fc3faaff278aed85/> add `file::Store::cached_packed_buffer()` for packed refs snapshots
 - <csr-id-b030884447284daf0f2251f574c0ddf9993b2792/> add file::Store::try_find_packed(…, packed_buffer)
   That way, abstractions can still be built that have other ways of
   managing the packed-refs buffer, allowing it to stay more persistent.

### Changed (BREAKING)

 - <csr-id-b431fb0fb58b5e2e8aadbbd6aead55c0e42bd67b/> rename `file::Store::packed_buffer()` to `…::open_packed_buffer()`
   This makes much clearer what it actually does, as previously it might
   have been a stored packed buffer as well.
 - <csr-id-80f3d504eeb669f16c5621fac06f6c763ce84e47/> file::Store::from(PathBuf) removed
   At this low level, it's important to be clear about RefLogs and rather
   force the caller to specify the ref-log mode. Technically it depends
   on a few factors, `git-repository` deals with them, but certainly
   shouldn't default to anything without being clear.
 - <csr-id-bfb32aee4e64fd6b1f18c830623cc3fddd059874/> Reference log line access
   `Reference::log_iter(…)` now is a platform instead of a forward iterator,
   which requires a call to `.all()` to return the forward iterator like
   previously.
   
   `Reference::log_iter_rev(…)` was removed in favor of
   `Reference::log_iter(…).rev()`.
 - <csr-id-5d498a33236391d8e456f267b1bf6af24de66f11/> file::Store::iter() is now a platform, with `.all()` and `.prefixed(…)` respectively
   This way, it's possible to keep shared ownership of the packed buffer
   while allowing the exact same iterator machinery to work as before.
 - <csr-id-4641499abe00acf6eef0ab6d6bf261b0a27795f8/> file::ReferenceExt::follow(…) now without packed refs parameter
 - <csr-id-55940eb8316d83ac1376c57ba25b3115d62f2012/> `file::ReferenceExt::peel_to_id_in_place(…)` now without packed-refs buffer
   It is instead read from the internally synchronized buffer, shared
   across all instances.
 - <csr-id-15d429bb50602363292453606902bdce5042d9a5/> file::Store::(try_)find(…, packed) was removed
   The packed buffer is now handled internally while loading it on demand.
   When compiled with `git-features/parallel` the `file::Store` remains
   send and sync.
   
   The packed refs buffer is shared across clones and it's recommended
   to clone one `file::Store` instance per thread, each of which can
   use its own namespace.
 - <csr-id-95247322a8191edfa7fac9c5aa72b40239f3aa88/> move `git_ref::file::WriteRefLog` to `git_ref::store::WriteRefLog`

### Other (BREAKING)

 - <csr-id-951c050ecbb70c9de216603e55c7cfbc89a067e3/> Reference::logs() -> Reference::log_iter()
   The latter now returns a standard Platform to iterate over all
   reflog entries from oldest to newest or vice versa.

### Refactor (BREAKING)

 - <csr-id-0e1875363fea09452789d7a90fc6860a7996d6d3/> `file::Store::base` is now `file::Store::base()` and read-only
   That way, file databases can't be repositioned anymore, it's recommended
   to recreate it if that's desired.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 35 commits contributed to the release over the course of 3 calendar days.
 - 12 days passed between releases.
 - 13 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#259](https://github.com/Byron/gitoxide/issues/259), [#263](https://github.com/Byron/gitoxide/issues/263)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#259](https://github.com/Byron/gitoxide/issues/259)**
    - btree/hashmap free lookup of packs in store, keeping things more bundled ([`a88981b`](https://github.com/Byron/gitoxide/commit/a88981b6f38b86624588f0c8ff200d17f38d0263))
 * **[#263](https://github.com/Byron/gitoxide/issues/263)**
    - Add `file::Store::iter_(prefixed_)packed()` ([`6a17416`](https://github.com/Byron/gitoxide/commit/6a17416557112a6464e548c5de1c46e563b3a187))
    - add `file::Store::cached_packed_buffer()` for packed refs snapshots ([`9eb3a31`](https://github.com/Byron/gitoxide/commit/9eb3a31d1f9f519e153e8df3fc3faaff278aed85))
    - rename `file::Store::packed_buffer()` to `…::open_packed_buffer()` ([`b431fb0`](https://github.com/Byron/gitoxide/commit/b431fb0fb58b5e2e8aadbbd6aead55c0e42bd67b))
    - add file::Store::try_find_packed(…, packed_buffer) ([`b030884`](https://github.com/Byron/gitoxide/commit/b030884447284daf0f2251f574c0ddf9993b2792))
    - file::Store::from(PathBuf) removed ([`80f3d50`](https://github.com/Byron/gitoxide/commit/80f3d504eeb669f16c5621fac06f6c763ce84e47))
    - Put general Store on hold - ref-table is needed to know how to go about it ([`bfa417b`](https://github.com/Byron/gitoxide/commit/bfa417baa79fb7ba3c1b5f559ef5b12278dbc839))
    - Don't even think about setting up test duplication for the general store ([`72a6464`](https://github.com/Byron/gitoxide/commit/72a6464ed8bf869704615bc5f4f98b604f2d8001))
    - Reference::logs() -> Reference::log_iter() ([`951c050`](https://github.com/Byron/gitoxide/commit/951c050ecbb70c9de216603e55c7cfbc89a067e3))
    - Reference log line access ([`bfb32ae`](https://github.com/Byron/gitoxide/commit/bfb32aee4e64fd6b1f18c830623cc3fddd059874))
    - Add platform for log iteration to hold byte buffers ([`1cd2362`](https://github.com/Byron/gitoxide/commit/1cd23621f9d5a7ad22b0216aec9866cf3786b007))
    - Assure the packed buffer is reloaded after a modification ([`f5570ff`](https://github.com/Byron/gitoxide/commit/f5570ff0e0d134144e86e3b06f426e2827469a88))
    - fmt ([`fbeddeb`](https://github.com/Byron/gitoxide/commit/fbeddebcab999f4898f768a3184906091f8ce0b8))
    - file::Store::iter() is now a platform, with `.all()` and `.prefixed(…)` respectively ([`5d498a3`](https://github.com/Byron/gitoxide/commit/5d498a33236391d8e456f267b1bf6af24de66f11))
    - refactor ([`5fc3817`](https://github.com/Byron/gitoxide/commit/5fc381718693256562474d2b6bf551e4eb366293))
    - refactor packed buffer sharing to allow for sharing snapshots ([`00c2545`](https://github.com/Byron/gitoxide/commit/00c254525d4e028a16cb70028be1311432d006fc))
    - Let file transactions reuse the cached packed buffer ([`a9096b9`](https://github.com/Byron/gitoxide/commit/a9096b9e6b09ef5394b71a58cba3bc2b72a66a8b))
    - file::ReferenceExt::follow(…) now without packed refs parameter ([`4641499`](https://github.com/Byron/gitoxide/commit/4641499abe00acf6eef0ab6d6bf261b0a27795f8))
    - `file::ReferenceExt::peel_to_id_in_place(…)` now without packed-refs buffer ([`55940eb`](https://github.com/Byron/gitoxide/commit/55940eb8316d83ac1376c57ba25b3115d62f2012))
    - file::Store::(try_)find(…, packed) was removed ([`15d429b`](https://github.com/Byron/gitoxide/commit/15d429bb50602363292453606902bdce5042d9a5))
    - Load packed buffer with interior mutability ([`ae2eef1`](https://github.com/Byron/gitoxide/commit/ae2eef11152b6c16dd08cb244b78b582e6351ec7))
    - Make it possible to return read guards with packed buffers ([`f5c3c8f`](https://github.com/Byron/gitoxide/commit/f5c3c8f7309bf53b9e53f786e75931d701a8585c))
    - `file::Store::base` is now `file::Store::base()` and read-only ([`0e18753`](https://github.com/Byron/gitoxide/commit/0e1875363fea09452789d7a90fc6860a7996d6d3))
    - refactor, realize why having a packed-buffer with the loose db is valuable ([`a76f041`](https://github.com/Byron/gitoxide/commit/a76f04166f652ebb3304b396f5dadf302270854d))
    - A mad attempt to use thread-local everywhere and avoid Sync… ([`0af5077`](https://github.com/Byron/gitoxide/commit/0af5077e1f028c1c69bbdc098bb567e486282c37))
    - Try implementing find_reference to realize that this extra abstraction is overkill ([`82ea1b8`](https://github.com/Byron/gitoxide/commit/82ea1b822ac658efecb0f74643fb5d62a8269e89))
    - Look into iteration, but realize that it's harder than finding refs ([`fc753a8`](https://github.com/Byron/gitoxide/commit/fc753a8503592752d95db2aecaa33dc3615aa1fd))
    - Sketch of State is seen in store handle and store itself ([`f87f852`](https://github.com/Byron/gitoxide/commit/f87f85261f661c337b0f1638e1eabeca6957381c))
    - sketch a store handle ([`fc6480b`](https://github.com/Byron/gitoxide/commit/fc6480ba1323cf3c606a1cded100ba3ea3e983e0))
    - move `git_ref::file::WriteRefLog` to `git_ref::store::WriteRefLog` ([`9524732`](https://github.com/Byron/gitoxide/commit/95247322a8191edfa7fac9c5aa72b40239f3aa88))
 * **Uncategorized**
    - Release git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0 ([`d3f9227`](https://github.com/Byron/gitoxide/commit/d3f922781a81e8fbb81aa47afdbe9afeb06d666b))
    - Release git-features v0.18.0, git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0, safety bump 12 crates ([`acd3737`](https://github.com/Byron/gitoxide/commit/acd37371dcd92ebac3d1f039224d02f2b4e9fa0b))
    - Adjust changelogs prior to release ([`ec38950`](https://github.com/Byron/gitoxide/commit/ec3895005d141abe79764eaff7c0f04153e38d73))
    - thanks clippy ([`a74f27c`](https://github.com/Byron/gitoxide/commit/a74f27c042bdf0c1e30a1767b56032e32cbc81a9))
    - Merge branch 'pack-consistency' ([`5982406`](https://github.com/Byron/gitoxide/commit/5982406b4e1b26fd383d9ec21a3cf652ec8ab25f))
</details>

## 0.9.1 (2021-11-16)

### New Features

 - <csr-id-c0fc4f6f1b42c117275be85e1c2e6893b58007ba/> PartialNameRef<'static>::join() for building paths on the fly
 - <csr-id-b7aab9efd42975e8f2dcb5c97e51495996175702/> Allow `PartialNameRef` to be created from owned items

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 11 calendar days.
 - 27 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#251](https://github.com/Byron/gitoxide/issues/251), [#254](https://github.com/Byron/gitoxide/issues/254)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#251](https://github.com/Byron/gitoxide/issues/251)**
    - PartialNameRef<'static>::join() for building paths on the fly ([`c0fc4f6`](https://github.com/Byron/gitoxide/commit/c0fc4f6f1b42c117275be85e1c2e6893b58007ba))
    - refactor ([`244a646`](https://github.com/Byron/gitoxide/commit/244a646370dcc4e35478825922b86fe59646d86c))
    - Allow `PartialNameRef` to be created from owned items ([`b7aab9e`](https://github.com/Byron/gitoxide/commit/b7aab9efd42975e8f2dcb5c97e51495996175702))
 * **[#254](https://github.com/Byron/gitoxide/issues/254)**
    - Adjust changelogs prior to git-pack release ([`6776a3f`](https://github.com/Byron/gitoxide/commit/6776a3ff9fa5a283da06c9ec5723d13023a0b267))
 * **Uncategorized**
    - Release git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0 ([`f606fa9`](https://github.com/Byron/gitoxide/commit/f606fa9a0ca338534252df8921cd5e9d3875bf94))
    - Adjusting changelogs prior to release of git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0, safety bump 5 crates ([`39b40c8`](https://github.com/Byron/gitoxide/commit/39b40c8c3691029cc146b893fa0d8d25d56d0819))
</details>

## v0.9.0 (2021-10-19)

A maintenance release to properly dealing with previously breaking changes in `git-hash`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#222](https://github.com/Byron/gitoxide/issues/222)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#222](https://github.com/Byron/gitoxide/issues/222)**
    - update changelogs prior to release ([`9a493d0`](https://github.com/Byron/gitoxide/commit/9a493d0651b0b6d71cf230dc510a658be7f8cb19))
    - stabilize changelogs ([`920e832`](https://github.com/Byron/gitoxide/commit/920e83219911df1c440d3fe42fd5ec3a295b0bb8))
    - Update changelogs prior to release ([`b3e2252`](https://github.com/Byron/gitoxide/commit/b3e2252f7461a003d9a4612da60ba931dd8c0bef))
 * **Uncategorized**
    - Release git-hash v0.8.0, git-features v0.17.0, git-actor v0.6.0, git-object v0.15.0, git-diff v0.11.0, git-traverse v0.10.0, git-pack v0.13.0, git-odb v0.23.0, git-packetline v0.12.0, git-transport v0.13.0, git-protocol v0.12.0, git-ref v0.9.0, git-repository v0.11.0, git-commitgraph v0.6.0, gitoxide-core v0.12.0, gitoxide v0.10.0, cargo-smart-release v0.5.0, safety bump 16 crates ([`0e02953`](https://github.com/Byron/gitoxide/commit/0e029537a7f6242d02ccf7e63d8d92f5246e6c5e))
</details>

## v0.8.0 (2021-10-15)

<csr-id-4ed4b2da50557aff540685441f4b5c7d5e582977/>
<csr-id-293bfc0278c5983c0beaec93253fb51f00d81156/>
<csr-id-90e6128727932f917c485f411e623fc6a9c2ad4d/>

This release contains no functional changes, but is considered breaking for safety reasons 
as `git-traverse` is signalling a breaking change.

### Other

 - <csr-id-293bfc0278c5983c0beaec93253fb51f00d81156/> loose reference iteration with non-dir prefixes…
   Previously it was expected for the prefix `Path` to always exist for
   the prefix to be valid. This, however, is not similar to packed
   prefixes, which allow non-dir prefixes as well.
   
   Now we will check if the prefix is actually a directory, and if not
   split it into its parent directory and the filename portion. The latter
   is then used for prefix matching file names within that directory.
 - <csr-id-90e6128727932f917c485f411e623fc6a9c2ad4d/> improve changelog format

### Other

 - <csr-id-4ed4b2da50557aff540685441f4b5c7d5e582977/> add panicking `Target::id()` and `TargetRef::id()`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 22 commits contributed to the release over the course of 31 calendar days.
 - 34 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#198](https://github.com/Byron/gitoxide/issues/198)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Fix stop-release-for-changelog logic and fix all affected changelogs ([`52b38bc`](https://github.com/Byron/gitoxide/commit/52b38bc4856be5ba8b5372a3dd20f5d06504e7ed))
    - deduplicate conventional message ids ([`e695eda`](https://github.com/Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - regenerate all changelogs to get links ([`0c81769`](https://github.com/Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com/Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - respect release-wide ignore list to allow removing entire conventional headlines ([`145103d`](https://github.com/Byron/gitoxide/commit/145103d4aa715386da9d4953f7f85fadc49fff9a))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com/Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com/Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com/Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - Generate changelogs with details ([`e1861ca`](https://github.com/Byron/gitoxide/commit/e1861caa435d312953a9fea7ceff6d2e07b03443))
    - Update all changelogs with details ([`58ab2ae`](https://github.com/Byron/gitoxide/commit/58ab2aee23ba70a536e9487b44fb04c610374d1a))
    - Update changelogs ([`c857d61`](https://github.com/Byron/gitoxide/commit/c857d61ce3ce342012a2c4ba10a8327822aa530e))
    - Avoid adding newlines which make writing unstable ([`6b5c394`](https://github.com/Byron/gitoxide/commit/6b5c394f49282a8d09c2a9ffece840e4683572db))
    - Fix section headline level ([`9d6f263`](https://github.com/Byron/gitoxide/commit/9d6f263beef289d227dec1acc2d4240087cb9be6))
    - Write first version of changlogs thus far… ([`719b6bd`](https://github.com/Byron/gitoxide/commit/719b6bdf543b8269ccafad9ad6b46e0c55efaa38))
    - add panicking `Target::id()` and `TargetRef::id()` ([`4ed4b2d`](https://github.com/Byron/gitoxide/commit/4ed4b2da50557aff540685441f4b5c7d5e582977))
    - loose reference iteration with non-dir prefixes… ([`293bfc0`](https://github.com/Byron/gitoxide/commit/293bfc0278c5983c0beaec93253fb51f00d81156))
    - improve changelog format ([`90e6128`](https://github.com/Byron/gitoxide/commit/90e6128727932f917c485f411e623fc6a9c2ad4d))
 * **Uncategorized**
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com/Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - Update changelogs just for fun ([`21541b3`](https://github.com/Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
    - Merge branch 'changelog-generation' ([`bf0106e`](https://github.com/Byron/gitoxide/commit/bf0106ea21734d4e59d190b424c22743c22da966))
    - Bump git-traverse v0.9.0, safety bump 8 crates ([`d39fabb`](https://github.com/Byron/gitoxide/commit/d39fabb8757369aa19452a457f610fe21dc13a14))
</details>

## v0.7.3 (2021-09-10)

<csr-id-293bfc0278c5983c0beaec93253fb51f00d81156/>

- Compatibility with Rust 1.55. It informed about the incorrect usage of `std::io::ErrorKind::Other` which this crate also dependent on causing
  runtime bugs. The latter were detected by tests and this release contains the fix to not rely on certain behaviour anymore.

### Other

 - <csr-id-293bfc0278c5983c0beaec93253fb51f00d81156/> loose reference iteration with non-dir prefixes…
   Previously it was expected for the prefix `Path` to always exist for
   the prefix to be valid. This, however, is not similar to packed
   prefixes, which allow non-dir prefixes as well.
   
   Now we will check if the prefix is actually a directory, and if not
   split it into its parent directory and the filename portion. The latter
   is then used for prefix matching file names within that directory.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.7.3 ([`b0a9815`](https://github.com/Byron/gitoxide/commit/b0a98157ab3b240af027acb9965c981a543e55fa))
    - Update changelogs once more… ([`d57d279`](https://github.com/Byron/gitoxide/commit/d57d279dc603cf450c151bbb6dc6c6505cc6da10))
    - Update changelogs retro-actively… ([`78cfe0a`](https://github.com/Byron/gitoxide/commit/78cfe0ac341c6c2257743d913884b50042078e6c))
</details>

## v0.7.2 (2021-09-10)

<csr-id-90e6128727932f917c485f411e623fc6a9c2ad4d/>

### Other

 - <csr-id-90e6128727932f917c485f411e623fc6a9c2ad4d/> improve changelog format

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 day passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.7.2 ([`e940e9a`](https://github.com/Byron/gitoxide/commit/e940e9a21938035eb8791bba19cc16814a0fb4e7))
    - [#195] Fix previously incorrect usage of io::Kind::Other… ([`4dae07d`](https://github.com/Byron/gitoxide/commit/4dae07dc7f562395a174be6cb2220e754ff902f7))
    - thanks clippy ([`4701296`](https://github.com/Byron/gitoxide/commit/4701296bd5e2c4ad2f80f4e1de498db49f93385a))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.7.1 (2021-09-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 day passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.7.1 ([`d34191d`](https://github.com/Byron/gitoxide/commit/d34191dfd3ac3b34a3ae0d772c8b4302e5115fd6))
    - Bump git-object v0.14.0 ([`d4fc81f`](https://github.com/Byron/gitoxide/commit/d4fc81f6390443f8c8561d91ac27ea4a6318fb62))
</details>

## v0.7.0 (2021-09-07)

### Breaking

* Replace `transaction::Create` with `transaction::PreviousValue` and remove `transaction::Create`
* Remove `file::Reference` in favor of `Reference`
* Move `file::log::Line` to `log::Line`
* `TargetRef::Symbolic(&BStr)` -> `TargetRef::Symbolic(FullNameRef)`
* replace `Transaction::namespacce()` with `file::Store::namespace`
 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 50 commits contributed to the release over the course of 5 calendar days.
 - 10 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge branch 'repository-integration' ([`49f5453`](https://github.com/Byron/gitoxide/commit/49f5453629646ac24d752f53c532e5f67eb09374))
    - [repository #190] refactor ([`e7188e0`](https://github.com/Byron/gitoxide/commit/e7188e047529cb0f4b20b3876f36b4592e9d2dc4))
    - [ref #190] refactor ([`010be48`](https://github.com/Byron/gitoxide/commit/010be48d2cd2dfebf7a1b6302e94b5f2e95fedc6))
    - [ref #190] fix tests ([`e426e15`](https://github.com/Byron/gitoxide/commit/e426e15188d8ec38ee0029f1d080dbab9afd8642))
    - [ref #190] don't provide namespace support for loose and packed refs… ([`c663da1`](https://github.com/Byron/gitoxide/commit/c663da16646bc3371e5a31f5c488a775aac4f795))
    - [ref #190] find() with namespace support ([`1240c21`](https://github.com/Byron/gitoxide/commit/1240c21a353c7df736f40b6639076af94eae0f15))
    - [ref #190] prepare test for namespaced find(…) ([`5fcd0e4`](https://github.com/Byron/gitoxide/commit/5fcd0e4c3c803a372360ef4cc2a7663b17ccebdb))
    - [repository #190] leverage git-ref namespace support ([`1aa9c11`](https://github.com/Byron/gitoxide/commit/1aa9c113488175f03758f8a64338a33b3417dd87))
    - [ref #190] iteration with namespace support ([`d5987d4`](https://github.com/Byron/gitoxide/commit/d5987d41753cf083573d86e8d5bc86c7a825605c))
    - [ref #190] refactor ([`3c7968c`](https://github.com/Byron/gitoxide/commit/3c7968c7fe8ac166b01f5338b23f817899dc085e))
    - [repository #190] prepare for namespacing support on file store level ([`d2d1db6`](https://github.com/Byron/gitoxide/commit/d2d1db647e6ad0dd92b88ce57df866f5195b8dd6))
    - [repository #190] refactor ([`609c249`](https://github.com/Byron/gitoxide/commit/609c249916ca64f4beecdab86eb4562adbd1ca4f))
    - [ref #190] refactor ([`1ef6cb3`](https://github.com/Byron/gitoxide/commit/1ef6cb344176aeafcc61a1f1af503a3f8afd1f77))
    - [repository #190] fix build ([`f5e118c`](https://github.com/Byron/gitoxide/commit/f5e118c8871e45ed3db9da9cd6bc63a5ea99621e))
    - [repository #190] note a known limitation about finding references in namespaces… ([`d335731`](https://github.com/Byron/gitoxide/commit/d3357318cf100fc3e0751e5b6de3922b1c209ddb))
    - [ref #190] more assetions to understand 'find(…)' for namespaced refs… ([`f58a0ff`](https://github.com/Byron/gitoxide/commit/f58a0ff8be6144d1dcb97f9b8030e1ee36ce41de))
    - [repository #190] transparent namespace support ([`d14f073`](https://github.com/Byron/gitoxide/commit/d14f073707c2f4641a271ba7965ec8281638e8df))
    - [ref #190] Make References sortable ([`16b2232`](https://github.com/Byron/gitoxide/commit/16b2232c70ad331e17e76ccca3b950963906aa81))
    - [repository #190] cleanup usage of bstr… ([`e4411ff`](https://github.com/Byron/gitoxide/commit/e4411ff43b24af79fefeaa4411e004dc504a4e2a))
    - [ref #190] more conversion trait impls ([`1795a33`](https://github.com/Byron/gitoxide/commit/1795a333c05c60a1a2f3164d5c4c78289eb7050c))
    - Bump git-hash v0.6.0 ([`6efd90d`](https://github.com/Byron/gitoxide/commit/6efd90db54f7f7441b76159dba3be80c15657a3d))
    - [repository #190] obtain the kind fo hash used in a repo ([`a985491`](https://github.com/Byron/gitoxide/commit/a985491bcea5f76942b863de8a9a89dd235dd0c9))
    - [ref #190] refactor ([`e34be7e`](https://github.com/Byron/gitoxide/commit/e34be7e24ee49a539b6ee8dc5737fdb23f416922))
    - [ref #190] more Target conversions… ([`1fe1b42`](https://github.com/Byron/gitoxide/commit/1fe1b42ac2b04f8145fc7312ea03cb47f791aec5))
    - [repository #190] refactor ([`7a111b1`](https://github.com/Byron/gitoxide/commit/7a111b126cfb318acb2d09d119315150a38b7cd3))
    - [ref #190] refactor ([`49fe1dc`](https://github.com/Byron/gitoxide/commit/49fe1dc37c040206839c9d4399001ff12dc91174))
    - [ref #190] reverse reflog ergonomics ([`2de86f9`](https://github.com/Byron/gitoxide/commit/2de86f904f6ee63e292f9c701cc3524e8bfe87e4))
    - [ref #190] check for zero sized buffers in reverse log iterators… ([`998c7c6`](https://github.com/Byron/gitoxide/commit/998c7c65abb2c3eb5fc248b11ba816d09f1bedea))
    - [ref #190] move remaining file store functions to extension trait ([`60fc215`](https://github.com/Byron/gitoxide/commit/60fc215ccac529b4a14cb9d8260ab9ddec86758a))
    - [ref #190] Move file-log-specific functionality into own extension trait. ([`0b635e9`](https://github.com/Byron/gitoxide/commit/0b635e9778a98235cc9b47b12e58a175d1ca02b7))
    - [repository #190] a major step forward with `head()` access ([`43ac4f5`](https://github.com/Byron/gitoxide/commit/43ac4f5acbe3ace5d43ed3ed1bc394d721f0e273))
    - [ref #190] cache peeled objects properly ([`2cb511e`](https://github.com/Byron/gitoxide/commit/2cb511efe5833f860f3c17b8e5f5b4cd643baddb))
    - [ref #190] fix docs ([`3e64ec1`](https://github.com/Byron/gitoxide/commit/3e64ec102146e348b8d870377f180f8dadf5e876))
    - Bump git-ref v0.7.0 ([`ac4413c`](https://github.com/Byron/gitoxide/commit/ac4413ce4e45703d5fe722e7220d039217f0bdef))
    - [ref #190] fix remaining tests ([`df21f25`](https://github.com/Byron/gitoxide/commit/df21f25baaf867015fc9fc46a2cf4e778b0e80ee))
    - thanks clippy ([`14dff63`](https://github.com/Byron/gitoxide/commit/14dff63fbc0d318bbc8a2618e0d72aaa98948acf))
    - [ref #190] Use Raw Reference everywhere for great simplification… ([`7aeea9c`](https://github.com/Byron/gitoxide/commit/7aeea9c36d4da04a806e68968356f8cc0dc11475))
    - [ref #190] raw reference peeling ([`9473a71`](https://github.com/Byron/gitoxide/commit/9473a71e5533e1474181241f8d3e1aebd9dea8d8))
    - [ref #190] introduce Raw reference type that simplifies everything… ([`8634341`](https://github.com/Byron/gitoxide/commit/86343416dec8026f32c57d164dec4bf9b75b6536))
    - [ref #190] more tests ([`980e16a`](https://github.com/Byron/gitoxide/commit/980e16a10806edba4553716d9533716a727f0c9e))
    - [ref #190] deletions also use PreviousValue now ([`74f85b1`](https://github.com/Byron/gitoxide/commit/74f85b1fd8d9c34eca34a5ae516c4768f96b092f))
    - [ref #190] refactor ([`0e65559`](https://github.com/Byron/gitoxide/commit/0e65559e6d5a4b06c552e99e9c463559737f4b4d))
    - [ref #190] be explicit about what the previous reflog oid is for… ([`c04c8b9`](https://github.com/Byron/gitoxide/commit/c04c8b98a074d277067cee73ddef0609419a7bb8))
    - [ref #190] don't claim there was a previous oid unnecessarily… ([`68f7fc2`](https://github.com/Byron/gitoxide/commit/68f7fc2f2f57c32412ee2e46befc9cd2fdd7e973))
    - [ref #190] refactor ([`07126d6`](https://github.com/Byron/gitoxide/commit/07126d65946e981b339b6535986597cb328a1c9e))
    - [ref #190] Allow for explicit expected previous values ([`1a4786f`](https://github.com/Byron/gitoxide/commit/1a4786fb3bdb3d3a86b026dbf04e6baef6d3c695))
    - [ref #190] prepare massive refactoring to get additional constraint ([`9741987`](https://github.com/Byron/gitoxide/commit/9741987e2f82b5ae202804882c728c1642d8e3a4))
    - [refs #190] refactor; handle value-checks in dereffed symlinks correctly ([`63bedc7`](https://github.com/Byron/gitoxide/commit/63bedc7647bb584353289e19972adf351765a526))
    - [ref #190] refactor ([`3f36a01`](https://github.com/Byron/gitoxide/commit/3f36a01976a149d518021f19d83e56dec43cfb98))
    - [object #190] More conversion methods for Object ([`78bacf9`](https://github.com/Byron/gitoxide/commit/78bacf97d669f3adfebdb093054c162cfd5214fa))
</details>

## v0.6.1

### Bugfixes

* splits of edits to symbolic references will now 'move' the desired previous values down to the
  referents while resorting to not having any requirements in the symbolic ref instead.

## v0.6.0 (2021-08-27)

### BREAKING

- rename `file::Store::packed()` to `file::Store::packed_buffer()`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 27 commits contributed to the release over the course of 8 calendar days.
 - 10 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [odb #180] refactor ([`eff21da`](https://github.com/Byron/gitoxide/commit/eff21dae1083042412f45cd2f7a0faaf7d6400e6))
    - [pack #179] refactor ([`ab6554b`](https://github.com/Byron/gitoxide/commit/ab6554b0cd5838f1ea4e82f6b5019798288076fa))
    - [object #177] fix docs ([`2fd23ed`](https://github.com/Byron/gitoxide/commit/2fd23ed9ad556b8e46cf650e23f0c6726e304708))
    - [object #177] tag::RefIter -> TagRefIter ([`28587c6`](https://github.com/Byron/gitoxide/commit/28587c691eb74e5cb097afb2b63f9d9e2561c45d))
    - [object #177] migrate immutable::tree to crate::tree ([`fa5cd06`](https://github.com/Byron/gitoxide/commit/fa5cd0648d5c855060ab2b75ee933851987c2dcf))
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments ([`461dc53`](https://github.com/Byron/gitoxide/commit/461dc53ba3bc07d55fdb4aad7570ba9176a8b360))
    - [object #177] rename immutable::* to immutable::*Ref ([`6deb012`](https://github.com/Byron/gitoxide/commit/6deb01291fb382b7fb9206682e319afa81bacc05))
    - Release git-object v0.13.0 ([`708fc5a`](https://github.com/Byron/gitoxide/commit/708fc5abd8af4dd7459f388c7092bf35915c6662))
    - Merge branch 'git-ref-refactor' ([`5dbf753`](https://github.com/Byron/gitoxide/commit/5dbf753ce2035ffd07e4bce7ceb3bcd4e309c16e))
    - [ref #175] follow (try_)find(_what) naming convention ([`679895c`](https://github.com/Byron/gitoxide/commit/679895cf866d643e768e353af614a55aeed2ba5c))
    - [ref #175] fix docs ([`dd1edc3`](https://github.com/Byron/gitoxide/commit/dd1edc34f88231fa95cf6f88beead700c6289ba1))
    - [ref #175] refactor log line ([`7ac948a`](https://github.com/Byron/gitoxide/commit/7ac948a8f8610b87aa2773ba2841cbfa43eecae4))
    - [ref #175] refactor ([`1243459`](https://github.com/Byron/gitoxide/commit/1243459e917b394d007bd7c157143670dc8dd51f))
    - [ref #175] make 'mutable' module private ([`a80dbcf`](https://github.com/Byron/gitoxide/commit/a80dbcf083bfcf2e291013f7b13bba9e787c5cb4))
    - Release git-actor v0.5.0 ([`a684b0f`](https://github.com/Byron/gitoxide/commit/a684b0ff96ebfc5e4b3ce78452dc21ce856a6869))
    - [actor #175] refactor ([`ec88c59`](https://github.com/Byron/gitoxide/commit/ec88c5905194150cc94db4d4c20e9f4e2f6595c3))
    - [ref #175] refactor ([`292e567`](https://github.com/Byron/gitoxide/commit/292e567eaa04a121fb4d7262bb316d37dd8ad11f))
    - Release git-actor v0.4.0 ([`16358c9`](https://github.com/Byron/gitoxide/commit/16358c9bf03604857d51bfa4dbfd2fc8c5210da7))
    - [actor #173] rename immutable::Signature to SignatureRef! ([`96461ac`](https://github.com/Byron/gitoxide/commit/96461ace776d6b351b313d4f2697f2d95b9e196e))
    - Release git-lock v1.0.0 ([`f38f72c`](https://github.com/Byron/gitoxide/commit/f38f72c73f69775358d8b047de2e354364fcafc2))
    - Release git-tempfile v1.0.0 ([`1238535`](https://github.com/Byron/gitoxide/commit/123853539dc30ddea2d822ab177ee09b191bdf1b))
    - Merge branch 'Byron:main' into main ([`dc58eca`](https://github.com/Byron/gitoxide/commit/dc58eca510e5a067acdeaad4b595a34b4598a0cd))
    - Upgrade to nom-7 ([`f0aa3e1`](https://github.com/Byron/gitoxide/commit/f0aa3e1b5b407b2afd187c9cb622676fcddaf706))
    - [repository #165] fix docs ([`b4fdfd7`](https://github.com/Byron/gitoxide/commit/b4fdfd7a21057f89f4b6263c0c291003241e2833))
    - Release git-ref v0.6.0 ([`0bb4c13`](https://github.com/Byron/gitoxide/commit/0bb4c133da96f6a96d9f1767848ada792a27c2be))
    - [ref #165] refactor ([`66624c3`](https://github.com/Byron/gitoxide/commit/66624c3ef1faf7048ee86ed73cf5f622802c061e))
    - [repository #165] refactor ([`00ec15d`](https://github.com/Byron/gitoxide/commit/00ec15dcfdb839095e508139d238df384ea418eb))
</details>

## v0.5.4 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.5.4 ([`bc5d860`](https://github.com/Byron/gitoxide/commit/bc5d860a616fd5a4371792a8ecde6e6356e217f8))
    - [smart-release #162] FAIL: one level down, using the cache isn't really working… ([`65db010`](https://github.com/Byron/gitoxide/commit/65db0104146248b273081fc6616a6ed484aa948e))
    - [ref] Out of bounds check to prevent legitimate panic ([`303608c`](https://github.com/Byron/gitoxide/commit/303608cbc1ade71c635dd1bbbe60988d09184351))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.5.3 (2021-08-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.5.3 ([`e6a8020`](https://github.com/Byron/gitoxide/commit/e6a8020ff9b85c6dfedd80525c571514e039edae))
    - [ref #157] Support for unsorted packed refs and those without header ([`2724688`](https://github.com/Byron/gitoxide/commit/272468892c02133efd68d15ffc5cacb4d5c5cd78))
</details>

## v0.5.2 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 2 calendar days.
 - 2 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.5.2 ([`50dcca9`](https://github.com/Byron/gitoxide/commit/50dcca97e207ec608e506adcef90dd0599b4441d))
    - remove dev-dependency cycles by removing their version ([`c40faca`](https://github.com/Byron/gitoxide/commit/c40faca41632cd2a226daf4ddf5293b65d1fdc82))
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 ([`f123f69`](https://github.com/Byron/gitoxide/commit/f123f69c7a4f9fd1c98bd2f60ebc953a6739fe04))
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 ([`c67291f`](https://github.com/Byron/gitoxide/commit/c67291ff9bcdff9a747d87241f6a71015607af05))
    - Release git-object v0.12.0 ([`7006150`](https://github.com/Byron/gitoxide/commit/7006150ac314d19814608723f69f6e70a72f9262))
    - Release git-actor-0.3.1 ([`727087d`](https://github.com/Byron/gitoxide/commit/727087dca243da4bc40bc87611a2f66234565be7))
    - [utils #154] commit manifest changes; create tags ([`95dcd9d`](https://github.com/Byron/gitoxide/commit/95dcd9d7d060101596c51116218102cc8049d0dd))
    - (cargo-release) version 0.3.0 ([`263088b`](https://github.com/Byron/gitoxide/commit/263088b3faaccd9edae8c21dfc7d39b191d76207))
    - (cargo-release) version 0.18.0 ([`b327590`](https://github.com/Byron/gitoxide/commit/b327590d02fec5536c380b2d39dd7be089ca7c40))
    - (cargo-release) version 0.17.0 ([`c52a491`](https://github.com/Byron/gitoxide/commit/c52a49176bd294bb36db74b4293cdb684a2ab7f6))
    - (cargo-release) version 0.6.0 ([`d58f37e`](https://github.com/Byron/gitoxide/commit/d58f37e3b5a000fbe069aa869bd84f66d5c3210b))
    - (cargo-release) version 0.11.0 ([`a5be31c`](https://github.com/Byron/gitoxide/commit/a5be31c4cf7c0b538a1ed4a52ff5c3a992c6feff))
    - (cargo-release) version 0.5.0 ([`bf15c2a`](https://github.com/Byron/gitoxide/commit/bf15c2a2f285046b094093760c1969007ee75e25))
    - (cargo-release) version 0.3.0 ([`64efc05`](https://github.com/Byron/gitoxide/commit/64efc0534ddc372b6e668b23c1e9d276098679c9))
    - (cargo-release) version 0.4.0 ([`70ef344`](https://github.com/Byron/gitoxide/commit/70ef3442775b54ba9e4ee9ebfffb37af9804cc5b))
    - Revert "[ref] break dev-dependency cycle" ([`436e89b`](https://github.com/Byron/gitoxide/commit/436e89b18cb157b3d30bd24b8d1acef25631ec2a))
</details>

## v0.5.1 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.1 ([`6f61fca`](https://github.com/Byron/gitoxide/commit/6f61fcaf9528f2ba6752ce94524b59ff505cc518))
    - [ref] break dev-dependency cycle ([`d5af428`](https://github.com/Byron/gitoxide/commit/d5af42898487a82f2fbd000fac2f0db9505a587c))
</details>

## v0.5.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 390 commits contributed to the release over the course of 78 calendar days.
 - 233 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`ae02dab`](https://github.com/Byron/gitoxide/commit/ae02dabae961089a92a21e6a60a7006de4b56dad))
    - (cargo-release) version 0.4.0 ([`0d5c8b9`](https://github.com/Byron/gitoxide/commit/0d5c8b96dfdfb96e4fc82623f756f6c7f7046e90))
    - (cargo-release) version 0.16.0 ([`1231dbd`](https://github.com/Byron/gitoxide/commit/1231dbd16dacefb39adec8e067c312d313a82e3c))
    - (cargo-release) version 0.2.0 ([`20d8e27`](https://github.com/Byron/gitoxide/commit/20d8e27dd4e93ae2234a3fe19b5f1511365eee2e))
    - (cargo-release) version 0.5.0 ([`0e11e98`](https://github.com/Byron/gitoxide/commit/0e11e98f0562c7baa9c90e18db6240731d165217))
    - (cargo-release) version 0.2.0 ([`8ff5115`](https://github.com/Byron/gitoxide/commit/8ff511583e6d859e43ffda0ef75e2fecce3ed03c))
    - [ref] refactor ([`501182b`](https://github.com/Byron/gitoxide/commit/501182b106b70af73db4f23cc01291d30481f76e))
    - [ref #152] remaining tests for transaction namespacing ([`63d80c0`](https://github.com/Byron/gitoxide/commit/63d80c0d0fbcf4fd1b7c3db652f622b59bc6fd18))
    - [ref #152] first succeeding test for namespace rewriting ([`758c8f6`](https://github.com/Byron/gitoxide/commit/758c8f60ca6567cd0a12892490ce27f88d1140df))
    - [ref #152] first failing test for namespaced updates ([`a81f1d4`](https://github.com/Byron/gitoxide/commit/a81f1d44a83474152d53140f8d9fdd0ace8060ac))
    - [ref #152] refactor ([`f9c63fb`](https://github.com/Byron/gitoxide/commit/f9c63fbe70ceb10bc3ef3edee008f72c3494b18c))
    - [ref #152] namespace prefix stripping and fixed test expectations ([`bce135b`](https://github.com/Byron/gitoxide/commit/bce135b7c58ba5f709aad2daab0e1668a834a4cd))
    - [ref #152] a test for namespaced iteration ([`2338c6e`](https://github.com/Byron/gitoxide/commit/2338c6e96e3dbd0759c122e264044c195f16a269))
    - [ref #152] packed-refs are optional for generalized iteration, too ([`88525a9`](https://github.com/Byron/gitoxide/commit/88525a9f028e94c8647ad5f2f7067b5b4e01c0a3))
    - [ref #152] FAIL: cleanup iter API by allowing Option<packed::Buffer> ([`1836243`](https://github.com/Byron/gitoxide/commit/1836243b6ec42eaf162463cded4a613c8984ac3a))
    - [ref #152] prepare namespaced iteration tests ([`cf5abc9`](https://github.com/Byron/gitoxide/commit/cf5abc96115f4bab0ee52f58295f06f689173bf8))
    - [ref #152] no silent failure if path conversion isn't possible ([`8df04d8`](https://github.com/Byron/gitoxide/commit/8df04d8973fc62eae0e8d98c8116351907dd282f))
    - [ref #152] introduce Namespace type ([`67d5c85`](https://github.com/Byron/gitoxide/commit/67d5c8526d8356bcee81b690a38559a01128863b))
    - [ref #152] sketch API for namespaces ([`138be95`](https://github.com/Byron/gitoxide/commit/138be9588576eca84921cedcf5f697b5c98e85a7))
    - [ref #152] docs ([`8d6c856`](https://github.com/Byron/gitoxide/commit/8d6c8564faeccafc1430a2184a4060d953349e3f))
    - [ref #152] refactor ([`bfb82fb`](https://github.com/Byron/gitoxide/commit/bfb82fb13350d986c93cc6dc67d6f86506dd80a5))
    - [ref #152] all tests and impl for refname expansion ([`9cef2f2`](https://github.com/Byron/gitoxide/commit/9cef2f2f166514048fae52ceec5a86a2849be286))
    - [ref #152] refactor ([`431dd86`](https://github.com/Byron/gitoxide/commit/431dd8655397b0ae88a5144d5c8553ba63e46c8f))
    - [ref #152] basic test setup for namespace expansion ([`e852399`](https://github.com/Byron/gitoxide/commit/e8523996b73fb93218c651b6f6041935833293d0))
    - clippy on tests and thanks clippy ([`a77a71c`](https://github.com/Byron/gitoxide/commit/a77a71cf02d328a2a964388928d6b2a235a0aa85))
    - thanks clippy ([`e1964e4`](https://github.com/Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - [ref #140] finish implementation of tag peeling, with test ([`c06e729`](https://github.com/Byron/gitoxide/commit/c06e72916e9622df62579baa6817603af0c7c747))
    - [ref #140] refactor ([`edcc395`](https://github.com/Byron/gitoxide/commit/edcc3951bd0fc98589207a1b1f8941d6bb9652ab))
    - [ref #140] sketch ref tag peeling ([`ef90652`](https://github.com/Byron/gitoxide/commit/ef90652dfcd84b2fc140c38e1364b42578fdfbde))
    - [ref #140] refactor ([`8e1a730`](https://github.com/Byron/gitoxide/commit/8e1a7305e869979751230f23c614f276ebce3f1d))
    - [ref #139] add missing docs ([`5422ec8`](https://github.com/Byron/gitoxide/commit/5422ec8923a5f3c284f7094894a952a392812e63))
    - [ref #139] my first empty test but where else to document this :)? ([`0f00065`](https://github.com/Byron/gitoxide/commit/0f00065fa3360a55cc52926bfaa94d72598933b5))
    - [ref #139] refactor ([`a8f5d8d`](https://github.com/Byron/gitoxide/commit/a8f5d8dbaecaa26509d568a36acbf350ee86a03c))
    - [ref #139] peeling for all refs to be written to a pack ([`cc891a1`](https://github.com/Byron/gitoxide/commit/cc891a1809a6678f168b08766f67644742386a5d))
    - [ref #139] refactor ([`7e15817`](https://github.com/Byron/gitoxide/commit/7e1581788356889a936f4a778119b0bce36d3041))
    - [ref #139] Allow packed-refs creation in the presence of updates ([`0cf7314`](https://github.com/Byron/gitoxide/commit/0cf7314df7a6ab79478525544e0ed28d07cf3642))
    - [ref #139] impl of loose ref deletion, but it doens't work yet… ([`f6631ad`](https://github.com/Byron/gitoxide/commit/f6631ad537b4c7fd6dec2a511214552e606462d4))
    - [ref #139] a failing test for pruning loose refs into packed refs ([`437c610`](https://github.com/Byron/gitoxide/commit/437c610eeb3b4a5874f001ba6fbbd42c7dc1188e))
    - [ref #139] refactor ([`62558cb`](https://github.com/Byron/gitoxide/commit/62558cb562747d3c6f2b4e1b62dd44e4f1e95019))
    - [ref #139] a first sketch to resolve object chains for packed ref peeling ([`54bc116`](https://github.com/Byron/gitoxide/commit/54bc1161128f0c719622935728a870820918038b))
    - [ref #139] Allow 'git pack-ref --no-purge' essentially ([`c32d8b7`](https://github.com/Byron/gitoxide/commit/c32d8b7a599c0ee0d8936a0c5aee658b5d986453))
    - [ref #139] refactor ([`e5fbc4c`](https://github.com/Byron/gitoxide/commit/e5fbc4c92f0ea74afdff45c243a762e7a978d749))
    - [ref #139] refactor ([`4e1b95e`](https://github.com/Byron/gitoxide/commit/4e1b95e40e94b0c9398c40985e092bd1d8607a4c))
    - [ref #139] refactor ([`42215a1`](https://github.com/Byron/gitoxide/commit/42215a15ce53bd78fe1d8d9b15d7a08919f5f980))
    - [ref #139] a complete test for the first packed-refs mode ([`f332dcf`](https://github.com/Byron/gitoxide/commit/f332dcf2b1beda319871f7b0de585c8a1d9b813f))
    - [ref #138] delete packed-refs when it's empty after rewrite ([`8b7c359`](https://github.com/Byron/gitoxide/commit/8b7c359db1c81ae69321c9c2637d0af8b303d9bb))
    - [ref #138] refactor ([`3fc0014`](https://github.com/Byron/gitoxide/commit/3fc0014dbf3c6a0d0c3e34d39c3068c71f867fd1))
    - [ref #138] no need for preprocessing, input is already checked ([`a6fca6e`](https://github.com/Byron/gitoxide/commit/a6fca6e0f81cdccfd7284d70ad4218e94b6cbe24))
    - [ref #138] less is more… ([`6f39713`](https://github.com/Byron/gitoxide/commit/6f3971325380dee93370a2d6a05d43adec94181b))
    - thanks clippy ([`169a39d`](https://github.com/Byron/gitoxide/commit/169a39d72106c24dac78af2198e54ca6e09b743e))
    - [ref] the first green packed deletion… ([`76a23b0`](https://github.com/Byron/gitoxide/commit/76a23b0e3e508a3445a9e1c77045e59bb7bbef69))
    - [ref] refactor (packed refs aren't changed in memory) ([`0a7e8ce`](https://github.com/Byron/gitoxide/commit/0a7e8ce1be7c7e6cb8a7646a8dacc7e95acf5efd))
    - [ref] basic packed transaction commit impl, but it doesn't work yet ([`1913099`](https://github.com/Byron/gitoxide/commit/1913099eeb84e78d9b4373e6ba9823a493d82343))
    - [ref] fix order of operations when committing the transaction ([`be5774a`](https://github.com/Byron/gitoxide/commit/be5774a3d5e8fa20eadc6ef6f0bbfceab35f1827))
    - [ref] refactor ([`69d53f9`](https://github.com/Byron/gitoxide/commit/69d53f99097220cf3a5e3e5afa855d1847715007))
    - [ref] first revised sketch of packed-refs writing ([`f942c76`](https://github.com/Byron/gitoxide/commit/f942c7622cf09d3c6937c7fa78089991d58482a0))
    - [ref] work on first naive transaction, but… ([`b08cc4a`](https://github.com/Byron/gitoxide/commit/b08cc4a47ecf8ad5f4b56ffdaf678946549b0ae9))
    - [ref] tests incorporating packed-ref deletion ([`399096e`](https://github.com/Byron/gitoxide/commit/399096e0f611a649fb99facc0925adc1c306cbfe))
    - [ref] validate packed refs are taken into consideration during create/update ([`25999b4`](https://github.com/Byron/gitoxide/commit/25999b4cebcb925bf0f0d4f451c7ca557f03dbc2))
    - [ref] allow creating new packed-refs files as well; prepare test arena ([`8494c74`](https://github.com/Byron/gitoxide/commit/8494c7452f68bb3ebe7bc9115b7feb36871a406a))
    - [ref] refactor ([`e379177`](https://github.com/Byron/gitoxide/commit/e379177a1937fdc23cba843d2dc6fecd3dfd2ab2))
    - [ref] refactor ([`a844146`](https://github.com/Byron/gitoxide/commit/a844146a799e07c3d95c4224b4a114b77cd94832))
    - [ref] refactor ([`bd94ea5`](https://github.com/Byron/gitoxide/commit/bd94ea55c1b598e507b5717ee5a5d6f14830c3bb))
    - [ref] actually make use of packed refs in file transactions ([`7746238`](https://github.com/Byron/gitoxide/commit/7746238207b637d4f241a05af7814916736cce24))
    - [ref] refactor ([`7a7b0dc`](https://github.com/Byron/gitoxide/commit/7a7b0dcd8b9156a5c67bbdcdebb6a2a2e2757a7e))
    - [ref] refactor ([`74ed358`](https://github.com/Byron/gitoxide/commit/74ed358c7ef6147095e8df9eb29b34ab55c850f4))
    - [ref] first basic sketch of packed-ref transaction ([`8aac30c`](https://github.com/Byron/gitoxide/commit/8aac30c77b03aa6c020d46c79f54d031043351df))
    - [ref] on the way to requiring a packed transaction for file transactions ([`85f30ac`](https://github.com/Byron/gitoxide/commit/85f30ac10fa740293d72f558dbd48a14aee82fde))
    - [ref] prepare existing refs to take packed-refs into account… ([`5849b44`](https://github.com/Byron/gitoxide/commit/5849b44c87c8b9ca68d7d30623540d8d441b6a3f))
    - [ref] remove one todo, add another… ([`46c47ab`](https://github.com/Byron/gitoxide/commit/46c47ab440df49d0f3a5324b243cdcf5a2898e03))
    - [ref] all todos done ([`7632573`](https://github.com/Byron/gitoxide/commit/763257327632b39a5ec777df4f07da9f87005a36))
    - [ref] refactor ([`fb37e96`](https://github.com/Byron/gitoxide/commit/fb37e9612c03cf1fcf5cdef9241a35242b9ff1d0))
    - [ref] refactor ([`23ea139`](https://github.com/Byron/gitoxide/commit/23ea139e0af622e8d40774fa2a890ef3525a991a))
    - [ref] rev-iter for overlay references ([`8b28d4a`](https://github.com/Byron/gitoxide/commit/8b28d4a326a2ee43bd00e475a0376eb577145a8b))
    - [ref] refactor ([`a80b8c1`](https://github.com/Byron/gitoxide/commit/a80b8c18eb5cfc77ca5e071e9163df0a89a35fd4))
    - [ref] tests for remaining todos ([`0ef6b3d`](https://github.com/Byron/gitoxide/commit/0ef6b3dbdc7f8c67e69eeb453122ce2722d171fa))
    - [ref] remove loose::Reference backref to simplify everything ([`9f1d960`](https://github.com/Byron/gitoxide/commit/9f1d960ae07d368f3ab208cf886ea1af99dfe25f))
    - Revert "[ref] back-reference of packed refs to their packed buffer" ([`464aefe`](https://github.com/Byron/gitoxide/commit/464aefe563c045b30ead0144b97a41d7b353235e))
    - Revert "[ref] FAIL: let's not add more back-refs, let's add less" ([`eaf4e9a`](https://github.com/Byron/gitoxide/commit/eaf4e9a1582fcd3c1d1da9eba3fb4c7046a5cdb9))
    - [ref] FAIL: let's not add more back-refs, let's add less ([`8e90d75`](https://github.com/Byron/gitoxide/commit/8e90d7545d4bda92e339387acfa1c882e2a99264))
    - [ref] back-reference of packed refs to their packed buffer ([`da860ef`](https://github.com/Byron/gitoxide/commit/da860efa8fb42f9f755cd9070732fc4403843cc9))
    - [ref] refactor ([`61972a2`](https://github.com/Byron/gitoxide/commit/61972a298bfcbad7efe23a480895fc26bb53bf24))
    - [ref] refactor ([`f03c614`](https://github.com/Byron/gitoxide/commit/f03c6144f395fd8713157a4a3137c6c0dacd41da))
    - thanks clippy ([`08f8bc4`](https://github.com/Byron/gitoxide/commit/08f8bc4c09ad85df0ea75916f8bd9beb061069ea))
    - [ref] probably fix windows ([`6eb2532`](https://github.com/Byron/gitoxide/commit/6eb2532724d6be1b25b68b10b58cd504ff1a7af9))
    - [ref] refactor ([`3df606a`](https://github.com/Byron/gitoxide/commit/3df606aa33ab8c161a7b36b79a9661eefac218e7))
    - [ref] test for peel one level of packed ref ([`3d8602f`](https://github.com/Byron/gitoxide/commit/3d8602f2fff98e3a1078c24e65cd887bebc7fa78))
    - [ref] assure packed-refs have a consistent target after peeling. ([`29a352a`](https://github.com/Byron/gitoxide/commit/29a352a24c0e2685d06672967e4898abfa1c2f8c))
    - thanks clippy ([`321908e`](https://github.com/Byron/gitoxide/commit/321908e12a885978dc4fa3fa1f71cebc8efdf741))
    - [ref] improve import paths ([`2dbe785`](https://github.com/Byron/gitoxide/commit/2dbe785d80d56b2d9f5a617b57a02926dba70434))
    - [ref] refactor ([`49fc212`](https://github.com/Byron/gitoxide/commit/49fc212e9e82382d06da16dc9b84e3952a73ddce))
    - [ref] prepare to create loose:Reference ([`8ed3916`](https://github.com/Byron/gitoxide/commit/8ed3916564917fd99a74dda06d35f4390e918fa5))
    - [ref] refactor ([`f222525`](https://github.com/Byron/gitoxide/commit/f2225253de054ce8cfa8f8ce33a93c3ac613dc85))
    - [ref] finally peeling works again ([`d5bd75a`](https://github.com/Byron/gitoxide/commit/d5bd75acdf48f7a274dbb88441f003d5d287e3b8))
    - [ref] packed-refs are now enforcing valid names ([`5d92919`](https://github.com/Byron/gitoxide/commit/5d9291976370edae3a8429e745174147c1fadf90))
    - [ref] prepare peel test; realize another refactoring requirement ([`62f7155`](https://github.com/Byron/gitoxide/commit/62f71552da037c126058b7bcaa9e6bab8e2c168b))
    - [ref] refactor ([`ae4d5da`](https://github.com/Byron/gitoxide/commit/ae4d5da10fc6e0ec5015539a1285f1a3dbbc9628))
    - [ref] refactor ([`e26c72f`](https://github.com/Byron/gitoxide/commit/e26c72fb1bf9392932ffe42843f3dec52c7bbd7d))
    - [ref] refactor ([`f4bb7a0`](https://github.com/Byron/gitoxide/commit/f4bb7a02d8e8b820f30894ac74613bee10532c79))
    - [ref] another test to run into one more todo ([`13502f5`](https://github.com/Byron/gitoxide/commit/13502f5bb7b1df7abd1d2de4f9e93a9e5439b84f))
    - [ref] some TODOs to not forget ([`4d6a75c`](https://github.com/Byron/gitoxide/commit/4d6a75cc6835cbd1f6ab321e158310c97def2a71))
    - [ref] and it compiles again, may todos left ([`16618b9`](https://github.com/Byron/gitoxide/commit/16618b916ff67316717d95575fc1344d956d2c49))
    - [ref] all required Reference methods are defined, but… ([`3c976a6`](https://github.com/Byron/gitoxide/commit/3c976a65cad62e4e04c686b1e8f645bf300ccf41))
    - [ref] refactor ([`65f7a7d`](https://github.com/Byron/gitoxide/commit/65f7a7db56d6db974db197101b6306dbb7483ff5))
    - [ref] changing the ref type means a lot of breakage and some unsolved problems ([`407dc4d`](https://github.com/Byron/gitoxide/commit/407dc4d79a4281fc3ec09456bb6f969f42bbabd7))
    - [ref] refactor to be able to use loose_then_packed::Reference for top-level find ([`2c4e45a`](https://github.com/Byron/gitoxide/commit/2c4e45a5bf997530d84a214714ff25fdbbcafd16))
    - [ref] figure out how peeling works with packed-refs… ([`2801f7a`](https://github.com/Byron/gitoxide/commit/2801f7aa137c6167bd392ca585f1aad378cae0b4))
    - Revert "[ref] FAIL: actually it's enough to give access to 'packed' when peeling only" ([`8dc6295`](https://github.com/Byron/gitoxide/commit/8dc62955f1a8b92f08924f155c932d0dfbf415ef))
    - [ref] FAIL: actually it's enough to give access to 'packed' when peeling only ([`5173a97`](https://github.com/Byron/gitoxide/commit/5173a97531f213573da12d0d9dda8e0bc808c013))
    - [ref] put packed-ref lookups into the correct spot ([`6d11e22`](https://github.com/Byron/gitoxide/commit/6d11e22c723f03155f12878ac7b94ef959f633a4))
    - [ref] remove over-complicated refs store trait which… ([`1cc876c`](https://github.com/Byron/gitoxide/commit/1cc876cde25820a7a8afa8d867dec59e6079d72e))
    - [ref] refactor ([`62e682c`](https://github.com/Byron/gitoxide/commit/62e682c269c48a9eb2c25f4bb6421b8647fb3fab))
    - [ref] API sketch for allowing packed-refs to be used in find() ([`ca736ab`](https://github.com/Byron/gitoxide/commit/ca736ab2ee8eab337683ff66e6e07d4488ff15da))
    - [ref] fix windows build ([`f99851b`](https://github.com/Byron/gitoxide/commit/f99851bc3195aca958409bd5773e6210037b07f8))
    - [ref] assure names are using forward slashes in file-based refs ([`ff695e4`](https://github.com/Byron/gitoxide/commit/ff695e4dae73d1497290d1efcc77b0cf1b265617))
    - [ref] prefix iteration for all references ([`228ca00`](https://github.com/Byron/gitoxide/commit/228ca00a91069ebe32dddbae3d716cc6bb59542e))
    - [ref] improve structure; fix docs ([`aa6052a`](https://github.com/Byron/gitoxide/commit/aa6052a41e44a13ea31c9ec585663b0904cdd929))
    - [ref] overlay really seems to work ([`d2ec30a`](https://github.com/Byron/gitoxide/commit/d2ec30af1be4bc54d69ef7d794c1bf372c80463b))
    - [ref] more detailed overlay test ([`d747d73`](https://github.com/Byron/gitoxide/commit/d747d730afd4db6c0c20c3c63cc09824fbd6e223))
    - thanks clippy ([`636e1fd`](https://github.com/Byron/gitoxide/commit/636e1fd85ceb3a1dc3cf5d3c7224f6f36d8eb695))
    - [ref] fix windows build… ([`65e6953`](https://github.com/Byron/gitoxide/commit/65e6953d1a9e751cb4644056aabd7c6edfbf7978))
    - [ref] first successful test for overlay iterator ([`5f92488`](https://github.com/Byron/gitoxide/commit/5f924885f343d8a60737de74c651e8e5c11a8d48))
    - [ref] conversion for packed refs ([`929bb0f`](https://github.com/Byron/gitoxide/commit/929bb0f75715a547993e8ce9c885d7de1a030013))
    - [ref] loose refs iteration in overlay iterator ([`0b0f64d`](https://github.com/Byron/gitoxide/commit/0b0f64d16acb97d2282b982647362b164ac280ad))
    - [ref] leverage sorted file iteration ([`036257e`](https://github.com/Byron/gitoxide/commit/036257eee036c2d5edea2ac8b16aad6bae8ba7fd))
    - [ref] add setup for parallel file traversal tests ([`1306647`](https://github.com/Byron/gitoxide/commit/1306647447f712805b3d8c8ca38e90fb4f94ca67))
    - [ref] reproducible loose ref iteration with built-in sorting ([`e138748`](https://github.com/Byron/gitoxide/commit/e13874807ccc3cbc2b4aacccf63ac5c3dd21c445))
    - [ref] sketch remaining overlay types, now on to 'next()' ([`6792cf1`](https://github.com/Byron/gitoxide/commit/6792cf1362ed21948d9b5f8b252b1c08ca8ca7ca))
    - [ref] a way to obtain valid ref names along with their path for overlay iteration ([`bbaa1eb`](https://github.com/Byron/gitoxide/commit/bbaa1eb10b3d2fd0de6afde61e5b6378be2e110c))
    - [ref] first steps towards test and impl for overlay iterator ([`f5d07b6`](https://github.com/Byron/gitoxide/commit/f5d07b67af4fdf68f3109a8bc1481474cd5c3807))
    - [ref] add missing docs ([`e6052a5`](https://github.com/Byron/gitoxide/commit/e6052a5a36b27bbcf79c05cd517eab9ec7507d8d))
    - [ref] all remaining tests ([`ee9bc21`](https://github.com/Byron/gitoxide/commit/ee9bc211e857ed2bbf9eb5fc6e46f5e126b11ab2))
    - [ref] first successful test for prefix filtering in packed refs ([`430549d`](https://github.com/Byron/gitoxide/commit/430549da137c5469a0ee17eca8d52a6f3ed8b04b))
    - [ref] run all performance tests ([`3635b25`](https://github.com/Byron/gitoxide/commit/3635b25deee7ded4307458abcf83d0c1181030f4))
    - [ref] simple performance tests to get an idea of what it can do… ([`06bedcd`](https://github.com/Byron/gitoxide/commit/06bedcd7a79c64ece443a34cc21a9ca32ac38ca9))
    - [ref] perf 'test' for ref iteration ([`922d129`](https://github.com/Byron/gitoxide/commit/922d129ff3b741a3091cf899a8e1400e98417093))
    - thanks clippy ([`a39a68a`](https://github.com/Byron/gitoxide/commit/a39a68a3d51bf0185df86ca34f90b9755f31f2b5))
    - [ref] rename find_one to 'find' in git-ref… ([`ae7746a`](https://github.com/Byron/gitoxide/commit/ae7746a0815bb94659de67383ba372ac522d53b8))
    - [ref] refactor ([`758c090`](https://github.com/Byron/gitoxide/commit/758c0907df8dc6987f374e326304e0f9fad29812))
    - [ref] finish packed find() lookup testing ([`5f67c19`](https://github.com/Byron/gitoxide/commit/5f67c19a1f4f62419bfc7d6e52c56aa5be40b723))
    - [ref] refactor ([`953939c`](https://github.com/Byron/gitoxide/commit/953939c2ce7922efd6df4654dc329743d3052492))
    - [ref] prevent unnecessary rounds for full names that aren't found ([`fb765de`](https://github.com/Byron/gitoxide/commit/fb765de831aa704b04b6a23c6a1d4ff183d784e0))
    - [ref] Assure ref-misses misses aren't parse-errors ([`d9d1360`](https://github.com/Byron/gitoxide/commit/d9d13602c83d0725d23d3abb3d2d5bf30355e1d9))
    - [ref] basic lookup rule impl; needs more test cases ([`3226f77`](https://github.com/Byron/gitoxide/commit/3226f775129231b4bc4735baf9e14a187665ace3))
    - [ref] fix compile warning on windows ([`c328774`](https://github.com/Byron/gitoxide/commit/c32877415aba8df6d5a37cfd799b218e3a29b18a))
    - [ref] a test case specifically for lookup rules ([`ab3a34f`](https://github.com/Byron/gitoxide/commit/ab3a34f481ebe335578e3a7dbff325087b4ba647))
    - Remove unnecessary pub(crate) exports ([`3d2456e`](https://github.com/Byron/gitoxide/commit/3d2456e11709f0461b37c6df55ecc3861ca4cab5))
    - [ref] refactor ([`140da9a`](https://github.com/Byron/gitoxide/commit/140da9a0b77c423649d9fd291babef80532015a2))
    - [ref] improve parse failure handling in packed-ref lookup ([`ba62aab`](https://github.com/Byron/gitoxide/commit/ba62aab4308d44092d151d11d9be44ba6bfddb02))
    - [ref] refactor ([`959abc7`](https://github.com/Byron/gitoxide/commit/959abc70c754cf4cd812f6014c29fd2f6d1a7fc4))
    - [ref] prepare for proper full-name conversion ([`0e6d3f2`](https://github.com/Byron/gitoxide/commit/0e6d3f29a6abe54b04424697009bb8524faaca7e))
    - [ref] searching fully qualified reference names actually works. ([`9b2579c`](https://github.com/Byron/gitoxide/commit/9b2579c3713b3bd185895318868378b8831dbc96))
    - [ref] prepare find() impl… ([`b26dd1e`](https://github.com/Byron/gitoxide/commit/b26dd1ed253d8714cf4f9a77c0c29f67cc952c76))
    - [ref] assure packed-refs buffers are sorted ([`a797493`](https://github.com/Byron/gitoxide/commit/a797493c93aa2d1b6e46442f714c8d5b98032456))
    - [ref] refactor ([`897a49a`](https://github.com/Byron/gitoxide/commit/897a49a9973ccb225dbc9b75be624b7e4c9ec608))
    - [ref] windows fix; now maybe? ([`0e1a204`](https://github.com/Byron/gitoxide/commit/0e1a20424a25902e80ad8dd6b6a413cb00f77904))
    - [ref] windows pathname replacement: \ -> /… ([`94a1e02`](https://github.com/Byron/gitoxide/commit/94a1e02d3e03f29d56b83e92c176c8d245ff44fc))
    - [ref] fix one test failure on windows ([`21f1031`](https://github.com/Byron/gitoxide/commit/21f10319d4047401bb6b11dec975c9386788773b))
    - [ref] rough frame for finding packed refs ([`a24a54f`](https://github.com/Byron/gitoxide/commit/a24a54fb2b2620a0c86c2b9bc2a094412ed73fb8))
    - [ref] learn more about the windows issue… ([`dde6276`](https://github.com/Byron/gitoxide/commit/dde6276a52b0f067bfeb8bb355a05696df6f134f))
    - [ref] refactor ([`c150aba`](https://github.com/Byron/gitoxide/commit/c150abaa86ebcbd10ccee4359b45b4a0b802b68e))
    - [ref] prefixed loose ref iteration ([`49ce1e2`](https://github.com/Byron/gitoxide/commit/49ce1e2184841ecd9c54573ba026341f4fecc0b5))
    - [ref] refactor; tests for prefix iteration ([`63566eb`](https://github.com/Byron/gitoxide/commit/63566eb81cdd14a98f25491fbb7f363a2fb6a0c7))
    - [ref] loose ref iteration with broken ref support ([`2d1234f`](https://github.com/Byron/gitoxide/commit/2d1234f9f8ae55c13af18ef5978e4ef9634e1606))
    - [ref] maybe fix windows ([`6fc7784`](https://github.com/Byron/gitoxide/commit/6fc778455c374fa289d15e64d1d67ad9310e0d0a))
    - [ref] first rough implementation of loose ref iteration ([`918af42`](https://github.com/Byron/gitoxide/commit/918af425298a1fdbb8e7dd6328daefe9eaa10cef))
    - [ref] packed-refs iteration… ([`ea97e06`](https://github.com/Byron/gitoxide/commit/ea97e063bfa5cbafac521dbd7f8becd357083356))
    - [ref] docs for packed refs iterator ([`02690bc`](https://github.com/Byron/gitoxide/commit/02690bc96903071108ffc54594bd4c31ebd054d1))
    - [ref] fix 'small' build ([`5fd10fe`](https://github.com/Byron/gitoxide/commit/5fd10fe1e901a0c8d9627f76c4a040922847cd15))
    - [ref] packed-refs iteration works, incl. decent error handling ([`e5a6b9d`](https://github.com/Byron/gitoxide/commit/e5a6b9d2f637ee746ccaf67354f64c3999cf971a))
    - [ref] the first packed-refs iterator tests ([`f6d769e`](https://github.com/Byron/gitoxide/commit/f6d769ec5948fefe363ffa436e326e5fae820a66))
    - [ref] refactor ([`207a799`](https://github.com/Byron/gitoxide/commit/207a799c1fcf490425f2e5dcf8274da83125af6f))
    - [ref] flexible and simple support for different hash lengths ([`9c2edd5`](https://github.com/Byron/gitoxide/commit/9c2edd537fb86d2d7db874ec976d0cb1b8ec7c2e))
    - Revert "[ref] parameterize all uses of hash length…" ([`21f187e`](https://github.com/Byron/gitoxide/commit/21f187e6b7011bb59ed935fc1a2d0a5557890ffe))
    - [ref] sketch of iterator ([`6c05243`](https://github.com/Byron/gitoxide/commit/6c05243b53a74c770fc41e50a7df55f01ba21b3d))
    - [ref] refactor ([`79184cf`](https://github.com/Byron/gitoxide/commit/79184cfe1035ad8665972c796c27448dc1fe3430))
    - [ref] parameterize all uses of hash length… ([`5c7285e`](https://github.com/Byron/gitoxide/commit/5c7285e7233390fd7589188084fcd05febcbbac2))
    - [ref] less lenient packed-ref header parsing ([`45b41e0`](https://github.com/Byron/gitoxide/commit/45b41e0f522ac491e49be5e36a1744c9d07a4286))
    - thanks clippy ([`33f1b00`](https://github.com/Byron/gitoxide/commit/33f1b00e134222641a71521561db4671a4285462))
    - [ref] refactor ([`de526b3`](https://github.com/Byron/gitoxide/commit/de526b31dbd84ddf05cbc5d447862fa0559a7561))
    - [ref] first working packed ref line parsing ([`bc60229`](https://github.com/Byron/gitoxide/commit/bc60229403ae075b66bb457a80695e2ab959448c))
    - [ref] first test for line (and peeled ref) parsin ([`7af27c5`](https://github.com/Byron/gitoxide/commit/7af27c5676c986b05953995d216b78389e986ee0))
    - [ref] refactor ([`b74913e`](https://github.com/Byron/gitoxide/commit/b74913ef90c6d827dff50ca5df13c826be4fc86d))
    - [ref] refactor ([`d0eb819`](https://github.com/Byron/gitoxide/commit/d0eb8196e3faed6c013f2e746ba50bba1330d87e))
    - [ref] packed refs header line parsing ([`fde5543`](https://github.com/Byron/gitoxide/commit/fde5543ad22395e27266db02a5442a33d16e68c5))
    - [ref] first rough steps to testing parsing a little ([`57659e9`](https://github.com/Byron/gitoxide/commit/57659e92de9a525a72dc3cba50b844bef7e021a1))
    - [ref] sketch packed refs, but… ([`8951b3f`](https://github.com/Byron/gitoxide/commit/8951b3fd96735adc2eed5b0035bc0a97759e2207))
    - [ref] refactor + docs review ([`4b9b034`](https://github.com/Byron/gitoxide/commit/4b9b034e3600cc3dc6dc35a257231914802a60fb))
    - [ref] the last TODO is gone ([`01dc422`](https://github.com/Byron/gitoxide/commit/01dc422cef924f26943dbc5b41b45098853d4868))
    - [ref] down to the last todo ([`23cea99`](https://github.com/Byron/gitoxide/commit/23cea99f645dfc27a89296f7bbd30c1b22015dba))
    - [ref] two more tests but only one todo down ([`bf947d6`](https://github.com/Byron/gitoxide/commit/bf947d65b508511d90299e93f285989c1a3eafd1))
    - [ref] the drop test ([`e472bde`](https://github.com/Byron/gitoxide/commit/e472bde7bf24eaeefa93a3dbc269cea41f6ddcc8))
    - [ref] refactor ([`059f836`](https://github.com/Byron/gitoxide/commit/059f836f490261cf5257349e0a7bfb69d9b68d89))
    - [ref] refactor ([`7faf6f2`](https://github.com/Byron/gitoxide/commit/7faf6f24f90854bd885e59c517b73db8ba5082af))
    - [ref] adjust expectation to not do any special HEAD business ([`49d294a`](https://github.com/Byron/gitoxide/commit/49d294a292709882179cf3b7934ec1885c60ccaa))
    - Revert "[ref] FAIL: realize that HEAD-reverse-lookup isn't done in transactions…" ([`8b0d7b6`](https://github.com/Byron/gitoxide/commit/8b0d7b62ff2ee96692d3014299fad67e0c82f3a1))
    - [ref] FAIL: realize that HEAD-reverse-lookup isn't done in transactions… ([`6098ba0`](https://github.com/Byron/gitoxide/commit/6098ba0f4288b379f84f48bb2d3245309a70ce7c))
    - [ref] test to validate HEAD update as special case of… ([`276aa9a`](https://github.com/Byron/gitoxide/commit/276aa9a89b41df43ad47f2096b4d89bdf697acea))
    - [ref] refactor ([`861483a`](https://github.com/Byron/gitoxide/commit/861483a4e7b7d61447d6bbfa91937ddfdf69ba02))
    - [ref] validate non-empty directories ([`8fb625d`](https://github.com/Byron/gitoxide/commit/8fb625d577fad376b28f5f568b8455aa901c2f0a))
    - [ref] moving a ref onto empty directories works now… ([`a237f77`](https://github.com/Byron/gitoxide/commit/a237f77ee0eb395bf89f7ed1b7496bf33c2d30af))
    - [ref] refactor ([`ed40a87`](https://github.com/Byron/gitoxide/commit/ed40a87e14d38b7f8b9a3a605b70a0fb1dc92220))
    - [ref] another complex test works ([`ebdbfae`](https://github.com/Byron/gitoxide/commit/ebdbfae9e26aa11f7afda7f60f0fbf6757dabb76))
    - [ref] fix build ([`b4dcdfc`](https://github.com/Byron/gitoxide/commit/b4dcdfc9b2f2edcbcf9fb144d1f97e9a841463ad))
    - [ref] try fix windows, once again ([`95e74dd`](https://github.com/Byron/gitoxide/commit/95e74dd9f1510fd288f281beea3f560319ad323d))
    - [ref] refactor ([`a261b82`](https://github.com/Byron/gitoxide/commit/a261b82c1ee7ebdbbc82ce1c8286ca6a0f221cea))
    - [ref] probably fix windows ([`a8b7c8d`](https://github.com/Byron/gitoxide/commit/a8b7c8d2fef9438a23a96c35497d34e816af96c7))
    - [ref] allow reflogs to be created in place of empty directory trees ([`80a6e0e`](https://github.com/Byron/gitoxide/commit/80a6e0e1ff2321d9162e799d5fc0f457e7de4ade))
    - [tempfile] a way to delete empty dirs recursively ([`6025aa0`](https://github.com/Byron/gitoxide/commit/6025aa08d93cd5124c8df38c51b795b9c7d1c911))
    - [ref] refactor ([`21920ec`](https://github.com/Byron/gitoxide/commit/21920ec173da4642ad335fcd5fbc3b85c940061e))
    - [ref] refactor directory handling ([`45dbf22`](https://github.com/Byron/gitoxide/commit/45dbf2253d13ee8eba7654ef294614c3b9651a9d))
    - [ref] refactor ([`92867c5`](https://github.com/Byron/gitoxide/commit/92867c58467e66d1b6b13d2ca4375d268fbafde5))
    - [ref] handle existng empty directories more gracefully… ([`0849c70`](https://github.com/Byron/gitoxide/commit/0849c70596ed7674e7e18cd444b6cd99d37da4ff))
    - thanks clippy ([`d967e30`](https://github.com/Byron/gitoxide/commit/d967e30f1652f29c3c13ea0014d8d3910a4f7245))
    - [ref] handle create-or-append when writing valid reflog files… ([`9175085`](https://github.com/Byron/gitoxide/commit/9175085248855a7ffa0d4e006740eafc0f4e1c92))
    - [ref] refactor ([`1ee3419`](https://github.com/Byron/gitoxide/commit/1ee341922d4a8343bc5146378da4353a99b28a73))
    - [ref] auto-creation logic for reflogs ([`80f71dc`](https://github.com/Byron/gitoxide/commit/80f71dc85836b640b264f146d37fc74a0bd99fd9))
    - [ref] reflog creation test is quite complete ([`b67e79c`](https://github.com/Byron/gitoxide/commit/b67e79c861f644756e9bd12cc3a28bd6355250d3))
    - [ref] allow commiter to be passed for use in reflog ([`80f5627`](https://github.com/Byron/gitoxide/commit/80f5627d6fe5aef8d0a82cdad1746d5d2509f2c3))
    - [ref] tests for converting reflock paths into log paths ([`1f2e754`](https://github.com/Byron/gitoxide/commit/1f2e75439d2ff5b7db40a979fde289e68c578d81))
    - [ref] refactor ([`a29fcf1`](https://github.com/Byron/gitoxide/commit/a29fcf1d61ec9f387a401a1a4a903256b6413536))
    - [ref] frame for reflog creation or update ([`81cb790`](https://github.com/Byron/gitoxide/commit/81cb79017ca5a2f18531bc6caedc28de94a0a064))
    - [ref] refactor ([`a76929b`](https://github.com/Byron/gitoxide/commit/a76929b45b4f82488b1e713d1012e1d431257fcd))
    - [ref] disambiguate create-or-update logic ([`585f369`](https://github.com/Byron/gitoxide/commit/585f369ea7bb7ee3d8f5103583628e3d68ef3de5))
    - [ref] write out Create-or-Update logic to see that's its probably not going to cut it. ([`54d084f`](https://github.com/Byron/gitoxide/commit/54d084ffe0d684ab4879973293f2efad4966c632))
    - [ref] show how the original name can be displayed for lock failures… ([`07f0c2d`](https://github.com/Byron/gitoxide/commit/07f0c2dc9b3949566b3c3d0a15302c416ae9ccb7))
    - [ref] write peeled previous OID through to parent refs ([`3355dd8`](https://github.com/Byron/gitoxide/commit/3355dd8295886b0dbeeaa802cbf32ea6e3264de6))
    - [ref] fix child link transformation ([`5d9a685`](https://github.com/Byron/gitoxide/commit/5d9a685fedd4d5614dd338d4b9baa37f11649cb0))
    - [ref] refactor ([`2f92f36`](https://github.com/Byron/gitoxide/commit/2f92f360e581a1a7b7bad389c915545cd6a5b31a))
    - [ref] sketch of inverting parent links for later oid lookup ([`a050f18`](https://github.com/Byron/gitoxide/commit/a050f1856f69b710f6e63898d11fa52cafd254c7))
    - [ref] refactor ([`1e88948`](https://github.com/Byron/gitoxide/commit/1e88948455111c01f2a8f9d24a4fcf835553e55b))
    - [ref] add reflog message to change… ([`b31e103`](https://github.com/Byron/gitoxide/commit/b31e103f2492b0507e2e1eab3a26ddc025dd470f))
    - [ref] sketch more detailed test for updating reflogs ([`5a657cd`](https://github.com/Byron/gitoxide/commit/5a657cdd0a342aa8b5a57398718bf27ef136997a))
    - thanks clippy ([`eb8ea22`](https://github.com/Byron/gitoxide/commit/eb8ea22a97f132169e81d71ca2ca64ef52463fe3))
    - [ref] the last deletion test ([`258a494`](https://github.com/Byron/gitoxide/commit/258a494562d8266561540e07c01d1e87466470d9))
    - [ref] refactor ([`db76cfd`](https://github.com/Byron/gitoxide/commit/db76cfd5585a5fa54739ce003837a8750dea9f99))
    - [ref] deletion won't have problems with broken refs ([`286b5c1`](https://github.com/Byron/gitoxide/commit/286b5c1a5529c58c35b8ff0504f9e784f7be10e1))
    - thanks clippy ([`e5da69e`](https://github.com/Byron/gitoxide/commit/e5da69e642c16ddaf39b59e6e0de6b3c4153acff))
    - [ref] add failing deletion test for broken refs ([`578413f`](https://github.com/Byron/gitoxide/commit/578413f5848cb8ab3b14fe149be3db12705182c3))
    - [ref] another del test ([`d935d6f`](https://github.com/Byron/gitoxide/commit/d935d6f67fff1d7b02f6b0805a3e6efb9f429fc1))
    - [ref] another deletion test ([`8b756e0`](https://github.com/Byron/gitoxide/commit/8b756e094bd4ecf47415d8eb8c7adf44b8a89039))
    - [ref] another deletion test ([`69ede1b`](https://github.com/Byron/gitoxide/commit/69ede1b90e6573df86829437f3c3adf3924b31cf))
    - [ref] refactor ([`d05a646`](https://github.com/Byron/gitoxide/commit/d05a6467c185d0f4dcb030e4bf751070a9b3d5bf))
    - [ref] Make sure edit preprocessing happens in the right order ([`2d5f9aa`](https://github.com/Byron/gitoxide/commit/2d5f9aaa68b065f84df3a2db3707cf9cf10b0321))
    - [ref] refactor ([`dd9c99b`](https://github.com/Byron/gitoxide/commit/dd9c99b9d1c0c6222f5a12f280c8ed0eb0c3daf2))
    - [ref] refactor ([`97fc864`](https://github.com/Byron/gitoxide/commit/97fc864fb4dd2903eb9f7dd671422dfbeaa304f3))
    - thanks clippy ([`f436f18`](https://github.com/Byron/gitoxide/commit/f436f18be3b4aafe40cb0e36432d22666795ecc6))
    - [ref] splitting handles reference cycles ([`09b4fc1`](https://github.com/Byron/gitoxide/commit/09b4fc1e6f01a9124f6563fa614b42356560e4b4))
    - [ref] splitting actually works! ([`a9f824b`](https://github.com/Byron/gitoxide/commit/a9f824bc95f157146f22b468d4a9d8dddc9f31a5))
    - [ref] first stab at splitting refs, needs more elaboration to fulfil expectations ([`66b1f37`](https://github.com/Byron/gitoxide/commit/66b1f3725cd710d991625bcd2c1994545b33aa53))
    - [ref] refactor ([`eb0328f`](https://github.com/Byron/gitoxide/commit/eb0328fb67ad677d8875bef5deb7efea2c55ae67))
    - [ref] first part of ref splitting is tested ([`ce7f83b`](https://github.com/Byron/gitoxide/commit/ce7f83b7e58762866e141d1b71e1ea68153fd075))
    - [ref] refactor; prep slitting tests ([`7ffc619`](https://github.com/Byron/gitoxide/commit/7ffc619a7c06f0d47572fac9f91444c3663ac316))
    - [ref] refactor ([`683651d`](https://github.com/Byron/gitoxide/commit/683651d2a7cc9b589b4490a1767677f3d7fb5e3e))
    - [ref] first sketch of generalized splitting of edits ([`1f2efdc`](https://github.com/Byron/gitoxide/commit/1f2efdcf9151f161a325680737f1992edf46228c))
    - [ref] working on splits really shows that we want more than one enum maybe… ([`1b62838`](https://github.com/Byron/gitoxide/commit/1b62838d00ec35cb45d43e5e9e5ce6573f1db2a7))
    - [ref] need ref splitting for the first time. ([`f52989f`](https://github.com/Byron/gitoxide/commit/f52989f325d50db66c0ffe75a964feaba075dc19))
    - [ref] better deletion tests; more useful return value ([`96848f6`](https://github.com/Byron/gitoxide/commit/96848f68a70a6721c9fc4c7d36763a3015527728))
    - thanks clippy ([`ef9bfd2`](https://github.com/Byron/gitoxide/commit/ef9bfd2806b0407ccbc7391e086592f4bf7a7424))
    - [ref] another deletion test succeeds ([`6037900`](https://github.com/Byron/gitoxide/commit/60379001d2729627c042f304217d6459f99f01bf))
    - [ref] refactor, not quite sure about delete mode… ([`683991a`](https://github.com/Byron/gitoxide/commit/683991a4edbc53c583603af94fbec625a211b52d))
    - [ref] another test; failing for now ([`1908b69`](https://github.com/Byron/gitoxide/commit/1908b693b75e8cb204dc5026ea2f311b88bddfc4))
    - [ref] another test green ([`104598e`](https://github.com/Byron/gitoxide/commit/104598eb71e830a5feed763dea1dc1fd03be6eff))
    - [ref] first succeeding deletion test ([`3445d7d`](https://github.com/Byron/gitoxide/commit/3445d7dfcade73bec8ba68d58d034608169e7758))
    - [ref] refactor ([`d2e2e8f`](https://github.com/Byron/gitoxide/commit/d2e2e8f49b3668235cf808b08f85bd89a592105f))
    - [ref] first deletion tests ([`e41f8c8`](https://github.com/Byron/gitoxide/commit/e41f8c8a48328fb0fe154e5212f1b1e41195d3c1))
    - [ref] write more details on how prepare and commit should work overall. ([`a7d988b`](https://github.com/Byron/gitoxide/commit/a7d988b8feb2aba87a19f3484470d8f77786ffd4))
    - [ref] refactor; get closer to what git does… ([`488f311`](https://github.com/Byron/gitoxide/commit/488f31160300bccaba6a510869c7c3e53d52d27b))
    - [ref] refactor ([`58a5653`](https://github.com/Byron/gitoxide/commit/58a5653a6647931bf90f88ff2d83c6b0322ad9b1))
    - [ref] first very basic ref writing ([`7ebed3f`](https://github.com/Byron/gitoxide/commit/7ebed3ff14e6944ba18be0c9876b10c42c2d840c))
    - [ref] remove complexity in the name of performance, fix windows… ([`77c3f24`](https://github.com/Byron/gitoxide/commit/77c3f24a935800d7643dc61466385a76a58bf365))
    - [ref] (probably) fix windows ([`7c1eead`](https://github.com/Byron/gitoxide/commit/7c1eead4b589975fb1dcfe63fb2071bb6d8ab611))
    - thanks clippy ([`6865549`](https://github.com/Byron/gitoxide/commit/6865549cf6df08999618bfa6cd658d44b8aba9c7))
    - [ref] slowly getting there ([`6506924`](https://github.com/Byron/gitoxide/commit/650692443459b253a56fb5bda78bd3a4a0de07f9))
    - [ref] a way to determine if a reflog exists. ([`e6fbba8`](https://github.com/Byron/gitoxide/commit/e6fbba87942b9138261ee70d8fa8408422149521))
    - [ref] reference::log_iter_rev() ([`1f7af5d`](https://github.com/Byron/gitoxide/commit/1f7af5dcf093a9169ce353c0b1d354ed7acda4a5))
    - [ref] reference.log_iter() works, but… ([`c298473`](https://github.com/Byron/gitoxide/commit/c298473f0f353f9f59d39ab530c133e13cfb47ec))
    - [ref] [FAIL] try to forward iterator creation to reference… ([`ef1737c`](https://github.com/Byron/gitoxide/commit/ef1737c7e67038c0541a619e77c0ea5451bcca28))
    - [ref] refactor ([`129bccf`](https://github.com/Byron/gitoxide/commit/129bccf8dfaaab1c487c49fe35a2877ff900d06e))
    - [ref] refactor ([`96dd98b`](https://github.com/Byron/gitoxide/commit/96dd98b800b9e808853fc954ac78b8778bf18f23))
    - [ref] refactor ([`a7dd994`](https://github.com/Byron/gitoxide/commit/a7dd9940a0a6e1f8685f5bb785d8c05023027393))
    - [ref] refactor ([`3460127`](https://github.com/Byron/gitoxide/commit/34601272230c37aad803409e89dc6b270de1f02d))
    - [ref] store ref log reverse iterator ([`34d7957`](https://github.com/Byron/gitoxide/commit/34d795700e89a264dcf3a40a6dec63cdc5998814))
    - [ref] store can provide reflog forward iter… ([`9adb9ca`](https://github.com/Byron/gitoxide/commit/9adb9ca2b2b63f9fc4b57e45732389077778c324))
    - [ref] more assertions ([`8000677`](https://github.com/Byron/gitoxide/commit/80006772e0ef9d9f9fc4d274f460194712138327))
    - [ref] a fully implemented first test with assertions ([`29a5893`](https://github.com/Byron/gitoxide/commit/29a58937a3e8d4fae861952d6bc34565da8c3e8c))
    - [ref] sketch more tests that will be needed ([`01690be`](https://github.com/Byron/gitoxide/commit/01690be8acf6a5f18b55db941f05644650f062f0))
    - [ref] add control over handling lock failures during transaction ([`7c4057a`](https://github.com/Byron/gitoxide/commit/7c4057aa4bd5e65195c80d0319798615b9571c0d))
    - [ref] generic operation on input edits, split-suitable now ([`7f4f637`](https://github.com/Byron/gitoxide/commit/7f4f63763249a614936be3baa702b93558a4d494))
    - [ref] try using borrow on a slice intead of iterator… ([`b2371d9`](https://github.com/Byron/gitoxide/commit/b2371d93408613ab0e07048398bd95e60da603e1))
    - [ref] duplicate ref edit checks… ([`3ec0182`](https://github.com/Byron/gitoxide/commit/3ec0182376fad623814408703f1d47736eea6349))
    - [ref] a more fleshed out API for file transactions ([`918123f`](https://github.com/Byron/gitoxide/commit/918123f7f951d7f773dd8b38a184de2f2c3e25b9))
    - [ref] on the way towards realistic transactions… ([`c808cb1`](https://github.com/Byron/gitoxide/commit/c808cb17b2fea12e018fabb789862e9b7703e49b))
    - [ref] on the way to setup the first transaction test ([`29c0b51`](https://github.com/Byron/gitoxide/commit/29c0b51625e2c7e3a8d60075bb925126a024dc83))
    - [ref] file store can ignore all writes; sketch transaction API ([`52a81e9`](https://github.com/Byron/gitoxide/commit/52a81e98f38657023d3eb384fd6db69917dd57ca))
    - [ref] refactor ([`6a84790`](https://github.com/Byron/gitoxide/commit/6a84790b13e445d5a1b85fd3cae2ec0feed4ff02))
    - [ref] log line writing ([`3da8fcf`](https://github.com/Byron/gitoxide/commit/3da8fcf0bfb77b80c06a3358416f10d6f393db8b))
    - [ref] Line::from_bytes(…); iter uses that now ([`7895995`](https://github.com/Byron/gitoxide/commit/7895995cf91fbaeb798c4277699e02107cb63909))
    - [ref] test for small buffer sizes ([`6183772`](https://github.com/Byron/gitoxide/commit/61837723f7c1f3150d7f853c055248116bba9633))
    - [ref] handle multiple buffer reloads ([`4559c7a`](https://github.com/Byron/gitoxide/commit/4559c7a184b9cdbd174785b84b41a218c683c94f))
    - [ref] refactor ([`65e333d`](https://github.com/Byron/gitoxide/commit/65e333de6194b48b558d02b503502bd7ab267945))
    - [ref] refactor ([`2b416ee`](https://github.com/Byron/gitoxide/commit/2b416ee7e788faadf280553464fd77f2c91e2d0a))
    - [ref] refactor ([`82b18e5`](https://github.com/Byron/gitoxide/commit/82b18e50f3c31fac10dc5a752ab9b0c134607e37))
    - [ref] multi-line reverse iteration works, without window shift for now ([`f1e3861`](https://github.com/Byron/gitoxide/commit/f1e38618371408d844144a736c3082d57b2d1015))
    - [ref] first reverse iter test succeeding ([`8875601`](https://github.com/Byron/gitoxide/commit/88756015d8fc77ddb3b12fcdd1df85a709f8189a))
    - [ref] let's not forget to simply not try to return borrowed things from iterators ([`bcc934d`](https://github.com/Byron/gitoxide/commit/bcc934dea0aa71502945a20d5987dec4eeb34aea))
    - [ref] FAIL: try it with included buffer ([`189080e`](https://github.com/Byron/gitoxide/commit/189080e8bc2d999ee4f1a76ed9b537cfda7ad82c))
    - [ref] FAIL another attempt this time without iterator… ([`5e73dc2`](https://github.com/Byron/gitoxide/commit/5e73dc2fa1a77b5bcf2319ed244004ac3ec86506))
    - [ref] FAIL at attempt to to have self-referential iterators :D… ([`bc4012e`](https://github.com/Byron/gitoxide/commit/bc4012eb8a1b0c27dd2b54d169c2058478449b0a))
    - [ref] first test for reverse iterator and more boilerplate ([`40db355`](https://github.com/Byron/gitoxide/commit/40db35547b855066b3584d8e81f62c8978ac5840))
    - [ref] refactor ([`4daddb1`](https://github.com/Byron/gitoxide/commit/4daddb13a7f7139b8e0e7c6817854dad00429dbc))
    - [ref] sketch of reverse iterator ([`c581d16`](https://github.com/Byron/gitoxide/commit/c581d169c2e21e568bce3d7bc8469836aa9d1e2c))
    - [ref] thanks clippy ([`4ba3b08`](https://github.com/Byron/gitoxide/commit/4ba3b08e69002ae20545e9d27c3130a672fa9ae6))
    - [ref] significantly simplify error messages… ([`b15cb16`](https://github.com/Byron/gitoxide/commit/b15cb16f022045207a9419266d3fe972fbd663e1))
    - [ref] don't include terminators to get slightly nicer error messges ([`09bbc6d`](https://github.com/Byron/gitoxide/commit/09bbc6d0b32b835d1a4ba2dca7e24522b94cee22))
    - [ref] another test for iter::forward() ([`1d84302`](https://github.com/Byron/gitoxide/commit/1d843029dbaa7d06f9338fa6eb90f583a4225094))
    - [ref] a forward iterator with a single test ([`917040c`](https://github.com/Byron/gitoxide/commit/917040cb58d9dda18835c255bff3a9d692cfe1de))
    - [ref] log line docs ([`10ab8e0`](https://github.com/Byron/gitoxide/commit/10ab8e0e4bcccc4e79203f06e16835b8e5d9504b))
    - [ref] refactor ([`cd89e21`](https://github.com/Byron/gitoxide/commit/cd89e21280463deb1fd22ef20d2c54926bbb9b6c))
    - [ref] more context for line parsing ([`ddb5f9d`](https://github.com/Byron/gitoxide/commit/ddb5f9d256cf0be36943e11a9df18b938551be87))
    - [ref] refactor ([`a08fb77`](https://github.com/Byron/gitoxide/commit/a08fb776a523040445006c81a890ef11f496f650))
    - [ref] be truly zero copy and delay work to when it's first asked for ([`b4e594b`](https://github.com/Byron/gitoxide/commit/b4e594bdeb06329beacd61b03ab90057284bcb54))
    - [actor] FAIL an attempt to remove btoi errors ([`3f99cf5`](https://github.com/Byron/gitoxide/commit/3f99cf531caacb93a3ce81b16d61be18e5d8a017))
    - [actor] pure nom error handling… ([`78cbe18`](https://github.com/Byron/gitoxide/commit/78cbe18888ec654f3410fc655d9beaaf63f68003))
    - [ref] refactor ([`8694488`](https://github.com/Byron/gitoxide/commit/869448833d9de5c0859e6fab267b48d19f1a9119))
    - [ref] getting there! ([`bd73d8e`](https://github.com/Byron/gitoxide/commit/bd73d8ee04f7baa9aeb05857484da6cb63175ebb))
    - [ref] a step forward to nom error handling, but… ([`426ae5b`](https://github.com/Byron/gitoxide/commit/426ae5b7db6cb943fdf6ee48e2be531157341e49))
    - [ref] try really hard to use generic verbose nom errors but… ([`1031625`](https://github.com/Byron/gitoxide/commit/10316252fa5dc02effe5596165268f8d806c55f8))
    - [ref] tests and impl for happy cases ([`7be82f0`](https://github.com/Byron/gitoxide/commit/7be82f09ce3c2421ba922e3f8bc1238ca5d494ab))
    - [ref] the first test for log line parsing; make serde1 work ([`cba3cdc`](https://github.com/Byron/gitoxide/commit/cba3cdc75280b247e59af878d1afe286638b95b7))
    - [refs] try to get structure in place for reflog parsing ([`727c66a`](https://github.com/Byron/gitoxide/commit/727c66a2560c00cc8e01fbe47503ffbb67147c59))
    - [refs] sketch more of transactions so it has all it needs ([`8f9a015`](https://github.com/Byron/gitoxide/commit/8f9a0157e876fadfe16a2cc58445543d1c10a21b))
    - [refs] allow writing any valid ref value instead of limiting ourselves to object ids ([`114fce8`](https://github.com/Byron/gitoxide/commit/114fce8368fe858bc64696b4d7253c425367560a))
    - [refs] finish transaction sketch (or so it seems) ([`976a079`](https://github.com/Byron/gitoxide/commit/976a0799a7862de7b85d45cb080102f41fc33d07))
    - [refs] this gets more and more interesting ([`e056495`](https://github.com/Byron/gitoxide/commit/e05649577a6cd5e2958884b10f7f75d48aa91a94))
    - [refs] finish research for transactions and their flags ([`2eb3bcc`](https://github.com/Byron/gitoxide/commit/2eb3bccadf338c07493e40cb8c5f357eb2502a5f))
    - [refs] sketch some parts of a transaction based on git source ([`d9a5d32`](https://github.com/Byron/gitoxide/commit/d9a5d328f575dfd86e414091688a545f931059e3))
    - (cargo-release) version 0.3.0 ([`87db688`](https://github.com/Byron/gitoxide/commit/87db688f23475d7232731429d770848aea228492))
    - (cargo-release) version 0.3.0 ([`6b33678`](https://github.com/Byron/gitoxide/commit/6b33678f83e6d261ca15c4a7634ff5b4e66d81dd))
    - (cargo-release) version 0.2.0 ([`3286e42`](https://github.com/Byron/gitoxide/commit/3286e42547b59df6365087cbae9ce1c9c959faad))
    - [git-refs] a way to build a big packed-refs file ([`5113529`](https://github.com/Byron/gitoxide/commit/51135291b60d38bdf50d24569596c421bcb4f0b9))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com/Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - [git-repository] traversal program uses new facilities, and it's cumbersome ([`29ea2de`](https://github.com/Byron/gitoxide/commit/29ea2de9ad48036f78d3776d8526d959f68bf287))
    - [git-repository] most of the git repository discovery ([`72a49c8`](https://github.com/Byron/gitoxide/commit/72a49c816253520230a04290619f469df608be19))
    - [git-ref] refactor ([`0c795c5`](https://github.com/Byron/gitoxide/commit/0c795c50834bcf52324ede46ec11eea26acb1107))
    - [git-ref] fix docs ([`4fbc476`](https://github.com/Byron/gitoxide/commit/4fbc476b2361afef25cff208ecfa66ac2ccb077a))
    - [git-ref] docs complete ([`93a1f4e`](https://github.com/Byron/gitoxide/commit/93a1f4e3fe48082abf5b0baa17a976808789ec20))
    - [git-ref] nicer semantics for peel_in_place_to_id() ([`d3250a7`](https://github.com/Byron/gitoxide/commit/d3250a7b5d0e16f8f1b38d10334282fe60f9d5ce))
    - Revert "[git-ref] refactor (Option<Result… -> Result<Option…" ([`d4046e9`](https://github.com/Byron/gitoxide/commit/d4046e94eb22d9e9b65ffa9861400c4fde4d0bd7))
    - [git-ref] refactor (Option<Result… -> Result<Option… ([`774e86c`](https://github.com/Byron/gitoxide/commit/774e86ce78159f7e07ec552c1847658b6f9ac288))
    - [git-ref] refactor ([`928b637`](https://github.com/Byron/gitoxide/commit/928b63789237b808b296c60c989b853b78d39f0e))
    - [git-ref] more docs ([`f962c74`](https://github.com/Byron/gitoxide/commit/f962c74215965f14e8f136ab0a4eddfbba97e8c2))
    - [git-ref] refactor ([`415f15a`](https://github.com/Byron/gitoxide/commit/415f15aa5751ee1a58d9e6723a9da9f3407a4d66))
    - [git-ref] a bunch of docs ([`7cfc5ab`](https://github.com/Byron/gitoxide/commit/7cfc5ab3c3b969e968b894161f73f3c69fe8e4c9))
    - thanks clippy ([`93915fa`](https://github.com/Byron/gitoxide/commit/93915fa6f1c00260e4f263ac4837c2ae7916b764))
    - [git-ref] peel to id done ([`f74771c`](https://github.com/Byron/gitoxide/commit/f74771c8caccb090066b5209721b8973c047f00c))
    - [git-ref] first working peel-to-id() ([`3574f87`](https://github.com/Byron/gitoxide/commit/3574f8717700ae3b33e167be2442c69f604f287c))
    - [git-ref] frame for peel_to_id ([`3710b6c`](https://github.com/Byron/gitoxide/commit/3710b6cfe5cf2e5e6f9199255ebb4ca68a195be5))
    - [git-ref] peeling without an iterator, fine ([`b118946`](https://github.com/Byron/gitoxide/commit/b118946ef68425ffa0a606d67df7b5d3b2d851df))
    - [git-ref] first stab at reference iteration… ([`806d10e`](https://github.com/Byron/gitoxide/commit/806d10ef735caf3575b84de0cca5b55374140571))
    - [git-ref] refactor ([`c363269`](https://github.com/Byron/gitoxide/commit/c363269e118a2dc53ce29ba245c079cecf061b7e))
    - [git-ref] find_one_existing(…) for convenience ([`7a443ff`](https://github.com/Byron/gitoxide/commit/7a443ffc148ae8161ba93351ffd16631f79e095c))
    - [git-ref] some find failure cases ([`d855051`](https://github.com/Byron/gitoxide/commit/d85505195541f3123527a337c9935e25bfc40ec4))
    - [git-ref] handle all find_one cases as per docs ([`3c0acc6`](https://github.com/Byron/gitoxide/commit/3c0acc6545ede1a3fef25ace2b7dbf79debdc754))
    - [git-ref] more ways of finding reference ([`b3c4e92`](https://github.com/Byron/gitoxide/commit/b3c4e928c6fb01e029f509e8b24516cd6c24e48f))
    - [git-ref] the first green find_one test ([`30177e8`](https://github.com/Byron/gitoxide/commit/30177e81451bd4fb51dd3297502fa3c63f67286e))
    - thanks clippy ([`8f0e9ed`](https://github.com/Byron/gitoxide/commit/8f0e9ed9220a874e8437ede6e129d345e9c8f737))
    - [git-ref] first basic impl shows validation needs a little adjustment ([`8b901c7`](https://github.com/Byron/gitoxide/commit/8b901c750f97a950cb162c9195770aee451d2e7e))
    - [git-ref] a sketch of find_one - easiest for the caller for sure ([`ec96256`](https://github.com/Byron/gitoxide/commit/ec96256c4be9ff6de15bb698f2d3b9559619a042))
    - [git-ref] refactor ([`5bac585`](https://github.com/Byron/gitoxide/commit/5bac5851367d77ead43feceefdb2bfaf24a1561e))
    - [git-ref] frame for loose store reference lookup ([`30b0d54`](https://github.com/Byron/gitoxide/commit/30b0d54ed04916a858af3101345c677dbf48594d))
    - (cargo-release) version 0.2.0 ([`1327894`](https://github.com/Byron/gitoxide/commit/132789475400abe660b30ef6d2c5ff57821dd2c4))
    - [git-ref] use git-validate crate ([`6b4f937`](https://github.com/Byron/gitoxide/commit/6b4f937f13ad62bc2c7e5b0fc14416feb9c313ba))
    - [git-ref] Setup more tests to realize we really want validate::tag ([`54ee5b5`](https://github.com/Byron/gitoxide/commit/54ee5b5eace8c35bc33ef1261778ba0fcee2ef37))
    - [git-ref] frame for validation ([`9656ac6`](https://github.com/Byron/gitoxide/commit/9656ac620a1a085122676052b9a0b32d9c4f6661))
    - [git-ref] failure tests ([`567e86c`](https://github.com/Byron/gitoxide/commit/567e86caf83c73497b021d636ea440cc817f10ba))
    - [git-ref] more tests ([`048fb77`](https://github.com/Byron/gitoxide/commit/048fb775764004ec5bb39bf243a102233dd9946c))
    - [git-ref] refactor ([`77d0cc0`](https://github.com/Byron/gitoxide/commit/77d0cc088d6de8c37fec9ae0136c9f85bfdbc643))
    - [git-ref] don't support serde for now ([`2a6295b`](https://github.com/Byron/gitoxide/commit/2a6295bbd8a30d84c0d6544ca83e79146aff088e))
    - [git-ref] refactor ([`02e545b`](https://github.com/Byron/gitoxide/commit/02e545ba6fe801f43e0a76e43e8bcfaaf77bd5f5))
    - [git-ref] first basic 'ref: ' parsing ([`60fa3ba`](https://github.com/Byron/gitoxide/commit/60fa3bac9bfff7b5e3ac331c77c1050e9359f481))
    - [git-ref] refactor ([`9a30f87`](https://github.com/Byron/gitoxide/commit/9a30f87292aff1d4a2f043ba160df6b09bce16c8))
    - [git-ref] the first succeeding test ([`cebfdb4`](https://github.com/Byron/gitoxide/commit/cebfdb463ac2d86f56bb3a2d57c0487a8b233fd8))
    - [git-ref] the first failing test ([`7e802a0`](https://github.com/Byron/gitoxide/commit/7e802a0576230dfc666c253d484ea255f265f92f))
    - [git-ref] sketch ref creation ([`c5241b8`](https://github.com/Byron/gitoxide/commit/c5241b835b93af497cda80ce0dceb8f49800df1c))
    - [git-ref] A sketch of how it looks like with Store backref ([`1a08f1c`](https://github.com/Byron/gitoxide/commit/1a08f1c0365afe7d5e6fbc80bdd382d193d4b881))
    - [git-ref] more scaffolding ([`8c6e884`](https://github.com/Byron/gitoxide/commit/8c6e8844627878e981e597de0c29408cf51582a4))
    - [git-ref] clear it out and move existing functionality to git-object ([`fa548ce`](https://github.com/Byron/gitoxide/commit/fa548ce94db3dd3969add494756fcc34e48985a3))
    - (cargo-release) version 0.5.0 ([`b6b5856`](https://github.com/Byron/gitoxide/commit/b6b58560b7c3bc88e2b8b780be5ceb4cb508a346))
    - [pack-gen] refactor ([`61554e2`](https://github.com/Byron/gitoxide/commit/61554e2effcbafef9cff0b407351c2fae0d2916c))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 15 times to make code idiomatic. 

## v0.4.1 (2020-12-19)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 94 calendar days.
 - 98 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.1 ([`25d2c2e`](https://github.com/Byron/gitoxide/commit/25d2c2e6ae70f46869ab0dabdda2b9f7840539d3))
    - Document `git-ref` ([`91dce23`](https://github.com/Byron/gitoxide/commit/91dce23c8faf74511c33e5cfa07d2f293b1cd0a2))
    - remove dash in all repository links ([`98c1360`](https://github.com/Byron/gitoxide/commit/98c1360ba4d2fb3443602b7da8775906224feb1d))
    - Merge from main. ([`b59bd5e`](https://github.com/Byron/gitoxide/commit/b59bd5e0b0895c7d1d585816cec8be4dea78c278))
    - refactor ([`ba1d883`](https://github.com/Byron/gitoxide/commit/ba1d88364424eb60a0874a5726b62740dc348592))
</details>

## v0.4.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 29 calendar days.
 - 30 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 ([`f9dd225`](https://github.com/Byron/gitoxide/commit/f9dd225afc4aafde1a8b8148943f56f2c547a9ea))
    - Allow dual-licensing with Apache 2.0 ([`ea353eb`](https://github.com/Byron/gitoxide/commit/ea353eb02fd4f75508600cc5676107bc7e627f1e))
    - refactor ([`63c1292`](https://github.com/Byron/gitoxide/commit/63c129292288cc626b09ad29e9ef5f1a1d8339e4))
</details>

## v0.3.0 (2020-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 20 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump minor version to 0.3 ([`4351e28`](https://github.com/Byron/gitoxide/commit/4351e2871c9dcf342b8471fffa74cae338a53269))
    - update to quick-error 2.0 ([`4b1b784`](https://github.com/Byron/gitoxide/commit/4b1b7849b47a54092b49821c39e864c86adda979))
</details>

## v0.2.0 (2020-07-23)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 6 calendar days.
 - 7 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 ([`d350a13`](https://github.com/Byron/gitoxide/commit/d350a13784685ea82b84646b18736986aeb68146))
    - Switch to latest quick-error ([`9760856`](https://github.com/Byron/gitoxide/commit/976085614ee13a19fc1347209259a3dcf36ef95b))
    - assert we don't exeed package sizes ([`df66d74`](https://github.com/Byron/gitoxide/commit/df66d74aa2a8cb62d8a03383135f08c8e8c579a8))
</details>

## v0.1.0 (2020-07-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - refactor ([`6ad9304`](https://github.com/Byron/gitoxide/commit/6ad93041813f78548c3bd813b8685a60d857336f))
    - refactor ([`1fd90f7`](https://github.com/Byron/gitoxide/commit/1fd90f739f4d8bb7c4f27103d2bb92e3f58b6f68))
    - test for common ascii control characters ([`ae0c885`](https://github.com/Byron/gitoxide/commit/ae0c885518d9ce4de05adbb048c0188f9ca934c3))
    - all test for valid ref name except for ascii control chars ([`a157acf`](https://github.com/Byron/gitoxide/commit/a157acfb1f68ec6af6bb0b76f52aa8c7f72d43bf))
    - add new 'git-ref' crate; place ref name validation code there ([`1a0e84e`](https://github.com/Byron/gitoxide/commit/1a0e84e627b17be1b1fb53b4dc98ab78e9cfb9a7))
</details>

