# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.5.1 (2022-12-19)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 27 calendar days.
 - 27 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Merge branch 'adjustments-for-cargo' ([`083909b`](https://github.com/Byron/gitoxide/commit/083909bc7eb902eeee2002034fdb6ed88280dc5c))
    - adjust to changes in `git-testtools` ([`4eb842c`](https://github.com/Byron/gitoxide/commit/4eb842c7150b980e1c2637217e1f9657a671cea7))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
</details>

## 0.5.0 (2022-11-21)

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

## 0.4.2 (2022-11-17)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 10 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-glob v0.4.2, git-config-value v0.8.2, git-lock v2.2.0, git-ref v0.19.0, git-config v0.11.0, git-discover v0.8.0, git-index v0.8.0, git-transport v0.22.0, git-protocol v0.23.0, git-worktree v0.8.0, git-repository v0.28.0, gitoxide-core v0.20.0, gitoxide v0.18.0, safety bump 9 crates ([`0c253b1`](https://github.com/Byron/gitoxide/commit/0c253b15143dcedfe4c66d64ab1ea6e097030651))
    - prepare changelogs prior to release ([`fe5721f`](https://github.com/Byron/gitoxide/commit/fe5721f888c64c79fe9a734a9e33b94a282f8d97))
    - Merge branch 'http-config' ([`665b53e`](https://github.com/Byron/gitoxide/commit/665b53e1c2e1de65fafa28b669f58977868bbc81))
    - remove unused and empty file ([`e7bc5f2`](https://github.com/Byron/gitoxide/commit/e7bc5f279fc3bc931b904b5b902b8fc1d1d4f67e))
</details>

## 0.4.1 (2022-11-06)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 47 calendar days.
 - 47 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - prepare changelogs prior to release ([`423af90`](https://github.com/Byron/gitoxide/commit/423af90c8202d62dc1ea4a76a0df6421d1f0aa06))
    - Merge branch 'main' into write-sparse-index (upgrade to Rust 1.65) ([`5406630`](https://github.com/Byron/gitoxide/commit/5406630466145990b5adbdadb59151036993060d))
    - thanks clippy ([`04cfa63`](https://github.com/Byron/gitoxide/commit/04cfa635a65ae34ad6d22391f2febd2ca7eabca9))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
</details>

## 0.4.0 (2022-09-20)

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 27 calendar days.
 - 27 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - fix git-glob tests ([`0c17681`](https://github.com/Byron/gitoxide/commit/0c17681602ef74cb5e9329d489df6fd7a07d546d))
    - frame for baseline for fetch-matching ([`2569da5`](https://github.com/Byron/gitoxide/commit/2569da5988a055372a1b85660f93185603900dbe))
    - upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **Uncategorized**
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
</details>

## 0.3.2 (2022-08-24)

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

 - 11 commits contributed to the release over the course of 30 calendar days.
 - 32 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - use docsrs feature in code to show what is feature-gated automatically on docs.rs ([`b1c40b0`](https://github.com/Byron/gitoxide/commit/b1c40b0364ef092cd52d03b34f491b254816b18d))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - pass --cfg docsrs when compiling for https://docs.rs ([`5176771`](https://github.com/Byron/gitoxide/commit/517677147f1c17304c62cf97a1dd09f232ebf5db))
    - remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
</details>

## 0.3.1 (2022-07-22)

This is a maintenance release with no functional changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 64 calendar days.
 - 64 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - assure document-features are available in all 'usable' and 'early' crates ([`238581c`](https://github.com/Byron/gitoxide/commit/238581cc46c7288691eed37dc7de5069e3d86721))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
</details>

## 0.3.0 (2022-05-18)

### New Features

 - <csr-id-455a72eb0c01c158f43d9b9a1180886f677bad00/> `fmt::Display` impl for `Pattern`.
   This way the original pattern can be reproduced on the fly without
   actually storing it, saving one allocation.
 - <csr-id-2c88b575630e1b179955dad578e779aad8dd58d8/> add `Default` impl for `pattern::Case`

### Changed (BREAKING)

 - <csr-id-8fd9f24e2f751292a99b4f92cc47df67e17ab537/> invert meaning of `wildcard::Mode::SLASH_IS_LITERAL`
   This is done by renaming it to
 - <csr-id-f76a426833530c7a7e787487cfceaba2c80b21ac/> remove `base_path` field from `Pattern`
   It's now passed as argument to the path pattern matcher and maybe
   it will even be removed one day.
   
   Even though it's convenient to have a base path per pattern, it's
   quite some duplication.
 - <csr-id-568f013e762423fc54a8fb1daed1e7b59c1dc0f0/> `Pattern::matches()` is now private
   It doesn't work as one would expect due to it wanting to match relative
   paths only. Thus it's better to spare folks the surprise and instead
   use `wildmatch()` directly. It works the same, but doesn't
   have certain shortcuts which aren't needed for standard matches
   anyway.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 36 commits contributed to the release over the course of 35 calendar days.
 - 35 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#301](https://github.com/Byron/gitoxide/issues/301), [#384](https://github.com/Byron/gitoxide/issues/384)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - `fmt::Display` impl for `Pattern`. ([`455a72e`](https://github.com/Byron/gitoxide/commit/455a72eb0c01c158f43d9b9a1180886f677bad00))
    - adapt to changes in git-path ([`cc2d810`](https://github.com/Byron/gitoxide/commit/cc2d81012d107da7a61bf4de5b28342dea5083b7))
    - add `Default` impl for `pattern::Case` ([`2c88b57`](https://github.com/Byron/gitoxide/commit/2c88b575630e1b179955dad578e779aad8dd58d8))
    - cleanup ([`1ab4705`](https://github.com/Byron/gitoxide/commit/1ab470589450ecda45826c38417616f227e3031b))
    - Allow basename matches to work like before ([`4f6cefc`](https://github.com/Byron/gitoxide/commit/4f6cefc96bea5f116eb26a9de8095271fd0f58e2))
    - adjust baseline to only handle patterns that work without a dir stack ([`fb65a39`](https://github.com/Byron/gitoxide/commit/fb65a39e1826c331545b7141c0741904ed5bb1a4))
    - discover an entirely new class of exclude matches… ([`f8dd5ce`](https://github.com/Byron/gitoxide/commit/f8dd5ce8ce27cd24b9d81795dcf01ce03efe802d))
    - Basic match group pattern matching ([`cc1312d`](https://github.com/Byron/gitoxide/commit/cc1312dc06d1dccfa2e3cf0ae134affa9a3fa947))
    - `Pattern::matches()` is now private ([`568f013`](https://github.com/Byron/gitoxide/commit/568f013e762423fc54a8fb1daed1e7b59c1dc0f0))
    - push base path handling to the caller ([`e4b57b1`](https://github.com/Byron/gitoxide/commit/e4b57b197884bc981b8e3c9ee8c7b5349afa594b))
    - A slightly ugly way of not adjusting input patterns too much ([`3912ee6`](https://github.com/Byron/gitoxide/commit/3912ee66b6117681331df5e6e0f8345335728bde))
    - Adjustments to support lower MSRV ([`16a0973`](https://github.com/Byron/gitoxide/commit/16a09737f0e81654cc7a5bbc9043385528524ca5))
    - a failing test to show that the absolute pattern handling isn't quite there yet ([`74c89eb`](https://github.com/Byron/gitoxide/commit/74c89ebbd235e8f5464e0665cc7bc7a930a8eb76))
    - remove `base_path` field from `Pattern` ([`f76a426`](https://github.com/Byron/gitoxide/commit/f76a426833530c7a7e787487cfceaba2c80b21ac))
    - make fmt ([`5fc5459`](https://github.com/Byron/gitoxide/commit/5fc5459b17b623726f99846c432a70106464e970))
    - cleanup tests ([`16570ef`](https://github.com/Byron/gitoxide/commit/16570ef96785c62eb813d4613df097aca3aa0d8f))
    - case-insensitive tests for baseline path matching ([`bc928f9`](https://github.com/Byron/gitoxide/commit/bc928f9c00b5f00527a122c8bf847278e90ffb04))
    - invert meaning of `wildcard::Mode::SLASH_IS_LITERAL` ([`8fd9f24`](https://github.com/Byron/gitoxide/commit/8fd9f24e2f751292a99b4f92cc47df67e17ab537))
    - make glob tests work on windows for now… ([`29738ed`](https://github.com/Byron/gitoxide/commit/29738edc56da6dbb9b853ac8f7482672eafd5050))
    - See if being less pedantic yields the correct results ([`18953e4`](https://github.com/Byron/gitoxide/commit/18953e4c367ef1d3c2b28a0b027acc715af6372f))
 * **[#384](https://github.com/Byron/gitoxide/issues/384)**
    - No need to isolate archives by crate name ([`19d46f3`](https://github.com/Byron/gitoxide/commit/19d46f35440419b9911b6e2bca2cfc975865dce9))
    - add archive files via git-lfs ([`7202a1c`](https://github.com/Byron/gitoxide/commit/7202a1c4734ad904c026ee3e4e2143c0461d51a2))
    - make sure existing files aren't written into ([`9b5a8a2`](https://github.com/Byron/gitoxide/commit/9b5a8a243d49b6567d1db31050d3bf3123dd54d3))
    - auto-set commit.gpgsign=false when executing git ([`c23feb6`](https://github.com/Byron/gitoxide/commit/c23feb64ad157180cfba8a11c882b829733ea8f6))
 * **Uncategorized**
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Merge branch 'main' into git_includeif ([`05eb340`](https://github.com/Byron/gitoxide/commit/05eb34023933918c51c03cf2afd774db89cc5a33))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - Merge branch 'worktree-stack' ([`98da8ba`](https://github.com/Byron/gitoxide/commit/98da8ba52cef8ec27f705fcbc84773e5bacc4e10))
    - thanks clippy ([`5bf6b52`](https://github.com/Byron/gitoxide/commit/5bf6b52cd51bef19079e87230e5ac463f8f881c0))
    - Merge branch 'worktree-stack' ([`39046e9`](https://github.com/Byron/gitoxide/commit/39046e98098da7d490757477986479126a45b3e5))
    - Merge branch 'main' into repo-status ([`0eb2372`](https://github.com/Byron/gitoxide/commit/0eb23721dca78f6e6bf864c5c3a3e44df8b419f0))
    - Merge branch 'test-archive-support' ([`350df01`](https://github.com/Byron/gitoxide/commit/350df01042d6ca8b93f8737fa101e69b50535a0f))
    - Merge branch 'main' into repo-status ([`4086335`](https://github.com/Byron/gitoxide/commit/40863353a739ec971b49410fbc2ba048b2762732))
    - thanks clippy ([`74f6420`](https://github.com/Byron/gitoxide/commit/74f64202dfc6d9b34228595e260014708ec388e3))
</details>

## 0.2.0 (2022-04-13)

### Changed (BREAKING)

 - <csr-id-6ce3611891d4b60c86055bf749a1b4060ee2c3e1/> `parse()` returns a `Pattern`.
   This is much more ergonomic as this is the only things we are ever
   interested in for matching. If necessary, from there one can also
   use the parts individually or alter them.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 52 commits contributed to the release over the course of 6 calendar days.
 - 6 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#301](https://github.com/Byron/gitoxide/issues/301)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - `parse()` returns a `Pattern`. ([`6ce3611`](https://github.com/Byron/gitoxide/commit/6ce3611891d4b60c86055bf749a1b4060ee2c3e1))
    - docs for git-glob ([`8f4969f`](https://github.com/Byron/gitoxide/commit/8f4969fe7c2e3f3bb38275d5e4ccb08d0bde02bb))
    - all wildmatch tests succeed ([`d3a7349`](https://github.com/Byron/gitoxide/commit/d3a7349b707911670f17a92a0f82681544ebc769))
    - add all character classes sans some of the more obscure ones ([`538d41d`](https://github.com/Byron/gitoxide/commit/538d41d51d7cdc472b2a712823a5a69810f75015))
    - frame for character classes ([`6b8d0d2`](https://github.com/Byron/gitoxide/commit/6b8d0d20b449f6adffd403d0555596041a6c1903))
    - fix all remaining bracket tests… ([`3afe2d2`](https://github.com/Byron/gitoxide/commit/3afe2d2b862c9a22b90cbfbf75da6c84ca91ebf4))
    - more bracket-range tests succeed ([`c64f71c`](https://github.com/Byron/gitoxide/commit/c64f71c38ff404e9c9f150e3e6d3e02ca11e9235))
    - make bracket matching work better ([`97aa9ed`](https://github.com/Byron/gitoxide/commit/97aa9ed22ccb927147a1e456ee6e3510ecc9f90a))
    - refactor ([`fa0440f`](https://github.com/Byron/gitoxide/commit/fa0440fb3c80f8052e08526cf260e929722ccf02))
    - first steps towards bracket matching ([`54fe029`](https://github.com/Byron/gitoxide/commit/54fe0294e36e6ae9a025ef8661d5e21fd488dc87))
    - adjust wildmatch corpus expectations as it won't match our preprocessor ([`48990af`](https://github.com/Byron/gitoxide/commit/48990af81110a411ad07e199916005a8885db920))
    - fix another issue around double-star ([`d15c2fb`](https://github.com/Byron/gitoxide/commit/d15c2fb0119edc7635efc174a703101e100c0b4c))
    - fix another special case ([`09095df`](https://github.com/Byron/gitoxide/commit/09095dfb123f419a3df715d48e60e1f8ec62d060))
    - fix double-star matches ([`43371b6`](https://github.com/Byron/gitoxide/commit/43371b6fa0d6e62d9cde0399f1c9dd3e76b95d99))
    - fix single-level double-star ([`e5a7995`](https://github.com/Byron/gitoxide/commit/e5a79951dc32d336ae5b6c4230b3058ed80456d6))
    - fix backslash handling; improve star handling ([`7907cb4`](https://github.com/Byron/gitoxide/commit/7907cb4e12b56bdbea6abdc59f1022a508a83c87))
    - new wildcard tests to help fix star matching ([`d21c654`](https://github.com/Byron/gitoxide/commit/d21c6541959b0fe34a3882ffcb9e657d6c685734))
    - All our simple wildmatches are working, a good start ([`321c4d2`](https://github.com/Byron/gitoxide/commit/321c4d2011617f2b13e29109cafe4566e53bfde3))
    - maybe even working double-star handling ([`48c57ff`](https://github.com/Byron/gitoxide/commit/48c57ff3299928fd427bfae3e4eeadf5a9ca8109))
    - slowly move towards star/double-star ([`4efd215`](https://github.com/Byron/gitoxide/commit/4efd21560c754062f09870d253b6a2809cb0efb1))
    - question mark support ([`e83c8df`](https://github.com/Byron/gitoxide/commit/e83c8df03e801e00571f5934331e004af9774c7f))
    - very basic beginnings of wildmatch ([`334c624`](https://github.com/Byron/gitoxide/commit/334c62459dbb6763a46647a64129f89e27b5781b))
    - fix logic in wildmatch tests; validate feasibility of all test cases ([`1336bc9`](https://github.com/Byron/gitoxide/commit/1336bc938cc43e3a2f9e47af64f2c9933c9fc961))
    - test corpus for wildcard matches ([`bd8f95f`](https://github.com/Byron/gitoxide/commit/bd8f95f757e45b3cf8523d3e11503f4571461abf))
    - frame for wildmatch function and its tests ([`04ca834`](https://github.com/Byron/gitoxide/commit/04ca8349e326f7b7505a9ea49a401565259f21dc))
    - more tests for early exit in case no-wildcard prefix doesn't match ([`1ff348c`](https://github.com/Byron/gitoxide/commit/1ff348c4f09839569dcd8bb93699e7004fa59d4a))
    - more non-basename shortcuts, and only wildcard matches left ([`45c6259`](https://github.com/Byron/gitoxide/commit/45c62597b50c3c4bac34e20cd2040b08833584cc))
    - make much clearer how base-path works and put in safe-guards ([`5bf503a`](https://github.com/Byron/gitoxide/commit/5bf503af86ce0dd4d0a79c9b1a451cf89b494a6e))
    - test that bases are ignored for basenames ([`1b26848`](https://github.com/Byron/gitoxide/commit/1b2684892419f234e6006b0f3820341f162dc28b))
    - refactor ([`056b368`](https://github.com/Byron/gitoxide/commit/056b3683eb2d4d4c478ae2655e6ef067d4d0d1e7))
    - a way to set a globs base path ([`3d58db8`](https://github.com/Byron/gitoxide/commit/3d58db8a9abfb91600216b8fc6f4109f5289d776))
    - get to the point where globs probably should have a base ([`2632988`](https://github.com/Byron/gitoxide/commit/263298876d1b10b12011c2a221b67126d6d8202d))
    - refactor ([`f2f3f53`](https://github.com/Byron/gitoxide/commit/f2f3f53574b4c0b5ba85780b134825f9128fa64f))
    - prepare for handling absolute patterns ([`df9778b`](https://github.com/Byron/gitoxide/commit/df9778b924610f6a82d93cdf12cfddda60e61789))
    - Keep track of absolute patterns, those that have to start with it ([`3956480`](https://github.com/Byron/gitoxide/commit/3956480e6fb5f4766a67ebf2860cae2f48125594))
    - basename parsing with simple pattern skips ([`d18ef14`](https://github.com/Byron/gitoxide/commit/d18ef14e7cbf9c6d316086d6c88b5676c4b7516c))
    - git-baseline now acts like a massive regression test ([`fe3d0a7`](https://github.com/Byron/gitoxide/commit/fe3d0a778210a46d46a7db15cc8d213706e45fee))
    - adjust signatures to know enough to implement git-like matching ([`b947ff9`](https://github.com/Byron/gitoxide/commit/b947ff9d2c5ae8810547066096c91c745d1466fe))
    - refactor; roughly sort regex by simplicity ([`a7c3a63`](https://github.com/Byron/gitoxide/commit/a7c3a630cd5661f26220b494f01e50c9f2dbd2e2))
    - Also parse the position of the first wildcard ([`4178a63`](https://github.com/Byron/gitoxide/commit/4178a6356ad11013ae08b6233de2bfb366bf4278))
    - prepare for upcoming wildcard-length field in glob pattern ([`a11f5d4`](https://github.com/Byron/gitoxide/commit/a11f5d441a22b844caefd31b9cb7783dd6b048ad))
    - refactor ([`f285ca0`](https://github.com/Byron/gitoxide/commit/f285ca03094655590d7014770ffb6f6a77d02289))
    - basic infrastructure for running git-baseline against our implementation ([`027869d`](https://github.com/Byron/gitoxide/commit/027869d57bd7fcb7234e814d1a22197cb64c05cf))
    - baseline tests for matches and no-matches ([`621c2ca`](https://github.com/Byron/gitoxide/commit/621c2cac7eed822cc8226c7b9aa8becf3db6872c))
    - bring in all ~140 tests for git pattern matching, git-ignore styile ([`f9ab830`](https://github.com/Byron/gitoxide/commit/f9ab830df2920387c1cffec048be3a4089f4aa40))
    - refactor ([`dbe7305`](https://github.com/Byron/gitoxide/commit/dbe7305d371c7dad02d8888492b60b882b418a46))
    - refactor ([`8a54341`](https://github.com/Byron/gitoxide/commit/8a543410e10326ce506b8a7ba65e662641835849))
 * **Uncategorized**
    - Release git-glob v0.2.0, safety bump 3 crates ([`ab6bed7`](https://github.com/Byron/gitoxide/commit/ab6bed7e2aa19eeb9990441741008c430f373708))
    - Merge branch 'worktree-stack' ([`e90d3fd`](https://github.com/Byron/gitoxide/commit/e90d3fd0a9764511e6280596f21d3a0494ed7021))
    - thanks clippy ([`b1a6100`](https://github.com/Byron/gitoxide/commit/b1a610029e1b40600f90194ce986155238f58101))
    - thanks clippy ([`1393403`](https://github.com/Byron/gitoxide/commit/1393403b826cf4a2fbaf6ef58d505c5c62fd5e0a))
    - thanks clippy ([`683233e`](https://github.com/Byron/gitoxide/commit/683233e86dab36cc438bed0f8b0338eb767f57a0))
</details>

## 0.1.0 (2022-04-07)

Initial release with pattern parsing functionality.

### Changed (BREAKING)

 - <csr-id-568f013e762423fc54a8fb1daed1e7b59c1dc0f0/> `Pattern::matches()` is now private
   It doesn't work as one would expect due to it wanting to match relative
   paths only. Thus it's better to spare folks the surprise and instead
   use `wildmatch()` directly. It works the same, but doesn't
   have certain shortcuts which aren't needed for standard matches
   anyway.

### New Features

 - <csr-id-2c88b575630e1b179955dad578e779aad8dd58d8/> add `Default` impl for `pattern::Case`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#301](https://github.com/Byron/gitoxide/issues/301)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - prepare changelog prior to release ([`2794bb2`](https://github.com/Byron/gitoxide/commit/2794bb2f6bd80cccba508fa9f251609499167646))
    - Add git-glob crate with pattern matching parsing from git-attributes::ignore ([`b3efc94`](https://github.com/Byron/gitoxide/commit/b3efc94134a32018db1d6a2d7f8cc397c4371999))
 * **Uncategorized**
    - Release git-glob v0.1.0 ([`0f66c5d`](https://github.com/Byron/gitoxide/commit/0f66c5d56bd3f0febff881065911638f22e71158))
</details>

