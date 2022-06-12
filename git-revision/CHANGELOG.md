# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### New Features

- support for parsing `revspec`s on a low level, meaning that the ground work for actually resolving them is done.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 74 commits contributed to the release over the course of 5 calendar days.
 - 24 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
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
    - make fmt ([`c665aef`](https://github.com/Byron/gitoxide/commit/c665aef4270c5ee54da89ee015cc0affd6337608))
    - thanks clippy ([`1bbd3f4`](https://github.com/Byron/gitoxide/commit/1bbd3f471d78e53a76b3e708c755fc9d72fc28fe))
    - thanks clippy ([`b93fa40`](https://github.com/Byron/gitoxide/commit/b93fa40a9abcfb7390276e4254f696c0cac2abb1))
    - thanks clippy ([`6dc9c44`](https://github.com/Byron/gitoxide/commit/6dc9c44fb2770d93badb8e1d506b7601107ea586))
    - thanks clippy ([`ec0ff74`](https://github.com/Byron/gitoxide/commit/ec0ff7404ba7df80bf98fd6d28b13426c2e3ee6c))
    - thanks clippy ([`1b40259`](https://github.com/Byron/gitoxide/commit/1b402596bb581ea84b285282a44bf81752c14bba))
    - thanks clippy ([`6d08d5f`](https://github.com/Byron/gitoxide/commit/6d08d5f518a94426420c973b8e6e561ef558627c))
    - thanks clippy ([`1f0545f`](https://github.com/Byron/gitoxide/commit/1f0545f3169824f4953727f7319324b60baaf92f))
    - thanks clippy ([`2bc1acc`](https://github.com/Byron/gitoxide/commit/2bc1acc1816ef95b60c0192ef8d956558ff58bb9))
</details>

## 0.2.0 (2022-05-18)

### Bug Fixes

 - <csr-id-99365f221065ebc315ac80940ad72cae253743bc/> Support for in truncated history in git-describe
   This allows `describe()` to work on shallow clones.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 42 calendar days.
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
    - make fmt ([`50ff7aa`](https://github.com/Byron/gitoxide/commit/50ff7aa7fa86e5e2a94fb15aab86470532ac3f51))
 * **[#384](https://github.com/Byron/gitoxide/issues/384)**
    - No need to isolate archives by crate name ([`19d46f3`](https://github.com/Byron/gitoxide/commit/19d46f35440419b9911b6e2bca2cfc975865dce9))
    - add archive files via git-lfs ([`7202a1c`](https://github.com/Byron/gitoxide/commit/7202a1c4734ad904c026ee3e4e2143c0461d51a2))
    - Assure we don't pick up unnecessary files during publishing ([`545b2d5`](https://github.com/Byron/gitoxide/commit/545b2d5121ba64efaee7564d5191cec37661efd7))
    - auto-set commit.gpgsign=false when executing git ([`c23feb6`](https://github.com/Byron/gitoxide/commit/c23feb64ad157180cfba8a11c882b829733ea8f6))
 * **Uncategorized**
    - Release git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0 ([`349c590`](https://github.com/Byron/gitoxide/commit/349c5904b0dac350838a896759d51576b66880a7))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
</details>

## 0.1.0 (2022-04-05)

<csr-id-0a7776b8cce4c40c391f46542f6e7ba6830d6fc0/>

### Refactor (BREAKING)

 - <csr-id-0a7776b8cce4c40c391f46542f6e7ba6830d6fc0/> Make `describe::Format` more consistent with other builder APIs
   Configuration methods now take an argument which makes it more
   straightforward to use for most.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 42 commits contributed to the release over the course of 56 calendar days.
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

