# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

A maintenance release to properly dealing with previously breaking changes in `git-hash`.

## v0.8.0 (2021-10-15)

This release contains no functional changes, but is considered breaking for safety reasons 
as `git-traverse` is signalling a breaking change.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 22 commits contributed to the release over the course of 31 calendar days.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#198](https://github.com//Byron/gitoxide/issues/198)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com//Byron/gitoxide/issues/198)**
    - Fix stop-release-for-changelog logic and fix all affected changelogs ([`52b38bc`](https://github.com//Byron/gitoxide/commit/52b38bc4856be5ba8b5372a3dd20f5d06504e7ed))
    - deduplicate conventional message ids ([`e695eda`](https://github.com//Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - regenerate all changelogs to get links ([`0c81769`](https://github.com//Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com//Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - respect release-wide ignore list to allow removing entire conventional headlines ([`145103d`](https://github.com//Byron/gitoxide/commit/145103d4aa715386da9d4953f7f85fadc49fff9a))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com//Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com//Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com//Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - Generate changelogs with details ([`e1861ca`](https://github.com//Byron/gitoxide/commit/e1861caa435d312953a9fea7ceff6d2e07b03443))
    - Update all changelogs with details ([`58ab2ae`](https://github.com//Byron/gitoxide/commit/58ab2aee23ba70a536e9487b44fb04c610374d1a))
    - Update changelogs ([`c857d61`](https://github.com//Byron/gitoxide/commit/c857d61ce3ce342012a2c4ba10a8327822aa530e))
    - Avoid adding newlines which make writing unstable ([`6b5c394`](https://github.com//Byron/gitoxide/commit/6b5c394f49282a8d09c2a9ffece840e4683572db))
    - Fix section headline level ([`9d6f263`](https://github.com//Byron/gitoxide/commit/9d6f263beef289d227dec1acc2d4240087cb9be6))
    - Write first version of changlogs thus far… ([`719b6bd`](https://github.com//Byron/gitoxide/commit/719b6bdf543b8269ccafad9ad6b46e0c55efaa38))
    - add panicking `Target::id()` and `TargetRef::id()` ([`4ed4b2d`](https://github.com//Byron/gitoxide/commit/4ed4b2da50557aff540685441f4b5c7d5e582977))
    - loose reference iteration with non-dir prefixes… ([`293bfc0`](https://github.com//Byron/gitoxide/commit/293bfc0278c5983c0beaec93253fb51f00d81156))
    - git-ref(docs): improve changelog format ([`90e6128`](https://github.com//Byron/gitoxide/commit/90e6128727932f917c485f411e623fc6a9c2ad4d))
 * **Uncategorized**
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com//Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com//Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - Update changelogs just for fun ([`21541b3`](https://github.com//Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
    - Merge branch 'main' into changelog-generation ([`c956f33`](https://github.com//Byron/gitoxide/commit/c956f3351d766c748faf0460780e32ac8dfe8165))
    - Bump git-traverse v0.9.0, safety bump 8 crates ([`d39fabb`](https://github.com//Byron/gitoxide/commit/d39fabb8757369aa19452a457f610fe21dc13a14))
</details>

## v0.7.3 (2021-09-10)

- Compatibility with Rust 1.55. It informed about the incorrect usage of `std::io::ErrorKind::Other` which this crate also dependent on causing
  runtime bugs. The latter were detected by tests and this release contains the fix to not rely on certain behaviour anymore.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.7.3 ([`b0a9815`](https://github.com//Byron/gitoxide/commit/b0a98157ab3b240af027acb9965c981a543e55fa))
    - Update changelogs once more… ([`d57d279`](https://github.com//Byron/gitoxide/commit/d57d279dc603cf450c151bbb6dc6c6505cc6da10))
    - Update changelogs retro-actively… ([`78cfe0a`](https://github.com//Byron/gitoxide/commit/78cfe0ac341c6c2257743d913884b50042078e6c))
</details>

## v0.7.2 (2021-09-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.7.2 ([`e940e9a`](https://github.com//Byron/gitoxide/commit/e940e9a21938035eb8791bba19cc16814a0fb4e7))
    - [#195] Fix previously incorrect usage of io::Kind::Other… ([`4dae07d`](https://github.com//Byron/gitoxide/commit/4dae07dc7f562395a174be6cb2220e754ff902f7))
    - thanks clippy ([`4701296`](https://github.com//Byron/gitoxide/commit/4701296bd5e2c4ad2f80f4e1de498db49f93385a))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.7.1 (2021-09-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.7.1 ([`d34191d`](https://github.com//Byron/gitoxide/commit/d34191dfd3ac3b34a3ae0d772c8b4302e5115fd6))
    - Bump git-object v0.14.0 ([`d4fc81f`](https://github.com//Byron/gitoxide/commit/d4fc81f6390443f8c8561d91ac27ea4a6318fb62))
</details>

## v0.7.0 (2021-09-07)

### Breaking

* Replace `transaction::Create` with `transaction::PreviousValue` and remove `transaction::Create`
* Remove `file::Reference` in favor of `Reference`
* Move `file::log::Line` to `log::Line`
* `TargetRef::Symbolic(&BStr)` -> `TargetRef::Symbolic(FullNameRef)`
* replace `Transaction::namespacce()` with `file::Store::namespace`
 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 49 commits contributed to the release over the course of 5 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [repository #190] refactor ([`e7188e0`](https://github.com//Byron/gitoxide/commit/e7188e047529cb0f4b20b3876f36b4592e9d2dc4))
    - [ref #190] refactor ([`010be48`](https://github.com//Byron/gitoxide/commit/010be48d2cd2dfebf7a1b6302e94b5f2e95fedc6))
    - [ref #190] fix tests ([`e426e15`](https://github.com//Byron/gitoxide/commit/e426e15188d8ec38ee0029f1d080dbab9afd8642))
    - [ref #190] don't provide namespace support for loose and packed refs… ([`c663da1`](https://github.com//Byron/gitoxide/commit/c663da16646bc3371e5a31f5c488a775aac4f795))
    - [ref #190] find() with namespace support ([`1240c21`](https://github.com//Byron/gitoxide/commit/1240c21a353c7df736f40b6639076af94eae0f15))
    - [ref #190] prepare test for namespaced find(…) ([`5fcd0e4`](https://github.com//Byron/gitoxide/commit/5fcd0e4c3c803a372360ef4cc2a7663b17ccebdb))
    - [repository #190] leverage git-ref namespace support ([`1aa9c11`](https://github.com//Byron/gitoxide/commit/1aa9c113488175f03758f8a64338a33b3417dd87))
    - [ref #190] iteration with namespace support ([`d5987d4`](https://github.com//Byron/gitoxide/commit/d5987d41753cf083573d86e8d5bc86c7a825605c))
    - [ref #190] refactor ([`3c7968c`](https://github.com//Byron/gitoxide/commit/3c7968c7fe8ac166b01f5338b23f817899dc085e))
    - [repository #190] prepare for namespacing support on file store level ([`d2d1db6`](https://github.com//Byron/gitoxide/commit/d2d1db647e6ad0dd92b88ce57df866f5195b8dd6))
    - [repository #190] refactor ([`609c249`](https://github.com//Byron/gitoxide/commit/609c249916ca64f4beecdab86eb4562adbd1ca4f))
    - [ref #190] refactor ([`1ef6cb3`](https://github.com//Byron/gitoxide/commit/1ef6cb344176aeafcc61a1f1af503a3f8afd1f77))
    - [repository #190] fix build ([`f5e118c`](https://github.com//Byron/gitoxide/commit/f5e118c8871e45ed3db9da9cd6bc63a5ea99621e))
    - [repository #190] note a known limitation about finding references in namespaces… ([`d335731`](https://github.com//Byron/gitoxide/commit/d3357318cf100fc3e0751e5b6de3922b1c209ddb))
    - [ref #190] more assetions to understand 'find(…)' for namespaced refs… ([`f58a0ff`](https://github.com//Byron/gitoxide/commit/f58a0ff8be6144d1dcb97f9b8030e1ee36ce41de))
    - [repository #190] transparent namespace support ([`d14f073`](https://github.com//Byron/gitoxide/commit/d14f073707c2f4641a271ba7965ec8281638e8df))
    - [ref #190] Make References sortable ([`16b2232`](https://github.com//Byron/gitoxide/commit/16b2232c70ad331e17e76ccca3b950963906aa81))
    - [repository #190] cleanup usage of bstr… ([`e4411ff`](https://github.com//Byron/gitoxide/commit/e4411ff43b24af79fefeaa4411e004dc504a4e2a))
    - [ref #190] more conversion trait impls ([`1795a33`](https://github.com//Byron/gitoxide/commit/1795a333c05c60a1a2f3164d5c4c78289eb7050c))
    - Bump git-hash v0.6.0 ([`6efd90d`](https://github.com//Byron/gitoxide/commit/6efd90db54f7f7441b76159dba3be80c15657a3d))
    - [repository #190] obtain the kind fo hash used in a repo ([`a985491`](https://github.com//Byron/gitoxide/commit/a985491bcea5f76942b863de8a9a89dd235dd0c9))
    - [ref #190] refactor ([`e34be7e`](https://github.com//Byron/gitoxide/commit/e34be7e24ee49a539b6ee8dc5737fdb23f416922))
    - [ref #190] more Target conversions… ([`1fe1b42`](https://github.com//Byron/gitoxide/commit/1fe1b42ac2b04f8145fc7312ea03cb47f791aec5))
    - [repository #190] refactor ([`7a111b1`](https://github.com//Byron/gitoxide/commit/7a111b126cfb318acb2d09d119315150a38b7cd3))
    - [ref #190] refactor ([`49fe1dc`](https://github.com//Byron/gitoxide/commit/49fe1dc37c040206839c9d4399001ff12dc91174))
    - [ref #190] reverse reflog ergonomics ([`2de86f9`](https://github.com//Byron/gitoxide/commit/2de86f904f6ee63e292f9c701cc3524e8bfe87e4))
    - [ref #190] check for zero sized buffers in reverse log iterators… ([`998c7c6`](https://github.com//Byron/gitoxide/commit/998c7c65abb2c3eb5fc248b11ba816d09f1bedea))
    - [ref #190] move remaining file store functions to extension trait ([`60fc215`](https://github.com//Byron/gitoxide/commit/60fc215ccac529b4a14cb9d8260ab9ddec86758a))
    - [ref #190] Move file-log-specific functionality into own extension trait. ([`0b635e9`](https://github.com//Byron/gitoxide/commit/0b635e9778a98235cc9b47b12e58a175d1ca02b7))
    - [repository #190] a major step forward with `head()` access ([`43ac4f5`](https://github.com//Byron/gitoxide/commit/43ac4f5acbe3ace5d43ed3ed1bc394d721f0e273))
    - [ref #190] cache peeled objects properly ([`2cb511e`](https://github.com//Byron/gitoxide/commit/2cb511efe5833f860f3c17b8e5f5b4cd643baddb))
    - [ref #190] fix docs ([`3e64ec1`](https://github.com//Byron/gitoxide/commit/3e64ec102146e348b8d870377f180f8dadf5e876))
    - Bump git-ref v0.7.0 ([`ac4413c`](https://github.com//Byron/gitoxide/commit/ac4413ce4e45703d5fe722e7220d039217f0bdef))
    - [ref #190] fix remaining tests ([`df21f25`](https://github.com//Byron/gitoxide/commit/df21f25baaf867015fc9fc46a2cf4e778b0e80ee))
    - thanks clippy ([`14dff63`](https://github.com//Byron/gitoxide/commit/14dff63fbc0d318bbc8a2618e0d72aaa98948acf))
    - [ref #190] Use Raw Reference everywhere for great simplification… ([`7aeea9c`](https://github.com//Byron/gitoxide/commit/7aeea9c36d4da04a806e68968356f8cc0dc11475))
    - [ref #190] raw reference peeling ([`9473a71`](https://github.com//Byron/gitoxide/commit/9473a71e5533e1474181241f8d3e1aebd9dea8d8))
    - [ref #190] introduce Raw reference type that simplifies everything… ([`8634341`](https://github.com//Byron/gitoxide/commit/86343416dec8026f32c57d164dec4bf9b75b6536))
    - [ref #190] more tests ([`980e16a`](https://github.com//Byron/gitoxide/commit/980e16a10806edba4553716d9533716a727f0c9e))
    - [ref #190] deletions also use PreviousValue now ([`74f85b1`](https://github.com//Byron/gitoxide/commit/74f85b1fd8d9c34eca34a5ae516c4768f96b092f))
    - [ref #190] refactor ([`0e65559`](https://github.com//Byron/gitoxide/commit/0e65559e6d5a4b06c552e99e9c463559737f4b4d))
    - [ref #190] be explicit about what the previous reflog oid is for… ([`c04c8b9`](https://github.com//Byron/gitoxide/commit/c04c8b98a074d277067cee73ddef0609419a7bb8))
    - [ref #190] don't claim there was a previous oid unnecessarily… ([`68f7fc2`](https://github.com//Byron/gitoxide/commit/68f7fc2f2f57c32412ee2e46befc9cd2fdd7e973))
    - [ref #190] refactor ([`07126d6`](https://github.com//Byron/gitoxide/commit/07126d65946e981b339b6535986597cb328a1c9e))
    - [ref #190] Allow for explicit expected previous values ([`1a4786f`](https://github.com//Byron/gitoxide/commit/1a4786fb3bdb3d3a86b026dbf04e6baef6d3c695))
    - [ref #190] prepare massive refactoring to get additional constraint ([`9741987`](https://github.com//Byron/gitoxide/commit/9741987e2f82b5ae202804882c728c1642d8e3a4))
    - [refs #190] refactor; handle value-checks in dereffed symlinks correctly ([`63bedc7`](https://github.com//Byron/gitoxide/commit/63bedc7647bb584353289e19972adf351765a526))
    - [ref #190] refactor ([`3f36a01`](https://github.com//Byron/gitoxide/commit/3f36a01976a149d518021f19d83e56dec43cfb98))
    - [object #190] More conversion methods for Object ([`78bacf9`](https://github.com//Byron/gitoxide/commit/78bacf97d669f3adfebdb093054c162cfd5214fa))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.6.1

### Bugfixes

* splits of edits to symbolic references will now 'move' the desired previous values down to the
  referents while resorting to not having any requirements in the symbolic ref instead.

## v0.6.0 (2021-08-27)

### BREAKING

- rename `file::Store::packed()` to `file::Store::packed_buffer()`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 27 commits contributed to the release over the course of 8 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [odb #180] refactor ([`eff21da`](https://github.com//Byron/gitoxide/commit/eff21dae1083042412f45cd2f7a0faaf7d6400e6))
    - [pack #179] refactor ([`ab6554b`](https://github.com//Byron/gitoxide/commit/ab6554b0cd5838f1ea4e82f6b5019798288076fa))
    - [object #177] fix docs ([`2fd23ed`](https://github.com//Byron/gitoxide/commit/2fd23ed9ad556b8e46cf650e23f0c6726e304708))
    - [object #177] tag::RefIter -> TagRefIter ([`28587c6`](https://github.com//Byron/gitoxide/commit/28587c691eb74e5cb097afb2b63f9d9e2561c45d))
    - [object #177] migrate immutable::tree to crate::tree ([`fa5cd06`](https://github.com//Byron/gitoxide/commit/fa5cd0648d5c855060ab2b75ee933851987c2dcf))
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments ([`461dc53`](https://github.com//Byron/gitoxide/commit/461dc53ba3bc07d55fdb4aad7570ba9176a8b360))
    - [object #177] rename immutable::* to immutable::*Ref ([`6deb012`](https://github.com//Byron/gitoxide/commit/6deb01291fb382b7fb9206682e319afa81bacc05))
    - Release git-object v0.13.0 ([`708fc5a`](https://github.com//Byron/gitoxide/commit/708fc5abd8af4dd7459f388c7092bf35915c6662))
    - [ref #175] follow (try_)find(_what) naming convention ([`679895c`](https://github.com//Byron/gitoxide/commit/679895cf866d643e768e353af614a55aeed2ba5c))
    - [ref #175] fix docs ([`dd1edc3`](https://github.com//Byron/gitoxide/commit/dd1edc34f88231fa95cf6f88beead700c6289ba1))
    - [ref #175] refactor log line ([`7ac948a`](https://github.com//Byron/gitoxide/commit/7ac948a8f8610b87aa2773ba2841cbfa43eecae4))
    - Merge pull request #172 from mellowagain/main ([`61aebbf`](https://github.com//Byron/gitoxide/commit/61aebbfff02eb87e0e8c49438a093a21b1134baf))
    - [ref #175] refactor ([`1243459`](https://github.com//Byron/gitoxide/commit/1243459e917b394d007bd7c157143670dc8dd51f))
    - [ref #175] make 'mutable' module private ([`a80dbcf`](https://github.com//Byron/gitoxide/commit/a80dbcf083bfcf2e291013f7b13bba9e787c5cb4))
    - Release git-actor v0.5.0 ([`a684b0f`](https://github.com//Byron/gitoxide/commit/a684b0ff96ebfc5e4b3ce78452dc21ce856a6869))
    - [actor #175] refactor ([`ec88c59`](https://github.com//Byron/gitoxide/commit/ec88c5905194150cc94db4d4c20e9f4e2f6595c3))
    - [ref #175] refactor ([`292e567`](https://github.com//Byron/gitoxide/commit/292e567eaa04a121fb4d7262bb316d37dd8ad11f))
    - Release git-actor v0.4.0 ([`16358c9`](https://github.com//Byron/gitoxide/commit/16358c9bf03604857d51bfa4dbfd2fc8c5210da7))
    - [actor #173] rename immutable::Signature to SignatureRef! ([`96461ac`](https://github.com//Byron/gitoxide/commit/96461ace776d6b351b313d4f2697f2d95b9e196e))
    - Release git-lock v1.0.0 ([`f38f72c`](https://github.com//Byron/gitoxide/commit/f38f72c73f69775358d8b047de2e354364fcafc2))
    - Release git-tempfile v1.0.0 ([`1238535`](https://github.com//Byron/gitoxide/commit/123853539dc30ddea2d822ab177ee09b191bdf1b))
    - Upgrade to nom-7 ([`f0aa3e1`](https://github.com//Byron/gitoxide/commit/f0aa3e1b5b407b2afd187c9cb622676fcddaf706))
    - Merge branch 'main' into 162-repo-design-sketch ([`e63b634`](https://github.com//Byron/gitoxide/commit/e63b63412c02db469fbdb17da82cd1e9fda1ef0f))
    - [repository #165] fix docs ([`b4fdfd7`](https://github.com//Byron/gitoxide/commit/b4fdfd7a21057f89f4b6263c0c291003241e2833))
    - Release git-ref v0.6.0 ([`0bb4c13`](https://github.com//Byron/gitoxide/commit/0bb4c133da96f6a96d9f1767848ada792a27c2be))
    - [ref #165] refactor ([`66624c3`](https://github.com//Byron/gitoxide/commit/66624c3ef1faf7048ee86ed73cf5f622802c061e))
    - [repository #165] refactor ([`00ec15d`](https://github.com//Byron/gitoxide/commit/00ec15dcfdb839095e508139d238df384ea418eb))
</details>

## v0.5.4 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.5.4 ([`bc5d860`](https://github.com//Byron/gitoxide/commit/bc5d860a616fd5a4371792a8ecde6e6356e217f8))
    - [smart-release #162] FAIL: one level down, using the cache isn't really working… ([`65db010`](https://github.com//Byron/gitoxide/commit/65db0104146248b273081fc6616a6ed484aa948e))
    - [ref] Out of bounds check to prevent legitimate panic ([`303608c`](https://github.com//Byron/gitoxide/commit/303608cbc1ade71c635dd1bbbe60988d09184351))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com//Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.5.3 (2021-08-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.5.3 ([`e6a8020`](https://github.com//Byron/gitoxide/commit/e6a8020ff9b85c6dfedd80525c571514e039edae))
    - [ref #157] Support for unsorted packed refs and those without header ([`2724688`](https://github.com//Byron/gitoxide/commit/272468892c02133efd68d15ffc5cacb4d5c5cd78))
</details>

## v0.5.2 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.5.2 ([`50dcca9`](https://github.com//Byron/gitoxide/commit/50dcca97e207ec608e506adcef90dd0599b4441d))
    - remove dev-dependency cycles by removing their version ([`c40faca`](https://github.com//Byron/gitoxide/commit/c40faca41632cd2a226daf4ddf5293b65d1fdc82))
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 ([`f123f69`](https://github.com//Byron/gitoxide/commit/f123f69c7a4f9fd1c98bd2f60ebc953a6739fe04))
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 ([`c67291f`](https://github.com//Byron/gitoxide/commit/c67291ff9bcdff9a747d87241f6a71015607af05))
    - Release git-object v0.12.0 ([`7006150`](https://github.com//Byron/gitoxide/commit/7006150ac314d19814608723f69f6e70a72f9262))
    - Release git-actor-0.3.1 ([`727087d`](https://github.com//Byron/gitoxide/commit/727087dca243da4bc40bc87611a2f66234565be7))
    - [utils #154] commit manifest changes; create tags ([`95dcd9d`](https://github.com//Byron/gitoxide/commit/95dcd9d7d060101596c51116218102cc8049d0dd))
    - (cargo-release) version 0.3.0 ([`263088b`](https://github.com//Byron/gitoxide/commit/263088b3faaccd9edae8c21dfc7d39b191d76207))
    - (cargo-release) version 0.18.0 ([`b327590`](https://github.com//Byron/gitoxide/commit/b327590d02fec5536c380b2d39dd7be089ca7c40))
    - (cargo-release) version 0.17.0 ([`c52a491`](https://github.com//Byron/gitoxide/commit/c52a49176bd294bb36db74b4293cdb684a2ab7f6))
    - (cargo-release) version 0.6.0 ([`d58f37e`](https://github.com//Byron/gitoxide/commit/d58f37e3b5a000fbe069aa869bd84f66d5c3210b))
    - (cargo-release) version 0.11.0 ([`a5be31c`](https://github.com//Byron/gitoxide/commit/a5be31c4cf7c0b538a1ed4a52ff5c3a992c6feff))
    - (cargo-release) version 0.5.0 ([`bf15c2a`](https://github.com//Byron/gitoxide/commit/bf15c2a2f285046b094093760c1969007ee75e25))
    - (cargo-release) version 0.3.0 ([`64efc05`](https://github.com//Byron/gitoxide/commit/64efc0534ddc372b6e668b23c1e9d276098679c9))
    - (cargo-release) version 0.4.0 ([`70ef344`](https://github.com//Byron/gitoxide/commit/70ef3442775b54ba9e4ee9ebfffb37af9804cc5b))
    - Revert "[ref] break dev-dependency cycle" ([`436e89b`](https://github.com//Byron/gitoxide/commit/436e89b18cb157b3d30bd24b8d1acef25631ec2a))
</details>

## v0.5.1 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.1 ([`6f61fca`](https://github.com//Byron/gitoxide/commit/6f61fcaf9528f2ba6752ce94524b59ff505cc518))
    - [ref] break dev-dependency cycle ([`d5af428`](https://github.com//Byron/gitoxide/commit/d5af42898487a82f2fbd000fac2f0db9505a587c))
</details>

## v0.5.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 394 commits contributed to the release over the course of 78 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`ae02dab`](https://github.com//Byron/gitoxide/commit/ae02dabae961089a92a21e6a60a7006de4b56dad))
    - (cargo-release) version 0.4.0 ([`0d5c8b9`](https://github.com//Byron/gitoxide/commit/0d5c8b96dfdfb96e4fc82623f756f6c7f7046e90))
    - (cargo-release) version 0.16.0 ([`1231dbd`](https://github.com//Byron/gitoxide/commit/1231dbd16dacefb39adec8e067c312d313a82e3c))
    - (cargo-release) version 0.2.0 ([`20d8e27`](https://github.com//Byron/gitoxide/commit/20d8e27dd4e93ae2234a3fe19b5f1511365eee2e))
    - (cargo-release) version 0.5.0 ([`0e11e98`](https://github.com//Byron/gitoxide/commit/0e11e98f0562c7baa9c90e18db6240731d165217))
    - (cargo-release) version 0.2.0 ([`8ff5115`](https://github.com//Byron/gitoxide/commit/8ff511583e6d859e43ffda0ef75e2fecce3ed03c))
    - [ref] refactor ([`501182b`](https://github.com//Byron/gitoxide/commit/501182b106b70af73db4f23cc01291d30481f76e))
    - [ref #152] remaining tests for transaction namespacing ([`63d80c0`](https://github.com//Byron/gitoxide/commit/63d80c0d0fbcf4fd1b7c3db652f622b59bc6fd18))
    - [ref #152] first succeeding test for namespace rewriting ([`758c8f6`](https://github.com//Byron/gitoxide/commit/758c8f60ca6567cd0a12892490ce27f88d1140df))
    - [ref #152] first failing test for namespaced updates ([`a81f1d4`](https://github.com//Byron/gitoxide/commit/a81f1d44a83474152d53140f8d9fdd0ace8060ac))
    - [ref #152] refactor ([`f9c63fb`](https://github.com//Byron/gitoxide/commit/f9c63fbe70ceb10bc3ef3edee008f72c3494b18c))
    - [ref #152] namespace prefix stripping and fixed test expectations ([`bce135b`](https://github.com//Byron/gitoxide/commit/bce135b7c58ba5f709aad2daab0e1668a834a4cd))
    - [ref #152] a test for namespaced iteration ([`2338c6e`](https://github.com//Byron/gitoxide/commit/2338c6e96e3dbd0759c122e264044c195f16a269))
    - [ref #152] packed-refs are optional for generalized iteration, too ([`88525a9`](https://github.com//Byron/gitoxide/commit/88525a9f028e94c8647ad5f2f7067b5b4e01c0a3))
    - [ref #152] FAIL: cleanup iter API by allowing Option<packed::Buffer> ([`1836243`](https://github.com//Byron/gitoxide/commit/1836243b6ec42eaf162463cded4a613c8984ac3a))
    - [ref #152] prepare namespaced iteration tests ([`cf5abc9`](https://github.com//Byron/gitoxide/commit/cf5abc96115f4bab0ee52f58295f06f689173bf8))
    - [ref #152] no silent failure if path conversion isn't possible ([`8df04d8`](https://github.com//Byron/gitoxide/commit/8df04d8973fc62eae0e8d98c8116351907dd282f))
    - [ref #152] introduce Namespace type ([`67d5c85`](https://github.com//Byron/gitoxide/commit/67d5c8526d8356bcee81b690a38559a01128863b))
    - [ref #152] sketch API for namespaces ([`138be95`](https://github.com//Byron/gitoxide/commit/138be9588576eca84921cedcf5f697b5c98e85a7))
    - [ref #152] docs ([`8d6c856`](https://github.com//Byron/gitoxide/commit/8d6c8564faeccafc1430a2184a4060d953349e3f))
    - [ref #152] refactor ([`bfb82fb`](https://github.com//Byron/gitoxide/commit/bfb82fb13350d986c93cc6dc67d6f86506dd80a5))
    - [ref #152] all tests and impl for refname expansion ([`9cef2f2`](https://github.com//Byron/gitoxide/commit/9cef2f2f166514048fae52ceec5a86a2849be286))
    - [ref #152] refactor ([`431dd86`](https://github.com//Byron/gitoxide/commit/431dd8655397b0ae88a5144d5c8553ba63e46c8f))
    - [ref #152] basic test setup for namespace expansion ([`e852399`](https://github.com//Byron/gitoxide/commit/e8523996b73fb93218c651b6f6041935833293d0))
    - clippy on tests and thanks clippy ([`a77a71c`](https://github.com//Byron/gitoxide/commit/a77a71cf02d328a2a964388928d6b2a235a0aa85))
    - thanks clippy ([`e1964e4`](https://github.com//Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - [ref #140] finish implementation of tag peeling, with test ([`c06e729`](https://github.com//Byron/gitoxide/commit/c06e72916e9622df62579baa6817603af0c7c747))
    - [ref #140] refactor ([`edcc395`](https://github.com//Byron/gitoxide/commit/edcc3951bd0fc98589207a1b1f8941d6bb9652ab))
    - [ref #140] sketch ref tag peeling ([`ef90652`](https://github.com//Byron/gitoxide/commit/ef90652dfcd84b2fc140c38e1364b42578fdfbde))
    - [ref #140] refactor ([`8e1a730`](https://github.com//Byron/gitoxide/commit/8e1a7305e869979751230f23c614f276ebce3f1d))
    - [ref #139] add missing docs ([`5422ec8`](https://github.com//Byron/gitoxide/commit/5422ec8923a5f3c284f7094894a952a392812e63))
    - [ref #139] my first empty test but where else to document this :)? ([`0f00065`](https://github.com//Byron/gitoxide/commit/0f00065fa3360a55cc52926bfaa94d72598933b5))
    - [ref #139] refactor ([`a8f5d8d`](https://github.com//Byron/gitoxide/commit/a8f5d8dbaecaa26509d568a36acbf350ee86a03c))
    - [ref #139] peeling for all refs to be written to a pack ([`cc891a1`](https://github.com//Byron/gitoxide/commit/cc891a1809a6678f168b08766f67644742386a5d))
    - [ref #139] refactor ([`7e15817`](https://github.com//Byron/gitoxide/commit/7e1581788356889a936f4a778119b0bce36d3041))
    - [ref #139] Allow packed-refs creation in the presence of updates ([`0cf7314`](https://github.com//Byron/gitoxide/commit/0cf7314df7a6ab79478525544e0ed28d07cf3642))
    - [ref #139] impl of loose ref deletion, but it doens't work yet… ([`f6631ad`](https://github.com//Byron/gitoxide/commit/f6631ad537b4c7fd6dec2a511214552e606462d4))
    - [ref #139] a failing test for pruning loose refs into packed refs ([`437c610`](https://github.com//Byron/gitoxide/commit/437c610eeb3b4a5874f001ba6fbbd42c7dc1188e))
    - [ref #139] refactor ([`62558cb`](https://github.com//Byron/gitoxide/commit/62558cb562747d3c6f2b4e1b62dd44e4f1e95019))
    - [ref #139] a first sketch to resolve object chains for packed ref peeling ([`54bc116`](https://github.com//Byron/gitoxide/commit/54bc1161128f0c719622935728a870820918038b))
    - [ref #139] Allow 'git pack-ref --no-purge' essentially ([`c32d8b7`](https://github.com//Byron/gitoxide/commit/c32d8b7a599c0ee0d8936a0c5aee658b5d986453))
    - [ref #139] refactor ([`e5fbc4c`](https://github.com//Byron/gitoxide/commit/e5fbc4c92f0ea74afdff45c243a762e7a978d749))
    - [ref #139] refactor ([`4e1b95e`](https://github.com//Byron/gitoxide/commit/4e1b95e40e94b0c9398c40985e092bd1d8607a4c))
    - [ref #139] refactor ([`42215a1`](https://github.com//Byron/gitoxide/commit/42215a15ce53bd78fe1d8d9b15d7a08919f5f980))
    - [ref #139] a complete test for the first packed-refs mode ([`f332dcf`](https://github.com//Byron/gitoxide/commit/f332dcf2b1beda319871f7b0de585c8a1d9b813f))
    - [ref #138] delete packed-refs when it's empty after rewrite ([`8b7c359`](https://github.com//Byron/gitoxide/commit/8b7c359db1c81ae69321c9c2637d0af8b303d9bb))
    - [ref #138] refactor ([`3fc0014`](https://github.com//Byron/gitoxide/commit/3fc0014dbf3c6a0d0c3e34d39c3068c71f867fd1))
    - [ref #138] no need for preprocessing, input is already checked ([`a6fca6e`](https://github.com//Byron/gitoxide/commit/a6fca6e0f81cdccfd7284d70ad4218e94b6cbe24))
    - [ref #138] less is more… ([`6f39713`](https://github.com//Byron/gitoxide/commit/6f3971325380dee93370a2d6a05d43adec94181b))
    - thanks clippy ([`169a39d`](https://github.com//Byron/gitoxide/commit/169a39d72106c24dac78af2198e54ca6e09b743e))
    - [ref] the first green packed deletion… ([`76a23b0`](https://github.com//Byron/gitoxide/commit/76a23b0e3e508a3445a9e1c77045e59bb7bbef69))
    - [ref] refactor (packed refs aren't changed in memory) ([`0a7e8ce`](https://github.com//Byron/gitoxide/commit/0a7e8ce1be7c7e6cb8a7646a8dacc7e95acf5efd))
    - [ref] basic packed transaction commit impl, but it doesn't work yet ([`1913099`](https://github.com//Byron/gitoxide/commit/1913099eeb84e78d9b4373e6ba9823a493d82343))
    - [ref] fix order of operations when committing the transaction ([`be5774a`](https://github.com//Byron/gitoxide/commit/be5774a3d5e8fa20eadc6ef6f0bbfceab35f1827))
    - [ref] refactor ([`69d53f9`](https://github.com//Byron/gitoxide/commit/69d53f99097220cf3a5e3e5afa855d1847715007))
    - [ref] first revised sketch of packed-refs writing ([`f942c76`](https://github.com//Byron/gitoxide/commit/f942c7622cf09d3c6937c7fa78089991d58482a0))
    - [ref] work on first naive transaction, but… ([`b08cc4a`](https://github.com//Byron/gitoxide/commit/b08cc4a47ecf8ad5f4b56ffdaf678946549b0ae9))
    - [ref] tests incorporating packed-ref deletion ([`399096e`](https://github.com//Byron/gitoxide/commit/399096e0f611a649fb99facc0925adc1c306cbfe))
    - [ref] validate packed refs are taken into consideration during create/update ([`25999b4`](https://github.com//Byron/gitoxide/commit/25999b4cebcb925bf0f0d4f451c7ca557f03dbc2))
    - [ref] allow creating new packed-refs files as well; prepare test arena ([`8494c74`](https://github.com//Byron/gitoxide/commit/8494c7452f68bb3ebe7bc9115b7feb36871a406a))
    - [ref] refactor ([`e379177`](https://github.com//Byron/gitoxide/commit/e379177a1937fdc23cba843d2dc6fecd3dfd2ab2))
    - [ref] refactor ([`a844146`](https://github.com//Byron/gitoxide/commit/a844146a799e07c3d95c4224b4a114b77cd94832))
    - [ref] refactor ([`bd94ea5`](https://github.com//Byron/gitoxide/commit/bd94ea55c1b598e507b5717ee5a5d6f14830c3bb))
    - [ref] actually make use of packed refs in file transactions ([`7746238`](https://github.com//Byron/gitoxide/commit/7746238207b637d4f241a05af7814916736cce24))
    - [ref] refactor ([`7a7b0dc`](https://github.com//Byron/gitoxide/commit/7a7b0dcd8b9156a5c67bbdcdebb6a2a2e2757a7e))
    - [ref] refactor ([`74ed358`](https://github.com//Byron/gitoxide/commit/74ed358c7ef6147095e8df9eb29b34ab55c850f4))
    - [ref] first basic sketch of packed-ref transaction ([`8aac30c`](https://github.com//Byron/gitoxide/commit/8aac30c77b03aa6c020d46c79f54d031043351df))
    - [ref] on the way to requiring a packed transaction for file transactions ([`85f30ac`](https://github.com//Byron/gitoxide/commit/85f30ac10fa740293d72f558dbd48a14aee82fde))
    - [ref] prepare existing refs to take packed-refs into account… ([`5849b44`](https://github.com//Byron/gitoxide/commit/5849b44c87c8b9ca68d7d30623540d8d441b6a3f))
    - [ref] remove one todo, add another… ([`46c47ab`](https://github.com//Byron/gitoxide/commit/46c47ab440df49d0f3a5324b243cdcf5a2898e03))
    - [ref] all todos done ([`7632573`](https://github.com//Byron/gitoxide/commit/763257327632b39a5ec777df4f07da9f87005a36))
    - [ref] refactor ([`fb37e96`](https://github.com//Byron/gitoxide/commit/fb37e9612c03cf1fcf5cdef9241a35242b9ff1d0))
    - [ref] refactor ([`23ea139`](https://github.com//Byron/gitoxide/commit/23ea139e0af622e8d40774fa2a890ef3525a991a))
    - [ref] rev-iter for overlay references ([`8b28d4a`](https://github.com//Byron/gitoxide/commit/8b28d4a326a2ee43bd00e475a0376eb577145a8b))
    - [ref] refactor ([`a80b8c1`](https://github.com//Byron/gitoxide/commit/a80b8c18eb5cfc77ca5e071e9163df0a89a35fd4))
    - [ref] tests for remaining todos ([`0ef6b3d`](https://github.com//Byron/gitoxide/commit/0ef6b3dbdc7f8c67e69eeb453122ce2722d171fa))
    - [ref] remove loose::Reference backref to simplify everything ([`9f1d960`](https://github.com//Byron/gitoxide/commit/9f1d960ae07d368f3ab208cf886ea1af99dfe25f))
    - Revert "[ref] back-reference of packed refs to their packed buffer" ([`464aefe`](https://github.com//Byron/gitoxide/commit/464aefe563c045b30ead0144b97a41d7b353235e))
    - Revert "[ref] FAIL: let's not add more back-refs, let's add less" ([`eaf4e9a`](https://github.com//Byron/gitoxide/commit/eaf4e9a1582fcd3c1d1da9eba3fb4c7046a5cdb9))
    - [ref] FAIL: let's not add more back-refs, let's add less ([`8e90d75`](https://github.com//Byron/gitoxide/commit/8e90d7545d4bda92e339387acfa1c882e2a99264))
    - [ref] back-reference of packed refs to their packed buffer ([`da860ef`](https://github.com//Byron/gitoxide/commit/da860efa8fb42f9f755cd9070732fc4403843cc9))
    - [ref] refactor ([`61972a2`](https://github.com//Byron/gitoxide/commit/61972a298bfcbad7efe23a480895fc26bb53bf24))
    - [ref] refactor ([`f03c614`](https://github.com//Byron/gitoxide/commit/f03c6144f395fd8713157a4a3137c6c0dacd41da))
    - thanks clippy ([`08f8bc4`](https://github.com//Byron/gitoxide/commit/08f8bc4c09ad85df0ea75916f8bd9beb061069ea))
    - [ref] probably fix windows ([`6eb2532`](https://github.com//Byron/gitoxide/commit/6eb2532724d6be1b25b68b10b58cd504ff1a7af9))
    - [ref] refactor ([`3df606a`](https://github.com//Byron/gitoxide/commit/3df606aa33ab8c161a7b36b79a9661eefac218e7))
    - [ref] test for peel one level of packed ref ([`3d8602f`](https://github.com//Byron/gitoxide/commit/3d8602f2fff98e3a1078c24e65cd887bebc7fa78))
    - [ref] assure packed-refs have a consistent target after peeling. ([`29a352a`](https://github.com//Byron/gitoxide/commit/29a352a24c0e2685d06672967e4898abfa1c2f8c))
    - thanks clippy ([`321908e`](https://github.com//Byron/gitoxide/commit/321908e12a885978dc4fa3fa1f71cebc8efdf741))
    - [ref] improve import paths ([`2dbe785`](https://github.com//Byron/gitoxide/commit/2dbe785d80d56b2d9f5a617b57a02926dba70434))
    - [ref] refactor ([`49fc212`](https://github.com//Byron/gitoxide/commit/49fc212e9e82382d06da16dc9b84e3952a73ddce))
    - [ref] prepare to create loose:Reference ([`8ed3916`](https://github.com//Byron/gitoxide/commit/8ed3916564917fd99a74dda06d35f4390e918fa5))
    - [ref] refactor ([`f222525`](https://github.com//Byron/gitoxide/commit/f2225253de054ce8cfa8f8ce33a93c3ac613dc85))
    - [ref] finally peeling works again ([`d5bd75a`](https://github.com//Byron/gitoxide/commit/d5bd75acdf48f7a274dbb88441f003d5d287e3b8))
    - [ref] packed-refs are now enforcing valid names ([`5d92919`](https://github.com//Byron/gitoxide/commit/5d9291976370edae3a8429e745174147c1fadf90))
    - [ref] prepare peel test; realize another refactoring requirement ([`62f7155`](https://github.com//Byron/gitoxide/commit/62f71552da037c126058b7bcaa9e6bab8e2c168b))
    - [ref] refactor ([`ae4d5da`](https://github.com//Byron/gitoxide/commit/ae4d5da10fc6e0ec5015539a1285f1a3dbbc9628))
    - [ref] refactor ([`e26c72f`](https://github.com//Byron/gitoxide/commit/e26c72fb1bf9392932ffe42843f3dec52c7bbd7d))
    - [ref] refactor ([`f4bb7a0`](https://github.com//Byron/gitoxide/commit/f4bb7a02d8e8b820f30894ac74613bee10532c79))
    - [ref] another test to run into one more todo ([`13502f5`](https://github.com//Byron/gitoxide/commit/13502f5bb7b1df7abd1d2de4f9e93a9e5439b84f))
    - [ref] some TODOs to not forget ([`4d6a75c`](https://github.com//Byron/gitoxide/commit/4d6a75cc6835cbd1f6ab321e158310c97def2a71))
    - [ref] and it compiles again, may todos left ([`16618b9`](https://github.com//Byron/gitoxide/commit/16618b916ff67316717d95575fc1344d956d2c49))
    - [ref] all required Reference methods are defined, but… ([`3c976a6`](https://github.com//Byron/gitoxide/commit/3c976a65cad62e4e04c686b1e8f645bf300ccf41))
    - [ref] refactor ([`65f7a7d`](https://github.com//Byron/gitoxide/commit/65f7a7db56d6db974db197101b6306dbb7483ff5))
    - [ref] changing the ref type means a lot of breakage and some unsolved problems ([`407dc4d`](https://github.com//Byron/gitoxide/commit/407dc4d79a4281fc3ec09456bb6f969f42bbabd7))
    - [ref] refactor to be able to use loose_then_packed::Reference for top-level find ([`2c4e45a`](https://github.com//Byron/gitoxide/commit/2c4e45a5bf997530d84a214714ff25fdbbcafd16))
    - [ref] figure out how peeling works with packed-refs… ([`2801f7a`](https://github.com//Byron/gitoxide/commit/2801f7aa137c6167bd392ca585f1aad378cae0b4))
    - Revert "[ref] FAIL: actually it's enough to give access to 'packed' when peeling only" ([`8dc6295`](https://github.com//Byron/gitoxide/commit/8dc62955f1a8b92f08924f155c932d0dfbf415ef))
    - [ref] FAIL: actually it's enough to give access to 'packed' when peeling only ([`5173a97`](https://github.com//Byron/gitoxide/commit/5173a97531f213573da12d0d9dda8e0bc808c013))
    - [ref] put packed-ref lookups into the correct spot ([`6d11e22`](https://github.com//Byron/gitoxide/commit/6d11e22c723f03155f12878ac7b94ef959f633a4))
    - [ref] remove over-complicated refs store trait which… ([`1cc876c`](https://github.com//Byron/gitoxide/commit/1cc876cde25820a7a8afa8d867dec59e6079d72e))
    - [ref] refactor ([`62e682c`](https://github.com//Byron/gitoxide/commit/62e682c269c48a9eb2c25f4bb6421b8647fb3fab))
    - [ref] API sketch for allowing packed-refs to be used in find() ([`ca736ab`](https://github.com//Byron/gitoxide/commit/ca736ab2ee8eab337683ff66e6e07d4488ff15da))
    - [ref] fix windows build ([`f99851b`](https://github.com//Byron/gitoxide/commit/f99851bc3195aca958409bd5773e6210037b07f8))
    - [ref] assure names are using forward slashes in file-based refs ([`ff695e4`](https://github.com//Byron/gitoxide/commit/ff695e4dae73d1497290d1efcc77b0cf1b265617))
    - [ref] prefix iteration for all references ([`228ca00`](https://github.com//Byron/gitoxide/commit/228ca00a91069ebe32dddbae3d716cc6bb59542e))
    - [ref] improve structure; fix docs ([`aa6052a`](https://github.com//Byron/gitoxide/commit/aa6052a41e44a13ea31c9ec585663b0904cdd929))
    - [ref] overlay really seems to work ([`d2ec30a`](https://github.com//Byron/gitoxide/commit/d2ec30af1be4bc54d69ef7d794c1bf372c80463b))
    - [ref] more detailed overlay test ([`d747d73`](https://github.com//Byron/gitoxide/commit/d747d730afd4db6c0c20c3c63cc09824fbd6e223))
    - thanks clippy ([`636e1fd`](https://github.com//Byron/gitoxide/commit/636e1fd85ceb3a1dc3cf5d3c7224f6f36d8eb695))
    - [ref] fix windows build… ([`65e6953`](https://github.com//Byron/gitoxide/commit/65e6953d1a9e751cb4644056aabd7c6edfbf7978))
    - [ref] first successful test for overlay iterator ([`5f92488`](https://github.com//Byron/gitoxide/commit/5f924885f343d8a60737de74c651e8e5c11a8d48))
    - [ref] conversion for packed refs ([`929bb0f`](https://github.com//Byron/gitoxide/commit/929bb0f75715a547993e8ce9c885d7de1a030013))
    - [ref] loose refs iteration in overlay iterator ([`0b0f64d`](https://github.com//Byron/gitoxide/commit/0b0f64d16acb97d2282b982647362b164ac280ad))
    - [ref] leverage sorted file iteration ([`036257e`](https://github.com//Byron/gitoxide/commit/036257eee036c2d5edea2ac8b16aad6bae8ba7fd))
    - [ref] add setup for parallel file traversal tests ([`1306647`](https://github.com//Byron/gitoxide/commit/1306647447f712805b3d8c8ca38e90fb4f94ca67))
    - [ref] reproducible loose ref iteration with built-in sorting ([`e138748`](https://github.com//Byron/gitoxide/commit/e13874807ccc3cbc2b4aacccf63ac5c3dd21c445))
    - [ref] sketch remaining overlay types, now on to 'next()' ([`6792cf1`](https://github.com//Byron/gitoxide/commit/6792cf1362ed21948d9b5f8b252b1c08ca8ca7ca))
    - [ref] a way to obtain valid ref names along with their path for overlay iteration ([`bbaa1eb`](https://github.com//Byron/gitoxide/commit/bbaa1eb10b3d2fd0de6afde61e5b6378be2e110c))
    - [ref] first steps towards test and impl for overlay iterator ([`f5d07b6`](https://github.com//Byron/gitoxide/commit/f5d07b67af4fdf68f3109a8bc1481474cd5c3807))
    - [ref] add missing docs ([`e6052a5`](https://github.com//Byron/gitoxide/commit/e6052a5a36b27bbcf79c05cd517eab9ec7507d8d))
    - [ref] all remaining tests ([`ee9bc21`](https://github.com//Byron/gitoxide/commit/ee9bc211e857ed2bbf9eb5fc6e46f5e126b11ab2))
    - [ref] first successful test for prefix filtering in packed refs ([`430549d`](https://github.com//Byron/gitoxide/commit/430549da137c5469a0ee17eca8d52a6f3ed8b04b))
    - [ref] run all performance tests ([`3635b25`](https://github.com//Byron/gitoxide/commit/3635b25deee7ded4307458abcf83d0c1181030f4))
    - [ref] simple performance tests to get an idea of what it can do… ([`06bedcd`](https://github.com//Byron/gitoxide/commit/06bedcd7a79c64ece443a34cc21a9ca32ac38ca9))
    - [ref] perf 'test' for ref iteration ([`922d129`](https://github.com//Byron/gitoxide/commit/922d129ff3b741a3091cf899a8e1400e98417093))
    - thanks clippy ([`a39a68a`](https://github.com//Byron/gitoxide/commit/a39a68a3d51bf0185df86ca34f90b9755f31f2b5))
    - [ref] rename find_one to 'find' in git-ref… ([`ae7746a`](https://github.com//Byron/gitoxide/commit/ae7746a0815bb94659de67383ba372ac522d53b8))
    - [ref] refactor ([`758c090`](https://github.com//Byron/gitoxide/commit/758c0907df8dc6987f374e326304e0f9fad29812))
    - [ref] finish packed find() lookup testing ([`5f67c19`](https://github.com//Byron/gitoxide/commit/5f67c19a1f4f62419bfc7d6e52c56aa5be40b723))
    - [ref] refactor ([`953939c`](https://github.com//Byron/gitoxide/commit/953939c2ce7922efd6df4654dc329743d3052492))
    - [ref] prevent unnecessary rounds for full names that aren't found ([`fb765de`](https://github.com//Byron/gitoxide/commit/fb765de831aa704b04b6a23c6a1d4ff183d784e0))
    - [ref] Assure ref-misses misses aren't parse-errors ([`d9d1360`](https://github.com//Byron/gitoxide/commit/d9d13602c83d0725d23d3abb3d2d5bf30355e1d9))
    - Merge branch 'remove-unnecessary-unsafe' ([`7a3c5c1`](https://github.com//Byron/gitoxide/commit/7a3c5c14dc56d8711548d1b219a969836693cbaa))
    - [ref] basic lookup rule impl; needs more test cases ([`3226f77`](https://github.com//Byron/gitoxide/commit/3226f775129231b4bc4735baf9e14a187665ace3))
    - [ref] fix compile warning on windows ([`c328774`](https://github.com//Byron/gitoxide/commit/c32877415aba8df6d5a37cfd799b218e3a29b18a))
    - [ref] a test case specifically for lookup rules ([`ab3a34f`](https://github.com//Byron/gitoxide/commit/ab3a34f481ebe335578e3a7dbff325087b4ba647))
    - Remove unnecessary pub(crate) exports ([`3d2456e`](https://github.com//Byron/gitoxide/commit/3d2456e11709f0461b37c6df55ecc3861ca4cab5))
    - [ref] refactor ([`140da9a`](https://github.com//Byron/gitoxide/commit/140da9a0b77c423649d9fd291babef80532015a2))
    - [ref] improve parse failure handling in packed-ref lookup ([`ba62aab`](https://github.com//Byron/gitoxide/commit/ba62aab4308d44092d151d11d9be44ba6bfddb02))
    - [ref] refactor ([`959abc7`](https://github.com//Byron/gitoxide/commit/959abc70c754cf4cd812f6014c29fd2f6d1a7fc4))
    - [ref] prepare for proper full-name conversion ([`0e6d3f2`](https://github.com//Byron/gitoxide/commit/0e6d3f29a6abe54b04424697009bb8524faaca7e))
    - [ref] searching fully qualified reference names actually works. ([`9b2579c`](https://github.com//Byron/gitoxide/commit/9b2579c3713b3bd185895318868378b8831dbc96))
    - [ref] prepare find() impl… ([`b26dd1e`](https://github.com//Byron/gitoxide/commit/b26dd1ed253d8714cf4f9a77c0c29f67cc952c76))
    - [ref] assure packed-refs buffers are sorted ([`a797493`](https://github.com//Byron/gitoxide/commit/a797493c93aa2d1b6e46442f714c8d5b98032456))
    - [ref] refactor ([`897a49a`](https://github.com//Byron/gitoxide/commit/897a49a9973ccb225dbc9b75be624b7e4c9ec608))
    - [ref] windows fix; now maybe? ([`0e1a204`](https://github.com//Byron/gitoxide/commit/0e1a20424a25902e80ad8dd6b6a413cb00f77904))
    - [ref] windows pathname replacement: \ -> /… ([`94a1e02`](https://github.com//Byron/gitoxide/commit/94a1e02d3e03f29d56b83e92c176c8d245ff44fc))
    - [ref] fix one test failure on windows ([`21f1031`](https://github.com//Byron/gitoxide/commit/21f10319d4047401bb6b11dec975c9386788773b))
    - [ref] rough frame for finding packed refs ([`a24a54f`](https://github.com//Byron/gitoxide/commit/a24a54fb2b2620a0c86c2b9bc2a094412ed73fb8))
    - [ref] learn more about the windows issue… ([`dde6276`](https://github.com//Byron/gitoxide/commit/dde6276a52b0f067bfeb8bb355a05696df6f134f))
    - [ref] refactor ([`c150aba`](https://github.com//Byron/gitoxide/commit/c150abaa86ebcbd10ccee4359b45b4a0b802b68e))
    - [ref] prefixed loose ref iteration ([`49ce1e2`](https://github.com//Byron/gitoxide/commit/49ce1e2184841ecd9c54573ba026341f4fecc0b5))
    - [ref] refactor; tests for prefix iteration ([`63566eb`](https://github.com//Byron/gitoxide/commit/63566eb81cdd14a98f25491fbb7f363a2fb6a0c7))
    - [ref] loose ref iteration with broken ref support ([`2d1234f`](https://github.com//Byron/gitoxide/commit/2d1234f9f8ae55c13af18ef5978e4ef9634e1606))
    - [ref] maybe fix windows ([`6fc7784`](https://github.com//Byron/gitoxide/commit/6fc778455c374fa289d15e64d1d67ad9310e0d0a))
    - [ref] first rough implementation of loose ref iteration ([`918af42`](https://github.com//Byron/gitoxide/commit/918af425298a1fdbb8e7dd6328daefe9eaa10cef))
    - [ref] packed-refs iteration… ([`ea97e06`](https://github.com//Byron/gitoxide/commit/ea97e063bfa5cbafac521dbd7f8becd357083356))
    - [ref] docs for packed refs iterator ([`02690bc`](https://github.com//Byron/gitoxide/commit/02690bc96903071108ffc54594bd4c31ebd054d1))
    - [ref] fix 'small' build ([`5fd10fe`](https://github.com//Byron/gitoxide/commit/5fd10fe1e901a0c8d9627f76c4a040922847cd15))
    - [ref] packed-refs iteration works, incl. decent error handling ([`e5a6b9d`](https://github.com//Byron/gitoxide/commit/e5a6b9d2f637ee746ccaf67354f64c3999cf971a))
    - [ref] the first packed-refs iterator tests ([`f6d769e`](https://github.com//Byron/gitoxide/commit/f6d769ec5948fefe363ffa436e326e5fae820a66))
    - [ref] refactor ([`207a799`](https://github.com//Byron/gitoxide/commit/207a799c1fcf490425f2e5dcf8274da83125af6f))
    - [ref] flexible and simple support for different hash lengths ([`9c2edd5`](https://github.com//Byron/gitoxide/commit/9c2edd537fb86d2d7db874ec976d0cb1b8ec7c2e))
    - Revert "[ref] parameterize all uses of hash length…" ([`21f187e`](https://github.com//Byron/gitoxide/commit/21f187e6b7011bb59ed935fc1a2d0a5557890ffe))
    - [ref] sketch of iterator ([`6c05243`](https://github.com//Byron/gitoxide/commit/6c05243b53a74c770fc41e50a7df55f01ba21b3d))
    - [ref] refactor ([`79184cf`](https://github.com//Byron/gitoxide/commit/79184cfe1035ad8665972c796c27448dc1fe3430))
    - [ref] parameterize all uses of hash length… ([`5c7285e`](https://github.com//Byron/gitoxide/commit/5c7285e7233390fd7589188084fcd05febcbbac2))
    - [ref] less lenient packed-ref header parsing ([`45b41e0`](https://github.com//Byron/gitoxide/commit/45b41e0f522ac491e49be5e36a1744c9d07a4286))
    - thanks clippy ([`33f1b00`](https://github.com//Byron/gitoxide/commit/33f1b00e134222641a71521561db4671a4285462))
    - [ref] refactor ([`de526b3`](https://github.com//Byron/gitoxide/commit/de526b31dbd84ddf05cbc5d447862fa0559a7561))
    - [ref] first working packed ref line parsing ([`bc60229`](https://github.com//Byron/gitoxide/commit/bc60229403ae075b66bb457a80695e2ab959448c))
    - [ref] first test for line (and peeled ref) parsin ([`7af27c5`](https://github.com//Byron/gitoxide/commit/7af27c5676c986b05953995d216b78389e986ee0))
    - [ref] refactor ([`b74913e`](https://github.com//Byron/gitoxide/commit/b74913ef90c6d827dff50ca5df13c826be4fc86d))
    - [ref] refactor ([`d0eb819`](https://github.com//Byron/gitoxide/commit/d0eb8196e3faed6c013f2e746ba50bba1330d87e))
    - [ref] packed refs header line parsing ([`fde5543`](https://github.com//Byron/gitoxide/commit/fde5543ad22395e27266db02a5442a33d16e68c5))
    - [ref] first rough steps to testing parsing a little ([`57659e9`](https://github.com//Byron/gitoxide/commit/57659e92de9a525a72dc3cba50b844bef7e021a1))
    - [ref] sketch packed refs, but… ([`8951b3f`](https://github.com//Byron/gitoxide/commit/8951b3fd96735adc2eed5b0035bc0a97759e2207))
    - [ref] refactor + docs review ([`4b9b034`](https://github.com//Byron/gitoxide/commit/4b9b034e3600cc3dc6dc35a257231914802a60fb))
    - [ref] the last TODO is gone ([`01dc422`](https://github.com//Byron/gitoxide/commit/01dc422cef924f26943dbc5b41b45098853d4868))
    - [ref] down to the last todo ([`23cea99`](https://github.com//Byron/gitoxide/commit/23cea99f645dfc27a89296f7bbd30c1b22015dba))
    - [ref] two more tests but only one todo down ([`bf947d6`](https://github.com//Byron/gitoxide/commit/bf947d65b508511d90299e93f285989c1a3eafd1))
    - [ref] the drop test ([`e472bde`](https://github.com//Byron/gitoxide/commit/e472bde7bf24eaeefa93a3dbc269cea41f6ddcc8))
    - [ref] refactor ([`059f836`](https://github.com//Byron/gitoxide/commit/059f836f490261cf5257349e0a7bfb69d9b68d89))
    - [ref] refactor ([`7faf6f2`](https://github.com//Byron/gitoxide/commit/7faf6f24f90854bd885e59c517b73db8ba5082af))
    - [ref] adjust expectation to not do any special HEAD business ([`49d294a`](https://github.com//Byron/gitoxide/commit/49d294a292709882179cf3b7934ec1885c60ccaa))
    - Revert "[ref] FAIL: realize that HEAD-reverse-lookup isn't done in transactions…" ([`8b0d7b6`](https://github.com//Byron/gitoxide/commit/8b0d7b62ff2ee96692d3014299fad67e0c82f3a1))
    - [ref] FAIL: realize that HEAD-reverse-lookup isn't done in transactions… ([`6098ba0`](https://github.com//Byron/gitoxide/commit/6098ba0f4288b379f84f48bb2d3245309a70ce7c))
    - [ref] test to validate HEAD update as special case of… ([`276aa9a`](https://github.com//Byron/gitoxide/commit/276aa9a89b41df43ad47f2096b4d89bdf697acea))
    - [ref] refactor ([`861483a`](https://github.com//Byron/gitoxide/commit/861483a4e7b7d61447d6bbfa91937ddfdf69ba02))
    - [ref] validate non-empty directories ([`8fb625d`](https://github.com//Byron/gitoxide/commit/8fb625d577fad376b28f5f568b8455aa901c2f0a))
    - [ref] moving a ref onto empty directories works now… ([`a237f77`](https://github.com//Byron/gitoxide/commit/a237f77ee0eb395bf89f7ed1b7496bf33c2d30af))
    - [ref] refactor ([`ed40a87`](https://github.com//Byron/gitoxide/commit/ed40a87e14d38b7f8b9a3a605b70a0fb1dc92220))
    - [ref] another complex test works ([`ebdbfae`](https://github.com//Byron/gitoxide/commit/ebdbfae9e26aa11f7afda7f60f0fbf6757dabb76))
    - [ref] fix build ([`b4dcdfc`](https://github.com//Byron/gitoxide/commit/b4dcdfc9b2f2edcbcf9fb144d1f97e9a841463ad))
    - [ref] try fix windows, once again ([`95e74dd`](https://github.com//Byron/gitoxide/commit/95e74dd9f1510fd288f281beea3f560319ad323d))
    - [ref] refactor ([`a261b82`](https://github.com//Byron/gitoxide/commit/a261b82c1ee7ebdbbc82ce1c8286ca6a0f221cea))
    - [ref] probably fix windows ([`a8b7c8d`](https://github.com//Byron/gitoxide/commit/a8b7c8d2fef9438a23a96c35497d34e816af96c7))
    - [ref] allow reflogs to be created in place of empty directory trees ([`80a6e0e`](https://github.com//Byron/gitoxide/commit/80a6e0e1ff2321d9162e799d5fc0f457e7de4ade))
    - [tempfile] a way to delete empty dirs recursively ([`6025aa0`](https://github.com//Byron/gitoxide/commit/6025aa08d93cd5124c8df38c51b795b9c7d1c911))
    - [ref] refactor ([`21920ec`](https://github.com//Byron/gitoxide/commit/21920ec173da4642ad335fcd5fbc3b85c940061e))
    - [ref] refactor directory handling ([`45dbf22`](https://github.com//Byron/gitoxide/commit/45dbf2253d13ee8eba7654ef294614c3b9651a9d))
    - [ref] refactor ([`92867c5`](https://github.com//Byron/gitoxide/commit/92867c58467e66d1b6b13d2ca4375d268fbafde5))
    - [ref] handle existng empty directories more gracefully… ([`0849c70`](https://github.com//Byron/gitoxide/commit/0849c70596ed7674e7e18cd444b6cd99d37da4ff))
    - thanks clippy ([`d967e30`](https://github.com//Byron/gitoxide/commit/d967e30f1652f29c3c13ea0014d8d3910a4f7245))
    - [ref] handle create-or-append when writing valid reflog files… ([`9175085`](https://github.com//Byron/gitoxide/commit/9175085248855a7ffa0d4e006740eafc0f4e1c92))
    - [ref] refactor ([`1ee3419`](https://github.com//Byron/gitoxide/commit/1ee341922d4a8343bc5146378da4353a99b28a73))
    - [ref] auto-creation logic for reflogs ([`80f71dc`](https://github.com//Byron/gitoxide/commit/80f71dc85836b640b264f146d37fc74a0bd99fd9))
    - [ref] reflog creation test is quite complete ([`b67e79c`](https://github.com//Byron/gitoxide/commit/b67e79c861f644756e9bd12cc3a28bd6355250d3))
    - [ref] allow commiter to be passed for use in reflog ([`80f5627`](https://github.com//Byron/gitoxide/commit/80f5627d6fe5aef8d0a82cdad1746d5d2509f2c3))
    - [ref] tests for converting reflock paths into log paths ([`1f2e754`](https://github.com//Byron/gitoxide/commit/1f2e75439d2ff5b7db40a979fde289e68c578d81))
    - [ref] refactor ([`a29fcf1`](https://github.com//Byron/gitoxide/commit/a29fcf1d61ec9f387a401a1a4a903256b6413536))
    - [ref] frame for reflog creation or update ([`81cb790`](https://github.com//Byron/gitoxide/commit/81cb79017ca5a2f18531bc6caedc28de94a0a064))
    - [ref] refactor ([`a76929b`](https://github.com//Byron/gitoxide/commit/a76929b45b4f82488b1e713d1012e1d431257fcd))
    - [ref] disambiguate create-or-update logic ([`585f369`](https://github.com//Byron/gitoxide/commit/585f369ea7bb7ee3d8f5103583628e3d68ef3de5))
    - [ref] write out Create-or-Update logic to see that's its probably not going to cut it. ([`54d084f`](https://github.com//Byron/gitoxide/commit/54d084ffe0d684ab4879973293f2efad4966c632))
    - [ref] show how the original name can be displayed for lock failures… ([`07f0c2d`](https://github.com//Byron/gitoxide/commit/07f0c2dc9b3949566b3c3d0a15302c416ae9ccb7))
    - [ref] write peeled previous OID through to parent refs ([`3355dd8`](https://github.com//Byron/gitoxide/commit/3355dd8295886b0dbeeaa802cbf32ea6e3264de6))
    - [ref] fix child link transformation ([`5d9a685`](https://github.com//Byron/gitoxide/commit/5d9a685fedd4d5614dd338d4b9baa37f11649cb0))
    - [ref] refactor ([`2f92f36`](https://github.com//Byron/gitoxide/commit/2f92f360e581a1a7b7bad389c915545cd6a5b31a))
    - [ref] sketch of inverting parent links for later oid lookup ([`a050f18`](https://github.com//Byron/gitoxide/commit/a050f1856f69b710f6e63898d11fa52cafd254c7))
    - [ref] refactor ([`1e88948`](https://github.com//Byron/gitoxide/commit/1e88948455111c01f2a8f9d24a4fcf835553e55b))
    - [ref] add reflog message to change… ([`b31e103`](https://github.com//Byron/gitoxide/commit/b31e103f2492b0507e2e1eab3a26ddc025dd470f))
    - [ref] sketch more detailed test for updating reflogs ([`5a657cd`](https://github.com//Byron/gitoxide/commit/5a657cdd0a342aa8b5a57398718bf27ef136997a))
    - thanks clippy ([`eb8ea22`](https://github.com//Byron/gitoxide/commit/eb8ea22a97f132169e81d71ca2ca64ef52463fe3))
    - [ref] the last deletion test ([`258a494`](https://github.com//Byron/gitoxide/commit/258a494562d8266561540e07c01d1e87466470d9))
    - [ref] refactor ([`db76cfd`](https://github.com//Byron/gitoxide/commit/db76cfd5585a5fa54739ce003837a8750dea9f99))
    - [ref] deletion won't have problems with broken refs ([`286b5c1`](https://github.com//Byron/gitoxide/commit/286b5c1a5529c58c35b8ff0504f9e784f7be10e1))
    - thanks clippy ([`e5da69e`](https://github.com//Byron/gitoxide/commit/e5da69e642c16ddaf39b59e6e0de6b3c4153acff))
    - [ref] add failing deletion test for broken refs ([`578413f`](https://github.com//Byron/gitoxide/commit/578413f5848cb8ab3b14fe149be3db12705182c3))
    - [ref] another del test ([`d935d6f`](https://github.com//Byron/gitoxide/commit/d935d6f67fff1d7b02f6b0805a3e6efb9f429fc1))
    - [ref] another deletion test ([`8b756e0`](https://github.com//Byron/gitoxide/commit/8b756e094bd4ecf47415d8eb8c7adf44b8a89039))
    - [ref] another deletion test ([`69ede1b`](https://github.com//Byron/gitoxide/commit/69ede1b90e6573df86829437f3c3adf3924b31cf))
    - [ref] refactor ([`d05a646`](https://github.com//Byron/gitoxide/commit/d05a6467c185d0f4dcb030e4bf751070a9b3d5bf))
    - [ref] Make sure edit preprocessing happens in the right order ([`2d5f9aa`](https://github.com//Byron/gitoxide/commit/2d5f9aaa68b065f84df3a2db3707cf9cf10b0321))
    - [ref] refactor ([`dd9c99b`](https://github.com//Byron/gitoxide/commit/dd9c99b9d1c0c6222f5a12f280c8ed0eb0c3daf2))
    - [ref] refactor ([`97fc864`](https://github.com//Byron/gitoxide/commit/97fc864fb4dd2903eb9f7dd671422dfbeaa304f3))
    - thanks clippy ([`f436f18`](https://github.com//Byron/gitoxide/commit/f436f18be3b4aafe40cb0e36432d22666795ecc6))
    - [ref] splitting handles reference cycles ([`09b4fc1`](https://github.com//Byron/gitoxide/commit/09b4fc1e6f01a9124f6563fa614b42356560e4b4))
    - [ref] splitting actually works! ([`a9f824b`](https://github.com//Byron/gitoxide/commit/a9f824bc95f157146f22b468d4a9d8dddc9f31a5))
    - [ref] first stab at splitting refs, needs more elaboration to fulfil expectations ([`66b1f37`](https://github.com//Byron/gitoxide/commit/66b1f3725cd710d991625bcd2c1994545b33aa53))
    - [ref] refactor ([`eb0328f`](https://github.com//Byron/gitoxide/commit/eb0328fb67ad677d8875bef5deb7efea2c55ae67))
    - [ref] first part of ref splitting is tested ([`ce7f83b`](https://github.com//Byron/gitoxide/commit/ce7f83b7e58762866e141d1b71e1ea68153fd075))
    - [ref] refactor; prep slitting tests ([`7ffc619`](https://github.com//Byron/gitoxide/commit/7ffc619a7c06f0d47572fac9f91444c3663ac316))
    - [ref] refactor ([`683651d`](https://github.com//Byron/gitoxide/commit/683651d2a7cc9b589b4490a1767677f3d7fb5e3e))
    - [ref] first sketch of generalized splitting of edits ([`1f2efdc`](https://github.com//Byron/gitoxide/commit/1f2efdcf9151f161a325680737f1992edf46228c))
    - [ref] working on splits really shows that we want more than one enum maybe… ([`1b62838`](https://github.com//Byron/gitoxide/commit/1b62838d00ec35cb45d43e5e9e5ce6573f1db2a7))
    - [ref] need ref splitting for the first time. ([`f52989f`](https://github.com//Byron/gitoxide/commit/f52989f325d50db66c0ffe75a964feaba075dc19))
    - [ref] better deletion tests; more useful return value ([`96848f6`](https://github.com//Byron/gitoxide/commit/96848f68a70a6721c9fc4c7d36763a3015527728))
    - thanks clippy ([`ef9bfd2`](https://github.com//Byron/gitoxide/commit/ef9bfd2806b0407ccbc7391e086592f4bf7a7424))
    - [ref] another deletion test succeeds ([`6037900`](https://github.com//Byron/gitoxide/commit/60379001d2729627c042f304217d6459f99f01bf))
    - [ref] refactor, not quite sure about delete mode… ([`683991a`](https://github.com//Byron/gitoxide/commit/683991a4edbc53c583603af94fbec625a211b52d))
    - [ref] another test; failing for now ([`1908b69`](https://github.com//Byron/gitoxide/commit/1908b693b75e8cb204dc5026ea2f311b88bddfc4))
    - [ref] another test green ([`104598e`](https://github.com//Byron/gitoxide/commit/104598eb71e830a5feed763dea1dc1fd03be6eff))
    - [ref] first succeeding deletion test ([`3445d7d`](https://github.com//Byron/gitoxide/commit/3445d7dfcade73bec8ba68d58d034608169e7758))
    - [ref] refactor ([`d2e2e8f`](https://github.com//Byron/gitoxide/commit/d2e2e8f49b3668235cf808b08f85bd89a592105f))
    - [ref] first deletion tests ([`e41f8c8`](https://github.com//Byron/gitoxide/commit/e41f8c8a48328fb0fe154e5212f1b1e41195d3c1))
    - [ref] write more details on how prepare and commit should work overall. ([`a7d988b`](https://github.com//Byron/gitoxide/commit/a7d988b8feb2aba87a19f3484470d8f77786ffd4))
    - [ref] refactor; get closer to what git does… ([`488f311`](https://github.com//Byron/gitoxide/commit/488f31160300bccaba6a510869c7c3e53d52d27b))
    - [ref] refactor ([`58a5653`](https://github.com//Byron/gitoxide/commit/58a5653a6647931bf90f88ff2d83c6b0322ad9b1))
    - [ref] first very basic ref writing ([`7ebed3f`](https://github.com//Byron/gitoxide/commit/7ebed3ff14e6944ba18be0c9876b10c42c2d840c))
    - [ref] remove complexity in the name of performance, fix windows… ([`77c3f24`](https://github.com//Byron/gitoxide/commit/77c3f24a935800d7643dc61466385a76a58bf365))
    - [ref] (probably) fix windows ([`7c1eead`](https://github.com//Byron/gitoxide/commit/7c1eead4b589975fb1dcfe63fb2071bb6d8ab611))
    - thanks clippy ([`6865549`](https://github.com//Byron/gitoxide/commit/6865549cf6df08999618bfa6cd658d44b8aba9c7))
    - [ref] slowly getting there ([`6506924`](https://github.com//Byron/gitoxide/commit/650692443459b253a56fb5bda78bd3a4a0de07f9))
    - [ref] a way to determine if a reflog exists. ([`e6fbba8`](https://github.com//Byron/gitoxide/commit/e6fbba87942b9138261ee70d8fa8408422149521))
    - [ref] reference::log_iter_rev() ([`1f7af5d`](https://github.com//Byron/gitoxide/commit/1f7af5dcf093a9169ce353c0b1d354ed7acda4a5))
    - [ref] reference.log_iter() works, but… ([`c298473`](https://github.com//Byron/gitoxide/commit/c298473f0f353f9f59d39ab530c133e13cfb47ec))
    - [ref] [FAIL] try to forward iterator creation to reference… ([`ef1737c`](https://github.com//Byron/gitoxide/commit/ef1737c7e67038c0541a619e77c0ea5451bcca28))
    - [ref] refactor ([`129bccf`](https://github.com//Byron/gitoxide/commit/129bccf8dfaaab1c487c49fe35a2877ff900d06e))
    - [ref] refactor ([`96dd98b`](https://github.com//Byron/gitoxide/commit/96dd98b800b9e808853fc954ac78b8778bf18f23))
    - [ref] refactor ([`a7dd994`](https://github.com//Byron/gitoxide/commit/a7dd9940a0a6e1f8685f5bb785d8c05023027393))
    - [ref] refactor ([`3460127`](https://github.com//Byron/gitoxide/commit/34601272230c37aad803409e89dc6b270de1f02d))
    - [ref] store ref log reverse iterator ([`34d7957`](https://github.com//Byron/gitoxide/commit/34d795700e89a264dcf3a40a6dec63cdc5998814))
    - [ref] store can provide reflog forward iter… ([`9adb9ca`](https://github.com//Byron/gitoxide/commit/9adb9ca2b2b63f9fc4b57e45732389077778c324))
    - [ref] more assertions ([`8000677`](https://github.com//Byron/gitoxide/commit/80006772e0ef9d9f9fc4d274f460194712138327))
    - [ref] a fully implemented first test with assertions ([`29a5893`](https://github.com//Byron/gitoxide/commit/29a58937a3e8d4fae861952d6bc34565da8c3e8c))
    - [ref] sketch more tests that will be needed ([`01690be`](https://github.com//Byron/gitoxide/commit/01690be8acf6a5f18b55db941f05644650f062f0))
    - [ref] add control over handling lock failures during transaction ([`7c4057a`](https://github.com//Byron/gitoxide/commit/7c4057aa4bd5e65195c80d0319798615b9571c0d))
    - [ref] generic operation on input edits, split-suitable now ([`7f4f637`](https://github.com//Byron/gitoxide/commit/7f4f63763249a614936be3baa702b93558a4d494))
    - [ref] try using borrow on a slice intead of iterator… ([`b2371d9`](https://github.com//Byron/gitoxide/commit/b2371d93408613ab0e07048398bd95e60da603e1))
    - [ref] duplicate ref edit checks… ([`3ec0182`](https://github.com//Byron/gitoxide/commit/3ec0182376fad623814408703f1d47736eea6349))
    - [ref] a more fleshed out API for file transactions ([`918123f`](https://github.com//Byron/gitoxide/commit/918123f7f951d7f773dd8b38a184de2f2c3e25b9))
    - [ref] on the way towards realistic transactions… ([`c808cb1`](https://github.com//Byron/gitoxide/commit/c808cb17b2fea12e018fabb789862e9b7703e49b))
    - [ref] on the way to setup the first transaction test ([`29c0b51`](https://github.com//Byron/gitoxide/commit/29c0b51625e2c7e3a8d60075bb925126a024dc83))
    - [ref] file store can ignore all writes; sketch transaction API ([`52a81e9`](https://github.com//Byron/gitoxide/commit/52a81e98f38657023d3eb384fd6db69917dd57ca))
    - [ref] refactor ([`6a84790`](https://github.com//Byron/gitoxide/commit/6a84790b13e445d5a1b85fd3cae2ec0feed4ff02))
    - [ref] log line writing ([`3da8fcf`](https://github.com//Byron/gitoxide/commit/3da8fcf0bfb77b80c06a3358416f10d6f393db8b))
    - [ref] Line::from_bytes(…); iter uses that now ([`7895995`](https://github.com//Byron/gitoxide/commit/7895995cf91fbaeb798c4277699e02107cb63909))
    - [ref] test for small buffer sizes ([`6183772`](https://github.com//Byron/gitoxide/commit/61837723f7c1f3150d7f853c055248116bba9633))
    - [ref] handle multiple buffer reloads ([`4559c7a`](https://github.com//Byron/gitoxide/commit/4559c7a184b9cdbd174785b84b41a218c683c94f))
    - [ref] refactor ([`65e333d`](https://github.com//Byron/gitoxide/commit/65e333de6194b48b558d02b503502bd7ab267945))
    - [ref] refactor ([`2b416ee`](https://github.com//Byron/gitoxide/commit/2b416ee7e788faadf280553464fd77f2c91e2d0a))
    - [ref] refactor ([`82b18e5`](https://github.com//Byron/gitoxide/commit/82b18e50f3c31fac10dc5a752ab9b0c134607e37))
    - [ref] multi-line reverse iteration works, without window shift for now ([`f1e3861`](https://github.com//Byron/gitoxide/commit/f1e38618371408d844144a736c3082d57b2d1015))
    - [ref] first reverse iter test succeeding ([`8875601`](https://github.com//Byron/gitoxide/commit/88756015d8fc77ddb3b12fcdd1df85a709f8189a))
    - [ref] let's not forget to simply not try to return borrowed things from iterators ([`bcc934d`](https://github.com//Byron/gitoxide/commit/bcc934dea0aa71502945a20d5987dec4eeb34aea))
    - [ref] FAIL: try it with included buffer ([`189080e`](https://github.com//Byron/gitoxide/commit/189080e8bc2d999ee4f1a76ed9b537cfda7ad82c))
    - [ref] FAIL another attempt this time without iterator… ([`5e73dc2`](https://github.com//Byron/gitoxide/commit/5e73dc2fa1a77b5bcf2319ed244004ac3ec86506))
    - [ref] FAIL at attempt to to have self-referential iterators :D… ([`bc4012e`](https://github.com//Byron/gitoxide/commit/bc4012eb8a1b0c27dd2b54d169c2058478449b0a))
    - [ref] first test for reverse iterator and more boilerplate ([`40db355`](https://github.com//Byron/gitoxide/commit/40db35547b855066b3584d8e81f62c8978ac5840))
    - [ref] refactor ([`4daddb1`](https://github.com//Byron/gitoxide/commit/4daddb13a7f7139b8e0e7c6817854dad00429dbc))
    - [ref] sketch of reverse iterator ([`c581d16`](https://github.com//Byron/gitoxide/commit/c581d169c2e21e568bce3d7bc8469836aa9d1e2c))
    - [ref] thanks clippy ([`4ba3b08`](https://github.com//Byron/gitoxide/commit/4ba3b08e69002ae20545e9d27c3130a672fa9ae6))
    - [ref] significantly simplify error messages… ([`b15cb16`](https://github.com//Byron/gitoxide/commit/b15cb16f022045207a9419266d3fe972fbd663e1))
    - [ref] don't include terminators to get slightly nicer error messges ([`09bbc6d`](https://github.com//Byron/gitoxide/commit/09bbc6d0b32b835d1a4ba2dca7e24522b94cee22))
    - [ref] another test for iter::forward() ([`1d84302`](https://github.com//Byron/gitoxide/commit/1d843029dbaa7d06f9338fa6eb90f583a4225094))
    - [ref] a forward iterator with a single test ([`917040c`](https://github.com//Byron/gitoxide/commit/917040cb58d9dda18835c255bff3a9d692cfe1de))
    - [ref] log line docs ([`10ab8e0`](https://github.com//Byron/gitoxide/commit/10ab8e0e4bcccc4e79203f06e16835b8e5d9504b))
    - [ref] refactor ([`cd89e21`](https://github.com//Byron/gitoxide/commit/cd89e21280463deb1fd22ef20d2c54926bbb9b6c))
    - [ref] more context for line parsing ([`ddb5f9d`](https://github.com//Byron/gitoxide/commit/ddb5f9d256cf0be36943e11a9df18b938551be87))
    - [ref] refactor ([`a08fb77`](https://github.com//Byron/gitoxide/commit/a08fb776a523040445006c81a890ef11f496f650))
    - [ref] be truly zero copy and delay work to when it's first asked for ([`b4e594b`](https://github.com//Byron/gitoxide/commit/b4e594bdeb06329beacd61b03ab90057284bcb54))
    - Merge branch 'negotiate-fallible' ([`27c8abe`](https://github.com//Byron/gitoxide/commit/27c8abe1948bc10c779efa33d4bc0b92741f6444))
    - [actor] FAIL an attempt to remove btoi errors ([`3f99cf5`](https://github.com//Byron/gitoxide/commit/3f99cf531caacb93a3ce81b16d61be18e5d8a017))
    - [actor] pure nom error handling… ([`78cbe18`](https://github.com//Byron/gitoxide/commit/78cbe18888ec654f3410fc655d9beaaf63f68003))
    - Merge branch 'ref-in-want' ([`f248557`](https://github.com//Byron/gitoxide/commit/f248557186384501e705473e0adab03d3fa10519))
    - [ref] refactor ([`8694488`](https://github.com//Byron/gitoxide/commit/869448833d9de5c0859e6fab267b48d19f1a9119))
    - [ref] getting there! ([`bd73d8e`](https://github.com//Byron/gitoxide/commit/bd73d8ee04f7baa9aeb05857484da6cb63175ebb))
    - [ref] a step forward to nom error handling, but… ([`426ae5b`](https://github.com//Byron/gitoxide/commit/426ae5b7db6cb943fdf6ee48e2be531157341e49))
    - [ref] try really hard to use generic verbose nom errors but… ([`1031625`](https://github.com//Byron/gitoxide/commit/10316252fa5dc02effe5596165268f8d806c55f8))
    - [ref] tests and impl for happy cases ([`7be82f0`](https://github.com//Byron/gitoxide/commit/7be82f09ce3c2421ba922e3f8bc1238ca5d494ab))
    - [ref] the first test for log line parsing; make serde1 work ([`cba3cdc`](https://github.com//Byron/gitoxide/commit/cba3cdc75280b247e59af878d1afe286638b95b7))
    - [refs] try to get structure in place for reflog parsing ([`727c66a`](https://github.com//Byron/gitoxide/commit/727c66a2560c00cc8e01fbe47503ffbb67147c59))
    - [refs] sketch more of transactions so it has all it needs ([`8f9a015`](https://github.com//Byron/gitoxide/commit/8f9a0157e876fadfe16a2cc58445543d1c10a21b))
    - [refs] allow writing any valid ref value instead of limiting ourselves to object ids ([`114fce8`](https://github.com//Byron/gitoxide/commit/114fce8368fe858bc64696b4d7253c425367560a))
    - [refs] finish transaction sketch (or so it seems) ([`976a079`](https://github.com//Byron/gitoxide/commit/976a0799a7862de7b85d45cb080102f41fc33d07))
    - [refs] this gets more and more interesting ([`e056495`](https://github.com//Byron/gitoxide/commit/e05649577a6cd5e2958884b10f7f75d48aa91a94))
    - [refs] finish research for transactions and their flags ([`2eb3bcc`](https://github.com//Byron/gitoxide/commit/2eb3bccadf338c07493e40cb8c5f357eb2502a5f))
    - [refs] sketch some parts of a transaction based on git source ([`d9a5d32`](https://github.com//Byron/gitoxide/commit/d9a5d328f575dfd86e414091688a545f931059e3))
    - (cargo-release) version 0.3.0 ([`87db688`](https://github.com//Byron/gitoxide/commit/87db688f23475d7232731429d770848aea228492))
    - (cargo-release) version 0.3.0 ([`6b33678`](https://github.com//Byron/gitoxide/commit/6b33678f83e6d261ca15c4a7634ff5b4e66d81dd))
    - Merge branch 'dependabot/cargo/crc-2.0.0' ([`683c44d`](https://github.com//Byron/gitoxide/commit/683c44db682d8dbef401286963e84cdca145abc8))
    - (cargo-release) version 0.2.0 ([`3286e42`](https://github.com//Byron/gitoxide/commit/3286e42547b59df6365087cbae9ce1c9c959faad))
    - [git-refs] a way to build a big packed-refs file ([`5113529`](https://github.com//Byron/gitoxide/commit/51135291b60d38bdf50d24569596c421bcb4f0b9))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com//Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - [git-repository] traversal program uses new facilities, and it's cumbersome ([`29ea2de`](https://github.com//Byron/gitoxide/commit/29ea2de9ad48036f78d3776d8526d959f68bf287))
    - [git-repository] most of the git repository discovery ([`72a49c8`](https://github.com//Byron/gitoxide/commit/72a49c816253520230a04290619f469df608be19))
    - [git-ref] refactor ([`0c795c5`](https://github.com//Byron/gitoxide/commit/0c795c50834bcf52324ede46ec11eea26acb1107))
    - [git-ref] fix docs ([`4fbc476`](https://github.com//Byron/gitoxide/commit/4fbc476b2361afef25cff208ecfa66ac2ccb077a))
    - [git-ref] docs complete ([`93a1f4e`](https://github.com//Byron/gitoxide/commit/93a1f4e3fe48082abf5b0baa17a976808789ec20))
    - [git-ref] nicer semantics for peel_in_place_to_id() ([`d3250a7`](https://github.com//Byron/gitoxide/commit/d3250a7b5d0e16f8f1b38d10334282fe60f9d5ce))
    - Revert "[git-ref] refactor (Option<Result… -> Result<Option…" ([`d4046e9`](https://github.com//Byron/gitoxide/commit/d4046e94eb22d9e9b65ffa9861400c4fde4d0bd7))
    - [git-ref] refactor (Option<Result… -> Result<Option… ([`774e86c`](https://github.com//Byron/gitoxide/commit/774e86ce78159f7e07ec552c1847658b6f9ac288))
    - [git-ref] refactor ([`928b637`](https://github.com//Byron/gitoxide/commit/928b63789237b808b296c60c989b853b78d39f0e))
    - [git-ref] more docs ([`f962c74`](https://github.com//Byron/gitoxide/commit/f962c74215965f14e8f136ab0a4eddfbba97e8c2))
    - [git-ref] refactor ([`415f15a`](https://github.com//Byron/gitoxide/commit/415f15aa5751ee1a58d9e6723a9da9f3407a4d66))
    - [git-ref] a bunch of docs ([`7cfc5ab`](https://github.com//Byron/gitoxide/commit/7cfc5ab3c3b969e968b894161f73f3c69fe8e4c9))
    - thanks clippy ([`93915fa`](https://github.com//Byron/gitoxide/commit/93915fa6f1c00260e4f263ac4837c2ae7916b764))
    - [git-ref] peel to id done ([`f74771c`](https://github.com//Byron/gitoxide/commit/f74771c8caccb090066b5209721b8973c047f00c))
    - [git-ref] first working peel-to-id() ([`3574f87`](https://github.com//Byron/gitoxide/commit/3574f8717700ae3b33e167be2442c69f604f287c))
    - [git-ref] frame for peel_to_id ([`3710b6c`](https://github.com//Byron/gitoxide/commit/3710b6cfe5cf2e5e6f9199255ebb4ca68a195be5))
    - [git-ref] peeling without an iterator, fine ([`b118946`](https://github.com//Byron/gitoxide/commit/b118946ef68425ffa0a606d67df7b5d3b2d851df))
    - [git-ref] first stab at reference iteration… ([`806d10e`](https://github.com//Byron/gitoxide/commit/806d10ef735caf3575b84de0cca5b55374140571))
    - [git-ref] refactor ([`c363269`](https://github.com//Byron/gitoxide/commit/c363269e118a2dc53ce29ba245c079cecf061b7e))
    - [git-ref] find_one_existing(…) for convenience ([`7a443ff`](https://github.com//Byron/gitoxide/commit/7a443ffc148ae8161ba93351ffd16631f79e095c))
    - [git-ref] some find failure cases ([`d855051`](https://github.com//Byron/gitoxide/commit/d85505195541f3123527a337c9935e25bfc40ec4))
    - [git-ref] handle all find_one cases as per docs ([`3c0acc6`](https://github.com//Byron/gitoxide/commit/3c0acc6545ede1a3fef25ace2b7dbf79debdc754))
    - [git-ref] more ways of finding reference ([`b3c4e92`](https://github.com//Byron/gitoxide/commit/b3c4e928c6fb01e029f509e8b24516cd6c24e48f))
    - [git-ref] the first green find_one test ([`30177e8`](https://github.com//Byron/gitoxide/commit/30177e81451bd4fb51dd3297502fa3c63f67286e))
    - thanks clippy ([`8f0e9ed`](https://github.com//Byron/gitoxide/commit/8f0e9ed9220a874e8437ede6e129d345e9c8f737))
    - [git-ref] first basic impl shows validation needs a little adjustment ([`8b901c7`](https://github.com//Byron/gitoxide/commit/8b901c750f97a950cb162c9195770aee451d2e7e))
    - [git-ref] a sketch of find_one - easiest for the caller for sure ([`ec96256`](https://github.com//Byron/gitoxide/commit/ec96256c4be9ff6de15bb698f2d3b9559619a042))
    - [git-ref] refactor ([`5bac585`](https://github.com//Byron/gitoxide/commit/5bac5851367d77ead43feceefdb2bfaf24a1561e))
    - [git-ref] frame for loose store reference lookup ([`30b0d54`](https://github.com//Byron/gitoxide/commit/30b0d54ed04916a858af3101345c677dbf48594d))
    - (cargo-release) version 0.2.0 ([`1327894`](https://github.com//Byron/gitoxide/commit/132789475400abe660b30ef6d2c5ff57821dd2c4))
    - [git-ref] use git-validate crate ([`6b4f937`](https://github.com//Byron/gitoxide/commit/6b4f937f13ad62bc2c7e5b0fc14416feb9c313ba))
    - [git-ref] Setup more tests to realize we really want validate::tag ([`54ee5b5`](https://github.com//Byron/gitoxide/commit/54ee5b5eace8c35bc33ef1261778ba0fcee2ef37))
    - [git-ref] frame for validation ([`9656ac6`](https://github.com//Byron/gitoxide/commit/9656ac620a1a085122676052b9a0b32d9c4f6661))
    - [git-ref] failure tests ([`567e86c`](https://github.com//Byron/gitoxide/commit/567e86caf83c73497b021d636ea440cc817f10ba))
    - [git-ref] more tests ([`048fb77`](https://github.com//Byron/gitoxide/commit/048fb775764004ec5bb39bf243a102233dd9946c))
    - [git-ref] refactor ([`77d0cc0`](https://github.com//Byron/gitoxide/commit/77d0cc088d6de8c37fec9ae0136c9f85bfdbc643))
    - [git-ref] don't support serde for now ([`2a6295b`](https://github.com//Byron/gitoxide/commit/2a6295bbd8a30d84c0d6544ca83e79146aff088e))
    - [git-ref] refactor ([`02e545b`](https://github.com//Byron/gitoxide/commit/02e545ba6fe801f43e0a76e43e8bcfaaf77bd5f5))
    - [git-ref] first basic 'ref: ' parsing ([`60fa3ba`](https://github.com//Byron/gitoxide/commit/60fa3bac9bfff7b5e3ac331c77c1050e9359f481))
    - [git-ref] refactor ([`9a30f87`](https://github.com//Byron/gitoxide/commit/9a30f87292aff1d4a2f043ba160df6b09bce16c8))
    - [git-ref] the first succeeding test ([`cebfdb4`](https://github.com//Byron/gitoxide/commit/cebfdb463ac2d86f56bb3a2d57c0487a8b233fd8))
    - [git-ref] the first failing test ([`7e802a0`](https://github.com//Byron/gitoxide/commit/7e802a0576230dfc666c253d484ea255f265f92f))
    - [git-ref] sketch ref creation ([`c5241b8`](https://github.com//Byron/gitoxide/commit/c5241b835b93af497cda80ce0dceb8f49800df1c))
    - [git-ref] A sketch of how it looks like with Store backref ([`1a08f1c`](https://github.com//Byron/gitoxide/commit/1a08f1c0365afe7d5e6fbc80bdd382d193d4b881))
    - [git-ref] more scaffolding ([`8c6e884`](https://github.com//Byron/gitoxide/commit/8c6e8844627878e981e597de0c29408cf51582a4))
    - [git-ref] clear it out and move existing functionality to git-object ([`fa548ce`](https://github.com//Byron/gitoxide/commit/fa548ce94db3dd3969add494756fcc34e48985a3))
    - (cargo-release) version 0.5.0 ([`b6b5856`](https://github.com//Byron/gitoxide/commit/b6b58560b7c3bc88e2b8b780be5ceb4cb508a346))
    - [pack-gen] refactor ([`61554e2`](https://github.com//Byron/gitoxide/commit/61554e2effcbafef9cff0b407351c2fae0d2916c))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 15 times to make code idiomatic. 

## v0.4.1 (2020-12-19)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 94 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.1 ([`25d2c2e`](https://github.com//Byron/gitoxide/commit/25d2c2e6ae70f46869ab0dabdda2b9f7840539d3))
    - Document `git-ref` ([`91dce23`](https://github.com//Byron/gitoxide/commit/91dce23c8faf74511c33e5cfa07d2f293b1cd0a2))
    - remove dash in all repository links ([`98c1360`](https://github.com//Byron/gitoxide/commit/98c1360ba4d2fb3443602b7da8775906224feb1d))
    - refactor ([`ba1d883`](https://github.com//Byron/gitoxide/commit/ba1d88364424eb60a0874a5726b62740dc348592))
</details>

## v0.4.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 29 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 ([`f9dd225`](https://github.com//Byron/gitoxide/commit/f9dd225afc4aafde1a8b8148943f56f2c547a9ea))
    - Allow dual-licensing with Apache 2.0 ([`ea353eb`](https://github.com//Byron/gitoxide/commit/ea353eb02fd4f75508600cc5676107bc7e627f1e))
    - refactor ([`63c1292`](https://github.com//Byron/gitoxide/commit/63c129292288cc626b09ad29e9ef5f1a1d8339e4))
</details>

## v0.3.0 (2020-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump minor version to 0.3 ([`4351e28`](https://github.com//Byron/gitoxide/commit/4351e2871c9dcf342b8471fffa74cae338a53269))
    - update to quick-error 2.0 ([`4b1b784`](https://github.com//Byron/gitoxide/commit/4b1b7849b47a54092b49821c39e864c86adda979))
</details>

## v0.2.0 (2020-07-23)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 6 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 ([`d350a13`](https://github.com//Byron/gitoxide/commit/d350a13784685ea82b84646b18736986aeb68146))
    - Switch to latest quick-error ([`9760856`](https://github.com//Byron/gitoxide/commit/976085614ee13a19fc1347209259a3dcf36ef95b))
    - assert we don't exeed package sizes ([`df66d74`](https://github.com//Byron/gitoxide/commit/df66d74aa2a8cb62d8a03383135f08c8e8c579a8))
</details>

## v0.1.0 (2020-07-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - refactor ([`6ad9304`](https://github.com//Byron/gitoxide/commit/6ad93041813f78548c3bd813b8685a60d857336f))
    - refactor ([`1fd90f7`](https://github.com//Byron/gitoxide/commit/1fd90f739f4d8bb7c4f27103d2bb92e3f58b6f68))
    - test for common ascii control characters ([`ae0c885`](https://github.com//Byron/gitoxide/commit/ae0c885518d9ce4de05adbb048c0188f9ca934c3))
    - all test for valid ref name except for ascii control chars ([`a157acf`](https://github.com//Byron/gitoxide/commit/a157acfb1f68ec6af6bb0b76f52aa8c7f72d43bf))
    - add new 'git-ref' crate; place ref name validation code there ([`1a0e84e`](https://github.com//Byron/gitoxide/commit/1a0e84e627b17be1b1fb53b4dc98ab78e9cfb9a7))
</details>

