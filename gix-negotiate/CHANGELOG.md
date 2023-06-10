# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.2.1 (2023-06-10)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Prepare changelogs prior to release ([`298f3d7`](https://github.com/Byron/gitoxide/commit/298f3d7359c5b183314d8c584e45dcdd559d88b3))
    - Merge branch 'walk-with-commitgraph' ([`fdee9a2`](https://github.com/Byron/gitoxide/commit/fdee9a22873a13ae644d3dc92f8fe93f8f0266c0))
    - Adapt to changes in `gix-revwalk` ([`f7d95d1`](https://github.com/Byron/gitoxide/commit/f7d95d189af1422a7ba48db1857452e32e1d9db9))
</details>

## 0.2.0 (2023-06-06)

<csr-id-1571528f8779330aa1d077b1452aa00d9b419033/>

### New Features

 - <csr-id-1bd93bedd2f184510239c50c345d3dbc41d7d13b/> allow graph sharing by unifying `Flags` type.
   This makes the graph used in `gix-negotiate` shareable by callers, which can
   do their own traversal and store their own flags. The knowlege of this traversal
   can be kept using such shared flags, like the `PARSED` bit which should be set whenever
   parents are traversed.
   
   That way we are able to emulate the algorithms git uses perfectly, as we keep exactly the
   same state.
 - <csr-id-4aad40d6b6ddee0bc01b222cc2426c61c61d0b1a/> implement `skipping` negotiation algorithm
 - <csr-id-01aba9e92941240eefa898890f1b8b8d824db509/> implement `consecutive` algorithm.
   This is the default negotiation algorithm.
 - <csr-id-1f6e6d8aeb512b2afcd1911cf32e4f7e622bf73d/> introduce the `noop` negotiator to establish a basic trait for negotiators.

### Other

 - <csr-id-1571528f8779330aa1d077b1452aa00d9b419033/> try to change test-suite from --negotiate-only to the more realistic fetch with --dry-run.
   This means we will have to reproduce what git does naturally, to fill in common refs
   and also provide tips.
   
   Unfortunately this doesn't work as it's apparently not really dry-running, but modifying
   the repository underneath. This means it's not idempotent when running it multiple times.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 17 commits contributed to the release over the course of 17 calendar days.
 - 18 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.5.1, gix-hash v0.11.2, gix-features v0.30.0, gix-actor v0.21.0, gix-path v0.8.1, gix-glob v0.8.0, gix-quote v0.4.4, gix-attributes v0.13.0, gix-chunk v0.4.2, gix-commitgraph v0.16.0, gix-config-value v0.12.1, gix-fs v0.2.0, gix-tempfile v6.0.0, gix-utils v0.1.2, gix-lock v6.0.0, gix-validate v0.7.5, gix-object v0.30.0, gix-ref v0.30.0, gix-sec v0.8.1, gix-config v0.23.0, gix-command v0.2.5, gix-prompt v0.5.1, gix-url v0.19.0, gix-credentials v0.15.0, gix-diff v0.30.0, gix-discover v0.19.0, gix-hashtable v0.2.1, gix-ignore v0.3.0, gix-bitmap v0.2.4, gix-traverse v0.26.0, gix-index v0.17.0, gix-mailmap v0.13.0, gix-revision v0.15.0, gix-negotiate v0.2.0, gix-pack v0.36.0, gix-odb v0.46.0, gix-packetline v0.16.2, gix-transport v0.32.0, gix-protocol v0.33.0, gix-refspec v0.11.0, gix-worktree v0.18.0, gix v0.45.0, safety bump 29 crates ([`9a9fa96`](https://github.com/Byron/gitoxide/commit/9a9fa96fa8a722bddc5c3b2270b0edf8f6615141))
    - `just fmt` ([`ffc1276`](https://github.com/Byron/gitoxide/commit/ffc1276e0c991ac33ce842f5dca0b45ac69680c0))
    - Prepare changelogs prior to release ([`8f15cec`](https://github.com/Byron/gitoxide/commit/8f15cec1ec7d5a9d56bb158f155011ef2bb3539b))
    - Merge branch 'integrate-gix-negotiate' ([`ae845de`](https://github.com/Byron/gitoxide/commit/ae845dea6cee6523c88a23d7a14293589cf8092f))
    - Allow graph sharing by unifying `Flags` type. ([`1bd93be`](https://github.com/Byron/gitoxide/commit/1bd93bedd2f184510239c50c345d3dbc41d7d13b))
    - Merge branch 'main' into auto-clippy ([`3ef5c90`](https://github.com/Byron/gitoxide/commit/3ef5c90aebce23385815f1df674c1d28d58b4b0d))
    - Merge branch 'blinxen/main' ([`9375cd7`](https://github.com/Byron/gitoxide/commit/9375cd75b01aa22a0e2eed6305fe45fabfd6c1ac))
    - Include license files in all crates ([`facaaf6`](https://github.com/Byron/gitoxide/commit/facaaf633f01c857dcf2572c6dbe0a92b7105c1c))
    - Merge branch 'consecutive-negotiation' ([`97b3f7e`](https://github.com/Byron/gitoxide/commit/97b3f7e2eaddea20c98f2f7ab6a0d2e2117b0793))
    - Try to change test-suite from --negotiate-only to the more realistic fetch with --dry-run. ([`1571528`](https://github.com/Byron/gitoxide/commit/1571528f8779330aa1d077b1452aa00d9b419033))
    - Add a test to also validate interaction with known_common/remote refs ([`5bdd071`](https://github.com/Byron/gitoxide/commit/5bdd0716f359683060bab0f0695245a653bb6775))
    - Figure out what's wrong with 'skipping' and fix it ([`1b19ab1`](https://github.com/Byron/gitoxide/commit/1b19ab11c0928f26443d22ecfb6f211f4cdb5946))
    - Attempt to figure out what 'consecutive' needs to pass the tests ([`1809a99`](https://github.com/Byron/gitoxide/commit/1809a994c9d8a50bc73d283fd20ac825bfa6e92d))
    - Implement `skipping` negotiation algorithm ([`4aad40d`](https://github.com/Byron/gitoxide/commit/4aad40d6b6ddee0bc01b222cc2426c61c61d0b1a))
    - Implement `consecutive` algorithm. ([`01aba9e`](https://github.com/Byron/gitoxide/commit/01aba9e92941240eefa898890f1b8b8d824db509))
    - A baseline test for the noop negotiator ([`5cd7748`](https://github.com/Byron/gitoxide/commit/5cd7748279fd502f3651e37150f60a785f972a48))
    - Introduce the `noop` negotiator to establish a basic trait for negotiators. ([`1f6e6d8`](https://github.com/Byron/gitoxide/commit/1f6e6d8aeb512b2afcd1911cf32e4f7e622bf73d))
</details>

## v0.1.0 (2023-05-19)

Initial release with a single function to calculate the window size for `HAVE` lines.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-commitgraph v0.15.0, gix-revision v0.14.0, gix-negotiate v0.1.0, safety bump 7 crates ([`92832ca`](https://github.com/Byron/gitoxide/commit/92832ca2899cd2f222f4c7b1cc9e766178f55806))
    - Add new crate for implementing and testing git negotiation logic. ([`372ba09`](https://github.com/Byron/gitoxide/commit/372ba09bb00e3fab674f0251f697aab11c5559f8))
</details>

