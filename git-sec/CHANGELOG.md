# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Other

 - <csr-id-136eb37b00c9e7ba0fd0bc2a75dee2ac1b06516d/> adopt git-for-windows exception rules

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 5 calendar days.
 - 5 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#426](https://github.com/Byron/gitoxide/issues/426)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#426](https://github.com/Byron/gitoxide/issues/426)**
    - Assure windows home path is compared in absolute terms ([`e0b7bf1`](https://github.com/Byron/gitoxide/commit/e0b7bf18234efa5e43fe6d16ec88fc1894472b27))
 * **Uncategorized**
    - adopt git-for-windows exception rules ([`136eb37`](https://github.com/Byron/gitoxide/commit/136eb37b00c9e7ba0fd0bc2a75dee2ac1b06516d))
    - Merge branch 'davidkna-discover-x-fs' ([`9abaeda`](https://github.com/Byron/gitoxide/commit/9abaeda2d22e2dbb1db1632c6eb637f1458d06e1))
</details>

## 0.1.1 (2022-05-21)

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
    - Merge branch 'svetli-n-git_includeif' ([`cf24fbe`](https://github.com/Byron/gitoxide/commit/cf24fbe4b62d67b06138243d470dcc1805ebd55b))
    - adjust size limits ([`da6130d`](https://github.com/Byron/gitoxide/commit/da6130db9d39d2be3ad2dfbc63c82fbbb82ba07e))
    - Merge branch 'main' into git_includeif ([`229d938`](https://github.com/Byron/gitoxide/commit/229d9383bef8844111d2bf3c406a2ea570109c8b))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
</details>

## 0.1.0 (2022-05-18)

### New Features

 - <csr-id-95577e20d5e62cb6043d32f6a7b9023d827b9ce4/> A shared `permission::Error` type
 - <csr-id-de0226ab863f3d5d6688f1b89aa3ebc9bfdf1f34/> `permission::Error`
   A lightweight, general purpose error to display permissions violations
   that cause errors. This should make it useable across crates.
 - <csr-id-f6077978fd5697bd113a894ba68492213becea41/> obtain identities `from_path()` or `from_process()`
 - <csr-id-cdf3c3e42433a85e8b47b9dc5558f5c76df3c6ae/> add `Identity` type

### Changed (BREAKING)

 - <csr-id-f00f4a4a3a9149bf5cf925e931a8105aeb9b9db9/> simplify `Permission` type radically `
 - <csr-id-37a607db7c09ab897f306e3bbd4e0ca4e4387bae/> remove `Identity` in favor of `identity::Account` module; add `identity::UserId`
   As the fewest consumers will be able to deal with multiple identities,
   remove the enumeration approach in favor of individual type which deal
   with one specific way of identifying a user.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 33 commits contributed to the release over the course of 33 calendar days.
 - 33 days passed between releases.
 - 6 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#386](https://github.com/Byron/gitoxide/issues/386)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - upgrade dependencies ([`b039d39`](https://github.com/Byron/gitoxide/commit/b039d39613bb14d49670c4d8b586f76ffb420d03))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - initial refactoring ([`43a34a5`](https://github.com/Byron/gitoxide/commit/43a34a5bdae53fbb53d3ae095f03c9456115a013))
    - fix build ([`cb1c80f`](https://github.com/Byron/gitoxide/commit/cb1c80f8343691600797b61c61cba9cef82a59fc))
    - A shared `permission::Error` type ([`95577e2`](https://github.com/Byron/gitoxide/commit/95577e20d5e62cb6043d32f6a7b9023d827b9ce4))
    - `permission::Error` ([`de0226a`](https://github.com/Byron/gitoxide/commit/de0226ab863f3d5d6688f1b89aa3ebc9bfdf1f34))
 * **[#386](https://github.com/Byron/gitoxide/issues/386)**
    - Use strict ownership semantics on windows as well ([`84023cb`](https://github.com/Byron/gitoxide/commit/84023cbe7dc2e0d79aadd0863122af829e25bbba))
    - simplify `Permission` type radically ` ([`f00f4a4`](https://github.com/Byron/gitoxide/commit/f00f4a4a3a9149bf5cf925e931a8105aeb9b9db9))
    - refactor ([`b9e307b`](https://github.com/Byron/gitoxide/commit/b9e307bc9aea52459450c22f398f078f81aeb825))
    - more expressive and fuiture-proof handling of git dir access controls ([`b1d319b`](https://github.com/Byron/gitoxide/commit/b1d319b249fb6c6d4d5197734938836824789053))
    - A first PoC to show how the permissions model works in practice ([`67d5837`](https://github.com/Byron/gitoxide/commit/67d58372a8352da0197ec2992f120bd000ffe5de))
    - fully typed access control with tagged permissions ([`a43e25b`](https://github.com/Byron/gitoxide/commit/a43e25b2be744a46f2a73690f3cdd2440c3e1070))
    - refactor ([`0e74c71`](https://github.com/Byron/gitoxide/commit/0e74c7198607e2d44c0fab5a91789821d58ac9dc))
    - abstractions which should be powerful enough to handle our use-cases ([`b0d06ca`](https://github.com/Byron/gitoxide/commit/b0d06ca108c7f3f7078a8f00f62edc2011231581))
    - more details for path permissions ([`ca26659`](https://github.com/Byron/gitoxide/commit/ca26659eb870c8e947962fe0647a07d01b3e95e4))
    - a sketch on how to deal with permissions for executables ([`c066069`](https://github.com/Byron/gitoxide/commit/c06606991babd947f24e6d934b66b04f62dff1a9))
    - refactor ([`9a3f0ba`](https://github.com/Byron/gitoxide/commit/9a3f0ba8277d92eb75129931993bddbd9961ccdd))
    - See if checking for membership instead works ([`de5ff1b`](https://github.com/Byron/gitoxide/commit/de5ff1b5b0d0ba59fa10ec85ed849ed8e1f85f62))
    - see if this makes a difference on windows ([`0dac74e`](https://github.com/Byron/gitoxide/commit/0dac74e83fd8da00fc54765f22b0557f400e08c2))
    - refactor so that the windows implementation can happen ([`7bbe44c`](https://github.com/Byron/gitoxide/commit/7bbe44c979bd5ab7077206b6bb3adb1172030a3e))
    - refactor ([`a58d2cf`](https://github.com/Byron/gitoxide/commit/a58d2cf39b47e7a2c69ba639923bbece19f28230))
    - obtain identities `from_path()` or `from_process()` ([`f607797`](https://github.com/Byron/gitoxide/commit/f6077978fd5697bd113a894ba68492213becea41))
    - remove `Identity` in favor of `identity::Account` module; add `identity::UserId` ([`37a607d`](https://github.com/Byron/gitoxide/commit/37a607db7c09ab897f306e3bbd4e0ca4e4387bae))
    - fix installation test on windows ([`5cf8c27`](https://github.com/Byron/gitoxide/commit/5cf8c2769dd7b0d8a9ee0e304f255ae124524261))
    - add `Identity` type ([`cdf3c3e`](https://github.com/Byron/gitoxide/commit/cdf3c3e42433a85e8b47b9dc5558f5c76df3c6ae))
 * **Uncategorized**
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Merge branch 'main' into git_includeif ([`b1bfc8f`](https://github.com/Byron/gitoxide/commit/b1bfc8fe8efb6d8941f54dddd0fcad99aa13ed6c))
    - Merge branch 'main' into git_includeif ([`05eb340`](https://github.com/Byron/gitoxide/commit/05eb34023933918c51c03cf2afd774db89cc5a33))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - Merge branch 'git_includeif' of https://github.com/svetli-n/gitoxide into svetli-n-git_includeif ([`0e01da7`](https://github.com/Byron/gitoxide/commit/0e01da74dffedaa46190db6a7b60a2aaff190d81))
    - thanks clippy ([`f802a03`](https://github.com/Byron/gitoxide/commit/f802a03dc0b04d12fa360fb570d460ad4e1eb53a))
    - Merge branch 'main' into git-sec ([`2fe70f9`](https://github.com/Byron/gitoxide/commit/2fe70f96cfb68e108637ce78f8edda2eb4e2e61a))
</details>

## 0.0.0 (2022-04-15)

An empty crate without any content to reserve the name for the gitoxide project.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#386](https://github.com/Byron/gitoxide/issues/386)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#386](https://github.com/Byron/gitoxide/issues/386)**
    - An empty crate for git-sec ([`96a922c`](https://github.com/Byron/gitoxide/commit/96a922c4c9be194aaa4928fb21c9690a5c6e4445))
 * **Uncategorized**
    - Release git-sec v0.0.0 ([`07efb6f`](https://github.com/Byron/gitoxide/commit/07efb6ff2dfdc03c1867d1bd1fc1350cee134d16))
</details>

