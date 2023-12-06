# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.0 (2023-12-06)

### New Features

 - <csr-id-8f795e8abf706a24fe104500bf15efaa2bc07b15/> add basic connectivity check
   Implement a simple connectivity check in a new `gix-fsck` crate, and add
   this to `gix` via a new `fsck` subcommand. Currently this is
   functionally equivalent to:
   `git rev-list --objects --quiet --missing=print`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 26 calendar days.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - J fmt ([`51c7abc`](https://github.com/Byron/gitoxide/commit/51c7abc65f368b1b2bd3d82473793d3cd4fcbad5))
    - Merge branch 'fix-1096' ([`ff99a18`](https://github.com/Byron/gitoxide/commit/ff99a18e9f9388542a9cbf17d61b413f34b1d533))
    - Adapt to changes in `gix-object` ([`203d69c`](https://github.com/Byron/gitoxide/commit/203d69c8890acc716bd4f7a7b1b2b91a8c828bde))
    - Merge branch 'feat_basic_connectivity_check' ([`1f9aca5`](https://github.com/Byron/gitoxide/commit/1f9aca5de45c1f7c25606cd1ddc6b93a915dcd77))
    - Do not recurse into trees during fsck ([`9c1830c`](https://github.com/Byron/gitoxide/commit/9c1830c73557f71d930c918094e2d2e9a9ba3e2d))
    - Refactor ([`7a88b42`](https://github.com/Byron/gitoxide/commit/7a88b420f0a43fb1ce163698723b30566add97a9))
    - Add basic connectivity check ([`8f795e8`](https://github.com/Byron/gitoxide/commit/8f795e8abf706a24fe104500bf15efaa2bc07b15))
</details>

