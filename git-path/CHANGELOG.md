# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.2.0 (2022-06-13)

### New Features (BREAKING)

 - <csr-id-266d4379e9132fd7dd21e6c8fccb36e125069d6e/> Make `realpath()` easier to use by introducing `realpath_opt()`.
   That way there is consistency about how many symlinks to follow.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 20 calendar days.
 - 20 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - make fmt ([`c665aef`](https://github.com/Byron/gitoxide/commit/c665aef4270c5ee54da89ee015cc0affd6337608))
    - Merge branch 'main' into davidkna-envopen ([`bc0abc6`](https://github.com/Byron/gitoxide/commit/bc0abc643d3329f885f250b6880560dec861150f))
    - Make `realpath()` easier to use by introducing `realpath_opt()`. ([`266d437`](https://github.com/Byron/gitoxide/commit/266d4379e9132fd7dd21e6c8fccb36e125069d6e))
    - Merge branch 'davidkna-admin-sec' ([`3d0e2c2`](https://github.com/Byron/gitoxide/commit/3d0e2c2d4ebdbe3dff01846aac3375128353a2e1))
</details>

## 0.1.3 (2022-05-23)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-path v0.1.3, git-discover v0.1.2, git-repository v0.18.1, cargo-smart-release v0.10.1 ([`b7399cc`](https://github.com/Byron/gitoxide/commit/b7399cc44ee419355a649a7b0ba7b352cd48b400))
    - prepare for smart-release release ([`2f74cb0`](https://github.com/Byron/gitoxide/commit/2f74cb05e9b2399355af07517fe3c14e4e8724c5))
    - adjust git-path size limits ([`5ac8a3b`](https://github.com/Byron/gitoxide/commit/5ac8a3b58e0f61d4801a6f4dbd011f757208dbac))
    - Merge branch 'davidkna-discover-x-fs' ([`9abaeda`](https://github.com/Byron/gitoxide/commit/9abaeda2d22e2dbb1db1632c6eb637f1458d06e1))
</details>

## 0.1.2 (2022-05-21)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 2 calendar days.
 - 3 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#422](https://github.com/Byron/gitoxide/issues/422)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#422](https://github.com/Byron/gitoxide/issues/422)**
    - prepare changelog ([`de2d587`](https://github.com/Byron/gitoxide/commit/de2d5874b8d75c53165a9fc3ed35e2b37142bf52))
 * **Uncategorized**
    - Release git-path v0.1.2, git-sec v0.1.1, git-config v0.4.0, git-discover v0.1.1, git-pack v0.19.1, git-repository v0.18.0, cargo-smart-release v0.10.0, safety bump 2 crates ([`ceb6dff`](https://github.com/Byron/gitoxide/commit/ceb6dff13362a2b4318a551893217c1d11643b9f))
    - Remove forbid missing_docs ([`23acebb`](https://github.com/Byron/gitoxide/commit/23acebb8e9e53d89e7f629ab690253610358b0bb))
    - Merge branch 'main' into git_includeif ([`229d938`](https://github.com/Byron/gitoxide/commit/229d9383bef8844111d2bf3c406a2ea570109c8b))
    - declare `git-path` usable ([`496594d`](https://github.com/Byron/gitoxide/commit/496594d2d8b4216b51cfbd97805834c71c030c75))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
</details>

## 0.1.1 (2022-05-18)

<csr-id-e4f4c4b2c75a63a40a174e3a006ea64ef8d78809/>

### New Features

 - <csr-id-35f146a8573dcc9a1de3230373c0cf0794c6b897/> Add `absolutize_components()`
   It helps to cleanup paths a little which comes in handy when dealing
   with `commondir` appended paths.

### Other

 - <csr-id-e4f4c4b2c75a63a40a174e3a006ea64ef8d78809/> :discover()` now returns the shortest path.
   If and only if it canonicalized the source path. That way, users will
   still get a familiar path. This is due to `parent()` not operating
   in the file system, which otherwise would be equivalent to `..`,
   but that's not how we work.
   
   Maybe we should overhaul the way this works to use `../` instead
   and just 'absoluteize' the path later (std::path::absolute()) is
   on the way for that.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 49 commits contributed to the release over the course of 19 calendar days.
 - 20 days passed between releases.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#301](https://github.com/Byron/gitoxide/issues/301)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - `absolutize_*(dir)` is now `absolutize(dir, Option<cwd>)` ([`de87657`](https://github.com/Byron/gitoxide/commit/de87657194ad976cc73ebcc13c231537b35b4195))
    - More robust absolutize-paths implementation ([`4800ebe`](https://github.com/Byron/gitoxide/commit/4800ebec42f9bb6298cb5b2efdab71d6baf3b1ba))
    - Add `absolutize_components()` ([`35f146a`](https://github.com/Byron/gitoxide/commit/35f146a8573dcc9a1de3230373c0cf0794c6b897))
    - Allow reading patterns from stdin ([`0c597fe`](https://github.com/Byron/gitoxide/commit/0c597fe78acdd5672b4535a7d82620c5f7f93649))
    - :discover()` now returns the shortest path. ([`e4f4c4b`](https://github.com/Byron/gitoxide/commit/e4f4c4b2c75a63a40a174e3a006ea64ef8d78809))
    - Basic prefix support as well the first working version of `exclude query` ([`9cb8385`](https://github.com/Byron/gitoxide/commit/9cb83859f9bb76f38ab5bbd0ae6d6f20a691e9e1))
 * **Uncategorized**
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - make fmt ([`e043807`](https://github.com/Byron/gitoxide/commit/e043807abf364ca46d00760e2f281528efe20c75))
    - Fix create_symlink ([`714db70`](https://github.com/Byron/gitoxide/commit/714db70f02134c7f53dc7ba0461f43a0d6b659e9))
    - Add includeIf test with symlink. ([`5d74404`](https://github.com/Byron/gitoxide/commit/5d744049286632f3141ec07fa3f128093480d1c0))
    - Fix realpath tests. ([`0426f4d`](https://github.com/Byron/gitoxide/commit/0426f4deb5d73fd88529530f9a6d01ba55eeadc4))
    - Refactor real_path tests. ([`b696849`](https://github.com/Byron/gitoxide/commit/b696849e5fd210da397b0e7a3b26a63314d87607))
    - Refactor real_path tests. ([`8ade69f`](https://github.com/Byron/gitoxide/commit/8ade69fbddfa5d0be3bbe761210e49be647c3356))
    - Fix windows (probably) ([`c980014`](https://github.com/Byron/gitoxide/commit/c980014206ff071bc4f351416bb14995ac739e1b))
    - thanks clippy ([`da13aff`](https://github.com/Byron/gitoxide/commit/da13affabe34c3d691b18a70ce61eb00319668c5))
    - refactor ([`6bba054`](https://github.com/Byron/gitoxide/commit/6bba054a9a87219a7f94c155058fda5a3e6dffa6))
    - turn recursion into loop ([`9b83c2c`](https://github.com/Byron/gitoxide/commit/9b83c2c233d41034796694d000bed10d45f40c92))
    - refactor ([`1ca0540`](https://github.com/Byron/gitoxide/commit/1ca0540d170dcb8066a9141ce97631fcb9f2d5ae))
    - refactor ([`1f6ecd2`](https://github.com/Byron/gitoxide/commit/1f6ecd2ba91a34171d708ab7cb9414e853face95))
    - refactor ([`5efb972`](https://github.com/Byron/gitoxide/commit/5efb97251a9bf9e342d28bcbde27b0e69b0b7849))
    - refactor ([`353c245`](https://github.com/Byron/gitoxide/commit/353c2455dc01cf342b1186f0be263a87952b70be))
    - put `realpath` into its own module ([`d142e01`](https://github.com/Byron/gitoxide/commit/d142e01445ef545bd8284d3899d7e68f578943e9))
    - refactor ([`50583f0`](https://github.com/Byron/gitoxide/commit/50583f083be7ba890f7727a6491cbacf8b87ebe4))
    - rename `real_path()` to `realpath()` ([`478ff6c`](https://github.com/Byron/gitoxide/commit/478ff6caa630970847094fc11af10a6b69d72c34))
    - refactor ([`8f1daf5`](https://github.com/Byron/gitoxide/commit/8f1daf55c0027ec124bc6672ec545275065af9a7))
    - Fix linux test ([`8a36810`](https://github.com/Byron/gitoxide/commit/8a368102c89161006cad343839105d3a5ff284e2))
    - Fix windows test. ([`1afb2da`](https://github.com/Byron/gitoxide/commit/1afb2daa6704cc0c2efd9437dff5518ea3e64429))
    - Temp ignore real_path_tests. ([`c2f5db9`](https://github.com/Byron/gitoxide/commit/c2f5db9a3fcc7bdcdd84cdda30d970bdcedaff2a))
    - Windows absolute path. ([`8dc33cc`](https://github.com/Byron/gitoxide/commit/8dc33ccd1f5886b5e3f23eac5d6381473c386c2f))
    - Windows absolute path. ([`070f8c7`](https://github.com/Byron/gitoxide/commit/070f8c79a54141d3b3064622ac7b528a24875d4f))
    - Windows absolute path. ([`cefc8fb`](https://github.com/Byron/gitoxide/commit/cefc8fbfbb591fe714ffd87f39d0a7ca00e4a754))
    - Windows absolute path. ([`31a71f3`](https://github.com/Byron/gitoxide/commit/31a71f37d596a3a7a7279d4b6e2508c32383b2b6))
    - Fix merge. ([`f2b46df`](https://github.com/Byron/gitoxide/commit/f2b46dfbf73387d4501a7bf5039cb80ac4cb8d9c))
    - Merge branch 'main' into git_includeif ([`b1bfc8f`](https://github.com/Byron/gitoxide/commit/b1bfc8fe8efb6d8941f54dddd0fcad99aa13ed6c))
    - Add custom tempdir in. ([`8bfd52a`](https://github.com/Byron/gitoxide/commit/8bfd52a65fcecb33ae69917a67c48027f8fb3dff))
    - thanks clippy ([`a084951`](https://github.com/Byron/gitoxide/commit/a084951c72818d7cb2061053078793213890c899))
    - Temp ignore real_path_tests. ([`27f4bfc`](https://github.com/Byron/gitoxide/commit/27f4bfcb2fba45bd02d1977094acb31b7b989cac))
    - Windows fix. ([`ce0b408`](https://github.com/Byron/gitoxide/commit/ce0b408fcdeae80d6c9263955f70a00ead3841e1))
    - Windows fix. ([`25dd319`](https://github.com/Byron/gitoxide/commit/25dd319a2b46327fb553f824619311484726c742))
    - Windows fix. ([`61bc0e7`](https://github.com/Byron/gitoxide/commit/61bc0e776b9b02fdd36df6c0f54aecae63bf5895))
    - Merge branch 'main' into git_includeif ([`05eb340`](https://github.com/Byron/gitoxide/commit/05eb34023933918c51c03cf2afd774db89cc5a33))
    - Handle windows path prefix. ([`1723236`](https://github.com/Byron/gitoxide/commit/1723236377db483b09f123a5c24c949afa285b8a))
    - Max symlinks exceeded test. ([`cfff300`](https://github.com/Byron/gitoxide/commit/cfff30075d87045bf9def697c417a3eb46b4b215))
    - Use thiserror in `real_path()` ([`2bd7a44`](https://github.com/Byron/gitoxide/commit/2bd7a441beb7e0a86169ec89ca56a8ba448fbf2b))
    - input_path is Iterator. ([`c993d78`](https://github.com/Byron/gitoxide/commit/c993d7826fcf76ddaddffca619b4d35555b6636c))
    - real_path wip ([`3890a61`](https://github.com/Byron/gitoxide/commit/3890a6149683663b16dccdc3b50e2aab7eb4e048))
    - make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - Merge branch 'git_includeif' of https://github.com/svetli-n/gitoxide into svetli-n-git_includeif ([`0e01da7`](https://github.com/Byron/gitoxide/commit/0e01da74dffedaa46190db6a7b60a2aaff190d81))
</details>

## 0.1.0 (2022-04-28)

<csr-id-54801592488416ef2bb0f34c5061b62189c35c5e/>

### Refactor (BREAKING)

 - <csr-id-54801592488416ef2bb0f34c5061b62189c35c5e/> various name changes for more convenient API

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 1 calendar day.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#301](https://github.com/Byron/gitoxide/issues/301)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - frame for `gix repo exclude query` ([`a331314`](https://github.com/Byron/gitoxide/commit/a331314758629a93ba036245a5dd03cf4109dc52))
    - refactor ([`21d4076`](https://github.com/Byron/gitoxide/commit/21d407638285b728d0c64fabf2abe0e1948e9bec))
    - The first indication that directory-based excludes work ([`e868acc`](https://github.com/Byron/gitoxide/commit/e868acce2e7c3e2501497bf630e3a54f349ad38e))
    - various name changes for more convenient API ([`5480159`](https://github.com/Byron/gitoxide/commit/54801592488416ef2bb0f34c5061b62189c35c5e))
    - Use bstr intead of [u8] ([`9380e99`](https://github.com/Byron/gitoxide/commit/9380e9990065897e318b040f49b3c9a6de8bebb1))
    - Use `git-path` crate instead of `git_features::path` ([`47e607d`](https://github.com/Byron/gitoxide/commit/47e607dc256a43a3411406c645eb7ff04239dd3a))
    - Copy all existing functions from git-features::path to git-path:: ([`725e198`](https://github.com/Byron/gitoxide/commit/725e1985dc521d01ff9e1e89b6468ef62fc09656))
    - add empty git-path crate ([`8d13f81`](https://github.com/Byron/gitoxide/commit/8d13f81068b4663d322002a9617d39b307b63469))
 * **Uncategorized**
    - Release git-path v0.1.0 ([`ca019fc`](https://github.com/Byron/gitoxide/commit/ca019fca03c4ea0d70fabbf09808732925b58077))
</details>

## 0.0.0 (2022-03-31)

An empty crate without any content to reserve the name for the gitoxide project.

