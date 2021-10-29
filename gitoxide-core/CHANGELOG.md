# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.12.0 (2021-10-19)

A maintenance release to properly dealing with previously breaking changes in `git-hash`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#222](https://github.com//Byron/gitoxide/issues/222)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#222](https://github.com//Byron/gitoxide/issues/222)**
    - stabilize changelogs ([`920e832`](https://github.com//Byron/gitoxide/commit/920e83219911df1c440d3fe42fd5ec3a295b0bb8))
    - Update changelogs prior to release ([`b3e2252`](https://github.com//Byron/gitoxide/commit/b3e2252f7461a003d9a4612da60ba931dd8c0bef))
</details>

## v0.11.0 (2021-10-15)

<csr-id-ac3b9efb7b90958274ce55800959d930f8641115/>
<csr-id-a19567eceab0dd7f5478b83c2ff9ce79754db308/>
<csr-id-da68bfb8104ecf58e73e3f99d87f81c90712a2ca/>
<csr-id-c77bd7a01820110154f2c66cd954c1ccfff173c1/>

This is a maintenance release signalling breaking changes because some of the crates it depends on have breaking changes.

### New Features

 - <csr-id-60c9fad8002b4e3f6b9607bba6361871752f4d3d/> control pack and object cache size in megabytes in some sub-commands
 - <csr-id-5a8c2da6cb1e2accf7cfdccc16bc3a1d0b2a7dbc/> object cache size is configurable in some sub-commands

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 25 commits contributed to the release over the course of 34 calendar days.
 - 12 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 5 unique issues were worked on: [#164](https://github.com//Byron/gitoxide/issues/164), [#198](https://github.com//Byron/gitoxide/issues/198), [#200](https://github.com//Byron/gitoxide/issues/200), [#205](https://github.com//Byron/gitoxide/issues/205), [#67](https://github.com//Byron/gitoxide/issues/67)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#164](https://github.com//Byron/gitoxide/issues/164)**
    - rename path::is_git to path::is ([`ac3b9ef`](https://github.com//Byron/gitoxide/commit/ac3b9efb7b90958274ce55800959d930f8641115))
    - rename ObjectIdExt::ancestors_iter() to *::ancestors() ([`a19567e`](https://github.com//Byron/gitoxide/commit/a19567eceab0dd7f5478b83c2ff9ce79754db308))
 * **[#198](https://github.com//Byron/gitoxide/issues/198)**
    - A changelog for gitoxide-core ([`b9f6a37`](https://github.com//Byron/gitoxide/commit/b9f6a37b9c27f2405694795adb476c01574b31ed))
    - Fixup remaining changelogs… ([`2f75db2`](https://github.com//Byron/gitoxide/commit/2f75db294fcf20c325555822f65629611be52971))
    - set package cache via RepositoryAccessExt ([`66292fd`](https://github.com//Byron/gitoxide/commit/66292fd1076c2c9db4694c5ded09799a0be11a03))
    - prepare for configurable pack cache ([`7d2b6b6`](https://github.com//Byron/gitoxide/commit/7d2b6b66e09ff39727fccd68d190679b52d90126))
 * **[#200](https://github.com//Byron/gitoxide/issues/200)**
    - feat: Add --reference/-r flag to gix pack-receive ([`637d12c`](https://github.com//Byron/gitoxide/commit/637d12cf368e044f59ccde37c6365d9528d2c43f))
 * **[#205](https://github.com//Byron/gitoxide/issues/205)**
    - '(null)' symref targets are turned into direct refs instead… ([`c77bd7a`](https://github.com//Byron/gitoxide/commit/c77bd7a01820110154f2c66cd954c1ccfff173c1))
    - fetch::Ref::Symbolic::target is now an option… ([`da68bfb`](https://github.com//Byron/gitoxide/commit/da68bfb8104ecf58e73e3f99d87f81c90712a2ca))
 * **[#67](https://github.com//Byron/gitoxide/issues/67)**
    - control pack and object cache size in megabytes ([`60c9fad`](https://github.com//Byron/gitoxide/commit/60c9fad8002b4e3f6b9607bba6361871752f4d3d))
    - Use 'cache::Object' trait where it matters ([`71c628d`](https://github.com//Byron/gitoxide/commit/71c628d46088ab455b54eb2330d24dcff96c911d))
    - split data::output::count::objects into files ([`8fe4612`](https://github.com//Byron/gitoxide/commit/8fe461281842b58aa11437445637c6e587bedd63))
    - object cache size is configurable ([`5a8c2da`](https://github.com//Byron/gitoxide/commit/5a8c2da6cb1e2accf7cfdccc16bc3a1d0b2a7dbc))
    - Count ref-deltas in thin packs as well ([`80c6994`](https://github.com//Byron/gitoxide/commit/80c6994149d19917c25e36e1bdf0dc8c9678365e))
    - Use Easy in the one spot where it is possible… ([`6a97bfa`](https://github.com//Byron/gitoxide/commit/6a97bfabcec6597efe9282e6d5c9f0ac3ada61dc))
    - try to create persistent Easy iterator, but can't make it Send… ([`54a64a5`](https://github.com//Byron/gitoxide/commit/54a64a588ff72515451a3d0343306ac4abe1cb35))
    - Add '--thin' flag to pack-create and pass it on ([`2664d73`](https://github.com//Byron/gitoxide/commit/2664d73f531a4b1f4bc784c1fe3a991711c86475))
 * **Uncategorized**
    - Release git-commitgraph v0.5.0, gitoxide-core v0.11.0, gitoxide v0.9.0 ([`960eb0e`](https://github.com//Byron/gitoxide/commit/960eb0e5e5a7df117ed2ae2a8e2ec167b074c332))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com//Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - make fmt, but now it picked up some parts that usually don't get altered… ([`01f7b72`](https://github.com//Byron/gitoxide/commit/01f7b729337bd2c99498321c479a9a13b1858e3e))
    - fix immediate dereference warning ([`0456312`](https://github.com//Byron/gitoxide/commit/0456312dddbd9ffd01e29c6705bb794cb1abb414))
    - Merge branch 'changelog-generation' ([`bf0106e`](https://github.com//Byron/gitoxide/commit/bf0106ea21734d4e59d190b424c22743c22da966))
    - Merge branch 'main' into changelog-generation ([`c956f33`](https://github.com//Byron/gitoxide/commit/c956f3351d766c748faf0460780e32ac8dfe8165))
    - Bump git-repository v0.10.0 ([`5a10dde`](https://github.com//Byron/gitoxide/commit/5a10dde1bcbc03157f3ba45104638a8b5b296cb9))
    - Release git-repository v0.9.1 ([`262c122`](https://github.com//Byron/gitoxide/commit/262c1229d6d2d55c70fe0e199ab15d10954d967b))
</details>

## v0.10.5 (2021-09-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gitoxide-core v0.10.5 ([`590e59b`](https://github.com//Byron/gitoxide/commit/590e59b2b41a419574443e6b850bdb119a172279))
    - Bump git-repository v0.9.0 ([`b797fc1`](https://github.com//Byron/gitoxide/commit/b797fc10f3f3d1fbc23916a4ff6e5e860e2dd4ed))
    - [repository #193] Add feature flags for async/blocking ([`57f482c`](https://github.com//Byron/gitoxide/commit/57f482c59ac47b7a5f1abf01b4a3e25364e061c2))
</details>

## v0.10.4 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 8 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gitoxide-core v0.10.4 ([`5ae584c`](https://github.com//Byron/gitoxide/commit/5ae584c65cc6e9ad306f077abb609159a15c6375))
    - [ref #190] move remaining file store functions to extension trait ([`60fc215`](https://github.com//Byron/gitoxide/commit/60fc215ccac529b4a14cb9d8260ab9ddec86758a))
    - [various #190] rename 'local-offset' to 'local-time-support' ([`3a7d379`](https://github.com//Byron/gitoxide/commit/3a7d3793a235ac872437f3bfedb9dd8fde9b31b1))
    - [repository #190] Make local-offset available on demand only… ([`1927be7`](https://github.com//Byron/gitoxide/commit/1927be7764f6af04ecc715dd52c631a3c8e16577))
    - [repository #185] support for initializing bare repositories ([`9e8a39e`](https://github.com//Byron/gitoxide/commit/9e8a39e3cbd620bd48f379743df0d5783c33a86f))
    - [repository #185] refactor ([`63089ff`](https://github.com//Byron/gitoxide/commit/63089ff356ea0f62963ae213ea0dbb09f891ada6))
    - [repository #185] refactor repository initialization… ([`5ff7eaa`](https://github.com//Byron/gitoxide/commit/5ff7eaa86bddfa94aec97355a5d6adb117045693))
</details>

## v0.10.3 (2021-08-29)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 33 commits contributed to the release over the course of 11 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gitoxide-core v0.10.3 ([`e132680`](https://github.com//Byron/gitoxide/commit/e1326808a24fa7e797106cbd4bf3f34aba59b148))
    - [various #184] configure docs.rs build features ([`cc50249`](https://github.com//Byron/gitoxide/commit/cc502492c512293e93e95610ca80a71896076ded))
    - Bump git-repository v0.8.0 ([`cdb45ff`](https://github.com//Byron/gitoxide/commit/cdb45ffa0810e9fcc9fd25bff7b696c2d27eeef5))
    - Bump git-protocol v0.10.0 ([`82d5a0b`](https://github.com//Byron/gitoxide/commit/82d5a0bb38903a8389e43cd5416e02e5496e661a))
    - [odb #180] refactor ([`eff21da`](https://github.com//Byron/gitoxide/commit/eff21dae1083042412f45cd2f7a0faaf7d6400e6))
    - [pack #179] refactor bundle ([`420dca2`](https://github.com//Byron/gitoxide/commit/420dca29bccca6e7d759880d8342f23b33eead0d))
    - [pack #179] refactor ([`ab6554b`](https://github.com//Byron/gitoxide/commit/ab6554b0cd5838f1ea4e82f6b5019798288076fa))
    - [object #177] cleanup CommitRefIter imports and git_object::Error ([`058f68a`](https://github.com//Byron/gitoxide/commit/058f68a9e1cd79fd5a2a1235da42358bc92ed255))
    - [object #177]  commit::RefIter -> CommitRefIter ([`e603306`](https://github.com//Byron/gitoxide/commit/e603306e81f392af97aa5afd232653de56bf3ce9))
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments ([`461dc53`](https://github.com//Byron/gitoxide/commit/461dc53ba3bc07d55fdb4aad7570ba9176a8b360))
    - [object #177] rename immutable::* to immutable::*Ref ([`6deb012`](https://github.com//Byron/gitoxide/commit/6deb01291fb382b7fb9206682e319afa81bacc05))
    - [ref #175] follow (try_)find(_what) naming convention ([`679895c`](https://github.com//Byron/gitoxide/commit/679895cf866d643e768e353af614a55aeed2ba5c))
    - Merge pull request #172 from mellowagain/main ([`61aebbf`](https://github.com//Byron/gitoxide/commit/61aebbfff02eb87e0e8c49438a093a21b1134baf))
    - [stability #171] Simply commit on git-ref/git-config stability tier 1… ([`f6560ff`](https://github.com//Byron/gitoxide/commit/f6560ffe8b9280c7e9c32afe0294ea3ee169dcf5))
    - Merge branch 'main' into stability ([`11bae43`](https://github.com//Byron/gitoxide/commit/11bae437e473fef6ed09c178d54ad11eee001b1d))
    - cleanup imports ([`e669303`](https://github.com//Byron/gitoxide/commit/e6693032f1391416fd704c21617051ddfb862a3a))
    - [stability #171] Don't leak unstable plumbing crates in git-repository… ([`71eb30f`](https://github.com//Byron/gitoxide/commit/71eb30f1caa41c1f9fe5d2785b71c9d77922c2af))
    - [pack #170] there can only be one ([`dce4f97`](https://github.com//Byron/gitoxide/commit/dce4f97a84aa6a73e31e7397501cfce27241c5b8))
    - [pack #170] clru allows for free lists, reducing allocation pressure... ([`4d820d2`](https://github.com//Byron/gitoxide/commit/4d820d2f94dc3afc062bbd25e969c87410212c3a))
    - Revert "[pack #67] Don't pre-fetch packed objects during counting" ([`811bb54`](https://github.com//Byron/gitoxide/commit/811bb54991636f7e517087b62cf0c8c8cc2ad9e6))
    - [pack #67] Don't pre-fetch packed objects during counting ([`d08b673`](https://github.com//Byron/gitoxide/commit/d08b6739d8e9294b795aba75e9c7f9f20645af2b))
    - [pack #67] refactor ([`14717f6`](https://github.com//Byron/gitoxide/commit/14717f6132672a5d271832a68de0b323b73abb2a))
    - [pack #67] Optimize caches based on cache debugging ([`1271c01`](https://github.com//Byron/gitoxide/commit/1271c01d2635ab49474add61a9feb78b98bd6180))
    - Merge branch 'main' into 162-repo-design-sketch ([`e63b634`](https://github.com//Byron/gitoxide/commit/e63b63412c02db469fbdb17da82cd1e9fda1ef0f))
    - [pack #167] a single-threaded special case for counting… ([`65e29de`](https://github.com//Byron/gitoxide/commit/65e29de45a92c82cebd832634ab194db19a1b590))
    - [pack #167] Error handling for object input ([`0aac40c`](https://github.com//Byron/gitoxide/commit/0aac40c88a5c26f7c295db8433b510b168f15ca3))
    - [pack #167] remove iterator based count objects impl… ([`7ec2f2b`](https://github.com//Byron/gitoxide/commit/7ec2f2b40e83aaa218360a8b5989792cd67de2ed))
    - [pack] A non-iterator version of parallel object counting… ([`04fe855`](https://github.com//Byron/gitoxide/commit/04fe855a37577d3da5bbd619807b44e449947893))
    - [ref #165] refactor ([`66624c3`](https://github.com//Byron/gitoxide/commit/66624c3ef1faf7048ee86ed73cf5f622802c061e))
    - [repository #165] prepare for writing light docs for Easy ([`f8834c9`](https://github.com//Byron/gitoxide/commit/f8834c9c8d2ab2ce87857c6773c6204f60df240e))
    - [repository #165] refactor ([`3a0160e`](https://github.com//Byron/gitoxide/commit/3a0160ed1c5bc33d330ad4e9189c4937d194e98d))
    - thanks clippy ([`1f2d458`](https://github.com//Byron/gitoxide/commit/1f2d4584f8b650f7e751c8d2df9a5d27725f4f2f))
    - [smart-release #162] rename git-repository::object -> objs ([`ac70d81`](https://github.com//Byron/gitoxide/commit/ac70d81791cad04ffdeb04916d7a2a6b533eee6c))
</details>

## v0.10.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gitoxide-core v0.10.2 ([`b96a518`](https://github.com//Byron/gitoxide/commit/b96a518610256bb2a684c940908aca26089db54b))
    - bump git-protocol to v0.9.0 as there are breaking changes ([`b4e3340`](https://github.com//Byron/gitoxide/commit/b4e33408b8eb12c9418704f663322385fd1dfb25))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com//Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.10.1 (2021-08-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 3 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gitoxide-core v0.10.1 ([`8b21d82`](https://github.com//Byron/gitoxide/commit/8b21d8214ddc0ad05ff559261aedb4a010ba8726))
    - [protocol] Make fetch-connection usage explicit ([`29696f9`](https://github.com//Byron/gitoxide/commit/29696f9b8e3ba3a72af1b099dac1c0866194d5ce))
</details>

## v0.10.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 146 commits contributed to the release over the course of 90 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#83](https://github.com//Byron/gitoxide/issues/83)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 4 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#83](https://github.com//Byron/gitoxide/issues/83)**
    - [organize] Auto-strip .git suffix for non-bare repos ([`ea0ecc2`](https://github.com//Byron/gitoxide/commit/ea0ecc2f9b0dc25bbaa7788aac4eeed566f075cb))
 * **Uncategorized**
    - (cargo-release) version 0.10.0 ([`310dd22`](https://github.com//Byron/gitoxide/commit/310dd22cc5a01980ca604a0110e78d9b804902a6))
    - (cargo-release) version 0.7.0 ([`1c5dfb8`](https://github.com//Byron/gitoxide/commit/1c5dfb86028f266435475ca8bdddc57f95002330))
    - [core] refactor ([`e3d708f`](https://github.com//Byron/gitoxide/commit/e3d708f953f0e88180d3c2c6bd7c028e07faa583))
    - [core] refactor ([`869d162`](https://github.com//Byron/gitoxide/commit/869d162276ec5dbc9e8b02875dd4695a9e5256cb))
    - [gitoxide-core] avoid lossy path conversions ([`63c2951`](https://github.com//Byron/gitoxide/commit/63c2951970391fb7a708bf874661417a51a4cb97))
    - Use AsRef<Path> when opening from path ([`515d256`](https://github.com//Byron/gitoxide/commit/515d2564e430da77c092ceb9414a3b3e7071c158))
    - [protocol #145] Unify the `previous` and `previous_result` parameters… ([`96f77c7`](https://github.com//Byron/gitoxide/commit/96f77c78a08e975d367ca25ac5d07eb2253cf4e5))
    - thanks clippy ([`e1964e4`](https://github.com//Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - Bump async-trait from 0.1.50 to 0.1.51 ([`ce0b81e`](https://github.com//Byron/gitoxide/commit/ce0b81e8f5c652d389ff876844bc42bcfa687921))
    - Bump serde_json from 1.0.64 to 1.0.65 ([`9117feb`](https://github.com//Byron/gitoxide/commit/9117feb5a228fa62f6f17fc4a0918717ccdb0b14))
    - [ref #140] do actual tag peeling in programs that matter ([`e404852`](https://github.com//Byron/gitoxide/commit/e40485233ee7a32c11426665a3d50f939d9637eb))
    - [ref #140] sketch ref tag peeling ([`ef90652`](https://github.com//Byron/gitoxide/commit/ef90652dfcd84b2fc140c38e1364b42578fdfbde))
    - [pack] fix build ([`e680854`](https://github.com//Byron/gitoxide/commit/e680854b12603ea898713e900a6b4407e93ebe91))
    - Bump futures-io from 0.3.15 to 0.3.16 ([`3c23820`](https://github.com//Byron/gitoxide/commit/3c23820d3f0d3567f44215cdb0ad13ab675a201f))
    - [pack] Make use of thin-pack resolver when writing bundles… ([`9f43bf0`](https://github.com//Byron/gitoxide/commit/9f43bf029624f7c94346646465e366609b89e2e1))
    - [pack] it seems git is just skipping bad objects during pack-gen ([`0f29b82`](https://github.com//Byron/gitoxide/commit/0f29b82b48f45f509016eb16ea92af7f6dbf65a6))
    - [pack] In single-threaded mode, use a huge cache for some speedup ([`aec8a9b`](https://github.com//Byron/gitoxide/commit/aec8a9b4b9deb102b06390a19727eab7660621f9))
    - [pack] pack-create with immediate counting and traversing… ([`b74a98f`](https://github.com//Byron/gitoxide/commit/b74a98fc87a92a8ccbaec59aeea5284731e2fe49))
    - [pack] refactor; entry-iterator now produces delta-objects ([`5dc370b`](https://github.com//Byron/gitoxide/commit/5dc370ba01d25a6e8b7f4bfa03259c83e6b1d758))
    - [pack] support poor reference resolution if input is not an object hash… ([`1b985a1`](https://github.com//Byron/gitoxide/commit/1b985a195e09c5681a6b6732c9a23895053dbcd2))
    - [pack] better identify the currently implemented pack generation mode. ([`f9e3b3c`](https://github.com//Byron/gitoxide/commit/f9e3b3ca3bbf063e8d71c62fe607b812c745a969))
    - [pack] refactor ([`78d46c1`](https://github.com//Byron/gitoxide/commit/78d46c13d0510ee3e2e2f33cd60d624d63e85900))
    - [ref] fix build ([`0b732e1`](https://github.com//Byron/gitoxide/commit/0b732e1349760eebf9d954fe7904d3c4b218b8b2))
    - [ref] figure out how peeling works with packed-refs… ([`2801f7a`](https://github.com//Byron/gitoxide/commit/2801f7aa137c6167bd392ca585f1aad378cae0b4))
    - [ref] fix build ([`83002df`](https://github.com//Byron/gitoxide/commit/83002df0296a431de839ebb3522f57d42a17515f))
    - [ref] rename find_one to 'find' in git-ref… ([`ae7746a`](https://github.com//Byron/gitoxide/commit/ae7746a0815bb94659de67383ba372ac522d53b8))
    - Bump anyhow from 1.0.41 to 1.0.42 ([`352e468`](https://github.com//Byron/gitoxide/commit/352e4689959dee169f08b8fbd7be3c1f234202b8))
    - Bump async-io from 1.4.1 to 1.6.0 ([`99e4732`](https://github.com//Byron/gitoxide/commit/99e4732f5148787b767afc6ad57666e31faac960))
    - [protocol] fix build ([`38aca40`](https://github.com//Byron/gitoxide/commit/38aca4076037a6f8288c2cf483f134ea16c328d5))
    - [ref] rename Action::Close to Action::Cancel… ([`cac1f6c`](https://github.com//Byron/gitoxide/commit/cac1f6c757709797d193c6bca30e99fe40466ddc))
    - [protocol] fallible negotiation ([`e269a2c`](https://github.com//Byron/gitoxide/commit/e269a2cde18f604a36b33efb7e53f31ea5c45e2d))
    - [protocol] support ref-in-want ([`b6df400`](https://github.com//Byron/gitoxide/commit/b6df400dccd66ad2f01c80d2fa05b8f9bb130b23))
    - [actor] fix gix hours ([`b4e95fd`](https://github.com//Byron/gitoxide/commit/b4e95fdbb6664adcb2603d9cb6e6a69182de050f))
    - [actor] git-object uses git-actor ([`d01dd2f`](https://github.com//Byron/gitoxide/commit/d01dd2f9e9e8e2b81cdb1131a436d32b5819b731))
    - clippy cleanup; fix CI build ([`3e943f2`](https://github.com//Byron/gitoxide/commit/3e943f2afd5f0cfe7294a21cca8e0344c7dd0216))
    - thanks clippy ([`3f7e27b`](https://github.com//Byron/gitoxide/commit/3f7e27b91e2c7d66959f5f4c1a667f3315111cd6))
    - Fix everything up so that… ([`5930563`](https://github.com//Byron/gitoxide/commit/5930563601d6c2148cf39e109f69f8b7c7dfcb36))
    - A first attempt to make intrerupt tools work, but… ([`8fb8d37`](https://github.com//Byron/gitoxide/commit/8fb8d374ecfeffa3ae1bd07bf9bc5014351730f5))
    - fix build ([`ea2bfac`](https://github.com//Byron/gitoxide/commit/ea2bfac65f742ca7617bc77a50376c29156c4ea5))
    - refactor ([`7f9be36`](https://github.com//Byron/gitoxide/commit/7f9be36ea909ee67555591287bcb140fdc54c801))
    - And one less usage of the global interrupt handler… ([`5da57a3`](https://github.com//Byron/gitoxide/commit/5da57a3b0efef75ad82cb4d1cd496fc7fc0f1c23))
    - Make most interrupts local to the method or function ([`4588993`](https://github.com//Byron/gitoxide/commit/458899306a3f3c8578f185d7ecbf1ade2a7142dd))
    - [hours] use new interrupt::Iter; refactor ([`2355f0b`](https://github.com//Byron/gitoxide/commit/2355f0b2de4a6d7b1e206aa2a445814947983e55))
    - [pack-create] also show throughput ([`74d8d57`](https://github.com//Byron/gitoxide/commit/74d8d57f5da84b55219c2d3b115709dc0b422897))
    - [tempfile] interruptible traversal ([`4eeaa1b`](https://github.com//Byron/gitoxide/commit/4eeaa1bb9ca4af2eb21807007eabeb714c98fdfe))
    - [pack-create] better handling of input paths ([`1825e1a`](https://github.com//Byron/gitoxide/commit/1825e1a68d2a3274b0cc7d6ae56a31cb8145d944))
    - [pack-create] progress for ancestor traversal ([`9349286`](https://github.com//Byron/gitoxide/commit/9349286c3afa89a472e33d6281414dd6bf2b90a2))
    - refactor ([`e0b7f69`](https://github.com//Byron/gitoxide/commit/e0b7f695ee6bd1032544a29d91906f9b75e12d46))
    - [pack] refactor ([`25f04ba`](https://github.com//Byron/gitoxide/commit/25f04baa100bd1996f48fbeb4c87e40ff1b27d90))
    - [pack] validate tips as well… ([`ec8864f`](https://github.com//Byron/gitoxide/commit/ec8864ff23f18a00fe39d5d0061a1ab73810e283))
    - [pack] refactor ([`18cabb8`](https://github.com//Byron/gitoxide/commit/18cabb8618ffc324412302bfda208948abffb61f))
    - [pack] Force single-threading (with toggle) for counting phase… ([`8d3ba0b`](https://github.com//Byron/gitoxide/commit/8d3ba0b863f82d4eed0d9ca1ddf439f6feaf5041))
    - [pack] also put counts in order for stable packs ([`f299160`](https://github.com//Byron/gitoxide/commit/f299160cafd00f0fea00a2402901570f5ddf27d5))
    - [pack] gix pack-create uses in-order adapter as well ([`365c582`](https://github.com//Byron/gitoxide/commit/365c58286b9e09c9a8b1b5d6ee3b76484a458ca7))
    - [pack] refactor ([`cfdf802`](https://github.com//Byron/gitoxide/commit/cfdf8021ea1448ac4844b1f3bf252fefde2572fa))
    - [pack] print the pack file name even if there is no output directory ([`832fa29`](https://github.com//Byron/gitoxide/commit/832fa291595aaae7f7862d95bb1cbebcc34f2271))
    - [pack] refactor ([`9d9def3`](https://github.com//Byron/gitoxide/commit/9d9def30784a1b90f27c8181bfb0b0ba4ed4f1c8))
    - [pack] pack-create --output-directory is now optional ([`2150be8`](https://github.com//Byron/gitoxide/commit/2150be816f8a9d0ec049841045c904be1bb57ed6))
    - [pack] print statistics for entries iteration as well ([`eb6554b`](https://github.com//Byron/gitoxide/commit/eb6554b84131a09e5779edee302709c8ab62f47d))
    - [pack] add --statistics flag to pack-create ([`51a3077`](https://github.com//Byron/gitoxide/commit/51a307730b8514acffa75c78ecca3f02b1eb467b))
    - refactor ([`24697bc`](https://github.com//Byron/gitoxide/commit/24697bc66363f8e8b1ff14a59fdf303ffdab132d))
    - [async-receive] refactor ([`7e28831`](https://github.com//Byron/gitoxide/commit/7e288316a4cc402bd32489dbf5ca0050f84cfb18))
    - Bump anyhow from 1.0.40 to 1.0.41 ([`f6d48c8`](https://github.com//Byron/gitoxide/commit/f6d48c8c0ad2d92b587ba9cfc5f6e941203c7c4d))
    - [pack] write packs to a directory with the proper name ([`3fbca7d`](https://github.com//Byron/gitoxide/commit/3fbca7dd62752a7dd752b83a39ec8dfd7b2f2ea8))
    - [pack] refactor ([`f10adea`](https://github.com//Byron/gitoxide/commit/f10adea76d92eada3ca204fe69e7b5f81a06d8cc))
    - [pack] fix build ([`81ee633`](https://github.com//Byron/gitoxide/commit/81ee633c7f482746bc28a2a43d74ebbaded7af5f))
    - [pack] refactor ([`0514f1d`](https://github.com//Byron/gitoxide/commit/0514f1df113c5f6bf1c934b15741ca8ea47316ae))
    - [pack] refactor ([`37922d1`](https://github.com//Byron/gitoxide/commit/37922d12765c221e747fad4ca813597490525279))
    - Bump itertools from 0.10.0 to 0.10.1 ([`b54f21d`](https://github.com//Byron/gitoxide/commit/b54f21da9d41aa4fc67e5b1bf7ab979ec1bd9760))
    - [async-client] refactor ([`e7d115c`](https://github.com//Byron/gitoxide/commit/e7d115c4be758b48172b07d94139810b6fcc7fa3))
    - [async-client] cleanup Send bounds! ([`c7dee44`](https://github.com//Byron/gitoxide/commit/c7dee44267462d5ece491b8a45cf35afa904ce81))
    - [async-client] refactor ([`89e6f66`](https://github.com//Byron/gitoxide/commit/89e6f66e6e549fcc9bf72e4e837ff4e3dce66d2d))
    - Revert "[async-client] FAIL with the brutal copy-paste way" ([`7f29adc`](https://github.com//Byron/gitoxide/commit/7f29adc2936e1266a0e2c698c1a4677cf822a5f6))
    - [async-client] FAIL with the brutal copy-paste way ([`b91ecb5`](https://github.com//Byron/gitoxide/commit/b91ecb536c9c3e1a77647025f7a72fb098e83082))
    - Revert "[async-client] the beginning of an unholy transformation…" ([`c8423a8`](https://github.com//Byron/gitoxide/commit/c8423a83b5212b5381ae03accf386be2f882e78c))
    - [async-client] the beginning of an unholy transformation… ([`1f314df`](https://github.com//Byron/gitoxide/commit/1f314df4f0101bc3970201b66298afa8a35bf22c))
    - [async-client] refactor ([`b252932`](https://github.com//Byron/gitoxide/commit/b252932ee3eb26bb26560a849a9b13aca11cf00f))
    - [async-client] unblock the async delegate in the cheapest possible way… ([`a3b5d75`](https://github.com//Byron/gitoxide/commit/a3b5d75d387dc5d6c44f695f63df8803613637a2))
    - [async-client] prepare for unblocking the protocol delegate ([`796c7d5`](https://github.com//Byron/gitoxide/commit/796c7d54a20ef32a581be572e1d681f9727482de))
    - [async-client] refactor ([`0d5b911`](https://github.com//Byron/gitoxide/commit/0d5b911ad5f47ab8f044d6bbe660a6d1dfeecb5f))
    - Revert "[async-client] Try to bring 'Send' back but…" ([`52eb953`](https://github.com//Byron/gitoxide/commit/52eb953fcc44cce19604b1df6a600237b8c81392))
    - [async-client] Try to bring 'Send' back but… ([`3a06adb`](https://github.com//Byron/gitoxide/commit/3a06adb41f6b2946f78044e4ab1385e6441fc40f))
    - [async-client] refactor ([`dc742df`](https://github.com//Byron/gitoxide/commit/dc742dfbc877f4a39b5659ea4960408ce0a1d247))
    - [async-client] Unblock printing in pack-receive ([`156bed6`](https://github.com//Byron/gitoxide/commit/156bed6be1d830eb853d90dcb98c81978725d958))
    - [async-client] Sketch of (partially blocking) pack-receive ([`e58859d`](https://github.com//Byron/gitoxide/commit/e58859d133ee23b098df9107b6da5c0cc9bb696a))
    - [async-client] ls-remote in async (but for git protocol only) ([`fd8edca`](https://github.com//Byron/gitoxide/commit/fd8edca42a58a901e749d599eb552315d7b24a78))
    - [async-client] basic git_connect functionality using async_io/async_net ([`af60297`](https://github.com//Byron/gitoxide/commit/af60297cf2b80d862880a2178e08f3f23b796f1d))
    - [async-client] frame for async connect ([`9ada080`](https://github.com//Byron/gitoxide/commit/9ada0805fc5896f8ef1a31dc821b789b7f0438a6))
    - [async-client] frame from A to Z to actually implement it… ([`ac4715c`](https://github.com//Byron/gitoxide/commit/ac4715c53798c8438fb30802d6b83c868915522b))
    - Separate networking via feature toggles and pass that through in the main crate ([`2c749f1`](https://github.com//Byron/gitoxide/commit/2c749f10dd03ea0b027fb046e8c40c77869fb2e9))
    - [git-protocol] refactor ([`94d7be4`](https://github.com//Byron/gitoxide/commit/94d7be4a16f2c2e68a9dacf120eef7a417a8a6b9))
    - [gix-organize] fast-prefilter + close look at the repository itself ([`eda440a`](https://github.com//Byron/gitoxide/commit/eda440ab7efc81749b20a0f21a46825c945ff6db))
    - [gix-organize]: this version fails to detect any git repo ([`8802fa7`](https://github.com//Byron/gitoxide/commit/8802fa7e28ea8fcd3ef8dbca84be4e1f55eca665))
    - [gix-organize] use git-repository a little more ([`20f76a5`](https://github.com//Byron/gitoxide/commit/20f76a5fc93c9a59e26688dce3e82114ccaeffe3))
    - Revert 'gix-organize' to normal thanks to performance regression ([`eda452e`](https://github.com//Byron/gitoxide/commit/eda452e14564e802e9314d94993ae8c8590c5301))
    - (cargo-release) version 0.6.0 ([`d35c55d`](https://github.com//Byron/gitoxide/commit/d35c55d8ff4b52e25befb8bff839d805b9f3caf4))
    - thanks clippy ([`6a80d5c`](https://github.com//Byron/gitoxide/commit/6a80d5c02d01ab1fc6388eb0eb79d0a4407efab6))
    - [git-repository] gitoxide-core uses more of git-repository ([`bb5b074`](https://github.com//Byron/gitoxide/commit/bb5b0747dfd3a3985a904b7748f296a591fcb26e))
    - [git-repository] replaces git-features and git-protocol in gitoxide-core ([`081d20f`](https://github.com//Byron/gitoxide/commit/081d20f927f222daa69f2a1a492957fd3146bfc1))
    - refactor ([`2ba9f91`](https://github.com//Byron/gitoxide/commit/2ba9f915035a518bef3eb8b0ed1c9972c4a47cfa))
    - [git-repository] used by gix-hours ([`24e0258`](https://github.com//Byron/gitoxide/commit/24e0258b9691b82df5c35a35111d19df56087cdc))
    - [git-repository] refactor ([`b5ebcfa`](https://github.com//Byron/gitoxide/commit/b5ebcfa278a0be85ea10893fd40a8b3e2e28efd5))
    - [git-repository] now used by gix-organize ([`aa91fad`](https://github.com//Byron/gitoxide/commit/aa91fad3cf237f6d6f9d588ed390baa6e55f6540))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com//Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - [git-repository] towards git-repository as one stop shop ([`aea6cc5`](https://github.com//Byron/gitoxide/commit/aea6cc536f438050cc0e02223de7702cd7912e75))
    - [git-odb] much better docs; cleanup exposed API ([`3d5b229`](https://github.com//Byron/gitoxide/commit/3d5b229c2605060f2cac9695ff2479777deabdd0))
    - (cargo-release) version 0.2.0 ([`b213628`](https://github.com//Byron/gitoxide/commit/b213628feeb8dfa87dab489c7d3155a60e6a236d))
    - [git-odb] refactor ([`2958145`](https://github.com//Byron/gitoxide/commit/2958145a0ae1ef582bbf88352f5567d5c2b5eaf0))
    - [git-odb] refactor ([`1eab15d`](https://github.com//Byron/gitoxide/commit/1eab15dfb42c819050b0277c4cb6a1045d2fd58d))
    - [git-pack] compilation ([`b392a55`](https://github.com//Byron/gitoxide/commit/b392a55b97a30b10ac0db94a96230e22ea7ab0dc))
    - [git-pack] refactor ([`157b6ff`](https://github.com//Byron/gitoxide/commit/157b6ff7b55ba2b7f8f90f66864212906426f8d7))
    - (cargo-release) version 0.16.0 ([`769c649`](https://github.com//Byron/gitoxide/commit/769c649c00c009bf5a3f7c0611a7b999618f2938))
    - [git-pack] refactor ([`e5b00ee`](https://github.com//Byron/gitoxide/commit/e5b00ee257b712477413f48448b0bccf9a06bfaf))
    - [git-pack] the world compiles again ([`f0c0e36`](https://github.com//Byron/gitoxide/commit/f0c0e36a1fb15d44776678567162ac754fdd26c0))
    - [git-odb] refactor ([`e07478c`](https://github.com//Byron/gitoxide/commit/e07478c7b212e4d1d21ce151d9eb26d0fae422a8))
    - [git-odb] refactor ([`721303d`](https://github.com//Byron/gitoxide/commit/721303db232f87857aae58e12b342e5fb0139306))
    - [git-odb] refactor ([`ea224e9`](https://github.com//Byron/gitoxide/commit/ea224e9ee5f7efcbf4942a2a6fc7e4d790b2be50))
    - [git-odb] refactor ([`6a1b16a`](https://github.com//Byron/gitoxide/commit/6a1b16ae98edc9a694b945a12a7866eb17fc6be3))
    - [git-odb] refactor ([`47c4042`](https://github.com//Byron/gitoxide/commit/47c4042f16a0e0e6a536bab7150b7cb21958a7ed))
    - Configure git-features properly for gitoxide-core… ([`251e690`](https://github.com//Byron/gitoxide/commit/251e69030c2c25493a7e2ff0cb79ca01dfa228f5))
    - (cargo-release) version 0.15.0 ([`d69d9fb`](https://github.com//Byron/gitoxide/commit/d69d9fb0931f8257cef96ef14a89da9340ad9738))
    - Merge pull request #88 from avoidscorn/traverse-partial-ancestors ([`966f058`](https://github.com//Byron/gitoxide/commit/966f058beac9bec8277abb26b7cb3caf76df0cbf))
    - Prevent pack-index-from-data to block if stdin is a terminal ([`39dec0e`](https://github.com//Byron/gitoxide/commit/39dec0e25b23162cfd8171bc44477c4d936fc00a))
    - [pack-gen] release a little memory, hopefully ([`f25293a`](https://github.com//Byron/gitoxide/commit/f25293ae7885a21db72b84a3aa49eca3aafbdaef))
    - Revert "[pack-gen] remove tree-diff as traversal option." ([`2907a5f`](https://github.com//Byron/gitoxide/commit/2907a5facb08a7decbdfa652e76eb0ebd5e29dcf))
    - [pack-gen] remove tree-diff as traversal option. ([`8373671`](https://github.com//Byron/gitoxide/commit/8373671fd4f3f7e9d78c480e9f68c0a7ae423c69))
    - [pack-gen] a lot more progress, even though it's not perfect yet ([`480f8b7`](https://github.com//Byron/gitoxide/commit/480f8b720d84502bddd06cdbb35bf5cb69f9249d))
    - [pack-gen] basic progress for entry generation ([`953190d`](https://github.com//Byron/gitoxide/commit/953190d70a5df22b54dc1fffe78d41dc7d81cc61))
    - [pack-gen] better progress ([`fdee381`](https://github.com//Byron/gitoxide/commit/fdee381073459dc7d1e2e964a930aaf8db36def5))
    - [pack-gen] the first barely working progress ([`5b89a0e`](https://github.com//Byron/gitoxide/commit/5b89a0e4203d405a50bc2e8de9d87b79e545412d))
    - [pack-gen] the basics to get the program going ([`03b67b0`](https://github.com//Byron/gitoxide/commit/03b67b09e4127ae4bd791501d74794d9360f7ef6))
    - [pack-gen] very close to a basic impl of count + entries-gen… ([`c927429`](https://github.com//Byron/gitoxide/commit/c9274295e62f59cd8db06a307cc4a69d096a006e))
    - [pack-gen] Try to just ignore the amount of objects inside… ([`918b222`](https://github.com//Byron/gitoxide/commit/918b222343dbcb5fb0177526a997d0f3cb4ac585))
    - thanks clippy ([`89b1ee4`](https://github.com//Byron/gitoxide/commit/89b1ee48d4c93e8ecee7630bc894c8ca994cb989))
    - [pack-gen] And it shows we really need to let the traversal be done first ([`a870eb2`](https://github.com//Byron/gitoxide/commit/a870eb2b46a95e8ea69632eceef3fc4e37bbac4c))
    - [pack-gen] And now it creates an entries iterator ([`27c9bc1`](https://github.com//Byron/gitoxide/commit/27c9bc1e8a254689d6c337677f71d51518f6800e))
    - [pack-gen] A step further, but it looks like input object iteration is tricky ([`abf4276`](https://github.com//Byron/gitoxide/commit/abf427674805f8624ec381a00d8c70c569515878))
    - [pack-gen] Frame for plumbing command ([`a2203ca`](https://github.com//Byron/gitoxide/commit/a2203ca7a403ece79dda5c568f0bf6da34535882))
    - (cargo-release) version 0.10.0 ([`5d7ee6a`](https://github.com//Byron/gitoxide/commit/5d7ee6a105abbb6efeed8624bade936bb59dbc55))
    - refactor ([`9f0a8cc`](https://github.com//Byron/gitoxide/commit/9f0a8cc1561589088f44a1775832110449a4f1ab))
    - (cargo-release) version 0.3.0 ([`684de4b`](https://github.com//Byron/gitoxide/commit/684de4b376ecd4cc5330f7ac8643352ea9580ed3))
    - (cargo-release) version 0.8.0 ([`ccea4b6`](https://github.com//Byron/gitoxide/commit/ccea4b6bcdaba0ee6c6a6236d225ea1276d2547c))
    - [git-transport] remove default features to force being explicit everywhere ([`d1b39f8`](https://github.com//Byron/gitoxide/commit/d1b39f8093c032a172237a584c9208479611a866))
    - [organize] Be clear about what the traversal really does ([`ed945ab`](https://github.com//Byron/gitoxide/commit/ed945abfd80a4e5f994a3dee1b1deae30f57a3aa))
    - refactor ([`ef80fd6`](https://github.com//Byron/gitoxide/commit/ef80fd693204d42fdc125ea89f1c26643e99bde9))
</details>

## v0.9.0 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 36 commits contributed to the release over the course of 27 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.9.0 ([`e6cdd84`](https://github.com//Byron/gitoxide/commit/e6cdd8423a57b8b3982adb168c2652676df5fa37))
    - (cargo-release) version 0.7.0 ([`069184e`](https://github.com//Byron/gitoxide/commit/069184e55057a1655d2754cb1fd68a4424beff34))
    - (cargo-release) version 0.15.0 ([`d91b241`](https://github.com//Byron/gitoxide/commit/d91b2412381e3c8c1f24c38469e821c3c3960e34))
    - (cargo-release) version 0.2.0 ([`3fb8377`](https://github.com//Byron/gitoxide/commit/3fb8377ff36422fe7607fb9172edf8bd5a4db995))
    - (cargo-release) version 0.9.0 ([`84897fd`](https://github.com//Byron/gitoxide/commit/84897fd8e6e1b0269da0303d6a0de8f9e0eb58e5))
    - Merge branch 'patch-2' ([`f01dc54`](https://github.com//Byron/gitoxide/commit/f01dc54010683b232c5f5813bd5370e93f1681f5))
    - Merge branch 'patch-1' ([`5edc076`](https://github.com//Byron/gitoxide/commit/5edc0762524112bb6716b3afcf23b2a4a0f5efd3))
    - [hours-tool] interruptability of long-running commit interations ([`4fd8a63`](https://github.com//Byron/gitoxide/commit/4fd8a63d82e87e2a9f3b0268726d08e7b954942c))
    - Add missing docs; add local-only snapshot file ([`7c56366`](https://github.com//Byron/gitoxide/commit/7c56366f4d34e67cfd31dbbcdb6f96ba61fd1668))
    - [hours-tool] Better error messages ([`86b4570`](https://github.com//Byron/gitoxide/commit/86b4570effdb756d86c105926ae0dc942399981c))
    - [hours-tool] integrate progress, remove direct writes to stderr ([`2778447`](https://github.com//Byron/gitoxide/commit/27784478c6e365bc92cb0ae7d7b372f073c47293))
    - [hours-tool] bring in all the code, mostly unchanged. ([`df16b3c`](https://github.com//Byron/gitoxide/commit/df16b3c269c0a2fa4d487988fd4fdd029b3a26f7))
    - [hours-tool] hookup new gitoxide-core command ([`680f274`](https://github.com//Byron/gitoxide/commit/680f2742a04749a9d692741381b4c662f28f3179))
    - thanks clippy ([`17258cc`](https://github.com//Byron/gitoxide/commit/17258cc58767caa6e71227898decd160ad0cdf13))
    - refactor ([`8b10434`](https://github.com//Byron/gitoxide/commit/8b1043483cb46fd1b7f47a90c9dce24a65d58d1b))
    - (cargo-release) version 0.14.0 ([`a760f8c`](https://github.com//Byron/gitoxide/commit/a760f8c013e13ba82daa1acf1a4a57e0818a008d))
    - (cargo-release) version 0.14.0 ([`d9514ee`](https://github.com//Byron/gitoxide/commit/d9514eec64579ef77c9f2ac5dfe87cd302180eb9))
    - rename 'Locate' to 'Find' - shorter and just as good ([`60f72f5`](https://github.com//Byron/gitoxide/commit/60f72f573a7696323e09bf4add80d5fbce22c99d))
    - (cargo-release) version 0.13.0 ([`5c791af`](https://github.com//Byron/gitoxide/commit/5c791af217fac6a171d174ad9f4ee5f4d5282892))
    - (cargo-release) version 0.8.0 ([`a1ce210`](https://github.com//Byron/gitoxide/commit/a1ce210003ff07bf11291018bb182cbc7913647b))
    - (cargo-release) version 0.3.0 ([`e9665c7`](https://github.com//Byron/gitoxide/commit/e9665c784ae7e5cdaf662151395ee2355e9b57b6))
    - Don't mention skips anymore… ([`afb87d9`](https://github.com//Byron/gitoxide/commit/afb87d9be442a1f62a069ed58948e49cd7595a3a))
    - refactor ([`c1013dd`](https://github.com//Byron/gitoxide/commit/c1013dddbc221b366b91d186cfd1732f1d72be10))
    - refactor ([`ca98221`](https://github.com//Byron/gitoxide/commit/ca98221d5a512dabf683cc1da56d40a17285f2fb))
    - refactor ([`d490b65`](https://github.com//Byron/gitoxide/commit/d490b65ebbc6666cd59d88f8677dc1c52bfe1e1c))
    - refactor ([`08fafaa`](https://github.com//Byron/gitoxide/commit/08fafaa03144fc3ddea9120a4a1943e18c454ae8))
    - git-odb::borrowed::Object => git-odb::data::Object ([`747a13e`](https://github.com//Byron/gitoxide/commit/747a13e9a1fe5200c53055dd961507c9fef667e1))
    - bump git-odb minor version ([`5c833ce`](https://github.com//Byron/gitoxide/commit/5c833ce64babd00b7ced3e3a1c9ed3dbd260c9f4))
    - Remove loose::Object entirely #(67) ([`5cf4840`](https://github.com//Byron/gitoxide/commit/5cf4840b10a3fac43266bc9defa72977a004bf8c))
    - (cargo-release) version 0.13.0 ([`ac2eddb`](https://github.com//Byron/gitoxide/commit/ac2eddb06eb3d8a9a3dcdcd796eb54a7e45ab935))
    - (cargo-release) version 0.11.0 ([`fd698e3`](https://github.com//Byron/gitoxide/commit/fd698e334e44d5c478c162f98d09afd9ce7a6895))
    - Introduce pack_id for use in pack cache, preventing (most collisions) ([`ad04ad3`](https://github.com//Byron/gitoxide/commit/ad04ad3b8ac54e78bee307dd44c85c1389edced2))
    - Feature toggle for uluru based Lru cache ([`98eec48`](https://github.com//Byron/gitoxide/commit/98eec4837d605a408b026a859e53a7e2eae8e4da))
    - gitoxide-core:pack-verify: be explicit about pack-cache choice in relation to algorithm ([`e7971a9`](https://github.com//Byron/gitoxide/commit/e7971a924df0ab958d56239f48eaafda30f15159))
    - refactor ([`d624d09`](https://github.com//Byron/gitoxide/commit/d624d097784eed216f8d0e94544d8b62ef6c3010))
    - LruCache with const-generics ([`93618d1`](https://github.com//Byron/gitoxide/commit/93618d107e9defadb603209251f77948caddc121))
</details>

## v0.8.0 (2021-04-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 56 commits contributed to the release over the course of 98 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#63](https://github.com//Byron/gitoxide/issues/63)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#63](https://github.com//Byron/gitoxide/issues/63)**
    - git-protocol uses `oid` type ([`3930a6f`](https://github.com//Byron/gitoxide/commit/3930a6ff508f5bb2249fb2c2f21e00b74fecda22))
    - Use new `oid` where possible in git-odb ([`68a709e`](https://github.com//Byron/gitoxide/commit/68a709e0337d4969138d30a5c25d60b7dbe51a73))
    - Make ObjectId/oid happen! ([`ca78d15`](https://github.com//Byron/gitoxide/commit/ca78d15373ec988d909be8f240baefe75555e077))
    - Remove all public exports of git-hash types in git-object ([`accf89d`](https://github.com//Byron/gitoxide/commit/accf89d25560e5ded6f44a1c4a898ee65d14f8f6))
 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`02df134`](https://github.com//Byron/gitoxide/commit/02df1345a22889a573adfc1be80bda271b2dc9a5))
    - (cargo-release) version 0.8.0 ([`1a2a5cc`](https://github.com//Byron/gitoxide/commit/1a2a5cc093139cecb1516e8235f087ad12cfb703))
    - (cargo-release) version 0.6.0 ([`8513f0f`](https://github.com//Byron/gitoxide/commit/8513f0fafbf8ae61d86df2d8b0aefa52d3eb1680))
    - (cargo-release) version 0.10.0 ([`3161777`](https://github.com//Byron/gitoxide/commit/316177729e42f8d000a40ab01b9b97621e7179e8))
    - (cargo-release) version 0.7.0 ([`b900914`](https://github.com//Byron/gitoxide/commit/b900914a00292217ba7b9bcac260591800395287))
    - (cargo-release) version 0.4.0 ([`06612eb`](https://github.com//Byron/gitoxide/commit/06612eb12d4679bec7dae08a511dd87d80087151))
    - (cargo-release) version 0.12.0 ([`3b71e7e`](https://github.com//Byron/gitoxide/commit/3b71e7e8416e550b47e5aed2259c1181497ac9e8))
    - (cargo-release) version 0.2.0 ([`4ec09f4`](https://github.com//Byron/gitoxide/commit/4ec09f4d2239ea1d44f7145027e64191bf2c158c))
    - Remove locate(…) -> Option<Result<…>> in favor of Result<Option<…>> ([`40ee743`](https://github.com//Byron/gitoxide/commit/40ee7438a98c4094c0fd04977cd4904668087512))
    - A trial for Result<Option<Object>>  for loose object databases ([`3842859`](https://github.com//Byron/gitoxide/commit/3842859c5bddb8b4583443685c26dcae3f8db558))
    - Added [directory] argument to init. ([`62f8dc6`](https://github.com//Byron/gitoxide/commit/62f8dc62ec3e76efd7311ced32094035856dbcbb))
    - (cargo-release) version 0.9.0 ([`efc8983`](https://github.com//Byron/gitoxide/commit/efc898381d830e44487c62e35a665d3ccd0a2d39))
    - (cargo-release) version 0.5.0 ([`3cc4a57`](https://github.com//Byron/gitoxide/commit/3cc4a5799fa1f487452b5c346b57fea97e45b47e))
    - (cargo-release) version 0.3.0 ([`d5c6643`](https://github.com//Byron/gitoxide/commit/d5c6643a41d295eaf7aabb84eab435e42a11dd42))
    - thanks clippy ([`f25598a`](https://github.com//Byron/gitoxide/commit/f25598a82256d2c7d538e9be90437cb5ca8c973f))
    - thanks clippy ([`0fc239c`](https://github.com//Byron/gitoxide/commit/0fc239cf9b773f72928b7c42344b578c6ff5d19f))
    - [gix] Use flate2 by default ([`f1158a1`](https://github.com//Byron/gitoxide/commit/f1158a1a4bc8e13913461db4d4851e32d57816ff))
    - [gix] Add optional zlib feature ([`f1f9665`](https://github.com//Byron/gitoxide/commit/f1f96658a6cd6165ba9c9d7acb809fcaf2c46f9c))
    - [organize]: make it work with bare and non-bare repositories ([`b85a389`](https://github.com//Byron/gitoxide/commit/b85a3891a08248aaa0d7ec429940c9793a2ddcd1))
    - [organize]: Make client state meaning explicit ([`0f4265f`](https://github.com//Byron/gitoxide/commit/0f4265fa93060f47637ac7e9bd286d4918b3db62))
    - [gitoxide-core] Fix find_origin_remote location ([`a3c19fc`](https://github.com//Byron/gitoxide/commit/a3c19fcfdf144119caf469c0d18278a1578c483e))
    - [gitoxide-core] Use git-config for remote url parsing ([`c45feed`](https://github.com//Byron/gitoxide/commit/c45feed6124601a8bbef609d5b47c5b8a9d5defa))
    - [gitoxide-core] Use git-config as dependency ([`c567925`](https://github.com//Byron/gitoxide/commit/c567925906c73a00753f4ddb6bcbd64d99d78885))
    - Make 'find' reproducable ([`c5af6eb`](https://github.com//Byron/gitoxide/commit/c5af6eb1f044d1396f23839ecec08bb0e6776fe6))
    - mildly improve performance in case there is nothing to do for 'organize' ([`4f9fdc5`](https://github.com//Byron/gitoxide/commit/4f9fdc5be6eac4ad518469990b3258a40262d337))
    - Fix journey tests by not allowing canonicalization of possibly… ([`532ff2b`](https://github.com//Byron/gitoxide/commit/532ff2b9deb491d870b772d91fad0024790e8f59))
    - Avoid claiming we would move something even though we won't (in 'organize') ([`47c7fb3`](https://github.com//Byron/gitoxide/commit/47c7fb3bb1a24e5d2fc1aa71f2febe8fe87172d4))
    - (cargo-release) version 0.8.0 ([`1ccfdcd`](https://github.com//Byron/gitoxide/commit/1ccfdcdb96b59c6415e7fbc800371d594b2ef7a1))
    - Implement `find` subcommand ([`28d506a`](https://github.com//Byron/gitoxide/commit/28d506a6c0df18fc0c2e4a578707203f8e89577d))
    - (cargo-release) version 0.11.0 ([`1aa1f5e`](https://github.com//Byron/gitoxide/commit/1aa1f5e84a07427d5d7f3231735fe9c1923f506f))
    - Fix tests ([`da94cfc`](https://github.com//Byron/gitoxide/commit/da94cfcfa3e745d1174fd9b065f57f56e9f70efe))
    - thanks clippy ([`de32204`](https://github.com//Byron/gitoxide/commit/de32204cdac809fb20c9fe56d5ea6fa828217038))
    - Avoid moving nested repositories out of their place ([`5d7e6bf`](https://github.com//Byron/gitoxide/commit/5d7e6bf22af432a3a813daaf485b3a72f64bf257))
    - Recurse into directories much less… ([`87561eb`](https://github.com//Byron/gitoxide/commit/87561eb0df8a9da3e0befcdf3d0976cc6a66550d))
    - Better use of jwalk filter capabilities… ([`781ea7f`](https://github.com//Byron/gitoxide/commit/781ea7fe00fd48c68c0bdc5e0bf03d47dfce4f63))
    - optimize number of CPUs for directory walk for M1 chips ([`129a699`](https://github.com//Byron/gitoxide/commit/129a69997fb5c50d28fe9340a9b20bab0a69121e))
    - Remove usage of gitfeatures::fs in organize subcommand ([`b567d37`](https://github.com//Byron/gitoxide/commit/b567d3709a74e9fdafef54b0fe58ca82721cd773))
    - prepare to put 'organize' behind a feature flag ([`9986509`](https://github.com//Byron/gitoxide/commit/9986509af150a90c4c9271b402fcac419090d9d4))
    - refactor; planning ([`5df492c`](https://github.com//Byron/gitoxide/commit/5df492c7d7322bde2b268deaf590f1ba012a6b8e))
    - fix progress ([`1abd761`](https://github.com//Byron/gitoxide/commit/1abd761670f6b6aba3af10ab4a60c86a3f314f6a))
    - Assure basic 'organize' operation is working as expected ([`deb6073`](https://github.com//Byron/gitoxide/commit/deb6073671ae95de674aaef7ca01e03f95b41ca8))
    - A version of organize which works; in theory ([`800a2f4`](https://github.com//Byron/gitoxide/commit/800a2f4a488112fbd31882c734889e4841aaa120))
    - A first stab at finding git repositories ([`e4dc964`](https://github.com//Byron/gitoxide/commit/e4dc96403894f1fe509335905679347ecdf535c7))
    - Fix verbose parsing unit tests ([`ce38ede`](https://github.com//Byron/gitoxide/commit/ce38edee5b8c7f6829fc8050ce3eeffe5943eedf))
    - (cargo-release) version 0.2.0 ([`0c39373`](https://github.com//Byron/gitoxide/commit/0c39373de5aba0acc4aaa330bf51b6abd4f50474))
    - thanks clippy ([`9e93a71`](https://github.com//Byron/gitoxide/commit/9e93a71c6664b3d40c9e76811ada81d6e1180bfe))
    - first sketch of parsing git remotes (from git :D) ([`f8ab261`](https://github.com//Byron/gitoxide/commit/f8ab261fe77a9339c121e6254a523d09fa339e40))
    - first tiny journey test for dry run of organize subcommand ([`7bbba5a`](https://github.com//Byron/gitoxide/commit/7bbba5a76d8cccb527dd6e782b830dbc4ce426bd))
    - refactor ([`64495b0`](https://github.com//Byron/gitoxide/commit/64495b0a6468679d882e5bebda45704891e7bf4e))
    - first sketch of interface for 'organize' subcommand ([`4f64d12`](https://github.com//Byron/gitoxide/commit/4f64d1277308bc2281c065236f2f14d66826d14d))
    - silence so far unknown clippy lints ([`b5f2a4b`](https://github.com//Byron/gitoxide/commit/b5f2a4b079665daa8b9e0228acc59d1eddd603b2))
    - thanks clippy ([`343ab9a`](https://github.com//Byron/gitoxide/commit/343ab9adb62da1dde495fc209c179137bbe59a10))
</details>

## v0.7.0 (2020-12-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - All crates use git-hash::Kind and its types, sometimes through git-object ([`124c171`](https://github.com//Byron/gitoxide/commit/124c171aaf546d8977e9913ff84e65383a80ee98))
    - use git-hash in git-features ([`5b307e0`](https://github.com//Byron/gitoxide/commit/5b307e076f6f5975592c8b177c122c91c1d809c6))
</details>

## v0.6.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 74 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 ([`4df97ce`](https://github.com//Byron/gitoxide/commit/4df97ce6869a53a688b9af18405b284d9ff27b24))
    - (cargo-release) version 0.3.0 ([`e60dbe6`](https://github.com//Byron/gitoxide/commit/e60dbe6c21843eab44d6f05fe70927252453cb41))
    - (cargo-release) version 0.6.0 ([`27f5955`](https://github.com//Byron/gitoxide/commit/27f5955e047f35e21a86789eb46bfd89e1c99b44))
    - (cargo-release) version 0.2.0 ([`d61ad88`](https://github.com//Byron/gitoxide/commit/d61ad884021d3c0a61a14ba1df4daadfa1a0b561))
    - (cargo-release) version 0.9.0 ([`a89fdb9`](https://github.com//Byron/gitoxide/commit/a89fdb98f64bb0ca070fa79a1f58f1232bb14090))
    - (cargo-release) version 0.5.0 ([`fc7d600`](https://github.com//Byron/gitoxide/commit/fc7d600ac2c438c8b6b91f67cb69b0ac5ec37675))
    - (cargo-release) version 0.5.0 ([`ae9c52b`](https://github.com//Byron/gitoxide/commit/ae9c52bdbe43488bb9d5b5448bf07367a1d0a24a))
    - (cargo-release) version 0.2.0 ([`a476a46`](https://github.com//Byron/gitoxide/commit/a476a46b7b933a3c2fa4aa8c285beec1777a3f2d))
    - (cargo-release) version 0.5.0 ([`c767e07`](https://github.com//Byron/gitoxide/commit/c767e07ccfc58a28e3e8ec22b590afdf0d92b9f2))
    - (cargo-release) version 0.8.0 ([`47c00c2`](https://github.com//Byron/gitoxide/commit/47c00c2228cf25c79e1fa3eb4229c7ab24de91e5))
    - cargo clippy Rust 1.48 ([`475a68c`](https://github.com//Byron/gitoxide/commit/475a68ce33b895de911939c51afa159df534f7b8))
    - finish refactoring git-odb ([`ec282ae`](https://github.com//Byron/gitoxide/commit/ec282ae1a3d9f16eb9c89a44e17259112d097a41))
    - (cargo-release) version 0.7.0 ([`7fa7bae`](https://github.com//Byron/gitoxide/commit/7fa7baeb3e7d008a25e4d714eff908e2516c828b))
    - refactor ([`6b909a2`](https://github.com//Byron/gitoxide/commit/6b909a22cf981b33060cb6f1324ec3231146d159))
    - refactor ([`b511a2b`](https://github.com//Byron/gitoxide/commit/b511a2b1d9b6d55b1937ad3f4bbbb331b5cdd9a3))
    - refactor ([`8c658da`](https://github.com//Byron/gitoxide/commit/8c658da05a4649814eef9f7ab57525aff0605afc))
    - [commitgraph] Implement basic commit-graph file verification. ([`2571113`](https://github.com//Byron/gitoxide/commit/2571113fea516737acedac08d66632ead499b474))
    - [commitgraph] Add `Graph::at` constructor. ([`a783052`](https://github.com//Byron/gitoxide/commit/a783052d0cc2d3c9fa1dda3ea77286a79690d2c1))
    - [commitgraph] Stub out commit-graph-verify plumbing command. ([`aacf0f0`](https://github.com//Byron/gitoxide/commit/aacf0f05a909e5b7d9ffd5623ef9833e0465be93))
    - remove dash in all repository links ([`98c1360`](https://github.com//Byron/gitoxide/commit/98c1360ba4d2fb3443602b7da8775906224feb1d))
</details>

## v0.4.1 (2020-09-18)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 5 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.1 ([`105c501`](https://github.com//Byron/gitoxide/commit/105c50132c8ad1f15ace0821278a11b06c81103c))
    - Finish removal of rust 2018 idioms ([`0d1699e`](https://github.com//Byron/gitoxide/commit/0d1699e0e0bc9052be0a74b1b3f3d3eeeec39e3e))
    - (cargo-release) version 0.6.0 ([`9ef184e`](https://github.com//Byron/gitoxide/commit/9ef184e35712f938fb4f9f6da7390a8777a9284e))
    - Switch to prodash 10 and safe a lot of trait bounds in the process ([`e2fb1d9`](https://github.com//Byron/gitoxide/commit/e2fb1d944b4d803a11c91f868b831d406fb5e35f))
</details>

## v0.4.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 33 commits contributed to the release over the course of 28 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 ([`92e8b27`](https://github.com//Byron/gitoxide/commit/92e8b273654c3dedce60de244944683c7cf153e7))
    - (cargo-release) version 0.4.0 ([`2b1bca8`](https://github.com//Byron/gitoxide/commit/2b1bca83c453544972e370dc0adff57cb7590b42))
    - (cargo-release) version 0.4.0 ([`2272fa4`](https://github.com//Byron/gitoxide/commit/2272fa4bcacdaf1898e4cd8b791232fc1321227f))
    - (cargo-release) version 0.4.0 ([`0d7b60e`](https://github.com//Byron/gitoxide/commit/0d7b60e856325009431172e1df742a1cd2165575))
    - (cargo-release) version 0.5.0 ([`82b7313`](https://github.com//Byron/gitoxide/commit/82b73131b79ec3c42a712dad1c0766a72209d737))
    - thanks clippy ([`e5d80b1`](https://github.com//Byron/gitoxide/commit/e5d80b19b83dc03d49606b7ccba20ff0c39bc5d9))
    - [clone] make cloning the linux kernel work ([`e780526`](https://github.com//Byron/gitoxide/commit/e78052649c734f16f4d154edcbf54f4cc4484f5e))
    - refactor ([`dc022ce`](https://github.com//Byron/gitoxide/commit/dc022ce94505ce091e52fd64076bba01f0fe0eb0))
    - [clone] refs can now be written into a specified directory ([`fb1f048`](https://github.com//Byron/gitoxide/commit/fb1f04837be994fa5bcb9aa24f25b5f4f72e4ce0))
    - [clone] First version of writing references, but… ([`445be27`](https://github.com//Byron/gitoxide/commit/445be27cf81663ba4fe941c00262448444efbac2))
    - [clone] better JSON output for pack-receive ([`bc6b8e8`](https://github.com//Byron/gitoxide/commit/bc6b8e86f258835b6da60ea7e749fe01243a4010))
    - [clone] initial implementation of Json format for pack-receive ([`9090ac6`](https://github.com//Byron/gitoxide/commit/9090ac6c6acdb5e050c597a279a963b48c08871a))
    - [clone] nicer pack-receive output for humans ([`09c6c57`](https://github.com//Byron/gitoxide/commit/09c6c576ddb4c791b1b5f9b1812485e73a080f93))
    - [clone] Don't hide nested pack-decoding information ([`4d4be97`](https://github.com//Byron/gitoxide/commit/4d4be975707d017a67a0b2c081a07c4092b2801d))
    - [clone] When unpacking peeled refs, use the object that refers to the tag… ([`fe8bb39`](https://github.com//Byron/gitoxide/commit/fe8bb3985bd5529a36c71fa170ca48df91060491))
    - [clone] minor refactor; it's definitely the read() that doesn't work… ([`406829b`](https://github.com//Byron/gitoxide/commit/406829b951164673c0b8152d1e9de76f1318df0a))
    - [clone] This actually works: first MVP of retrieving packs via clone ([`c06d819`](https://github.com//Byron/gitoxide/commit/c06d8194173f9ec468ddd0faf72dd6d8dbf7d35d))
    - [clone] First step towards implementing a working pack receiving… ([`264ec82`](https://github.com//Byron/gitoxide/commit/264ec821ca92a08d1756222abab11ffebb6dc0ff))
    - [clone] Support for reading multi-step negoritaions, but… ([`507d342`](https://github.com//Byron/gitoxide/commit/507d342dfe2a714a4dd0bc100d96ed9e64a58243))
    - [clone] support for progress that can handle writing pack files ([`46e0055`](https://github.com//Byron/gitoxide/commit/46e0055eab47e402807b15c63b6a4577f5c0b7bb))
    - [clone] Actually pass pack file to the delegate ([`94c5e62`](https://github.com//Byron/gitoxide/commit/94c5e62b274b0fc39f64ee5b04273db5ead4a470))
    - refactor ([`61e9812`](https://github.com//Byron/gitoxide/commit/61e98128ddd85cde1a352b70f83870fdea0c6bac))
    - [ref-ls] first step towards supporting negotiation ([`27b6d2d`](https://github.com//Byron/gitoxide/commit/27b6d2d24a92c1ffc1579a116a044cece50d9d20))
    - [ref-ls] usable JSON output ([`735ae50`](https://github.com//Byron/gitoxide/commit/735ae50c1fdf1a7c403782f40b5234ea881da7b1))
    - [ref-ls] Fix progress display ([`2fcb557`](https://github.com//Byron/gitoxide/commit/2fcb557dce941eb94ca60f46ecee86b94e029db7))
    - [ref-ls] Make things compile ([`b6506a4`](https://github.com//Byron/gitoxide/commit/b6506a46ef59d8e25b245fa8caac5b4de4fdaa3d))
    - [ref-ls] And it even doesn't work if it is the very same transport ([`4ba50fe`](https://github.com//Byron/gitoxide/commit/4ba50fe06f7423c31f4cd78079d51ef3ffd51920))
    - [ref-ls] first actual call of ls-remote, but… ([`5fc4330`](https://github.com//Byron/gitoxide/commit/5fc4330eca42a0a3ba6c14fe8c27aeda16e440ec))
    - [ref-ls] Frame for remote-ref-ls command in gitoxide-core ([`161e7df`](https://github.com//Byron/gitoxide/commit/161e7df34a53db40551879c6d2319ee775dfd551))
    - bump git-features to 0.4 to allow publishes after breaking changes ([`9d6b879`](https://github.com//Byron/gitoxide/commit/9d6b8790e2edd7fa01b3239adff86a7cd2393f10))
    - [clone] first sketch of transport layer's connection logic ([`f10cee5`](https://github.com//Byron/gitoxide/commit/f10cee5638a220fff629af274baebbcc0f4f0f61))
    - [clone] link up lean plumbing command with gitoxide-core: pack-receive ([`5ea49c8`](https://github.com//Byron/gitoxide/commit/5ea49c8aa0d449bed98ce0147ad222ff25c27c32))
    - Allow dual-licensing with Apache 2.0 ([`ea353eb`](https://github.com//Byron/gitoxide/commit/ea353eb02fd4f75508600cc5676107bc7e627f1e))
</details>

## v0.3.0 (2020-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 67 commits contributed to the release over the course of 30 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump minor version to 0.3 ([`4351e28`](https://github.com//Byron/gitoxide/commit/4351e2871c9dcf342b8471fffa74cae338a53269))
    - first step towards parallelizing file hashes and traversal! ([`9573836`](https://github.com//Byron/gitoxide/commit/95738369e0d3accf7f6239c8cd966a7f5c36825a))
    - update to quick-error 2.0 ([`4b1b784`](https://github.com//Byron/gitoxide/commit/4b1b7849b47a54092b49821c39e864c86adda979))
    - better progress for Sha1 of pack and index ([`310a59e`](https://github.com//Byron/gitoxide/commit/310a59ee99ce78a4f14326c0058ea0c543a1d24c))
    - first successful test of moving the streaming iterator into its own thread ([`c9fcb68`](https://github.com//Byron/gitoxide/commit/c9fcb68c644c96a15cb9956a754bec7b65bb5fbd))
    - unify used ranges for line renderer amond pretty and lean interface ([`f59f66e`](https://github.com//Byron/gitoxide/commit/f59f66e189732f567414f68c7463364e510f41c4))
    - Add convenience method to get a new bundle for the index/data just written ([`a6d74ad`](https://github.com//Byron/gitoxide/commit/a6d74ad7b65cdc293c8504dae73ea1c717e5bfca))
    - support for JSON format output ([`1931575`](https://github.com//Byron/gitoxide/commit/19315750f4f409e3f105c3c4054c4afbef91daad))
    - first pieces of the index-from-pack journey tests ([`181d69c`](https://github.com//Byron/gitoxide/commit/181d69c1da46a931c513cbd7d8bca7b2fa53351c))
    - more flexible error types for processors - anything goes ([`be3a947`](https://github.com//Byron/gitoxide/commit/be3a947ba6197319fea0b38e48008850cc971bf6))
    - refactor ([`c7dd581`](https://github.com//Byron/gitoxide/commit/c7dd581348a05146d7a79f7622bf30a08d34f474))
    - interrupt support for pretty plumbing ([`bca7ce2`](https://github.com//Byron/gitoxide/commit/bca7ce2e668a4be2600d2d04d00f46b21c82eee2))
    - count object types as well ([`e04a8d1`](https://github.com//Byron/gitoxide/commit/e04a8d16fda3712663d8d9220f3a017e668b6283))
    - refactor ([`b77d148`](https://github.com//Byron/gitoxide/commit/b77d148ed1c5aec31cb0493b4f1e0f2d82d7e641))
    - remove memory mode entirely (and some complexity with it) ([`8812e91`](https://github.com//Byron/gitoxide/commit/8812e916a21983868a37c4aade10f79a1dc9b926))
    - turns out you never want to keep deltas in memory ([`657aa2c`](https://github.com//Byron/gitoxide/commit/657aa2c38673cf10174f42bcb97039ac37b2926e))
    - Remove support for keeping compressed memory to reduce the index size ([`1e2ec7e`](https://github.com//Byron/gitoxide/commit/1e2ec7e9d0ef2f2a4908860672080e411e945bff))
    - Use call to produce the resolver, allowing to delay opening a file mapping… ([`dd30e8d`](https://github.com//Byron/gitoxide/commit/dd30e8d3c8b6754bd90e2777ec0153e158d4a708))
    - minor fixes after first local tests - it's up to twice as fast!! ([`43c7fd1`](https://github.com//Byron/gitoxide/commit/43c7fd1f81b9b4c938f99c0bf1deabdf121226b9))
    - quick and dirty impl of lean command-line for index-from-pack ([`9660bbf`](https://github.com//Byron/gitoxide/commit/9660bbffd8ace621178b067e22d227ef8c50ba84))
    - quick and dirty impl of gitoxide layer for bundle writing, aka index-pack ([`e78386b`](https://github.com//Byron/gitoxide/commit/e78386b824010c5ca8efca87604c339d40b545ae))
    - first sketch of gitoxide index::from_pack(…) ([`da0eace`](https://github.com//Byron/gitoxide/commit/da0eacea838a0fcdf09e052334f944269a153f42))
    - refactor; better tests ([`12d14bf`](https://github.com//Byron/gitoxide/commit/12d14bfe2aa089723a395287c5100aad6e838935))
    - update tasks ([`45c3520`](https://github.com//Byron/gitoxide/commit/45c352009092dbfd80bcb3e367d848d5b10737d4))
    - it looks like something is wrong with the object stream implementation ([`d187b5a`](https://github.com//Byron/gitoxide/commit/d187b5a769b62ec706c1265e0db8403327d8e92d))
    - Loose object verifycation - but it doesn't seem to work as expected ([`9dd5676`](https://github.com//Byron/gitoxide/commit/9dd56761ae75eac691449cd86a1be04c11c0fecb))
    - prepare full 'verify' implementation ([`ee45c7f`](https://github.com//Byron/gitoxide/commit/ee45c7f47b95fc406cc5922a322c8fd6c0f52775))
    - refactor ([`0a33b24`](https://github.com//Byron/gitoxide/commit/0a33b24f5b61ccdf1358f1e9adcf0f6fd4099c1c))
    - Allow sink-compress configuration; choose best algorithm ([`29b9c23`](https://github.com//Byron/gitoxide/commit/29b9c230e35ba9b4334797b63ab9fa88c2fe59d0))
    - Always compress values when using a sink when exploding packs ([`70562fa`](https://github.com//Byron/gitoxide/commit/70562fa123faf51bd72a4aedb12acb0d3247e4e2))
    - Most tests and clearer error message if object directory is inaccessible ([`1d8f597`](https://github.com//Byron/gitoxide/commit/1d8f5974a5c754750f46697370cb2551f6660666))
    - Nice error message on failure ([`adbc82c`](https://github.com//Byron/gitoxide/commit/adbc82c31450681fcb38233eeb8095efc5e52a18))
    - inform about deleted files using progress ([`a3ee516`](https://github.com//Byron/gitoxide/commit/a3ee5160093c9326006fcedbf1f507d8978a97c2))
    - Don't uncondionally delete packs/indices on explode :D ([`1979715`](https://github.com//Byron/gitoxide/commit/19797156bafbacbaf0a53d01d72bbe86881aea9b))
    - The first 'explode' implementation… ([`0d31ad1`](https://github.com//Byron/gitoxide/commit/0d31ad1b61997fa0d0692c5919fb8032ffaaa35b))
    - Get all pieces ready for action ([`1805d64`](https://github.com//Byron/gitoxide/commit/1805d64b9222d6a05a8718f04b29b789c1f42fea))
    - Pass option for safety checks down to explode(…) ([`0bcb790`](https://github.com//Byron/gitoxide/commit/0bcb790dc8c35097916876afbb68bbfcc894c369))
    - Restore original verification functionality ([`0e3c1b9`](https://github.com//Byron/gitoxide/commit/0e3c1b9bb9841ae4bb0ef1df2e72e950f7a7fd33))
    - nearly there! Interesting that anyhow errors must be sync! ([`eaee77e`](https://github.com//Byron/gitoxide/commit/eaee77ea4ce10f5c85b42a33452eef996adac3bf))
    - refactor ([`bae7781`](https://github.com//Byron/gitoxide/commit/bae7781ab549f0daa73980a29d18d64320601470))
    - refactor ([`f66b116`](https://github.com//Byron/gitoxide/commit/f66b116ddfbee62c3e20a4c5e7cd878fbf064195))
    - basic tests and CLI args for explode pack ([`f932256`](https://github.com//Byron/gitoxide/commit/f932256a62d6fc5d5558446de079fb666ddc27da))
    - refactor ([`d3c00c8`](https://github.com//Byron/gitoxide/commit/d3c00c841ee1aeda6bb0534fe365db13c31f8d3c))
    - (cargo-release) version 0.2.0 ([`76fe0ab`](https://github.com//Byron/gitoxide/commit/76fe0ab5f0b58504a5ea5adb74b349b9d588e51e))
    - (cargo-release) version 0.2.0 ([`0bb8314`](https://github.com//Byron/gitoxide/commit/0bb831480d8657e1bb29ee7009aeac673471403e))
    - Run clippy first; pacify clippy ([`0a5b883`](https://github.com//Byron/gitoxide/commit/0a5b883c22f2df8a6d51f75c5e09bdfdf276fee4))
    - use faster algorithm by default ([`bb45c3d`](https://github.com//Byron/gitoxide/commit/bb45c3d8a2aabf87231981000240f0444abf6fc4))
    - refactor; enable testing of reverse-delta lookup ([`512daf9`](https://github.com//Byron/gitoxide/commit/512daf94038f675353271c930694e0577ac746b4))
    - Fix clippy ([`ec40e09`](https://github.com//Byron/gitoxide/commit/ec40e093d72f93d86168f39ebaca5b122ca0bec3))
    - refactor ([`fdfab40`](https://github.com//Byron/gitoxide/commit/fdfab408c38087c5afcdd028e988089c56311baf))
    - Easy access to sorted offsets in pack index files ([`d93540f`](https://github.com//Byron/gitoxide/commit/d93540fe2a6d4bb70248e82d039d6a2665354ef3))
    - refactor ([`cb8d561`](https://github.com//Byron/gitoxide/commit/cb8d56101bdc4cd7e3fa95ac79f82c1cda99871c))
    - Change course and do pack streaming first ([`bcb275e`](https://github.com//Byron/gitoxide/commit/bcb275e91cfd6f0a71b3cb59a2b706b60608a594))
    - Switch to latest quick-error ([`9760856`](https://github.com//Byron/gitoxide/commit/976085614ee13a19fc1347209259a3dcf36ef95b))
    - Fully implement --encode and --re-encode flags ([`a7cfac8`](https://github.com//Byron/gitoxide/commit/a7cfac83ddd859d9c2c25e457c0d7043738792dc))
    - prepare for re-encoding each pack object ([`afae684`](https://github.com//Byron/gitoxide/commit/afae684c72e5dc4b718976056dd5d34ed61de72a))
    - move git_object::Id into git_object::owned::Id - much better already! ([`50c7136`](https://github.com//Byron/gitoxide/commit/50c71368a69f57b0a43061df105685e992ed384a))
    - fix naming change, which was introduced accidentally ([`fbb9f98`](https://github.com//Byron/gitoxide/commit/fbb9f98508ec722e192466e28ded47aef2fb78b3))
    - refactor ([`34e85f2`](https://github.com//Byron/gitoxide/commit/34e85f2242b12ec1560b8e50bc9ab447cd1805fc))
    - refactor ([`2888f1b`](https://github.com//Byron/gitoxide/commit/2888f1b10a2baf40155544e667ddd461f3ddc938))
    - refactor ([`dcacd3b`](https://github.com//Byron/gitoxide/commit/dcacd3b06d7a4532c600dfdf62e03561e8ed55ef))
    - refactor ([`b113da9`](https://github.com//Byron/gitoxide/commit/b113da945715f9611eb0fb79925d1239eaf1569c))
    - refactor ([`bed5dc8`](https://github.com//Byron/gitoxide/commit/bed5dc80c5b307c6d35f7b4405693dce1f7f6d71))
    - refactor ([`8b416d4`](https://github.com//Byron/gitoxide/commit/8b416d4b8417c04ea5d3527a88190d867dc8b7c2))
    - Respect thread limit in 'in_parallel' ([`babfd84`](https://github.com//Byron/gitoxide/commit/babfd84cba77ef7a0f541ba921b31ebd3f3c50e3))
    - pass threadlimit down from CLIs ([`f98c5b1`](https://github.com//Byron/gitoxide/commit/f98c5b160db80a7cac530e18b9256562c25be47f))
    - add new Context argument to support more configuration options ([`7c5d8b8`](https://github.com//Byron/gitoxide/commit/7c5d8b8bb318e59a59ad74ad767a1525e2833632))
</details>

## v0.1.0 (2020-07-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 25 commits contributed to the release over the course of 19 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Make crates publishable ([`5688a34`](https://github.com//Byron/gitoxide/commit/5688a3427ff3673e1422d43106f4d685fa837aed))
    - Add metadata to allow docs.rs build all featueres ([`10f9386`](https://github.com//Byron/gitoxide/commit/10f9386a12decc1f13999aee72be484c8f6d48ce))
    - support for json in pretty-plumbing and gitoxide (on demand) ([`b3780f8`](https://github.com//Byron/gitoxide/commit/b3780f87438d34b372c48b7385199f7ea22b3965))
    - git-odb with serde support ([`0da930c`](https://github.com//Byron/gitoxide/commit/0da930cf23f215cc1e2bda8f7340a5d69370735a))
    - pass serde1 through from gitoxide ([`1991b9f`](https://github.com//Byron/gitoxide/commit/1991b9ffef1a2b9a402d080d0a31e0857c434bc4))
    - don't print 'OK' at the end of verify-pack ([`4956ef2`](https://github.com//Byron/gitoxide/commit/4956ef23783104d64c35983934c69db918f3027a))
    - \#[forbid(unsafe)] for all crates ([`afda803`](https://github.com//Byron/gitoxide/commit/afda8039259b7a30cfed5dbcdd9caf4773b4c234))
    - Allow for more screen space when formatting ([`6794300`](https://github.com//Byron/gitoxide/commit/67943002e7f4215b5383bd0538786ce2857f011e))
    - disable LRU cache if we have to get statistics ([`befba3b`](https://github.com//Byron/gitoxide/commit/befba3b769195fb592d714afe12194a61ae4a330))
    - wonderful statistics on compression efficiency! ([`1bb09c5`](https://github.com//Byron/gitoxide/commit/1bb09c509dae4e493ab05022bbf51c0b1786d479))
    - pretty-print objects per delta chain length ([`66553b1`](https://github.com//Byron/gitoxide/commit/66553b1c544a25c9703641ab6ea1a4a2a08b945a))
    - count objects per chain level ([`209d53f`](https://github.com//Byron/gitoxide/commit/209d53f531ec9bcffbb04ba060447bee59ad26f6))
    - Pretty-printing of some statistics ([`125b565`](https://github.com//Byron/gitoxide/commit/125b565f0fb4085c615fdf136f35a2285d69966a))
    - fix pretty build ([`6adf615`](https://github.com//Byron/gitoxide/commit/6adf615ed7d6c488c25589940fc0a55bf0fb3d5c))
    - pass average stats through to the top level ([`5b4979c`](https://github.com//Byron/gitoxide/commit/5b4979c1dfeb9a29974dd4e6529ae5da074d0b1a))
    - first very basic progress implementation ([`b820717`](https://github.com//Byron/gitoxide/commit/b8207177daee8a9ffa23c7c052cf9ca651b15804))
    - Pass progress everywhere, for now just to discard it ([`da3ae1c`](https://github.com//Byron/gitoxide/commit/da3ae1c82cd726b8fae9b8d26069719930e9ba99))
    - Control which hashing crates to use from the top-level as well. ([`dfe9b20`](https://github.com//Byron/gitoxide/commit/dfe9b203b2e877a7e345b4f2942bf5a1582ab43e))
    - Use git-features to toggle 'parallel' mode from the 'gitoxide' level ([`d944fbf`](https://github.com//Byron/gitoxide/commit/d944fbf181acc5fb83a841613174702af1e074d6))
    - first working version of actually parallel `in_parallel` ([`145ee39`](https://github.com//Byron/gitoxide/commit/145ee399e2c057aec3330e26bafb7910ca7dc56d))
    - Support for verifying pack files and index files ([`b09b4e1`](https://github.com//Byron/gitoxide/commit/b09b4e1f35c3802dfd3418bda42b96828acd9ec8))
    - cleanup - don't build and run tests while there is nothing to test ([`4a153da`](https://github.com//Byron/gitoxide/commit/4a153da0d60a30615fc402cfecb977f0d771594a))
    - First basic index file verification ([`994700f`](https://github.com//Byron/gitoxide/commit/994700f96b058a0910e734bdecced44bd0a7ea5d))
    - reorganize crates to make 'gitoxide' the CLI, and 'gitoxide-core' the library ([`0ac9c5a`](https://github.com//Byron/gitoxide/commit/0ac9c5af0cbb562d3cb48a661736afd98dd1a940))
    - rename grit to 'gitoxide', CLI name is 'gio' ([`9d6007f`](https://github.com//Byron/gitoxide/commit/9d6007f83b3b018d736d58aa0722b83b9cffb228))
</details>

