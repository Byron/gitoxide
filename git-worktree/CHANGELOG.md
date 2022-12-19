# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.10.0 (2022-12-19)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 27 calendar days.
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
    - Release git-hash v0.10.1, git-hashtable v0.1.0 ([`7717170`](https://github.com/Byron/gitoxide/commit/771717095d9a67b0625021eb0928828ab686e772))
    - Merge branch 'main' into http-config ([`6b9632e`](https://github.com/Byron/gitoxide/commit/6b9632e16c416841ffff1b767ee7a6c89b421220))
    - Release git-features v0.24.1, git-actor v0.14.1, git-index v0.9.1 ([`7893502`](https://github.com/Byron/gitoxide/commit/789350208efc9d5fc6f9bc4f113f77f9cb445156))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
</details>

## 0.9.0 (2022-11-21)

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

## 0.8.0 (2022-11-17)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 10 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-glob v0.4.2, git-config-value v0.8.2, git-lock v2.2.0, git-ref v0.19.0, git-config v0.11.0, git-discover v0.8.0, git-index v0.8.0, git-transport v0.22.0, git-protocol v0.23.0, git-worktree v0.8.0, git-repository v0.28.0, gitoxide-core v0.20.0, gitoxide v0.18.0, safety bump 9 crates ([`0c253b1`](https://github.com/Byron/gitoxide/commit/0c253b15143dcedfe4c66d64ab1ea6e097030651))
    - prepare changelogs prior to release ([`fe5721f`](https://github.com/Byron/gitoxide/commit/fe5721f888c64c79fe9a734a9e33b94a282f8d97))
</details>

## 0.7.0 (2022-11-06)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 25 calendar days.
 - 27 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - make it possible to clone empty remote repositories ([`e97eeda`](https://github.com/Byron/gitoxide/commit/e97eeda45c9cc0736273c735a9959ac1ff29fc9d))
    - refactor ([`dd3b336`](https://github.com/Byron/gitoxide/commit/dd3b336656f5282f386c1cba0974abd2b09d81e9))
 * **Uncategorized**
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - prepare changelogs prior to release ([`423af90`](https://github.com/Byron/gitoxide/commit/423af90c8202d62dc1ea4a76a0df6421d1f0aa06))
    - Merge branch 'main' into write-sparse-index (upgrade to Rust 1.65) ([`5406630`](https://github.com/Byron/gitoxide/commit/5406630466145990b5adbdadb59151036993060d))
    - thanks clippy ([`04cfa63`](https://github.com/Byron/gitoxide/commit/04cfa635a65ae34ad6d22391f2febd2ca7eabca9))
    - Merge branch 'main' into write-sparse-index ([`c4e6849`](https://github.com/Byron/gitoxide/commit/c4e68496c368611ebe17c6693d06c8147c28c717))
    - Merge branch 'gix-clone' ([`def53b3`](https://github.com/Byron/gitoxide/commit/def53b36c3dec26fa78939ab0584fe4ff930909c))
    - improved working of docs ([`1ef704e`](https://github.com/Byron/gitoxide/commit/1ef704e7daa83e298d99d94a3493296739338110))
    - Merge branch 'main' into gix-clone ([`de4fe06`](https://github.com/Byron/gitoxide/commit/de4fe06202906ea5c62e667826b42cf7b57b1ff0))
    - Merge branch 'fix-gix-index-from-tree' ([`da5f63c`](https://github.com/Byron/gitoxide/commit/da5f63cbc7506990f46d310f8064678decb86928))
    - fix build ([`bb81abe`](https://github.com/Byron/gitoxide/commit/bb81abece95f466c38567d7488c0e6ae1a22e06b))
</details>

## 0.6.0 (2022-10-10)

Maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 20 calendar days.
 - 20 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - Merge branch 'fix-git-features' ([`82fd251`](https://github.com/Byron/gitoxide/commit/82fd251ac80d07bc9da8a4d36e517aa35580d188))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
</details>

## 0.5.0 (2022-09-20)

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 22 calendar days.
 - 24 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **Uncategorized**
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Release git-features v0.22.6 ([`c9eda72`](https://github.com/Byron/gitoxide/commit/c9eda729d8f8bc266c7516c613d38acfb83a4743))
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

## 0.4.3 (2022-08-27)

Maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-worktree v0.4.3, git-testtools v0.8.0 ([`b2e4bf2`](https://github.com/Byron/gitoxide/commit/b2e4bf2c11ff2c3c32efcb91837fb5677714bdf9))
    - Release git-attributes v0.3.3, git-ref v0.15.3, git-index v0.4.3, git-worktree v0.4.3, git-testtools v0.8.0 ([`baad4ce`](https://github.com/Byron/gitoxide/commit/baad4ce51fe0e8c0c1de1b08148d8303878ca37b))
    - prepare changelogs prior to release of git-testtools ([`7668e38`](https://github.com/Byron/gitoxide/commit/7668e38fab8891ed7e73fae3a6f5a8772e0f0d0b))
    - Release git-features v0.22.3, git-revision v0.4.4 ([`c2660e2`](https://github.com/Byron/gitoxide/commit/c2660e2503323531ba02519eaa51124ee22fec51))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
</details>

## 0.4.2 (2022-08-24)

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

 - 10 commits contributed to the release over the course of 5 calendar days.
 - 6 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-worktree v0.4.2, git-repository v0.22.0 ([`2f0f530`](https://github.com/Byron/gitoxide/commit/2f0f530fb1d644bc0694e04f3c9309149f526e42))
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

## 0.4.1 (2022-08-17)

A maintenance release without user facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 26 calendar days.
 - 26 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-worktree v0.4.1, git-repository v0.21.0 ([`ee383f3`](https://github.com/Byron/gitoxide/commit/ee383f347371007f1c4d3a2a98c5511d7e0793a8))
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Merge branch 'kianmeng-fix-typos' ([`4e7b343`](https://github.com/Byron/gitoxide/commit/4e7b34349c0a01ad8686bbb4eb987e9338259d9c))
    - Fix typos ([`e9fcb70`](https://github.com/Byron/gitoxide/commit/e9fcb70e429edb2974afa3f58d181f3ef14c3da3))
</details>

## 0.4.0 (2022-07-22)

This is a maintenance release with no functional changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 33 calendar days.
 - 39 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`d4df661`](https://github.com/Byron/gitoxide/commit/d4df661dbf60dad75d07002ef9979cabe8a86935))
    - Release git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`aa639d8`](https://github.com/Byron/gitoxide/commit/aa639d8c43f3098cc4a5b50614c5ae94a8156928))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`daa71c3`](https://github.com/Byron/gitoxide/commit/daa71c3b753c6d76a3d652c29237906b3e28728f))
    - make it harder to forget documentation in git-worktree ([`15d87ee`](https://github.com/Byron/gitoxide/commit/15d87ee99ef269985e8f378bb2ab9c8931e8fd7d))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-path v0.3.0, safety bump 14 crates ([`400c9be`](https://github.com/Byron/gitoxide/commit/400c9bec49e4ec5351dc9357b246e7677a63ea35))
</details>

## 0.3.0 (2022-06-13)

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
    - Release git-worktree v0.3.0, git-repository v0.19.0 ([`0d8e856`](https://github.com/Byron/gitoxide/commit/0d8e8566dc5c6955487d67e235f86fbc75a3a88a))
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
</details>

## 0.2.0 (2022-05-18)

A maintenance release without documented changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 83 commits contributed to the release over the course of 34 calendar days.
 - 45 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#301](https://github.com/Byron/gitoxide/issues/301), [#384](https://github.com/Byron/gitoxide/issues/384)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - status quo test that shows gitoxide has the same limitation as git ([`5f6c2fb`](https://github.com/Byron/gitoxide/commit/5f6c2fb7787e674aa05af6185e665d6a33860f02))
    - refactor ([`36fa167`](https://github.com/Byron/gitoxide/commit/36fa16761bd59d9c314e29b1b0911608ae409c1f))
    - improve how directory excludes are handled ([`bea5ea5`](https://github.com/Byron/gitoxide/commit/bea5ea5cb3d304e73260fc1139b8fdc1acc139d7))
    - Fix inverted logic for matching non-negative pattern in `is_excluded()` ([`6d5784f`](https://github.com/Byron/gitoxide/commit/6d5784fc961c08fda7affffa4601baaea0000b98))
    - reorganize types to properly represent worktrees in their various 'states' ([`b46bff5`](https://github.com/Byron/gitoxide/commit/b46bff58e40bb9805af7ee7f96272f0dc19c0ac7))
    - A sketch for worktree state ([`55e17a4`](https://github.com/Byron/gitoxide/commit/55e17a402c70be64609f0ffa98d1eaeee5146439))
    - Basic prefix support as well the first working version of `exclude query` ([`9cb8385`](https://github.com/Byron/gitoxide/commit/9cb83859f9bb76f38ab5bbd0ae6d6f20a691e9e1))
    - Revert "Turn attribute files into a Cow to support other usecases…" ([`ed7f223`](https://github.com/Byron/gitoxide/commit/ed7f223b1bee688dbd257a59f3317f39bf5eb2cd))
    - Turn attribute files into a Cow to support other usecases… ([`d0c8407`](https://github.com/Byron/gitoxide/commit/d0c84079bdd4bb7746f47f132868ed4743f5dda0))
    - make use of new git-glob::Pattern::to_string() feature ([`d29932d`](https://github.com/Byron/gitoxide/commit/d29932dc579f0579990bca1dcfc656ac020be50e))
    - some tests to check pattern negation ([`2672a25`](https://github.com/Byron/gitoxide/commit/2672a25dae546f85807a7e5ec1939240221a5a14))
    - Test for case-sensitivity as well ([`120675d`](https://github.com/Byron/gitoxide/commit/120675db0508a6bb9d1e0eca45edf3f15632cd2f))
    - The stack now allows to change a non-dir into a dir ([`6793bab`](https://github.com/Byron/gitoxide/commit/6793bab687bf492da545981e0116322dab4455cb))
    - Allow check-ignore style queries with API that doesn't remove trailing slashes ([`e68cd69`](https://github.com/Byron/gitoxide/commit/e68cd692b5230592ca2ca17418d9b9fda9f3e317))
    - more tests and fixes to assure directory logic in stack works ([`2010ddd`](https://github.com/Byron/gitoxide/commit/2010dddf244335f3967d0debb5d8e0f3ffdac6a7))
    - improved testing… ([`e191b72`](https://github.com/Byron/gitoxide/commit/e191b7220c5286bb0d0038398810ae344de626d3))
    - refactor ([`21d4076`](https://github.com/Byron/gitoxide/commit/21d407638285b728d0c64fabf2abe0e1948e9bec))
    - Don't hardcode case in state::Ignore ([`a6532e7`](https://github.com/Byron/gitoxide/commit/a6532e7fd94757dc5b4b231b63cb2cbcacf1e602))
    - The first indication that directory-based excludes work ([`e868acc`](https://github.com/Byron/gitoxide/commit/e868acce2e7c3e2501497bf630e3a54f349ad38e))
    - adapt to all changes in git-path with bstr support ([`f158648`](https://github.com/Byron/gitoxide/commit/f158648aef8ad94d86550ceb2eeb20efb3df7596))
    - Use `git-path` crate instead of `git_features::path` ([`47e607d`](https://github.com/Byron/gitoxide/commit/47e607dc256a43a3411406c645eb7ff04239dd3a))
    - adjustments to go along with changes in git-features ([`c55cac6`](https://github.com/Byron/gitoxide/commit/c55cac6a1ada77619bb5723717a5a6d757499fa9))
    - refactor ([`8345b7c`](https://github.com/Byron/gitoxide/commit/8345b7caa0cc1cd8489e41822eea89da4c539e6d))
    - customize stack operation to support the notion of directories ([`2659816`](https://github.com/Byron/gitoxide/commit/26598163ce0a029e7eb92d862f899bdaadad3e90))
    - And finally, we can read ignore files from the index, too ([`910d500`](https://github.com/Byron/gitoxide/commit/910d5000d479939c14e330b6f1a12d50dd57cdd6))
    - wire everything up to have all data where it needs to be, but… ([`34d0d5c`](https://github.com/Byron/gitoxide/commit/34d0d5c5bedae5ed069fd147c19cfb7414b66fb5))
    - refactor ([`883d78d`](https://github.com/Byron/gitoxide/commit/883d78d3d17cae1b3bdd9801abb3ee6f9452c1a0))
    - fix MSRV ([`63f0839`](https://github.com/Byron/gitoxide/commit/63f08391af5da3901190797532566758e3dff9e3))
    - Support for shared attribute file names ([`e4044a4`](https://github.com/Byron/gitoxide/commit/e4044a48c606497e5de0fd711c7a5ce7afc44117))
    - Use a separate path mapping to enable clone-avoidance ([`e525b5e`](https://github.com/Byron/gitoxide/commit/e525b5e5138ec0050f1ff178b5985cc7ce440b3a))
    - Fix borrow check issues the fast way, but… ([`514e2f4`](https://github.com/Byron/gitoxide/commit/514e2f424fa4976693393c6d0911b724f94b1c70))
    - try to keep borrows to path backing alive but… ([`4234b84`](https://github.com/Byron/gitoxide/commit/4234b8497e3819eaae66f4c0462b5fc29509d675))
    - refactor ([`b14904b`](https://github.com/Byron/gitoxide/commit/b14904b54587f99f8741fa59eda6c2b9db98fff7))
    - doing things directly works fortunately ([`6f74f85`](https://github.com/Byron/gitoxide/commit/6f74f8516ba73c35b1b327aae491f70f83eefafd))
    - An attempt to build a lookup table of attribute files, but… ([`9841efb`](https://github.com/Byron/gitoxide/commit/9841efb566748dae6c79c5990c4fd1ecbc468aef))
    - refactor ([`475aa6a`](https://github.com/Byron/gitoxide/commit/475aa6a3e08f63df627a0988cd16c20494960c79))
    - Make .gitignore name overridable ([`155bb82`](https://github.com/Byron/gitoxide/commit/155bb820be03d4ac210b6ae4a76ecfb33445271e))
    - A test to check skip-worktree special case with ignore files ([`dec9f33`](https://github.com/Byron/gitoxide/commit/dec9f332ecd2eaf7bad8ce0f94194d68624d9ac7))
    - A baseline test that indicates how excludes aren't using data from the index initially ([`e58b771`](https://github.com/Byron/gitoxide/commit/e58b771cd514024e63c1ab7af7c0d0abad00797d))
    - First primitive ignore pattern test works ([`0424136`](https://github.com/Byron/gitoxide/commit/04241367e8ce99ce6c7583d5dac4955fad3d6542))
    - refactor to make push/pop with mutable state work; prepare to read .gitignore files ([`8d1000b`](https://github.com/Byron/gitoxide/commit/8d1000b30257675564195202b15dca1ab1538227))
    - Add baseline test to motivate implementing ignore file stack ([`ce40add`](https://github.com/Byron/gitoxide/commit/ce40add21add518374d9ff6d40fe488e2f29ce6d))
    - re-export `git-glob` as its `Case` type is part of the public API ([`4b72045`](https://github.com/Byron/gitoxide/commit/4b7204516a7c61162a2940eb66e8a7c64bf78ce7))
    - Sketch state for handling attributes as well ([`d87d62d`](https://github.com/Byron/gitoxide/commit/d87d62db5cf327397390ec7888c1d1155619ba38))
    - Sketch state for handling excludes ([`eb525f7`](https://github.com/Byron/gitoxide/commit/eb525f76134a2ffd770848941c976ec456fcc296))
    - sketch how attribute globals could be used in worktrees ([`97ee03d`](https://github.com/Byron/gitoxide/commit/97ee03d5e4703b583dd5bb741dbf43f310404882))
    - Adjustments to support lower MSRV ([`16a0973`](https://github.com/Byron/gitoxide/commit/16a09737f0e81654cc7a5bbc9043385528524ca5))
    - remove `git-dir` for `checkout()` as it's something to be dealt with elsewhere ([`f7996b8`](https://github.com/Byron/gitoxide/commit/f7996b8f6a877275b8725804c558b51732e8b469))
    - an idea on how to test excludes, but… ([`9c036e8`](https://github.com/Byron/gitoxide/commit/9c036e81b3abcd5dcde2b023459a15cbd281824d))
    - Make attributes and ignore configuration possible, but… ([`8a75fd7`](https://github.com/Byron/gitoxide/commit/8a75fd745a194786f0da7c1fd660211446ea51f7))
    - refactor ([`80af734`](https://github.com/Byron/gitoxide/commit/80af734a5ddfd0785ec946a3609887b5d503d03d))
    - provide a platform for multiple queries at a dir cache level ([`48be382`](https://github.com/Byron/gitoxide/commit/48be3828ea07124c4d21cb10121780f596116bcb))
    - Be explicit about the cache-modes that actually happen ([`dc12f88`](https://github.com/Byron/gitoxide/commit/dc12f88a5d2e54e2ff987127bab37e5bd7ce314a))
    - refactor ([`5d30018`](https://github.com/Byron/gitoxide/commit/5d300181c0696430c75bec7070da35cb308a1b9a))
    - refactor ([`fe46078`](https://github.com/Byron/gitoxide/commit/fe46078dd9496744b048165fba548df5c3f76991))
    - port PathCache over to `Stack` ([`ebfea8d`](https://github.com/Byron/gitoxide/commit/ebfea8d4be1afb3bd47bcffbaf5d705bed2d1ed6))
    - A sketch for a generalized version of a path stack ([`0d3ba1a`](https://github.com/Byron/gitoxide/commit/0d3ba1a02f076d32334d85f68d99e6b8033844ad))
    - refactor ([`fe6641c`](https://github.com/Byron/gitoxide/commit/fe6641c86704df67b020510700e9c087fff5a52c))
 * **[#384](https://github.com/Byron/gitoxide/issues/384)**
    - prevent line-ending conversions for shell scripts on windows ([`96bb4d4`](https://github.com/Byron/gitoxide/commit/96bb4d460db420e18dfd0f925109c740e971820d))
    - No need to isolate archives by crate name ([`19d46f3`](https://github.com/Byron/gitoxide/commit/19d46f35440419b9911b6e2bca2cfc975865dce9))
    - add archive files via git-lfs ([`7202a1c`](https://github.com/Byron/gitoxide/commit/7202a1c4734ad904c026ee3e4e2143c0461d51a2))
    - Assure we don't pick up unnecessary files during publishing ([`545b2d5`](https://github.com/Byron/gitoxide/commit/545b2d5121ba64efaee7564d5191cec37661efd7))
    - auto-set commit.gpgsign=false when executing git ([`c23feb6`](https://github.com/Byron/gitoxide/commit/c23feb64ad157180cfba8a11c882b829733ea8f6))
 * **Uncategorized**
    - Release git-worktree v0.2.0, git-repository v0.17.0 ([`3f71246`](https://github.com/Byron/gitoxide/commit/3f7124616ab9752007b8cf03e1c6a3a796ffee0b))
    - Release git-worktree v0.2.0, git-repository v0.17.0 ([`5845934`](https://github.com/Byron/gitoxide/commit/584593448b560afdd60dbdbdff901d267082765e))
    - Release git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0 ([`349c590`](https://github.com/Byron/gitoxide/commit/349c5904b0dac350838a896759d51576b66880a7))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Merge branch 'main' into git_includeif ([`b1bfc8f`](https://github.com/Byron/gitoxide/commit/b1bfc8fe8efb6d8941f54dddd0fcad99aa13ed6c))
    - Merge branch 'basic-worktree-support' ([`e058bda`](https://github.com/Byron/gitoxide/commit/e058bdabf8449b6a6fdff851e3929137d9b71568))
    - Merge branch 'main' into git_includeif ([`05eb340`](https://github.com/Byron/gitoxide/commit/05eb34023933918c51c03cf2afd774db89cc5a33))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - Merge branch 'worktree-stack' ([`98da8ba`](https://github.com/Byron/gitoxide/commit/98da8ba52cef8ec27f705fcbc84773e5bacc4e10))
    - thanks clippy ([`aeebc5f`](https://github.com/Byron/gitoxide/commit/aeebc5fe743faa7d436b1d0a30d60aafbbaeeb3d))
    - thanks clippy ([`b199367`](https://github.com/Byron/gitoxide/commit/b1993672f5a7c516611814fd7c5d6bf796419082))
    - Merge branch 'main' into worktree-stack ([`8674c11`](https://github.com/Byron/gitoxide/commit/8674c11973e5282d087e35a71c70e418b6cc75be))
    - fix release build ([`f7c1920`](https://github.com/Byron/gitoxide/commit/f7c1920214ebfc38676d1d53cc064b0f3d8ece4e))
    - fix release build ([`2705679`](https://github.com/Byron/gitoxide/commit/2705679ddf7e5fe12e93ad214c15d5006c073818))
    - Merge branch 'worktree-stack' ([`39046e9`](https://github.com/Byron/gitoxide/commit/39046e98098da7d490757477986479126a45b3e5))
    - thanks clippy ([`1d365d2`](https://github.com/Byron/gitoxide/commit/1d365d2c6fe19ac8e27c60e3d2596a583a183728))
    - Merge branch 'main' into repo-status ([`0eb2372`](https://github.com/Byron/gitoxide/commit/0eb23721dca78f6e6bf864c5c3a3e44df8b419f0))
    - Merge branch 'test-archive-support' ([`350df01`](https://github.com/Byron/gitoxide/commit/350df01042d6ca8b93f8737fa101e69b50535a0f))
</details>

## 0.1.0 (2022-04-03)

An initial release with the ability to checkout indices with simple files only.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 98 commits contributed to the release over the course of 59 calendar days.
 - 84 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#331](https://github.com/Byron/gitoxide/issues/331), [#333](https://github.com/Byron/gitoxide/issues/333), [#364](https://github.com/Byron/gitoxide/issues/364), [#366](https://github.com/Byron/gitoxide/issues/366)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - Use hash_hasher based hash state for better keys/less collisions ([`814de07`](https://github.com/Byron/gitoxide/commit/814de079f4226f42efa49ad334a348bce67184e4))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - refactor ([`f86eacc`](https://github.com/Byron/gitoxide/commit/f86eacc5cfaf6d88ead4f8dbd65989d32674c213))
    - use io-close instead of close-file - works ([`279461b`](https://github.com/Byron/gitoxide/commit/279461ba1741ace0399127ca9089230082bbf3e0))
    - better error handling on close ([`a28c9b3`](https://github.com/Byron/gitoxide/commit/a28c9b32466a431450a504e313d2e49926e36a98))
    - try close_file crate and see tests fail for some reason ([`c7e1400`](https://github.com/Byron/gitoxide/commit/c7e140094a3a5947cf59107d5a621245ea2ecbeb))
    - more multi-threaded test stability ([`be5a19e`](https://github.com/Byron/gitoxide/commit/be5a19e0eb2e895d03b80afc24c7b8d2d436458d))
    - avoid racyness in worktree tests ([`c8a1319`](https://github.com/Byron/gitoxide/commit/c8a13198a12939befa473b30131e5a763c6fc28c))
    - stabilize assertions in parallel mode ([`21d6f88`](https://github.com/Byron/gitoxide/commit/21d6f880293de4e8ffc6a8472eb1b54d8b1b105a))
    - a reducer which produces progress reporting each time it feeds ([`e83079d`](https://github.com/Byron/gitoxide/commit/e83079d219c96692725ab8af1c0e656cb331ecd8))
    - call chunk processing in threaded processor ([`6bfd865`](https://github.com/Byron/gitoxide/commit/6bfd865a0578eeacd8d19eaa89d8914ac947c62a))
    - conversions from Rc to arc for Handle ([`c19331e`](https://github.com/Byron/gitoxide/commit/c19331e001e587e4fca74f3e9fec28a7df922c0a))
    - basic parallelization, without proper reducer, just so it compiles ([`5f29c0f`](https://github.com/Byron/gitoxide/commit/5f29c0f66d0aa6c045bfdf6f39a806ce8c4a5100))
    - decouple amount of bytes written from progress ([`9ecdade`](https://github.com/Byron/gitoxide/commit/9ecdade0f117b966c98f48d1879bdba21ccaafd7))
    - parallel and non-parallel tests ([`1cd7eb3`](https://github.com/Byron/gitoxide/commit/1cd7eb3f720e8b66792c942a99d7d9d85069ec03))
    - switch index checkout to chunk-based operation ([`e5f6943`](https://github.com/Byron/gitoxide/commit/e5f69433e4a6cc7866b666e0baccfa32efb92a7f))
    - proper handling of interruptions during checkout ([`7575a58`](https://github.com/Byron/gitoxide/commit/7575a5854ebe61a5941177efb470143192223ef3))
    - add thread-count and chunk-size computation; interrupt capability ([`8cbe85d`](https://github.com/Byron/gitoxide/commit/8cbe85d135898826a91939726465a9e295c1e24b))
    - refactor ([`542f49b`](https://github.com/Byron/gitoxide/commit/542f49beb811f7f9bf9dff3cd19694498f6cf9e2))
    - refactor ([`c3c31af`](https://github.com/Byron/gitoxide/commit/c3c31afb9dee5040abef7a8d6f8e1e2cba29e2d7))
    - fix windows test expecations for good ([`81bcb8d`](https://github.com/Byron/gitoxide/commit/81bcb8d281099e952a5e3c075d9578f15f2f2a0d))
    - try to fix windows once again ([`ff95265`](https://github.com/Byron/gitoxide/commit/ff95265a35fb9f340c3a9fa78f8beba24d6734ff))
    - some more debugging on windows ([`0c18443`](https://github.com/Byron/gitoxide/commit/0c18443f5195e10c99504c4f527c1882fcf84e45))
    - debug mode for windows ([`8f3bc5a`](https://github.com/Byron/gitoxide/commit/8f3bc5a3195770753b0b6445259ce20ab609b393))
    - See if we can remove symlinks this way on windows ([`0bc9489`](https://github.com/Byron/gitoxide/commit/0bc94891c92f324d3940e064e8918b117db4641d))
    - delete directories recursively on overwrite-existing ([`ea561e6`](https://github.com/Byron/gitoxide/commit/ea561e6f7d398991f214957dbd92e1b6a81e9ab0))
    - better symlink checking on ubuntu ([`facad25`](https://github.com/Byron/gitoxide/commit/facad25c08b82a975eda70493d4818ca7c560aa8))
    - overwrite-existing support with tests ([`49d1d34`](https://github.com/Byron/gitoxide/commit/49d1d34dff76d8b1e5e7fa9d08e6ead4e8bca018))
    - Fix dir-cache to properly handle its valiity which fixes test ([`52c0058`](https://github.com/Byron/gitoxide/commit/52c0058531df1a0f3fc755c5c51e71d34841cb77))
    - delayed symlink creation for everyone, but… ([`ab5cd3d`](https://github.com/Byron/gitoxide/commit/ab5cd3d383c3c6cb31a7b8d387daedacb9e3838f))
    - delayed symlink creation for windows, but… ([`77b053d`](https://github.com/Byron/gitoxide/commit/77b053dfd38e30a8ab397704059283a4766b9601))
    - prepare for first overwrite test… ([`cd6e086`](https://github.com/Byron/gitoxide/commit/cd6e08644df3a2b52aa70a2f37e988ec10b280f0))
    - fix case-insensitive tests ([`ccd25cb`](https://github.com/Byron/gitoxide/commit/ccd25cb5929554c69ea1250c6d2762fdd6ef5bbd))
    - Allow symlinks to dirs to be returned, too ([`d3d7a7c`](https://github.com/Byron/gitoxide/commit/d3d7a7c3c67868ba0fda6b04e6874aa2f91f638b))
    - try to fix tests on linux ([`9f9d36d`](https://github.com/Byron/gitoxide/commit/9f9d36d7d7bba443fba5917e9920911596fd64f6))
    - a stab at making file writes safer… ([`805c0da`](https://github.com/Byron/gitoxide/commit/805c0da62204b8c4675c9c098e10eb0fe2bc12a9))
    - mior refactor and notes towards parallelization ([`99de1ef`](https://github.com/Byron/gitoxide/commit/99de1ef494719cb4d46e3414474e619225fe7bd4))
    - return proper errors during checkout object lookup ([`f9beac0`](https://github.com/Byron/gitoxide/commit/f9beac0471a38cb4c3b070ecb576ed1a39456bd6))
    - switch worktree to thiserror ([`bacc654`](https://github.com/Byron/gitoxide/commit/bacc65481d4ff5ecfbdf3755383b60f354deaf47))
    - sub-command to print multi-index entries ([`6c10e09`](https://github.com/Byron/gitoxide/commit/6c10e097a432d81b930008abc00c6821ed7ac9be))
    - bring back more detailed errors in case of keep-going ([`8198817`](https://github.com/Byron/gitoxide/commit/8198817507a5e9c6e6fb847a45ac47bd38de68f6))
    - use progress to print errors right when they happen ([`af03686`](https://github.com/Byron/gitoxide/commit/af03686b5abf9548300a83329500b27acd66e16a))
    - implement 'keep-going' for index checkout ([`ecebc55`](https://github.com/Byron/gitoxide/commit/ecebc55f8321c67f57111f8d0002e75388dd3734))
    - Support for forceful removal of symlinks or files during dir creation ([`749c310`](https://github.com/Byron/gitoxide/commit/749c3100d785f7ac373bdb109fda21f2ac62d5c0))
    - forbid symlinks and files in the path ([`de58f50`](https://github.com/Byron/gitoxide/commit/de58f50748bd70e39d29e503a7f4b1e6c9b20093))
    - avoid popping the entire cached path ([`a3501df`](https://github.com/Byron/gitoxide/commit/a3501df6eb8d2fd3176434c80c443316e91dabb6))
    - basic impl of the dir cache which already avoids unnecessary allocations ([`cb36d56`](https://github.com/Byron/gitoxide/commit/cb36d5691294971e1b0e097ed11908768283731a))
    - sketch out dir cache and realize that git uses chdir ([`f4621cc`](https://github.com/Byron/gitoxide/commit/f4621cc4dd48fcd4b1aba294c811bc92f2715981))
    - allow writing empty files during checkout but also query the odb ([`5388d80`](https://github.com/Byron/gitoxide/commit/5388d8091ef02cf927478a1492847ae1666040d4))
    - basic version of index checkout via command-line ([`f23b8d2`](https://github.com/Byron/gitoxide/commit/f23b8d2f1c4b767d337ec51888afaa8b3719798c))
    - basic progress reporting for checkout ([`039e822`](https://github.com/Byron/gitoxide/commit/039e822bb4e56e49432db5c53081e0eb39588d66))
    - support for unicode-precomposition for gix apps ([`e90c123`](https://github.com/Byron/gitoxide/commit/e90c123675a98ab62fc6bb22019f889cee8b7301))
    - fix symlink creation on windows, hopefully ([`4b1650b`](https://github.com/Byron/gitoxide/commit/4b1650ba1988f52a7a91ce4f5327eca350f32520))
    - gather more information about test failure on windows ([`be5e3fb`](https://github.com/Byron/gitoxide/commit/be5e3fb3a19f86e37244b17055bf31cc455e78e8))
    - hopefully fix symlink creation on windows ([`acb8acd`](https://github.com/Byron/gitoxide/commit/acb8acd905c4a7ec0fbc831b159f626962c0a37d))
    - refactor ([`48dc401`](https://github.com/Byron/gitoxide/commit/48dc40195fd3d41d1fa5cd6326422ae18266dd7d))
    - also validate symlink collisions ([`322c316`](https://github.com/Byron/gitoxide/commit/322c3161947cd5c10e3122c097d5a888726d42c1))
    - fix compile warnings ([`58145bc`](https://github.com/Byron/gitoxide/commit/58145bc0fc329c370638a336215679fa727a9f0f))
    - try to fix windows ([`5c1e727`](https://github.com/Byron/gitoxide/commit/5c1e727a1af4b9a0b5b7dcfca0d1ef5a533a66b6))
    - finally an understanding on collision checking ([`0454e4a`](https://github.com/Byron/gitoxide/commit/0454e4a6f039541255728c4c8e076578236f0d86))
    - Add check_stat and trust_ctime options to index checkout ([`1a502c7`](https://github.com/Byron/gitoxide/commit/1a502c7e456a191d8639b799648ea33eb5a7dac2))
    - validate that colliding files are checked out ([`09fecd9`](https://github.com/Byron/gitoxide/commit/09fecd9687cf3271f7138bca9214ba99c17b5ef7))
    - support for executable bit check ([`267e3a7`](https://github.com/Byron/gitoxide/commit/267e3a7f4718c8f724e3e4488dd24dcebfc69413))
    - probe precompose unicode ([`0c1c006`](https://github.com/Byron/gitoxide/commit/0c1c00689000dfc943ed25cd52eac42e3642a78c))
    - refactor ([`fc816bd`](https://github.com/Byron/gitoxide/commit/fc816bd12f142d1df4d10429ee5b56e9eb5fbf4d))
    - determine filesystem case ([`f8e1de0`](https://github.com/Byron/gitoxide/commit/f8e1de0dc031ad73084b2da6a6d39960b9b78b4b))
    - basic test for filesystem probing ([`adbed12`](https://github.com/Byron/gitoxide/commit/adbed121f969a05b622d0325b434b3c6d44ae248))
    - symlink probing ([`1bfbf1d`](https://github.com/Byron/gitoxide/commit/1bfbf1d120e31474367cd2008e1715c50af19071))
    - make clear that we are currently only dealing with checkout during clone ([`178beb4`](https://github.com/Byron/gitoxide/commit/178beb42eaf1112143299eafa7fc93106eb9fc5b))
    - refactor for checkout to use fs::Context ([`8914fcc`](https://github.com/Byron/gitoxide/commit/8914fcc114cdf920f2f4162e71d4d390007f6f3b))
    - document-features support for git-index and git-worktree ([`1367cf5`](https://github.com/Byron/gitoxide/commit/1367cf5bc5908639e67e12f78f57835c5fd68a90))
    - Support for 'serde1' feature in git-worktree ([`f11929c`](https://github.com/Byron/gitoxide/commit/f11929c9652b2f414029f2ad02dacee238a138d1))
    - sketch filesystem context, without probing for now ([`de3749e`](https://github.com/Byron/gitoxide/commit/de3749e1426d48a1d31a0ddc1fddfdb394a01078))
    - refactor ([`004394a`](https://github.com/Byron/gitoxide/commit/004394ad04a965b631c5d75a7eced632540d9e1e))
    - restructure tests ([`831c429`](https://github.com/Byron/gitoxide/commit/831c4294c87aae0594e1238177dd71efb997cbde))
    - make fmt ([`636fa8a`](https://github.com/Byron/gitoxide/commit/636fa8a97ce56982c76dffc64ee084e31d39afad))
    - strucural refactor ([`cdca1df`](https://github.com/Byron/gitoxide/commit/cdca1dfec590d24dd42f34294e21f4bdf61d36ad))
    - Allow mutation of entries during iteration, while obtaining their path ([`d0c4563`](https://github.com/Byron/gitoxide/commit/d0c4563f71ea18aaf8ae21dd8646ab256a550594))
    - refactor ([`72af261`](https://github.com/Byron/gitoxide/commit/72af261603ee38651e15015547871d0510ce6370))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - Fix build ([`f6d9693`](https://github.com/Byron/gitoxide/commit/f6d969370b8ef05b3b29983dcd9f6fa11d6225f2))
 * **[#333](https://github.com/Byron/gitoxide/issues/333)**
    - Use git_features::path everywhere where there is a path conversion ([`2e1437c`](https://github.com/Byron/gitoxide/commit/2e1437cb0b5dc77f2317881767f71eaf9b009ebf))
 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - update changelogs prior to release ([`746a676`](https://github.com/Byron/gitoxide/commit/746a676056cd4907da7137a00798344b5bdb4419))
 * **[#366](https://github.com/Byron/gitoxide/issues/366)**
    - the first possibly working version of loading a mailmap with multiple sources ([`98d745e`](https://github.com/Byron/gitoxide/commit/98d745e8080975a91cff1ce75e187258c851d3f4))
 * **Uncategorized**
    - Release git-diff v0.14.0, git-bitmap v0.1.0, git-index v0.2.0, git-tempfile v2.0.1, git-lock v2.0.0, git-mailmap v0.1.0, git-traverse v0.13.0, git-pack v0.17.0, git-quote v0.2.0, git-odb v0.27.0, git-packetline v0.12.4, git-url v0.4.0, git-transport v0.16.0, git-protocol v0.15.0, git-ref v0.12.0, git-worktree v0.1.0, git-repository v0.15.0, cargo-smart-release v0.9.0, safety bump 5 crates ([`e58dc30`](https://github.com/Byron/gitoxide/commit/e58dc3084cf17a9f618ae3a6554a7323e44428bf))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - make fmt ([`7cf3545`](https://github.com/Byron/gitoxide/commit/7cf354509b545f7e7c99e159b5989ddfbe86273d))
    - thanks clippy ([`07a4094`](https://github.com/Byron/gitoxide/commit/07a4094965ac1b4eb223da8e5ca5cc4a86c5f596))
    - thanks clippy ([`0e2a243`](https://github.com/Byron/gitoxide/commit/0e2a2438da35c0abb412682b103e5be171b1c3ad))
    - thanks clippy ([`3229240`](https://github.com/Byron/gitoxide/commit/322924037a1710f35e4134e5a35c82b3d4266a1f))
    - Merge branch 'svetli-n-path_value' ([`e8383ca`](https://github.com/Byron/gitoxide/commit/e8383caf6db211beb57d70019fe4ad13ce9066ee))
    - Merge branch 'unify-path-encoding' ([`566ff8a`](https://github.com/Byron/gitoxide/commit/566ff8a3597b889899d41ca15e5b9af7e05f1a4b))
    - thanks clippy ([`a8e9497`](https://github.com/Byron/gitoxide/commit/a8e9497caebf1c0e9faac537717cd86378f1acf6))
    - thanks clippy ([`e04cba8`](https://github.com/Byron/gitoxide/commit/e04cba8837340d1ca0f102a340e52e8610fb0750))
    - Merge branch 'AP2008-implement-worktree' ([`f32c669`](https://github.com/Byron/gitoxide/commit/f32c669bc519d59a1f1d90d61cc48a422c86aede))
    - Refactored code and tests ([`a4b880c`](https://github.com/Byron/gitoxide/commit/a4b880cf17665b61e3f7f193de57704b1db5318f))
    - Refactored tests ([`25a9dc1`](https://github.com/Byron/gitoxide/commit/25a9dc16dbb26e9aa0f3379b2af53cc0baa96663))
    - Reduce io calls ([`e838eaa`](https://github.com/Byron/gitoxide/commit/e838eaa5721d8b1b13155aa81234c9c44d9b15fe))
    - Refactor errors and remove unwraps ([`eaee855`](https://github.com/Byron/gitoxide/commit/eaee85595dc658549e62e3292b025ec016e70abd))
    - Implemented git-worktree ([`4177d72`](https://github.com/Byron/gitoxide/commit/4177d72c95bd94cf6a49e917dc21918044e8250b))
</details>

## 0.0.0 (2022-01-08)

Reserve the name for a necessary crate of the `gitoxide` project.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#293](https://github.com/Byron/gitoxide/issues/293)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - update changelog ([`b3ee7c6`](https://github.com/Byron/gitoxide/commit/b3ee7c6f7553de6bff4934bbdf38f6c6ea2cf349))
    - preempt the eventual need for a worktree implementation ([`bce67d8`](https://github.com/Byron/gitoxide/commit/bce67d8ec58f78a1fce1c76f7b93d9650f9f550e))
 * **Uncategorized**
    - Release git-worktree v0.0.0 ([`ddb1bf4`](https://github.com/Byron/gitoxide/commit/ddb1bf49e3b5b663fcf166d8cbce416e78d9fc18))
</details>

