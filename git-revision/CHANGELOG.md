# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.2.0 (2022-05-18)

### Bug Fixes

 - <csr-id-99365f221065ebc315ac80940ad72cae253743bc/> Support for in truncated history in git-describe
   This allows `describe()` to work on shallow clones.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 34 calendar days.
 - 43 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#384](https://github.com/Byron/gitoxide/issues/384)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - Support for in truncated history in git-describe ([`99365f2`](https://github.com/Byron/gitoxide/commit/99365f221065ebc315ac80940ad72cae253743bc))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
 * **[#384](https://github.com/Byron/gitoxide/issues/384)**
    - No need to isolate archives by crate name ([`19d46f3`](https://github.com/Byron/gitoxide/commit/19d46f35440419b9911b6e2bca2cfc975865dce9))
    - add archive files via git-lfs ([`7202a1c`](https://github.com/Byron/gitoxide/commit/7202a1c4734ad904c026ee3e4e2143c0461d51a2))
    - Assure we don't pick up unnecessary files during publishing ([`545b2d5`](https://github.com/Byron/gitoxide/commit/545b2d5121ba64efaee7564d5191cec37661efd7))
    - auto-set commit.gpgsign=false when executing git ([`c23feb6`](https://github.com/Byron/gitoxide/commit/c23feb64ad157180cfba8a11c882b829733ea8f6))
</details>

## 0.1.0 (2022-04-05)

<csr-id-0a7776b8cce4c40c391f46542f6e7ba6830d6fc0/>

### Refactor (BREAKING)

 - <csr-id-0a7776b8cce4c40c391f46542f6e7ba6830d6fc0/> Make `describe::Format` more consistent with other builder APIs
   Configuration methods now take an argument which makes it more
   straightforward to use for most.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 43 commits contributed to the release over the course of 56 calendar days.
 - 59 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#364](https://github.com/Byron/gitoxide/issues/364)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - fix git-revision dependencies ([`c336b03`](https://github.com/Byron/gitoxide/commit/c336b033ae8d94d859a04f0a19f82aa5c4d760e0))
    - fix ordering of commits to actually be by commit-time, then topo-time ([`8286eac`](https://github.com/Byron/gitoxide/commit/8286eacfb791bac3449f84c9a2990aa13fba5b81))
    - support for the --max-candidates flag ([`b9e6754`](https://github.com/Byron/gitoxide/commit/b9e67540801f2630be8aa1acbfddfec4202360ac))
    - Reduce amount of max candidates, add --debug flag ([`c8c13e3`](https://github.com/Byron/gitoxide/commit/c8c13e398671a21e96282547fc0e3bd445627e2f))
    - Use hashed-hasher for an eek of performance ([`324a839`](https://github.com/Byron/gitoxide/commit/324a839e6c72174f08779a97fa12cc313e2afac2))
    - early-abort if all work is done during traversal ([`5b2aa70`](https://github.com/Byron/gitoxide/commit/5b2aa7015f4adc7cedd8f5b2715d611c2df02d98))
    - Make `describe::Format` more consistent with other builder APIs ([`0a7776b`](https://github.com/Byron/gitoxide/commit/0a7776b8cce4c40c391f46542f6e7ba6830d6fc0))
    - All documentation for the git-revision crate ([`8e0fb0a`](https://github.com/Byron/gitoxide/commit/8e0fb0a49630a1e3a67f174df4a22fdf224171c3))
    - support for 'first-parent' traversal ([`52eae32`](https://github.com/Byron/gitoxide/commit/52eae32a5393113595cc8970528c8e78d6ce0525))
    - support for fallbacks if no candidate available ([`39708a7`](https://github.com/Byron/gitoxide/commit/39708a7a53e8bd82a769a90049b1e706e021b7e1))
    - describe-format with support for 'always' display style ([`79f386d`](https://github.com/Byron/gitoxide/commit/79f386d6bcd65b30b319c6113dd3070c940cfebd))
    - finish depth computation works! ([`2e80e36`](https://github.com/Byron/gitoxide/commit/2e80e365000f924be84c9c60820758f4a0661c8d))
    - prepare for finish-computation impl ([`9e10c7a`](https://github.com/Byron/gitoxide/commit/9e10c7a5d1873d618cc268e59681f230c6338df8))
    - Prepare test for 'gave_up_on' to motivate implementing finish_computation() ([`966ec3f`](https://github.com/Byron/gitoxide/commit/966ec3fc2246f44a67d2b24d98d14e491767f162))
    - use thiserror instead of quickerror ([`7dcd2a5`](https://github.com/Byron/gitoxide/commit/7dcd2a5a65d1ac7d4370198951a495f2e00fccfe))
    - Use quickerror to handle all error branches ([`1243417`](https://github.com/Byron/gitoxide/commit/12434170130c716dbd9daceb3f0510fe63d342ce))
    - Some TODOs to not forget where to continue ([`84c0f15`](https://github.com/Byron/gitoxide/commit/84c0f1576cd295b014fc1bf6907e4b0674444b33))
    - git-describe complete formatting ([`eefa6c5`](https://github.com/Byron/gitoxide/commit/eefa6c51da2bafb6a6bcfb1a2fdb785b73cf919c))
    - frame for testing describe(), first sketch of signature with return value ([`5841f47`](https://github.com/Byron/gitoxide/commit/5841f473c01ebc667922f654885a14dc289d9844))
    - first failing test for describe() ([`23b1973`](https://github.com/Byron/gitoxide/commit/23b1973997cd68e94396c9f0ea21d7ae2138877a))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - make fmt ([`50ff7aa`](https://github.com/Byron/gitoxide/commit/50ff7aa7fa86e5e2a94fb15aab86470532ac3f51))
    - sort parents by most recent to find recent tags first ([`d240740`](https://github.com/Byron/gitoxide/commit/d240740cd24bdd8ded1d9048e2861b88476dbbe1))
    - refactor; first green tests ([`92a37ed`](https://github.com/Byron/gitoxide/commit/92a37edbc419a4b95cac62aae2627bed9ec2eaad))
    - no need for ordering by date, keep it simple ([`02909ea`](https://github.com/Byron/gitoxide/commit/02909ea7f39bd3fe0fdd361478fc665664d09377))
    - a step closer to the first successful test ([`710d46b`](https://github.com/Byron/gitoxide/commit/710d46beefc00f59f2d841170ddf46a410af7e85))
    - a step towards traversing the graph ([`48cba41`](https://github.com/Byron/gitoxide/commit/48cba41eb623be4e7d4a67d8f5a24940b5d82324))
    - refactor ([`e22e2dd`](https://github.com/Byron/gitoxide/commit/e22e2dd5b25913cdb15b09e97897e652e50a67d9))
    - the trivial part of the actual implementation ([`92a67a6`](https://github.com/Byron/gitoxide/commit/92a67a6eb58f1e31181fc10c9fcf34b56313058f))
 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - More speedy access to author/committer ([`6129607`](https://github.com/Byron/gitoxide/commit/61296077cebaaf2eb939fa6082121304bc6cf39b))
 * **Uncategorized**
    - Release git-revision v0.1.0, git-repository v0.16.0, gitoxide-core v0.14.0, gitoxide v0.12.0 ([`f041c00`](https://github.com/Byron/gitoxide/commit/f041c00a7df2455ca52fac7b83af1e9f335f5688))
    - Release git-config v0.2.1, git-diff v0.15.0, git-traverse v0.14.0, git-pack v0.18.0, git-odb v0.28.0, git-ref v0.12.1, git-revision v0.1.0, git-repository v0.16.0, gitoxide-core v0.14.0, gitoxide v0.12.0, safety bump 6 crates ([`b612021`](https://github.com/Byron/gitoxide/commit/b612021683ba709b693bd48aef3e2e3c2f5b9ead))
    - thanks clippy ([`4d4fda6`](https://github.com/Byron/gitoxide/commit/4d4fda68c67eb02ce2055707bc62a577ad3d7b78))
    - thanks clippy ([`f2faa00`](https://github.com/Byron/gitoxide/commit/f2faa001ed2c8e96e25dbd56544320055f8dbe1b))
    - thanks clippy ([`9f18dca`](https://github.com/Byron/gitoxide/commit/9f18dca5dfde3f24ce2e81d60beb343aa85d9cd6))
    - Merge branch 'for-onefetch' ([`8e5cb65`](https://github.com/Byron/gitoxide/commit/8e5cb65da75036a13ed469334e7ae6c527d9fff6))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - Merge branch 'svetli-n-refactor_git_config_tests' ([`babaa9f`](https://github.com/Byron/gitoxide/commit/babaa9f5725ab8cdf14e0c7e002c2e1de09de103))
    - make fmt ([`7cf3545`](https://github.com/Byron/gitoxide/commit/7cf354509b545f7e7c99e159b5989ddfbe86273d))
    - Remove serde support for describe types due to warning ([`2ba33c8`](https://github.com/Byron/gitoxide/commit/2ba33c89e723c7ec44ff8b5597718ee7792f462d))
    - thanks clippy ([`2c8a504`](https://github.com/Byron/gitoxide/commit/2c8a504c2b1a8309c3176e8c829e129c8dd39f80))
    - INTERMEDIATE RESET ME ([`a4de008`](https://github.com/Byron/gitoxide/commit/a4de008b88f892e95bf6da36d09b27190e9c5ede))
    - thanks clippy ([`f1ef59d`](https://github.com/Byron/gitoxide/commit/f1ef59d8129231554158fc51ab967b4f857c5e12))
    - Merge branch 'AP2008-implement-worktree' ([`f32c669`](https://github.com/Byron/gitoxide/commit/f32c669bc519d59a1f1d90d61cc48a422c86aede))
</details>

## 0.0.0 (2022-02-05)

Reserve the name for a necessary crate of the `gitoxide` project.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-revision v0.0.0 ([`8e434d8`](https://github.com/Byron/gitoxide/commit/8e434d8d0046e4479f0a575247ce3c9cce7e1f77))
    - Rename git-rev to git-revision ([`2e939c9`](https://github.com/Byron/gitoxide/commit/2e939c973ab3635a946317af08f37c4e23450f18))
</details>

