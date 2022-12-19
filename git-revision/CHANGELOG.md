# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.8.0 (2022-12-19)

### Reverted (BREAKING)

 - <csr-id-2761466ef6734ad6484548d7a93e52db3a230864/> hash_hasher re-export in favor of using `git-hashtable`.
   Due to the importance of best-suited data structures for maximizing
   performance we need to take control over them. This is best done using
   a dedicated crate that can cater to our very needs. That very crate is
   named `git-hashtable`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 27 calendar days.
 - 27 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
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
    - Merge branch 'optimize_hashtables' ([`95ad56c`](https://github.com/Byron/gitoxide/commit/95ad56c11489bc46d6eb2b2f48cf0bf01e954c58))
    - hash_hasher re-export in favor of using `git-hashtable`. ([`2761466`](https://github.com/Byron/gitoxide/commit/2761466ef6734ad6484548d7a93e52db3a230864))
    - use newly added git-hashtable ([`50cb436`](https://github.com/Byron/gitoxide/commit/50cb4362010e1a5799fe782df36ac5fcdb48dd8a))
    - switch to custom Hasher implementation ([`269d59e`](https://github.com/Byron/gitoxide/commit/269d59e0bee1f072096667b143800a0d85b18403))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
</details>

## 0.7.0 (2022-11-21)

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
 - 42 days passed between releases.
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

## 0.6.0 (2022-10-10)

Maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 20 calendar days.
 - 20 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
</details>

## 0.5.0 (2022-09-20)

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 22 calendar days.
 - 24 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
    - adjust to deal with changes to git-repository ([`b99b6bf`](https://github.com/Byron/gitoxide/commit/b99b6bfea47a4485496c2eb565693a6a53efe166))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **Uncategorized**
    - Release git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0 ([`f5c36d8`](https://github.com/Byron/gitoxide/commit/f5c36d85755d1f0f503b77d9a565fad6aecf6728))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - make fmt ([`535e967`](https://github.com/Byron/gitoxide/commit/535e967666c6da657ff1b7eff7c64ab27cafb182))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - Merge branch 'main' into filter-refs-by-spec ([`1f6e5ab`](https://github.com/Byron/gitoxide/commit/1f6e5ab15f5fd8d23719b13e6aea59cd231ac0fe))
    - Merge branch 'git_date_parse' ([`75591fb`](https://github.com/Byron/gitoxide/commit/75591fb108ce440ba2f920bebf99158b407e3046))
    - refactor ([`e1a1406`](https://github.com/Byron/gitoxide/commit/e1a1406183ae4feadad7a91925144e62cd1592c3))
    - refactor  - don't degenerate error ([`976b31f`](https://github.com/Byron/gitoxide/commit/976b31f81c830facf6386ad8ae43867c57af77e2))
    - Merge branch 'fix-522' ([`5869e9f`](https://github.com/Byron/gitoxide/commit/5869e9ff2508d5a93c07635277af8764fcb57713))
    - Release git-hash v0.9.9 ([`da0716f`](https://github.com/Byron/gitoxide/commit/da0716f8c27b4f29cfff0e5ce7fcb3d7240f4aeb))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - `parse` is pure function. ([`9ad1a5f`](https://github.com/Byron/gitoxide/commit/9ad1a5fa2ce54e978396ff3eaa7061a8edd10d4a))
    - `parse()` returns Result. ([`206f392`](https://github.com/Byron/gitoxide/commit/206f3923f5da2e9e26677e917550e6e5baa2913a))
    - Merge branch 'main' into filter-refs-by-spec ([`cef0b51`](https://github.com/Byron/gitoxide/commit/cef0b51ade2a3301fa09ede7a425aa1fe3527e78))
</details>

## 0.4.4 (2022-08-27)

### Bug Fixes

 - <csr-id-4788270853d42be8405465a6b9b612783ae9ef6e/> decscribe() won't abort before the first name check if max-candidates is 0
   A test was missing too, which is now fixed.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 2 calendar days.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#503](https://github.com/Byron/gitoxide/issues/503)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#503](https://github.com/Byron/gitoxide/issues/503)**
    - prepare changelog ([`3c99e7f`](https://github.com/Byron/gitoxide/commit/3c99e7f02ada72a171856ffc5b870da83fffc703))
    - decscribe() won't abort before the first name check if max-candidates is 0 ([`4788270`](https://github.com/Byron/gitoxide/commit/4788270853d42be8405465a6b9b612783ae9ef6e))
 * **Uncategorized**
    - Release git-features v0.22.3, git-revision v0.4.4 ([`c2660e2`](https://github.com/Byron/gitoxide/commit/c2660e2503323531ba02519eaa51124ee22fec51))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Merge branch 'fix-ci-installation' ([`9245083`](https://github.com/Byron/gitoxide/commit/92450839621a4d99cb22d08cbf9f9a89ff6b9e3f))
</details>

## 0.4.3 (2022-08-24)

A maintenance release without user facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.1.0, git-actor v0.11.4, git-revision v0.4.3, git-repository v0.22.1, cargo-smart-release v0.11.0, git-commitgraph v0.8.2, gitoxide-core v0.17.0, gitoxide v0.15.0 ([`1fb931a`](https://github.com/Byron/gitoxide/commit/1fb931a7ea59f1cf895a6c1392fd8615b723c743))
    - update changelogs prior to release ([`23cb58f`](https://github.com/Byron/gitoxide/commit/23cb58f02043e0e5027136fd6e8e724c03a2efbe))
    - adjust to new version of git-date ([`b3fe26b`](https://github.com/Byron/gitoxide/commit/b3fe26bf03db7e1babb5ffbc89d71bf9614e3df3))
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
 - <csr-id-df62f5081291f65f994b2aa66f0599f47eea8d4d/> `describe()` aborts search early if there is no input name in the name map.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 4 calendar days.
 - 4 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
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
    - Merge branch 'main' into remote-ls-refs ([`c82bbfa`](https://github.com/Byron/gitoxide/commit/c82bbfaddc45bf9b5b55f056613046d977d9ef09))
    - `describe()` aborts search early if there is no input name in the name map. ([`df62f50`](https://github.com/Byron/gitoxide/commit/df62f5081291f65f994b2aa66f0599f47eea8d4d))
</details>

## 0.4.1 (2022-08-19)

### New Features

 - <csr-id-ca6651234a8c0d4718554323b197a49266b60a61/> revision describe can now short-cut what effectively is only a name-to-id lookup
   This makes situations easier where `max-candidates` is provided by the user or by
   configuration.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.0.4, git-actor v0.11.2, git-revision v0.4.1, git-repository v0.21.1 ([`2f9dc84`](https://github.com/Byron/gitoxide/commit/2f9dc847e0d54f4181ce35ddadd9286ba80ca01f))
    - prepare for release of git-repository ([`8aa5389`](https://github.com/Byron/gitoxide/commit/8aa5389d5a1bdd3a07f1caa1c2f55c8af4f9844a))
    - revision describe can now short-cut what effectively is only a name-to-id lookup ([`ca66512`](https://github.com/Byron/gitoxide/commit/ca6651234a8c0d4718554323b197a49266b60a61))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
</details>

## 0.4.0 (2022-08-17)

### New Features

 - <csr-id-36c70e1f4ce07bf69d7064de1b6f0516d13d8acf/> `Spec` with `Display` implementation to reproduce itself.
   That way it can be parsed back perfectly after displaying itself, and
   will work normally when used in backticks in the shell for simple
   include patterns.
 - <csr-id-5038ffab6a0f83e0566f99e3c92ae2dea266e10b/> Add `Spec` data strcuture to fully represent a revision specification
 - <csr-id-4bb200300b1665cab49b780ae13c277630b70f51/> Add support for `r1^@`
 - <csr-id-7e5d31cb253f994ef19b15978c5df0f3a7ccebb1/> Add support for `r1^!`
 - <csr-id-fa1615da63594acbe92c3c4a13e2aeb7c1ee1d94/> support for `<rev>^-<n>` and `<rev>^-`

### Changed (BREAKING)

 - <csr-id-42aea42c1f6c2a9681688825a9e31966bca1896c/> More intuitive variants for `Spec`.
 - <csr-id-487941ce557182c7ad02958e011959acb2dd5607/> rename various `Kind` variants to be more descrptive.
 - <csr-id-baf34c486f54e4699f88b06a0f8cbb10f0582bd0/> Rename `Kind::Single` to `Include` and add `Exclude` kind.
   So far I got ranges pretty wrong and was degenerating the `^rev` case
   due to misinterpretation of the docs.
   This summary corrected that: https://git-scm.com/docs/git-rev-parse#_revision_range_summary

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 42 commits contributed to the release over the course of 24 calendar days.
 - 26 days passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#427](https://github.com/Byron/gitoxide/issues/427), [#450](https://github.com/Byron/gitoxide/issues/450)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - make fmt ([`4b320e7`](https://github.com/Byron/gitoxide/commit/4b320e773368ac5e8c38dd8a779ef3d6d2d024ec))
    - remove unused type ([`ad3475d`](https://github.com/Byron/gitoxide/commit/ad3475d473109649eb904786db7847a4e61d0e89))
    - Better docs for `Spec` and `spec::Kind` ([`6b76c06`](https://github.com/Byron/gitoxide/commit/6b76c06c1e9e2317f6ee1ff26c3cc57c46ec0b69))
    - More intuitive variants for `Spec`. ([`42aea42`](https://github.com/Byron/gitoxide/commit/42aea42c1f6c2a9681688825a9e31966bca1896c))
    - `Spec` with `Display` implementation to reproduce itself. ([`36c70e1`](https://github.com/Byron/gitoxide/commit/36c70e1f4ce07bf69d7064de1b6f0516d13d8acf))
    - More fuzz success ([`f239796`](https://github.com/Byron/gitoxide/commit/f239796aaffce59eb30527dc3635356ca0bab699))
    - fix panics discovered by fuzzer input ([`0f9e959`](https://github.com/Byron/gitoxide/commit/0f9e959a98d7a15ad2b0eeeea8e21bde89ed6a42))
    - Add fuzz target ([`54108f4`](https://github.com/Byron/gitoxide/commit/54108f4e00155e96a450daace6721f174743026c))
    - Add support for `r1^@` ([`4bb2003`](https://github.com/Byron/gitoxide/commit/4bb200300b1665cab49b780ae13c277630b70f51))
    - Add support for `r1^!` ([`7e5d31c`](https://github.com/Byron/gitoxide/commit/7e5d31cb253f994ef19b15978c5df0f3a7ccebb1))
    - refactor ([`dd1a208`](https://github.com/Byron/gitoxide/commit/dd1a20824c43ab55cd8ab260a2fa381b276146f1))
    - the first test for @^! syntax ([`b97677c`](https://github.com/Byron/gitoxide/commit/b97677cecb5efa01445769ba10835ba4d8d263e5))
    - rename various `Kind` variants to be more descrptive. ([`487941c`](https://github.com/Byron/gitoxide/commit/487941ce557182c7ad02958e011959acb2dd5607))
    - Add all remainiing rev-spec kinds. ([`fcc737d`](https://github.com/Byron/gitoxide/commit/fcc737dbca587747bb9ba1d4b3376b5e455177c9))
    - Assure parsing ends after special syntax sugar ([`661bf29`](https://github.com/Byron/gitoxide/commit/661bf2992baf184224c16ca80172a132bee9129a))
    - support for `<rev>^-<n>` and `<rev>^-` ([`fa1615d`](https://github.com/Byron/gitoxide/commit/fa1615da63594acbe92c3c4a13e2aeb7c1ee1d94))
    - a way to intercept which ref or prefix was set ([`b7a823b`](https://github.com/Byron/gitoxide/commit/b7a823b246b6c10c5a191bde22a88678909ff4fd))
    - first steps toward implementing ^-n ([`4b105f8`](https://github.com/Byron/gitoxide/commit/4b105f88a1429108653238e7407fd3829af939c5))
    - tests for `r1^-`  and `r1^-n` syntactic sugar ([`5d983c6`](https://github.com/Byron/gitoxide/commit/5d983c631172a87fba646d62cc102a80ab7da17f))
    - Adjust RevSpec::range() to match changes in `git-revision` ([`05ea453`](https://github.com/Byron/gitoxide/commit/05ea45337e85583db5e57f14e995be49ba888ee1))
    - Omitted revisions after or before ../... are automatically defaulted to `HEAD`. ([`d6f481d`](https://github.com/Byron/gitoxide/commit/d6f481d0eee39d5b7e8ad7885f52b07ea876388e))
    - make it possible to see the ordering of calls ([`b04614c`](https://github.com/Byron/gitoxide/commit/b04614c8bdf85c4f8025daeba6d6b0794699104b))
    - Rename `Kind::Single` to `Include` and add `Exclude` kind. ([`baf34c4`](https://github.com/Byron/gitoxide/commit/baf34c486f54e4699f88b06a0f8cbb10f0582bd0))
 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - add fuzz target and basic docs on how to run it ([`febf070`](https://github.com/Byron/gitoxide/commit/febf0706b83b36a71efbe669ee760c2d4ef14b72))
 * **Uncategorized**
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge branch 'main' into remote-ls-refs ([`c4bf958`](https://github.com/Byron/gitoxide/commit/c4bf9585d815bc342e5fb383336cc654280dd34f))
    - adjust `git_date::parsea(str)` to use a str ([`0f8680a`](https://github.com/Byron/gitoxide/commit/0f8680a60913556b7fbd7543fda6a409ac05b121))
    - Merge branch 'main' into write-index-v2 ([`a938986`](https://github.com/Byron/gitoxide/commit/a938986877302c197d1aed087594c5605416fe5f))
    - Merge branch 'main' into remote-ls-refs ([`de61c4d`](https://github.com/Byron/gitoxide/commit/de61c4db7855d6925d66961f62ae3d12cc4acf78))
    - thanks clippy ([`4bd747c`](https://github.com/Byron/gitoxide/commit/4bd747cb3e126fe5b1d540270cfbd731cffd42ef))
    - raise `git-revision` to the status of 'usable' ([`09eb1a6`](https://github.com/Byron/gitoxide/commit/09eb1a6e1eb5888b66b211500c73d72951058685))
    - Merge branch 'parse-refspec' ([`2ba338e`](https://github.com/Byron/gitoxide/commit/2ba338e28eb45d4d3215dd6ff9882611880d4cd9))
    - Merge branch 'rev-parse-delegate' ([`2f506c7`](https://github.com/Byron/gitoxide/commit/2f506c7c2988477b0f97d272a9ac9ed47b236457))
    - Add `Spec` data strcuture to fully represent a revision specification ([`5038ffa`](https://github.com/Byron/gitoxide/commit/5038ffab6a0f83e0566f99e3c92ae2dea266e10b))
    - thanks clippy ([`ca82265`](https://github.com/Byron/gitoxide/commit/ca82265abfcce644265af64afc499d2de88c3cba))
    - thanks clippy ([`19db44a`](https://github.com/Byron/gitoxide/commit/19db44a97d42f4fa77c681263cf509ee91f8fa6c))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
</details>

## 0.3.0 (2022-07-22)

This is a maintenance release with no functional changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 38 calendar days.
 - 39 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#427](https://github.com/Byron/gitoxide/issues/427)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - provide better hints for parsing describe output ([`fb0b8ca`](https://github.com/Byron/gitoxide/commit/fb0b8ca6dfde391c28c83494e7280b2ea7e933da))
    - improve describe hinting to allow hinting with describe-anchors as well ([`d993992`](https://github.com/Byron/gitoxide/commit/d99399287966ba2adf143222c3bd9ccdb4d135f9))
    - support disambiguation of describe prefixes ([`637dcb0`](https://github.com/Byron/gitoxide/commit/637dcb09771c8df83436dc48d6a72804b400c5e1))
    - First implementation of object peeling ([`b1ef03a`](https://github.com/Byron/gitoxide/commit/b1ef03abc8342adb4a0b67d7c86136720ee600e2))
    - explicitly support leading `..` and `...` ([`723e803`](https://github.com/Byron/gitoxide/commit/723e8034eba511e5d98d559949ef6552a7ac7d27))
    - Support for explaining all navitation ([`ace9c89`](https://github.com/Byron/gitoxide/commit/ace9c8953bebc4a808c639e365010ed53c031622))
    - Handle lonely tilde gracefully ([`6fb834e`](https://github.com/Byron/gitoxide/commit/6fb834e06639febbe67a46e702cd523c4e7bd2a7))
    - refactor ([`1a15e12`](https://github.com/Byron/gitoxide/commit/1a15e120a75d29b3d3f7615af1a66a033dfd3c8b))
 * **Uncategorized**
    - Release git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`aa639d8`](https://github.com/Byron/gitoxide/commit/aa639d8c43f3098cc4a5b50614c5ae94a8156928))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - make fmt ([`0700b09`](https://github.com/Byron/gitoxide/commit/0700b09d6828849fa2470df89af1f75a67bfb27d))
    - assure document-features are available in all 'usable' and 'early' crates ([`238581c`](https://github.com/Byron/gitoxide/commit/238581cc46c7288691eed37dc7de5069e3d86721))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`daa71c3`](https://github.com/Byron/gitoxide/commit/daa71c3b753c6d76a3d652c29237906b3e28728f))
    - thanks clippy ([`e1003d5`](https://github.com/Byron/gitoxide/commit/e1003d5fdee5d4439c0cf0286c67dec9b5e34f53))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
</details>

## 0.2.1 (2022-06-13)

### New Features

- support for parsing `revspec`s on a low level, meaning that the ground work for actually resolving them is done.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 79 commits contributed to the release over the course of 25 calendar days.
 - 25 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#427](https://github.com/Byron/gitoxide/issues/427)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 8 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - docs ([`42969f8`](https://github.com/Byron/gitoxide/commit/42969f8a53e3210af179911d655646915046bcb8))
    - top-level regex handling ([`f9d6f9e`](https://github.com/Byron/gitoxide/commit/f9d6f9e84b852141aed8366044692af3a8344242))
    - support for index lookups by paths and stage ([`ea22d3e`](https://github.com/Byron/gitoxide/commit/ea22d3e7c134b9517079f865e9f6848aa27f1a8b))
    - All tests relevant for top-level colon parsing ([`cee04e1`](https://github.com/Byron/gitoxide/commit/cee04e1268ad3d3fcc3f0c45efb1415a30fb9e80))
    - Implement :<path> parsing ([`74e7a46`](https://github.com/Byron/gitoxide/commit/74e7a46199d3ae13d8bc3616d285c238942c2cad))
    - tests for path parsing ([`d51e438`](https://github.com/Byron/gitoxide/commit/d51e438041a243a9827fe638e1e6330835706446))
    - More thorough tests using more complex specs ([`beb6e25`](https://github.com/Byron/gitoxide/commit/beb6e25a3a77df3532154d62911148302e639e37))
    - Implement tilde handling ([`e8a16c9`](https://github.com/Byron/gitoxide/commit/e8a16c964ddc994d32e8a122278f40700ad90cbc))
    - greatly improve brace handling ([`546f4df`](https://github.com/Byron/gitoxide/commit/546f4df8d8adcfc86c435a7d408307e5de8762e4))
    - more testing of escaping ([`f3eaff6`](https://github.com/Byron/gitoxide/commit/f3eaff631a88994a69437e67682680e14505f3a8))
    - prepare for being able to escape backslashes properly ([`840d9d0`](https://github.com/Byron/gitoxide/commit/840d9d0702f835f6b92d04122c8e9a9b4f21c9d1))
    - more specific backslash testing ([`a958edd`](https://github.com/Byron/gitoxide/commit/a958eddc2920cc0512ef1f987c31957fbefa1161))
    - More regex error handling ([`edd36ba`](https://github.com/Byron/gitoxide/commit/edd36baad610d32aeb17ab34448f1b4a5b253732))
    - handle braces within braces and support escaping them ([`8c5d87b`](https://github.com/Byron/gitoxide/commit/8c5d87bdf886727b8d0f013fc2ee497140032644))
    - basic regex parsing ([`1caeae9`](https://github.com/Byron/gitoxide/commit/1caeae95004ed4ef19a9c587744fe2b6d972c61a))
    - fix regex API and first ignored test ([`7a3a5fa`](https://github.com/Byron/gitoxide/commit/7a3a5fa740751f024b88a92deb3ffe624842509b))
    - A sketch of the regex parsing API for the delegate ([`18d9331`](https://github.com/Byron/gitoxide/commit/18d9331745bdebb077730f79132c76a12e9e7e24))
    - provide a marker for the delegate to know parsing is done ([`159a482`](https://github.com/Byron/gitoxide/commit/159a48268ee1e5d53adafbf36aa6e5fdf2886323))
    - refactor ([`6638040`](https://github.com/Byron/gitoxide/commit/66380409611a06c56800454813eb018d4938ef32))
    - parseing of 'follow tags recursively' ([`f11916a`](https://github.com/Byron/gitoxide/commit/f11916a78c3747ef6e52b9cd48b3235608a2c598))
    - parsing of `^{commit}` etc. ([`4d2dd56`](https://github.com/Byron/gitoxide/commit/4d2dd569c1296a2f906da6c30c591a966fcc5716))
    - refactor ([`a52244b`](https://github.com/Byron/gitoxide/commit/a52244b75bdaf10716fc788c8ef30615318d4606))
    - proper stacking/consumption of navigation items ([`76f7c4d`](https://github.com/Byron/gitoxide/commit/76f7c4de4b781f59cfd95b04ff8342cab0fe2dd5))
    - refactor ([`6f00e33`](https://github.com/Byron/gitoxide/commit/6f00e33781e5db7ff7d2c4290fb7f57d1db147b1))
    - navigation doesn't stack yet ([`d83937b`](https://github.com/Byron/gitoxide/commit/d83937b16640c9021a16abab6a1c89dbbca10c5c))
    - handle special case `@^0` ([`fa7790b`](https://github.com/Byron/gitoxide/commit/fa7790bc5a2385351e0c61fa6ea8878317ce1fcc))
    - basic caret parsing ([`c064135`](https://github.com/Byron/gitoxide/commit/c0641354e43a33a851339fd9871d8eec1abb93d8))
    - refactor ([`9b0e2a4`](https://github.com/Byron/gitoxide/commit/9b0e2a4c9201d7c1dd65377fbc982e44b1c33886))
    - reflog lookup by date is complete ([`b3d009e`](https://github.com/Byron/gitoxide/commit/b3d009e80e3e81afd3d095fa2d7b5fc737d585c7))
    - prepare for date based reflog lookups. ([`2267b2b`](https://github.com/Byron/gitoxide/commit/2267b2b7c31f6ee9995126a0d4783699166a6a3c))
    - Sibling branch support ([`0d3fb7a`](https://github.com/Byron/gitoxide/commit/0d3fb7a880ffbb6156bfb1d0b34f9679a6c6957f))
    - refname reflog entries ([`b50d099`](https://github.com/Byron/gitoxide/commit/b50d09903932961c62fa57464aef842766bbbbcb))
    - Allow parsing `@{-n}` ([`faa9914`](https://github.com/Byron/gitoxide/commit/faa9914731d5202e8f162eb6c09cdf8680de6d18))
    - refactor ([`a5f8f58`](https://github.com/Byron/gitoxide/commit/a5f8f5806edb0be7fe97ad65dde8c37d0a9c198f))
    - basic number parsing for '@' navigation ([`3fedcc0`](https://github.com/Byron/gitoxide/commit/3fedcc0afad1fe4c5cf6ef487904b0b60dc19540))
    - refactor ([`bff11a0`](https://github.com/Byron/gitoxide/commit/bff11a066f73b43045064cd9d6ca0ac09468e8f3))
    - more information on how anchors work ([`d82b21f`](https://github.com/Byron/gitoxide/commit/d82b21f2cd4f863a9d3d39d90f233fa171f52067))
    - show that we can already parse ranged rev-specs better than git ([`418360c`](https://github.com/Byron/gitoxide/commit/418360c23b9fcf6e57fdaa2e1ea732dc6256dbbf))
    - basic brace parsing ([`43e4cc1`](https://github.com/Byron/gitoxide/commit/43e4cc15c7115dd40238051274f50fe10907c24e))
    - refactor ([`ad4d8af`](https://github.com/Byron/gitoxide/commit/ad4d8afb3036b4f626f09fb26ac78a426d7acc2d))
    - prevent double-kind calls on parser level ([`d6781da`](https://github.com/Byron/gitoxide/commit/d6781da221602c272a26ac4f45a54f77ddd340bd))
    - refactor ([`c3b03a2`](https://github.com/Byron/gitoxide/commit/c3b03a237f30091558ddd0325279953fced16131))
    - refactor ([`b2c80ee`](https://github.com/Byron/gitoxide/commit/b2c80ee4c78a45ac2d95b69d8cbdccf349b95f3c))
    - also handle short decribe output with dirty suffix ([`826f964`](https://github.com/Byron/gitoxide/commit/826f96416d3eb59f93380b4c12c92844d9fd690e))
    - finalize git-describe parsing ([`e1e369f`](https://github.com/Byron/gitoxide/commit/e1e369f0c1a36805d50826d6b48d2dc62195f8bd))
    - tests for parsing describe output ([`5be4ad8`](https://github.com/Byron/gitoxide/commit/5be4ad8ac40f984e88acc64fbf77f221b0902a6a))
    - refactor ([`4f53dc3`](https://github.com/Byron/gitoxide/commit/4f53dc304abf89e8b6cafaafbcec99264ea67a95))
    - more varied range testing ([`bb0a554`](https://github.com/Byron/gitoxide/commit/bb0a554efd1b68298a23bcd2e29dc60da7a127c5))
    - refactor ([`2e49831`](https://github.com/Byron/gitoxide/commit/2e498317e6637ac57de21fee8bf905daf1cc54bf))
    - Support for hex-lookups by prefix ([`16945ed`](https://github.com/Byron/gitoxide/commit/16945edd1e544caf34ffa318bc59eea635e8b060))
    - refactor ([`db97a2e`](https://github.com/Byron/gitoxide/commit/db97a2ed20ab13786b30e7ad17a1b24eaeb34648))
    - half-decent parsing of ref-names with preparation for parenthesis handling ([`9866986`](https://github.com/Byron/gitoxide/commit/9866986de74f2aaa6471cfb2ec8ea7e4572b3a09))
    - Tiny steps towards understanding rev-parsing better ([`13c07f4`](https://github.com/Byron/gitoxide/commit/13c07f4ef84c5e03e08d04259eeede5e4d487476))
    - decide to not implement regex support (yet) ([`d6a4838`](https://github.com/Byron/gitoxide/commit/d6a4838dbb91d43f84e319986c027e9cabf536b2))
    - Allow delegates to refuse spec kind changes ([`2d9465f`](https://github.com/Byron/gitoxide/commit/2d9465fe01021bdcc8ba0907a5847e970c3cea12))
    - refactor ([`d16a4e8`](https://github.com/Byron/gitoxide/commit/d16a4e8f75bac5df6a4e96a2bd93d256587457b3))
    - refactor ([`e059bd3`](https://github.com/Byron/gitoxide/commit/e059bd33647a2b35af241a1f88cb61dc5176b55d))
    - support for range parsing with range in the middle ([`5ada481`](https://github.com/Byron/gitoxide/commit/5ada481c3756e1717189b478fc458322c3acc7ac))
    - basic range parsing ([`0c1c48c`](https://github.com/Byron/gitoxide/commit/0c1c48c5b393eeb534d50bf4048fe9c049297f00))
    - parse initial carets ([`8573c8e`](https://github.com/Byron/gitoxide/commit/8573c8e3d6f11f015f7e586632a637269e70395b))
    - Some more thought about whitespace and empty input ([`7182d88`](https://github.com/Byron/gitoxide/commit/7182d88e245f3bb8740cab1058acb7c9a1d6d461))
    - refactor ([`91e2c43`](https://github.com/Byron/gitoxide/commit/91e2c43c20c0d6ff4fae9669bfca4fcfe03c37a0))
    - prepare range parsing ([`5bd4863`](https://github.com/Byron/gitoxide/commit/5bd4863ced766d71432e252c344a424a2fd1a4fd))
    - refactor ([`efc05e1`](https://github.com/Byron/gitoxide/commit/efc05e11fa2ec11952b06080ba76387a4c11c3b4))
    - A basis for 'pure' parsing of rev-specs ([`29ab704`](https://github.com/Byron/gitoxide/commit/29ab7049fd180fac2e443a99908db066c67938db))
 * **Uncategorized**
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - make fmt ([`c665aef`](https://github.com/Byron/gitoxide/commit/c665aef4270c5ee54da89ee015cc0affd6337608))
    - Merge branch 'revspec-parsing' ([`a2c8969`](https://github.com/Byron/gitoxide/commit/a2c8969ba821fd387c39b14248074767f54749c8))
    - thanks clippy ([`1bbd3f4`](https://github.com/Byron/gitoxide/commit/1bbd3f471d78e53a76b3e708c755fc9d72fc28fe))
    - thanks clippy ([`b93fa40`](https://github.com/Byron/gitoxide/commit/b93fa40a9abcfb7390276e4254f696c0cac2abb1))
    - thanks clippy ([`6dc9c44`](https://github.com/Byron/gitoxide/commit/6dc9c44fb2770d93badb8e1d506b7601107ea586))
    - thanks clippy ([`ec0ff74`](https://github.com/Byron/gitoxide/commit/ec0ff7404ba7df80bf98fd6d28b13426c2e3ee6c))
    - thanks clippy ([`1b40259`](https://github.com/Byron/gitoxide/commit/1b402596bb581ea84b285282a44bf81752c14bba))
    - thanks clippy ([`6d08d5f`](https://github.com/Byron/gitoxide/commit/6d08d5f518a94426420c973b8e6e561ef558627c))
    - thanks clippy ([`1f0545f`](https://github.com/Byron/gitoxide/commit/1f0545f3169824f4953727f7319324b60baaf92f))
    - thanks clippy ([`2bc1acc`](https://github.com/Byron/gitoxide/commit/2bc1acc1816ef95b60c0192ef8d956558ff58bb9))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
</details>

## 0.2.0 (2022-05-18)

### Bug Fixes

 - <csr-id-99365f221065ebc315ac80940ad72cae253743bc/> Support for in truncated history in git-describe
   This allows `describe()` to work on shallow clones.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 42 calendar days.
 - 43 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#384](https://github.com/Byron/gitoxide/issues/384)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - Support for in truncated history in git-describe ([`99365f2`](https://github.com/Byron/gitoxide/commit/99365f221065ebc315ac80940ad72cae253743bc))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - make fmt ([`50ff7aa`](https://github.com/Byron/gitoxide/commit/50ff7aa7fa86e5e2a94fb15aab86470532ac3f51))
 * **[#384](https://github.com/Byron/gitoxide/issues/384)**
    - No need to isolate archives by crate name ([`19d46f3`](https://github.com/Byron/gitoxide/commit/19d46f35440419b9911b6e2bca2cfc975865dce9))
    - add archive files via git-lfs ([`7202a1c`](https://github.com/Byron/gitoxide/commit/7202a1c4734ad904c026ee3e4e2143c0461d51a2))
    - Assure we don't pick up unnecessary files during publishing ([`545b2d5`](https://github.com/Byron/gitoxide/commit/545b2d5121ba64efaee7564d5191cec37661efd7))
    - auto-set commit.gpgsign=false when executing git ([`c23feb6`](https://github.com/Byron/gitoxide/commit/c23feb64ad157180cfba8a11c882b829733ea8f6))
 * **Uncategorized**
    - Release git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0 ([`349c590`](https://github.com/Byron/gitoxide/commit/349c5904b0dac350838a896759d51576b66880a7))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Merge branch 'main' into repo-status ([`0eb2372`](https://github.com/Byron/gitoxide/commit/0eb23721dca78f6e6bf864c5c3a3e44df8b419f0))
    - Merge branch 'test-archive-support' ([`350df01`](https://github.com/Byron/gitoxide/commit/350df01042d6ca8b93f8737fa101e69b50535a0f))
    - Merge branch 'main' into repo-status ([`4086335`](https://github.com/Byron/gitoxide/commit/40863353a739ec971b49410fbc2ba048b2762732))
    - Merge branch 'worktree-stack' ([`e90d3fd`](https://github.com/Byron/gitoxide/commit/e90d3fd0a9764511e6280596f21d3a0494ed7021))
</details>

## 0.1.0 (2022-04-05)

<csr-id-0a7776b8cce4c40c391f46542f6e7ba6830d6fc0/>

### Refactor (BREAKING)

 - <csr-id-0a7776b8cce4c40c391f46542f6e7ba6830d6fc0/> Make `describe::Format` more consistent with other builder APIs
   Configuration methods now take an argument which makes it more
   straightforward to use for most.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 41 commits contributed to the release over the course of 15 calendar days.
 - 59 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
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
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - make fmt ([`7cf3545`](https://github.com/Byron/gitoxide/commit/7cf354509b545f7e7c99e159b5989ddfbe86273d))
    - Remove serde support for describe types due to warning ([`2ba33c8`](https://github.com/Byron/gitoxide/commit/2ba33c89e723c7ec44ff8b5597718ee7792f462d))
    - Merge branch 'main' into mailmap ([`b2df941`](https://github.com/Byron/gitoxide/commit/b2df941feaf5ae9fa170fa49270189f3527f2eab))
    - Merge branch 'describe-rev' ([`77b7cd9`](https://github.com/Byron/gitoxide/commit/77b7cd9a7813aaa1a15d035ea42c1e3fe4eef8dd))
    - thanks clippy ([`2c8a504`](https://github.com/Byron/gitoxide/commit/2c8a504c2b1a8309c3176e8c829e129c8dd39f80))
    - INTERMEDIATE RESET ME ([`a4de008`](https://github.com/Byron/gitoxide/commit/a4de008b88f892e95bf6da36d09b27190e9c5ede))
    - thanks clippy ([`f1ef59d`](https://github.com/Byron/gitoxide/commit/f1ef59d8129231554158fc51ab967b4f857c5e12))
</details>

## 0.0.0 (2022-02-05)

Reserve the name for a necessary crate of the `gitoxide` project.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-revision v0.0.0 ([`8e434d8`](https://github.com/Byron/gitoxide/commit/8e434d8d0046e4479f0a575247ce3c9cce7e1f77))
    - Rename git-rev to git-revision ([`2e939c9`](https://github.com/Byron/gitoxide/commit/2e939c973ab3635a946317af08f37c4e23450f18))
</details>

