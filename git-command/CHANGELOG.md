# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

The first usable release.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 22 commits contributed to the release over the course of 26 calendar days.
 - 26 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - Allow programs to communicate errors by default ([`5a2168e`](https://github.com/Byron/gitoxide/commit/5a2168e62f664d463fc8849efecccf7e90b382cd))
    - fix docs ([`f86364c`](https://github.com/Byron/gitoxide/commit/f86364c4e2d9efd04027978679232946494a4734))
    - fix CI ([`6565b97`](https://github.com/Byron/gitoxide/commit/6565b97d7d293ae881590960bf3e29f46fdb2cd1))
    - remove `allow prompt` builder method as typical prompt implementations don't need it ([`0236d75`](https://github.com/Byron/gitoxide/commit/0236d753805003d5a09505fab7da0b5b47392c45))
    - A builder method to allow prompts specifically ([`3be1fc7`](https://github.com/Byron/gitoxide/commit/3be1fc7d97f87893cecbe5d880576ab690bb205f))
    - Only actually use the shell if it appears to be required ([`830ee07`](https://github.com/Byron/gitoxide/commit/830ee07d943725e55a40a546b3a1b7ecefb75c4b))
    - support for multiple arguments with shell-script support ([`d8e8b54`](https://github.com/Byron/gitoxide/commit/d8e8b541bd776a267aca6dbfb8e7e793e264885b))
    - Squelch errors by default ([`1cb2e96`](https://github.com/Byron/gitoxide/commit/1cb2e967416b0fa5c6d32a0ad0b015b41f81e92c))
    - Add a way to transform a `Prepare` into a `Command` for even more flexibility ([`eeedd2c`](https://github.com/Byron/gitoxide/commit/eeedd2cab3c201109aa5bd986eb38c1f31d5fd20))
    - set version to 0.1 to avoid surprises like happened with `git-date` ([`1322f72`](https://github.com/Byron/gitoxide/commit/1322f72fd2bd310c1c3c859ee4b49f47cdfaf100))
    - add remaining docs ([`6a39e62`](https://github.com/Byron/gitoxide/commit/6a39e62bb4aebf9c48daddf007c95b2117b4454d))
    - basic support for 'sh' based execution ([`8c61b0b`](https://github.com/Byron/gitoxide/commit/8c61b0bded71dff223e24ae68f8cf7fc50195ce9))
    - First sketch of git-command API ([`cd4a608`](https://github.com/Byron/gitoxide/commit/cd4a608f0b8ef3adeb7a7f1979f653b63e77ad4d))
 * **Uncategorized**
    - Merge branch 'index-from-tree' ([`172f73c`](https://github.com/Byron/gitoxide/commit/172f73cf26878d153d51790fa01853fa4ba6beb7))
    - make fmt ([`535e967`](https://github.com/Byron/gitoxide/commit/535e967666c6da657ff1b7eff7c64ab27cafb182))
    - Merge branch 'main' into filter-refs-by-spec ([`9aa1d3d`](https://github.com/Byron/gitoxide/commit/9aa1d3dc46d4b1c76af257f573aff3aeef2d3fa8))
    - Merge branch 'main' into filter-refs-by-spec ([`1f6e5ab`](https://github.com/Byron/gitoxide/commit/1f6e5ab15f5fd8d23719b13e6aea59cd231ac0fe))
    - Merge branch 'main' into filter-refs-by-spec ([`51dc828`](https://github.com/Byron/gitoxide/commit/51dc8282fb77b519ff7d2c94c6bd73af306cfe8b))
    - Merge branch 'main' into filter-refs-by-spec ([`56ba481`](https://github.com/Byron/gitoxide/commit/56ba481f4c48f74f10397feb1b6dc3d7dd3704fb))
    - Merge branch 'main' into filter-refs-by-spec ([`a36c05d`](https://github.com/Byron/gitoxide/commit/a36c05d281269f3f8b297e7adc463bfb3c306663))
    - Merge branch 'main' into filter-refs-by-spec ([`cef0b51`](https://github.com/Byron/gitoxide/commit/cef0b51ade2a3301fa09ede7a425aa1fe3527e78))
    - thanks clippy ([`0dc1da5`](https://github.com/Byron/gitoxide/commit/0dc1da5e636b2eecc26fcfa0ecd814af3b78ed29))
</details>

## 0.0.0 (2022-08-25)

Initial release to reserve the name.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - prepare changelog prior to release ([`579e8f1`](https://github.com/Byron/gitoxide/commit/579e8f138963a057d87837301b097fd804424447))
    - first frame of `git-command` crate ([`436632a`](https://github.com/Byron/gitoxide/commit/436632a3822d3671c073cdbbbaf8e569de62bb09))
 * **Uncategorized**
    - Release git-command v0.0.0 ([`6c27e94`](https://github.com/Byron/gitoxide/commit/6c27e94c8ed6fb6155704a04d876ab6129b3b413))
</details>

