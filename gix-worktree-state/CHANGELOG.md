# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Chore

 - <csr-id-93feea269eebd114e866e6f29f4a73c0096df9e0/> split tests off into their own crate to allow feature toggles.
   That way we can test with the `parallel` feature and won't have to
   create bogus feature toggles that are only used for testing, yet visbible
   to users.

### New Features

 - <csr-id-fc0529eaa805e696f7297ba8cf0179c5fac7c677/> `checkout()` now creates empty directories for submodules.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 2 calendar days.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - `checkout()` now creates empty directories for submodules. ([`fc0529e`](https://github.com/Byron/gitoxide/commit/fc0529eaa805e696f7297ba8cf0179c5fac7c677))
    - More cleanup of test crates ([`73c685a`](https://github.com/Byron/gitoxide/commit/73c685a67debcfa26a940f37bbca69cb3a4af57e))
    - Split tests off into their own crate to allow feature toggles. ([`93feea2`](https://github.com/Byron/gitoxide/commit/93feea269eebd114e866e6f29f4a73c0096df9e0))
    - Just fmt ([`0d258f4`](https://github.com/Byron/gitoxide/commit/0d258f40afcd848509e2b0c7c264e9f346ed1726))
    - Merge pull request #994 from bittrance/debug-checkout-outcome ([`518f9b1`](https://github.com/Byron/gitoxide/commit/518f9b1e38dc40b2874e543c4f4dad3bf9f73ee6))
    - Checkout outcome now implements debug. ([`995545c`](https://github.com/Byron/gitoxide/commit/995545c8f76f303452541ad6e098eb5dd0912b57))
    - Merge branch 'worktree-organization' ([`8d0d8e0`](https://github.com/Byron/gitoxide/commit/8d0d8e005d7f11924a6717954d892aae5cec45e7))
    - Adapt to changes in `gix-worktree` ([`e5717e1`](https://github.com/Byron/gitoxide/commit/e5717e1d12c49285d31a90b03b7f8e9cbc6c1108))
    - Move worktree- checkout functionality into its own crate ([`bd961b3`](https://github.com/Byron/gitoxide/commit/bd961b3065ca71ac4fa59e9988a3b7e705cd4c67))
</details>

