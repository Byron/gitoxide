# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.4.1 (2022-08-17)

### New Features

 - <csr-id-6d8d5e6198dfb4d648061807ed4f96868a36ee52/> `Stage::entry_index_by_path_and_stage()`, now with `::entry_by_path_and_stage()`
 - <csr-id-55363ea650001b7717545b4d2968419707a3b8c6/> `State::entry_by_path_and_stage()` to find entries.
 - <csr-id-40e6bde125778e3b50999331c4ed5a4b119937fa/> `Debug` and `Clone` for `File`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 60 commits contributed to the release over the course of 22 calendar days.
 - 26 days passed between releases.
 - 3 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#427](https://github.com/Byron/gitoxide/issues/427), [#XXX](https://github.com/Byron/gitoxide/issues/XXX)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - make fmt ([`4b320e7`](https://github.com/Byron/gitoxide/commit/4b320e773368ac5e8c38dd8a779ef3d6d2d024ec))
    - fix docs ([`5a0d6b7`](https://github.com/Byron/gitoxide/commit/5a0d6b76205d6e021348a930a5a17820e5dc4458))
    - `Stage::entry_index_by_path_and_stage()`, now with `::entry_by_path_and_stage()` ([`6d8d5e6`](https://github.com/Byron/gitoxide/commit/6d8d5e6198dfb4d648061807ed4f96868a36ee52))
    - `State::entry_by_path_and_stage()` to find entries. ([`55363ea`](https://github.com/Byron/gitoxide/commit/55363ea650001b7717545b4d2968419707a3b8c6))
    - refactor; prepare for entry-lookup by path ([`92de081`](https://github.com/Byron/gitoxide/commit/92de081dc9ab5660cb18fa750452345dd63550ea))
    - `Debug` and `Clone` for `File` ([`40e6bde`](https://github.com/Byron/gitoxide/commit/40e6bde125778e3b50999331c4ed5a4b119937fa))
 * **[#XXX](https://github.com/Byron/gitoxide/issues/XXX)**
    - add tests to run into long-paths special case ([`d7a8a7d`](https://github.com/Byron/gitoxide/commit/d7a8a7dfe3089e35fca249af7a3482a893f91111))
 * **Uncategorized**
    - prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - fix CI for good ([`e0c0b8c`](https://github.com/Byron/gitoxide/commit/e0c0b8c7c1898b2bc11a915e8e4fb8426295ccbb))
    - fix CI ([`2433be1`](https://github.com/Byron/gitoxide/commit/2433be173c2145198f7891dc7a1f7c4acf215b11))
    - refactor ([`3af5121`](https://github.com/Byron/gitoxide/commit/3af5121330ae96aec32d0360c8b2e24a8860a2e8))
    - refactor ([`b41d93a`](https://github.com/Byron/gitoxide/commit/b41d93ac604b9807c24d93c6849f852489f512c0))
    - thanks clippy ([`4390c32`](https://github.com/Byron/gitoxide/commit/4390c32f9ea0683561a78349456c87329fef3b41))
    - run tests against all input files we have ([`de8abe6`](https://github.com/Byron/gitoxide/commit/de8abe6923b01563db812ba007ea65b7f193082d))
    - combine more tests into one to reduce duplication ([`933ad9e`](https://github.com/Byron/gitoxide/commit/933ad9e8ff0d58ad2590907cf84b43bc424e3219))
    - Assure that extended flags receive version 3; make `version` an implementation detail ([`6d810a1`](https://github.com/Byron/gitoxide/commit/6d810a135eeb71b8b04f7d9cb6c5f115587c2a63))
    - Support for extended flags, and V3 as it's a requirements. ([`417d90e`](https://github.com/Byron/gitoxide/commit/417d90eb267dd74a5372f1c7d8feb7cb4e98d2a1))
    - refcator ([`27993c0`](https://github.com/Byron/gitoxide/commit/27993c01a1533d561629635336c5cedf53dd0efc))
    - fix tree ext reading and writing; round-trip with long path works now ([`f93febe`](https://github.com/Byron/gitoxide/commit/f93febe2d2c55938ac8f698b57144583caab54ef))
    - first PoC for writing long paths, even though it doens't produce the entire file yet ([`581cbd7`](https://github.com/Byron/gitoxide/commit/581cbd7afeac0f7654611c83deacae802ef5da6f))
    - Merge branch 'format_git_date_time' ([`99e12be`](https://github.com/Byron/gitoxide/commit/99e12bee16ab3f344c71818bfd1c95cf50e1721b))
    - Make it more explicit to write all available extensions by default ([`fbe9815`](https://github.com/Byron/gitoxide/commit/fbe981519446e55c4020e95841e7bff7e54e358e))
    - fix docs ([`9861a6c`](https://github.com/Byron/gitoxide/commit/9861a6ce8abc438a1e0739aa6d55ced450a4465b))
    - thanks clippy ([`834be93`](https://github.com/Byron/gitoxide/commit/834be93e6db84bb9160dd4677b7e9d63213c23cd))
    - thanks clippy ([`9b3a940`](https://github.com/Byron/gitoxide/commit/9b3a940d9f4694912f32cb86752f3f7507882010))
    - generalize extension writing so that writing more will be easier ([`8ef5378`](https://github.com/Byron/gitoxide/commit/8ef5378dfaefe2d562d16b861fb4bb0fa4fdfe93))
    - generalize EOIE exstension writing ([`18b722e`](https://github.com/Byron/gitoxide/commit/18b722e06bfb8bbbf0ada7438266e31a4317f2d4))
    - provide a stand-alone way of writing end-of-index extensions ([`7ca297a`](https://github.com/Byron/gitoxide/commit/7ca297af400e50d42cffcaab54b1684f6810eb4f))
    - refactor ([`a5b2ef9`](https://github.com/Byron/gitoxide/commit/a5b2ef9a33720312a6b30b7cdae564bf759b0218))
    - additional validation ([`ee7b5bb`](https://github.com/Byron/gitoxide/commit/ee7b5bba09bc20e1531cb733b5b2aac8232e7674))
    - refactor ([`e35aac6`](https://github.com/Byron/gitoxide/commit/e35aac66079464a9494744c201355ab2faa0a2b3))
    - refactor ([`52386f4`](https://github.com/Byron/gitoxide/commit/52386f4a8ee11b0d2858412b4b5ec4b73544ba30))
    - refactor ([`75a2338`](https://github.com/Byron/gitoxide/commit/75a2338fe9cfac478164b7b575c0f3c2b910111d))
    - refactor ([`f6f2861`](https://github.com/Byron/gitoxide/commit/f6f2861f57be8ad4c795c90ae6fc7e568aeb12da))
    - refactor ([`6cf9277`](https://github.com/Byron/gitoxide/commit/6cf92776b0349bf735c28b6275fdf551ce236d4d))
    - refactor ([`a6354c0`](https://github.com/Byron/gitoxide/commit/a6354c076b2967ff31feb30e7b73f0a6eb92a459))
    - Merge branch 'main' into write-index-v2 ([`a938986`](https://github.com/Byron/gitoxide/commit/a938986877302c197d1aed087594c5605416fe5f))
    - Fill in all remaining documentation, raise `git-index` to 'usable' state ([`3568ae3`](https://github.com/Byron/gitoxide/commit/3568ae3a1ed7c2d0c9b7e1dc690b055b4f43bdd2))
    - first step towards everything being documented ([`919923c`](https://github.com/Byron/gitoxide/commit/919923c08b641ca148c2f25d193d65bb068cc787))
    - remove quickerror in favor of thiserror ([`dd7ce3f`](https://github.com/Byron/gitoxide/commit/dd7ce3f77c868f81196103b021957ace54ca2b9c))
    - refactor ([`618736b`](https://github.com/Byron/gitoxide/commit/618736b614330d9576e58c5bb9b3696de3f76d84))
    - refactor ([`4dda27e`](https://github.com/Byron/gitoxide/commit/4dda27e716927a506e16da9d6cd50547de1fc84e))
    - added test for eoie extension ([`a433c0d`](https://github.com/Byron/gitoxide/commit/a433c0d7feebebdf17e9d362894a6fa38221b402))
    - estimate vector size for tree entries ([`74455e6`](https://github.com/Byron/gitoxide/commit/74455e65e3ef48ce55fa40b7d9ef050a8e0e9b84))
    - implemented File::write_to for hashed write ([`6b6db34`](https://github.com/Byron/gitoxide/commit/6b6db340b351836f055d8ab74ed3d3f370cca7be))
    - refactor and add more tests ([`6b32bcf`](https://github.com/Byron/gitoxide/commit/6b32bcfde1af3ada67a6fa0448611d0ecca2f605))
    - Merge branch 'write-index-files' into write-index-v2 ([`cddc2ca`](https://github.com/Byron/gitoxide/commit/cddc2ca06f63f66e887ff821452d1f56fb08fe6a))
    - thanks clippy ([`a66403c`](https://github.com/Byron/gitoxide/commit/a66403c8e14716023455d606e1c63787ac40f4f4))
    - write wrapper to count written bytes ([`b147090`](https://github.com/Byron/gitoxide/commit/b147090bafd22da07f475a167bb921f3e0fa0017))
    - refactor test ([`33a3009`](https://github.com/Byron/gitoxide/commit/33a3009c4d851fae9156cb3bdb664c05118ef442))
    - refactor... ([`81eef35`](https://github.com/Byron/gitoxide/commit/81eef353f8f2add26720e3dd3981ce1b790f996c))
    - refactor tests... ([`3a9b51b`](https://github.com/Byron/gitoxide/commit/3a9b51b0fe812d454378c8ac887bba11740a81ee))
    - convert 'in-memory' flags to 'storage' flags ([`017377d`](https://github.com/Byron/gitoxide/commit/017377d8c49c236bb3ab240794dbd7a714efb7e1))
    - Merge branch 'write-index-files' into rev-parse-delegate ([`370110d`](https://github.com/Byron/gitoxide/commit/370110d3356528af38150c2280ed505354ceca5b))
    - small improvements ([`e5cb6d9`](https://github.com/Byron/gitoxide/commit/e5cb6d94ef31e007847a6072ead8962b16eba105))
    - fix pathname in test ([`1f18e19`](https://github.com/Byron/gitoxide/commit/1f18e19fd3c07b540d56c86afa4cb708ad1126ac))
    - thanks clippy ([`3f72180`](https://github.com/Byron/gitoxide/commit/3f7218044fdc9d24693c04e0c1c97069c9a3f698))
    - sucesfully writing the first basic index files ([`a9c6f22`](https://github.com/Byron/gitoxide/commit/a9c6f2260e96928d678b29a765c26e88f0ff5908))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
</details>

## 0.4.0 (2022-07-22)

This is a maintenance release with no functional changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 2 calendar days.
 - 64 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`aa639d8`](https://github.com/Byron/gitoxide/commit/aa639d8c43f3098cc4a5b50614c5ae94a8156928))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - setup and refactor tests ([`7eed237`](https://github.com/Byron/gitoxide/commit/7eed2375a3f076e6fdf9dde4673733e85d5612aa))
    - generate index header ([`f1d7c1c`](https://github.com/Byron/gitoxide/commit/f1d7c1c137c712ecca76b8e69b11481bf0f1a860))
</details>

## 0.3.0 (2022-05-18)

### New Features

 - <csr-id-8ab219ac47ca67f2478b8715d7820fd6171c0db2/> `State::path_backing()`.
   That way it's possible to call certain methods that take a separate
   path buffer.
 - <csr-id-645ed50dc2ae5ded2df0c09daf4fe366b90cf47e/> support for separating lifetimes of entries and path-backing
   This way it should be possible to access paths immutably even while
   entries are available mutably, assuming we stagger accesses to put
   mutation of entries last.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 17 commits contributed to the release over the course of 34 calendar days.
 - 45 days passed between releases.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#384](https://github.com/Byron/gitoxide/issues/384)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - upgrade git-index->atoi to 1.0 ([`728dd65`](https://github.com/Byron/gitoxide/commit/728dd6501b86b12e1d0237256f94059a7ead14a9))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - Differentiate between owned and ref'ed path storage ([`c71b2bb`](https://github.com/Byron/gitoxide/commit/c71b2bb944c3066e7e44fbdd8a2e511a5a5d944a))
    - `State::path_backing()`. ([`8ab219a`](https://github.com/Byron/gitoxide/commit/8ab219ac47ca67f2478b8715d7820fd6171c0db2))
    - sketch `open_index()` on `Worktree`, but… ([`ff76261`](https://github.com/Byron/gitoxide/commit/ff76261f568f6b717a93b1f2dcf5d8e8b63acfca))
    - support for separating lifetimes of entries and path-backing ([`645ed50`](https://github.com/Byron/gitoxide/commit/645ed50dc2ae5ded2df0c09daf4fe366b90cf47e))
    - An attempt to build a lookup table of attribute files, but… ([`9841efb`](https://github.com/Byron/gitoxide/commit/9841efb566748dae6c79c5990c4fd1ecbc468aef))
    - refactor ([`475aa6a`](https://github.com/Byron/gitoxide/commit/475aa6a3e08f63df627a0988cd16c20494960c79))
    - Adjustments to support lower MSRV ([`16a0973`](https://github.com/Byron/gitoxide/commit/16a09737f0e81654cc7a5bbc9043385528524ca5))
 * **[#384](https://github.com/Byron/gitoxide/issues/384)**
    - prevent line-ending conversions for shell scripts on windows ([`96bb4d4`](https://github.com/Byron/gitoxide/commit/96bb4d460db420e18dfd0f925109c740e971820d))
    - No need to isolate archives by crate name ([`19d46f3`](https://github.com/Byron/gitoxide/commit/19d46f35440419b9911b6e2bca2cfc975865dce9))
    - add archive files via git-lfs ([`7202a1c`](https://github.com/Byron/gitoxide/commit/7202a1c4734ad904c026ee3e4e2143c0461d51a2))
    - Assure we don't pick up unnecessary files during publishing ([`545b2d5`](https://github.com/Byron/gitoxide/commit/545b2d5121ba64efaee7564d5191cec37661efd7))
    - auto-set commit.gpgsign=false when executing git ([`c23feb6`](https://github.com/Byron/gitoxide/commit/c23feb64ad157180cfba8a11c882b829733ea8f6))
 * **Uncategorized**
    - Release git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0 ([`349c590`](https://github.com/Byron/gitoxide/commit/349c5904b0dac350838a896759d51576b66880a7))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Merge branch 'git_includeif' of https://github.com/svetli-n/gitoxide into svetli-n-git_includeif ([`0e01da7`](https://github.com/Byron/gitoxide/commit/0e01da74dffedaa46190db6a7b60a2aaff190d81))
</details>

## 0.2.0 (2022-04-03)

### Bug Fixes

 - <csr-id-c2cc939d131a278c177c5f44d3b26127c65bd352/> lower MSRV to 1.52

### Bug Fixes (BREAKING)

 - <csr-id-0b1543d481337ed51dcfdeb907af21f0bc98dcb9/> lower rust edition to 2018

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 43 commits contributed to the release over the course of 73 calendar days.
 - 73 days passed between releases.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 5 unique issues were worked on: [#293](https://github.com/Byron/gitoxide/issues/293), [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#329](https://github.com/Byron/gitoxide/issues/329), [#333](https://github.com/Byron/gitoxide/issues/333)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - Remove performance bottleneck when reading TREE extension ([`50411c8`](https://github.com/Byron/gitoxide/commit/50411c8031e3103bb84da46b94b8faf92c597df9))
    - Assert store tree cache matches actual source objects ([`b062bec`](https://github.com/Byron/gitoxide/commit/b062becd01058f5c519538f89d9d8fec8342114d))
    - Sketch a surprisingly difficult way of loading objects in verify_extension() ([`3baeab4`](https://github.com/Byron/gitoxide/commit/3baeab4ab216132536d5c182b3e316ce65095085))
    - Properly sort cache tree entries upon load ([`421d1ba`](https://github.com/Byron/gitoxide/commit/421d1ba853a75560f59cb0ce2b353087db0dff56))
    - tree-ordering validation shows something wrong ([`5fb2857`](https://github.com/Byron/gitoxide/commit/5fb2857e9f970c150f5221ca721506e7bc046ef4))
    - First stab at tree verification ([`f928350`](https://github.com/Byron/gitoxide/commit/f9283500e8316ab949fc0ff9c2fc13a498380873))
    - Verify entry order ([`2d101eb`](https://github.com/Byron/gitoxide/commit/2d101ebbd36e000ffec0e965012081fec2e234f7))
    - refactor ([`017e915`](https://github.com/Byron/gitoxide/commit/017e9153aaaa1cdd6788d9f61ff1ffbd61bc1b30))
    - basic index file checksum verification ([`c644565`](https://github.com/Byron/gitoxide/commit/c644565d5b8d9ae3991ee82a7ffa5e21a2705f91))
    - At least check for the presence of extensions ([`28c056c`](https://github.com/Byron/gitoxide/commit/28c056c6d2bbfb16a826238fd6879adecbeb1171))
    - thorough checking of Tree extension ([`d1063aa`](https://github.com/Byron/gitoxide/commit/d1063aa20bfcefb064ba08089f095baef1299dcd))
    - refactor ([`d0725bd`](https://github.com/Byron/gitoxide/commit/d0725bd40f0b9af0e0af34ffe77c2d8406c6d24c))
    - Fix tree-extension loading for empty trees ([`2e13989`](https://github.com/Byron/gitoxide/commit/2e1398985edfaf9e62ff5643cf4756d9d9717862))
    - Now we are able to load indices correctly ([`762efa3`](https://github.com/Byron/gitoxide/commit/762efa3f5e4ebda4d3bcc6a9bba43c6cdb407937))
    - Add breaking test with conflicting file in index ([`791a9f8`](https://github.com/Byron/gitoxide/commit/791a9f84ff8871c7beb0e2100a4dcba0e9384737))
    - Print extension names instead of count ([`1cc07e0`](https://github.com/Byron/gitoxide/commit/1cc07e0cfdae74e388abb29d7acb9c6f643278b4))
    - Print basic index information, including the tree extension ([`9277cf8`](https://github.com/Byron/gitoxide/commit/9277cf877e1f2276dcad1efdeb97e0e3d96ed3f0))
    - lower rust edition to 2018 ([`0b1543d`](https://github.com/Byron/gitoxide/commit/0b1543d481337ed51dcfdeb907af21f0bc98dcb9))
    - lower MSRV to 1.52 ([`c2cc939`](https://github.com/Byron/gitoxide/commit/c2cc939d131a278c177c5f44d3b26127c65bd352))
 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - Use hash_hasher based hash state for better keys/less collisions ([`814de07`](https://github.com/Byron/gitoxide/commit/814de079f4226f42efa49ad334a348bce67184e4))
    - Also print stage of entries ([`003515f`](https://github.com/Byron/gitoxide/commit/003515f3c90a49fbe9db9b84987233486711beb8))
    - simple printing of basic entry information ([`329538b`](https://github.com/Byron/gitoxide/commit/329538b9c3f44bb8e70a4567ba90dc3b594c2dfc))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - basic version of index checkout via command-line ([`f23b8d2`](https://github.com/Byron/gitoxide/commit/f23b8d2f1c4b767d337ec51888afaa8b3719798c))
    - document-features support for git-index and git-worktree ([`1367cf5`](https://github.com/Byron/gitoxide/commit/1367cf5bc5908639e67e12f78f57835c5fd68a90))
    - make fmt ([`636fa8a`](https://github.com/Byron/gitoxide/commit/636fa8a97ce56982c76dffc64ee084e31d39afad))
    - strucural refactor ([`cdca1df`](https://github.com/Byron/gitoxide/commit/cdca1dfec590d24dd42f34294e21f4bdf61d36ad))
    - Allow mutation of entries during iteration, while obtaining their path ([`d0c4563`](https://github.com/Byron/gitoxide/commit/d0c4563f71ea18aaf8ae21dd8646ab256a550594))
 * **[#329](https://github.com/Byron/gitoxide/issues/329)**
    - Document all features related to serde1 ([`72b97f2`](https://github.com/Byron/gitoxide/commit/72b97f2ae4dc7642b160f183c6d5df4502dc186f))
 * **[#333](https://github.com/Byron/gitoxide/issues/333)**
    - Use git_features::path everywhere where there is a path conversion ([`2e1437c`](https://github.com/Byron/gitoxide/commit/2e1437cb0b5dc77f2317881767f71eaf9b009ebf))
 * **Uncategorized**
    - Release git-diff v0.14.0, git-bitmap v0.1.0, git-index v0.2.0, git-tempfile v2.0.1, git-lock v2.0.0, git-mailmap v0.1.0, git-traverse v0.13.0, git-pack v0.17.0, git-quote v0.2.0, git-odb v0.27.0, git-packetline v0.12.4, git-url v0.4.0, git-transport v0.16.0, git-protocol v0.15.0, git-ref v0.12.0, git-worktree v0.1.0, git-repository v0.15.0, cargo-smart-release v0.9.0, safety bump 5 crates ([`e58dc30`](https://github.com/Byron/gitoxide/commit/e58dc3084cf17a9f618ae3a6554a7323e44428bf))
    - Merge branch 'for-onefetch' ([`8e5cb65`](https://github.com/Byron/gitoxide/commit/8e5cb65da75036a13ed469334e7ae6c527d9fff6))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - Merge branch 'short-id' ([`5849d5b`](https://github.com/Byron/gitoxide/commit/5849d5b326b83f98a16cf1d956c720c7f0fd4445))
    - Merge branch 'AP2008-implement-worktree' ([`f32c669`](https://github.com/Byron/gitoxide/commit/f32c669bc519d59a1f1d90d61cc48a422c86aede))
    - Implemented git-worktree ([`4177d72`](https://github.com/Byron/gitoxide/commit/4177d72c95bd94cf6a49e917dc21918044e8250b))
    - Release git-hash v0.9.2, git-object v0.17.1, git-pack v0.16.1 ([`0db19b8`](https://github.com/Byron/gitoxide/commit/0db19b8deaf11a4d4cbc03fa3ae40eea104bc302))
    - refactor ([`afdeca1`](https://github.com/Byron/gitoxide/commit/afdeca1e5ec119607c5d1f5ccec5d216fc7d5261))
    - thanks clippy ([`2f25bf1`](https://github.com/Byron/gitoxide/commit/2f25bf1ebf44aef8c4886eaefb3e87836d535f61))
    - thanks clippy ([`d721019`](https://github.com/Byron/gitoxide/commit/d721019aebe5b01ddb15c9b1aab279647069452a))
    - Merge branch 'index-information' ([`025f157`](https://github.com/Byron/gitoxide/commit/025f157de10a509a4b36a9aed41de80487e8c15c))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - upgrade to tui 0.17 and prodash 18 ([`eba101a`](https://github.com/Byron/gitoxide/commit/eba101a576ecb7bc0f63357d0dd81eb817b94be4))
    - dependency update ([`ca59e44`](https://github.com/Byron/gitoxide/commit/ca59e448061698dd559db43123fe676ae61129a0))
</details>

## 0.1.0 (2022-01-19)

The initial release which can read a complete index, version 2 to 4, with all extensions.
The reading can be performed with multiple threads as well, partially depending on whether
certain extensions are present.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 72 commits contributed to the release over the course of 490 calendar days.
 - 509 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#293](https://github.com/Byron/gitoxide/issues/293)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - prepare changelogs for git-index and dependencies ([`f54bf4b`](https://github.com/Byron/gitoxide/commit/f54bf4bde92b892b6d425987a6a37e10319c4635))
    - Test for extra-long paths ([`3d61afe`](https://github.com/Byron/gitoxide/commit/3d61afe615227e2af96525eba5b0e8e2f94207f3))
    - Test for extended flags ([`ae3b697`](https://github.com/Byron/gitoxide/commit/ae3b69710cf316cb8164120d4b98f051eef363bc))
    - Use bitflags for Flags (in-memory and at-rest) ([`ea86eb0`](https://github.com/Byron/gitoxide/commit/ea86eb0bebb0f067fc8710779c2c296632451c54))
    - Use bitflags for entry Mode ([`53df605`](https://github.com/Byron/gitoxide/commit/53df6057a8c50007716d89e08f1efe70435f8613))
    - FSMN V2 decoding ([`04279bf`](https://github.com/Byron/gitoxide/commit/04279bffc8bd43abe85f559634436be789782829))
    - Failing test for fs-monitor V1 ([`625b89a`](https://github.com/Byron/gitoxide/commit/625b89a7786fe9de29c9ad2ca41a734174f53128))
    - Validate UNTR with exclude-file oids ([`20ebb81`](https://github.com/Byron/gitoxide/commit/20ebb81c9ece6c2d693edd6243eaa730fa7cf44c))
    - read remaining pieces of UNTR ([`9d9cc95`](https://github.com/Byron/gitoxide/commit/9d9cc95a24d86cf5f66f1746c09ece032640a892))
    - Make stat parsing more general/reusable ([`c41b933`](https://github.com/Byron/gitoxide/commit/c41b933d14f2e4538928e4fbd682e1017702e69c))
    - refactor ([`a1dc8de`](https://github.com/Byron/gitoxide/commit/a1dc8dedc5d9e1712295131d2332c21f3df4ac45))
    - simplify UNTR directory indexing ([`7857d08`](https://github.com/Byron/gitoxide/commit/7857d08a25eac1c7d4a91f04eb80a83ec5677d1b))
    - flatten UNTR directory list for later access via bitmaps ([`2e39184`](https://github.com/Byron/gitoxide/commit/2e391841af52f88b7a0472179e5dda89cc6c9808))
    - read UNTR directory blocks and bitmaps ([`59f46fe`](https://github.com/Byron/gitoxide/commit/59f46fe134e8619f501c79da4290cadd5548751c))
    - First portion of reading the untracked cache ([`ed2fe5d`](https://github.com/Byron/gitoxide/commit/ed2fe5dbfbcf79ffdcdceed90f6321792cff076d))
    - failing test for UNTR extension ([`223f2cc`](https://github.com/Byron/gitoxide/commit/223f2cc1c88f76dc75ca6706f1f61514ab52e496))
    - Add UNTR extension fixture ([`3c7ba24`](https://github.com/Byron/gitoxide/commit/3c7ba247a3fdab114d9d549de50d6143c7fcce0a))
    - REUC reading works ([`29c1af9`](https://github.com/Byron/gitoxide/commit/29c1af9b2d7b9879a806fc84cfc89ed6c0d7f083))
    - frame and test for REUC exstension ([`229cabe`](https://github.com/Byron/gitoxide/commit/229cabe8de9e1bd244d56d24327b05e3d80dfb6e))
    - add git index with REUC exstension ([`8359fdb`](https://github.com/Byron/gitoxide/commit/8359fdb6c49b263bc7ac2f3105254b83eac47638))
    - Support for 'sdir' extension ([`a38c3b8`](https://github.com/Byron/gitoxide/commit/a38c3b889cfbf1447c87d489d3eb9902e757aa4b))
    - Turn git-bitmap Array into Vec, as it will be able to adjust its size ([`9e99e01`](https://github.com/Byron/gitoxide/commit/9e99e016c17f0d5bcd2ab645261dfac58cb48be5))
    - first stab at decoding ewah bitmaps ([`353a53c`](https://github.com/Byron/gitoxide/commit/353a53ccab5af990e7c384b74b38e5429417d449))
    - 'link' extension decoding to the point where bitmaps are needed ([`e18a2fd`](https://github.com/Byron/gitoxide/commit/e18a2fd68e1c7c06fe2ff9cd1634704313822b5c))
    - support for errors in extensions ([`8971991`](https://github.com/Byron/gitoxide/commit/8971991808b1497b9584b52270259d3c625fa970))
    - failing test for link decoding ([`e1daf18`](https://github.com/Byron/gitoxide/commit/e1daf18041862570fdfe53ff88d879d0c3cb7182))
    - don't forget to fail on unknown mandatory extension ([`f7e2bdd`](https://github.com/Byron/gitoxide/commit/f7e2bdd7dcb21600d2aca7d29d131eefe43f0f4f))
    - Aggregation for index entries loaded in parallel ([`995994a`](https://github.com/Byron/gitoxide/commit/995994a895a6faa4537ae1a6564edc005be96a1a))
    - parallel loading of entries right before reducing them ([`de84a3a`](https://github.com/Byron/gitoxide/commit/de84a3a03bcc9dc3ff71810e35c869f9b73dd38f))
    - Frame for using the new 'scoped threads' feature in git-features ([`6fea17d`](https://github.com/Byron/gitoxide/commit/6fea17d1306679d0454d01aa59adf12cd83c7973))
    - single and multi-threaded index tests ([`a22cb0f`](https://github.com/Byron/gitoxide/commit/a22cb0f1ead9a2f32e43eb2fb378281e592a4ed3))
    - prepare decode options for better control of threads ([`30de988`](https://github.com/Byron/gitoxide/commit/30de988f6a97177fcb32ffce37f4c80f46306a20))
    - cleanup ([`99d7224`](https://github.com/Byron/gitoxide/commit/99d7224baa04c199a7eb7aa2675b39657b0aef6a))
    - Basic IEOT parsing ([`35bdee4`](https://github.com/Byron/gitoxide/commit/35bdee4bf77787bcbe6c3dd715a677e2e46a8ad1))
    - refactor ([`6f04f8b`](https://github.com/Byron/gitoxide/commit/6f04f8b8276de9c6b649642fb7c95eb5ffad77e4))
    - parse V4 delta-paths ([`06640e3`](https://github.com/Byron/gitoxide/commit/06640e3f98f25e9502db7ac68e1967d9fd25e8b2))
    - more thorough tests for more complex repo with more entries ([`273853f`](https://github.com/Byron/gitoxide/commit/273853f1614a0106c60d3d73c3bf72fb57b405e8))
    - The first test to validate an entry ([`f865ef6`](https://github.com/Byron/gitoxide/commit/f865ef6c626c9db39a09416333b6465fdd12c734))
    - Now with counting of consumed bytes in extensions ([`77a062c`](https://github.com/Byron/gitoxide/commit/77a062cdaff1bdf80556301f1e1aa41002af9cef))
    - Use correct post-header slice when parsing entries ([`da556b0`](https://github.com/Byron/gitoxide/commit/da556b0a64ac9ca8eaee62cab163789b55903b3d))
    - All code needed to load extensions… ([`0a03f19`](https://github.com/Byron/gitoxide/commit/0a03f196b7ec4dc1e0e2377c729467781c9e6c2c))
    - a step towards pasing V2 paths ([`01036ad`](https://github.com/Byron/gitoxide/commit/01036ad1bafb6a830734a9dd4f4e2949b8981a30))
    - Most of the entry decoding, name is still missing ([`53e2d75`](https://github.com/Byron/gitoxide/commit/53e2d754262d9752d3b106f7991543986ad5426f))
    - Extensions are optional, and so is their iteration ([`620d2e6`](https://github.com/Byron/gitoxide/commit/620d2e6bd4ef6d3281c096aaf344669bcf49e723))
    - Prepare a more complex test for tree parsing, requires entry parsing ([`e7e0679`](https://github.com/Byron/gitoxide/commit/e7e067977ef440cf3edb8812c0d614b5d8213b58))
    - parse TREE chunk ([`a2ea498`](https://github.com/Byron/gitoxide/commit/a2ea49841a333c8af18fd258781a649214a0ae0b))
    - Get closer to implementing a simple TREE extension decoding ([`49fcb6f`](https://github.com/Byron/gitoxide/commit/49fcb6f6ae9d6ed47e7c0c3ea2aa644d4e8cd264))
    - refactor ([`07e8fb2`](https://github.com/Byron/gitoxide/commit/07e8fb2cb6b7819eb34676ede57808b845298674))
    - the first actual assetion ([`c17240d`](https://github.com/Byron/gitoxide/commit/c17240d0cbd6134a77a69359611789f4eebc727d))
    - refactor ([`d4b3a07`](https://github.com/Byron/gitoxide/commit/d4b3a07489703fb6d5e9b9fb9328741172826db9))
    - refactor ([`9fdd34b`](https://github.com/Byron/gitoxide/commit/9fdd34b634f4f15eb6cf5c2e7912bdc32dd61de6))
    - Fix counting issue, checksum matches now ([`cc33752`](https://github.com/Byron/gitoxide/commit/cc337526365a04a23571123531f1ae565d386bcf))
    - Another big step, even though EOIE checksum is still bugged ([`9ffd523`](https://github.com/Byron/gitoxide/commit/9ffd5231c582a3870c6d25ea870c005e77e32276))
    - right before implementing a traversal over extension chunks ([`79ca582`](https://github.com/Byron/gitoxide/commit/79ca582045dd03434737c779b84c991acf1b0823))
    - refactor ([`9b28b18`](https://github.com/Byron/gitoxide/commit/9b28b18262c763608d60fba65e91fcb9ca3ddb3e))
    - first step towards reading the EOIE extension ([`068c716`](https://github.com/Byron/gitoxide/commit/068c716b46699234d6ad1db70be34b894e61d76a))
    - parse index header ([`5c731f8`](https://github.com/Byron/gitoxide/commit/5c731f831d007a4fe099cadc4ecaab113ab7e08a))
    - first stab at basic index file parsing ([`826ca0c`](https://github.com/Byron/gitoxide/commit/826ca0c6a6801ec2a67ca73ac17092e5f85fe9ce))
    - refactor ([`494ed46`](https://github.com/Byron/gitoxide/commit/494ed46acc54bd342f891416918032a2c4848cf1))
    - git-index uses memmap2 ([`fbfea28`](https://github.com/Byron/gitoxide/commit/fbfea28d2c9ed92e270c6a5aa603d3c84769ae8f))
    - The realization that FileBuffer really shouldn't be used anymore ([`b481f13`](https://github.com/Byron/gitoxide/commit/b481f136c4084b8839ebecb604dea5aa30d3a44e))
    - base setup for index testing ([`aa60fdf`](https://github.com/Byron/gitoxide/commit/aa60fdf3d86e08877c88f9e4973f546642ed1370))
    - notes on how test indices have been created ([`3040857`](https://github.com/Byron/gitoxide/commit/3040857ec4d2e0557b4920eaa77ddc4292d9adae))
 * **Uncategorized**
    - Release git-bitmap v0.0.1, git-hash v0.9.0, git-features v0.19.0, git-index v0.1.0, safety bump 9 crates ([`4624725`](https://github.com/Byron/gitoxide/commit/4624725f54a34dd6b35d3632fb3516965922f60a))
    - thanks clippy ([`09df2bc`](https://github.com/Byron/gitoxide/commit/09df2bcb4b45f72742d139530907be8aa4dc36f8))
    - thanks clippy ([`93c3d23`](https://github.com/Byron/gitoxide/commit/93c3d23d255a02d65b5404c2f62f96d94e36f33d))
    - Fix index without extension test & thanks clippy ([`066464d`](https://github.com/Byron/gitoxide/commit/066464d2ad2833012fa196fe41e93a54ab05457f))
    - thanks clippy ([`f477032`](https://github.com/Byron/gitoxide/commit/f47703256fe6a5c68ed3af6705bcdf01262500d6))
    - thanks clippy ([`5526020`](https://github.com/Byron/gitoxide/commit/552602074a99dc536624f0c6295e807caf32f58b))
    - thanks clippy ([`591511a`](https://github.com/Byron/gitoxide/commit/591511a739f91c5e8ff4243059ac98052a44c914))
    - remove dash in all repository links ([`98c1360`](https://github.com/Byron/gitoxide/commit/98c1360ba4d2fb3443602b7da8775906224feb1d))
    - refactor ([`e4bcfe6`](https://github.com/Byron/gitoxide/commit/e4bcfe6406b14feffa63598c7cdcc8ecc73222bd))
</details>

## v0.0.0 (2020-08-28)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - add placeholder for git-index crate ([`52ff13c`](https://github.com/Byron/gitoxide/commit/52ff13cf260b53423faf59e5c666ff1565bde947))
</details>

