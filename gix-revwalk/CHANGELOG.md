# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Bug Fixes (BREAKING)

 - <csr-id-e9205679ab017699fd2605d4211d7ac2528dbc4b/> rename `PriorityQueue::pop()` to `::pop_value()` and add `::pop()` that also pops the key.
   This aligns the method name with `peek()`, which also pops the key.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge branch 'walk-with-commitgraph' ([`fdee9a2`](https://github.com/Byron/gitoxide/commit/fdee9a22873a13ae644d3dc92f8fe93f8f0266c0))
    - Rename `PriorityQueue::pop()` to `::pop_value()` and add `::pop()` that also pops the key. ([`e920567`](https://github.com/Byron/gitoxide/commit/e9205679ab017699fd2605d4211d7ac2528dbc4b))
    - Add new `gix-revwalk` crate for support types related to revision walking. ([`13ce887`](https://github.com/Byron/gitoxide/commit/13ce887682f5c31d1f78a63613ca97b811e4ffba))
</details>

