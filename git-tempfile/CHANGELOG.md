# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 7 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#198](https://github.com//Byron/gitoxide/issues/198)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#198**
    - regenerate all changelogs to get links ([`d654788`](https://github.com//Byron/gitoxide/commit/d65478880a170235e4f838156862ed035894fd5b))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com//Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - respect release-wide ignore list to allow removing entire conventional headlines ([`145103d`](https://github.com//Byron/gitoxide/commit/145103d4aa715386da9d4953f7f85fadc49fff9a))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com//Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com//Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com//Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - Fixup remaining changelogs… ([`2f75db2`](https://github.com//Byron/gitoxide/commit/2f75db294fcf20c325555822f65629611be52971))
 * **Uncategorized**
    - make fmt, but now it picked up some parts that usually don't get altered… ([`01f7b72`](https://github.com//Byron/gitoxide/commit/01f7b729337bd2c99498321c479a9a13b1858e3e))
    - Update changelogs just for fun ([`21541b3`](https://github.com//Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
</details>

## v1.0.2 (2021-09-10)

- Compatibility with Rust 1.55. It informed about the incorrect usage of `std::io::ErrorKind::Other` which this crate also dependent on in its tests.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-tempfile v1.0.2 ([`310ea73`](https://github.com//Byron/gitoxide/commit/310ea7336e78fbedb2cefa1ecb773b25e6a77e0a))
    - Update changelogs once more… ([`d57d279`](https://github.com//Byron/gitoxide/commit/d57d279dc603cf450c151bbb6dc6c6505cc6da10))
    - Update changelogs retro-actively… ([`78cfe0a`](https://github.com//Byron/gitoxide/commit/78cfe0ac341c6c2257743d913884b50042078e6c))
</details>

## v1.0.1 (2021-09-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 15 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-tempfile v1.0.1 ([`295eb37`](https://github.com//Byron/gitoxide/commit/295eb374d104ac2775b9f864ef3234e2c5832b54))
    - [tempfile #195] adapt to Rust 1.55 ([`d9e71ac`](https://github.com//Byron/gitoxide/commit/d9e71acc5d619b5e78673da4fbb5a531c97ad6dd))
    - thanks clippy ([`4701296`](https://github.com//Byron/gitoxide/commit/4701296bd5e2c4ad2f80f4e1de498db49f93385a))
    - Merge pull request #172 from mellowagain/main ([`61aebbf`](https://github.com//Byron/gitoxide/commit/61aebbfff02eb87e0e8c49438a093a21b1134baf))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v1.0.0 (2021-08-25)

- initial release

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-tempfile v1.0.0 ([`1238535`](https://github.com//Byron/gitoxide/commit/123853539dc30ddea2d822ab177ee09b191bdf1b))
    - [stability #171] prepare git-lock and git-tempfile release ([`3a1cf4d`](https://github.com//Byron/gitoxide/commit/3a1cf4d441b53c880b5c887916302a493ad28b41))
    - [stability #171] Prime git-tempfile and git-lock for release ([`01278fe`](https://github.com//Byron/gitoxide/commit/01278fe4e28bf97ce6a2b8947198683646e361ee))
</details>

## v1.0.0 (2021-08-25)

## v0.6.1 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-tempfile v0.6.1 ([`eda952f`](https://github.com//Byron/gitoxide/commit/eda952f95e9ece78bbdbe6c26dd78f7ac5365d86))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com//Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.6.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 ([`d58f37e`](https://github.com//Byron/gitoxide/commit/d58f37e3b5a000fbe069aa869bd84f66d5c3210b))
    - [utils #154] refactor: bool.then(||this) - neat ([`1dec1c4`](https://github.com//Byron/gitoxide/commit/1dec1c49032c8acb449e463fde41f403cb640e45))
</details>

## v0.5.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 40 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`0e11e98`](https://github.com//Byron/gitoxide/commit/0e11e98f0562c7baa9c90e18db6240731d165217))
    - [pack #153] implement io traits for tempfiles ([`59d03d6`](https://github.com//Byron/gitoxide/commit/59d03d6133b301a19adfab212cf2c946110fc53b))
    - clippy on tests and thanks clippy ([`a77a71c`](https://github.com//Byron/gitoxide/commit/a77a71cf02d328a2a964388928d6b2a235a0aa85))
    - thanks clippy ([`e1964e4`](https://github.com//Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - Merge branch 'remove-unnecessary-unsafe' ([`7a3c5c1`](https://github.com//Byron/gitoxide/commit/7a3c5c14dc56d8711548d1b219a969836693cbaa))
    - Remove unnecessary pub(crate) exports ([`3d2456e`](https://github.com//Byron/gitoxide/commit/3d2456e11709f0461b37c6df55ecc3861ca4cab5))
    - [lock] support recoverable commits ([`b2217e7`](https://github.com//Byron/gitoxide/commit/b2217e7d25df9801354f702b0625d3168f8d3271))
    - [ref] support for persistence with recovery ([`d8b2d66`](https://github.com//Byron/gitoxide/commit/d8b2d661b9cf644b52950b9dedf8dbce0d348098))
    - [ref] refactor ([`a261b82`](https://github.com//Byron/gitoxide/commit/a261b82c1ee7ebdbbc82ce1c8286ca6a0f221cea))
    - [ref] allow reflogs to be created in place of empty directory trees ([`80a6e0e`](https://github.com//Byron/gitoxide/commit/80a6e0e1ff2321d9162e799d5fc0f457e7de4ade))
    - [tempfile] a way to delete empty dirs recursively ([`6025aa0`](https://github.com//Byron/gitoxide/commit/6025aa08d93cd5124c8df38c51b795b9c7d1c911))
    - Bump libc from 0.2.97 to 0.2.98 ([`caf6495`](https://github.com//Byron/gitoxide/commit/caf6495b08f77d7e4eaa058074693fffb5c5644b))
    - [tempfile] close a tempfile while keeping track of it ([`aa96ed1`](https://github.com//Byron/gitoxide/commit/aa96ed1776a615446b9864b1231f9f33805ab178))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.4.0 (2021-06-23)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 ([`4512798`](https://github.com//Byron/gitoxide/commit/45127986daba0a409f5b405d463fa23f5c4a053b))
    - [lock] add [must_use = "reason"] attribute where it matters ([`813c46b`](https://github.com//Byron/gitoxide/commit/813c46b1ac9ed5454c7832a6bad5a112f145b565))
    - [lock] refactor, remaining docs ([`956e69f`](https://github.com//Byron/gitoxide/commit/956e69fcb96085d96124b6c56d829607b36adf9f))
    - [lock] tests green ([`3706b26`](https://github.com//Byron/gitoxide/commit/3706b2669ebee5cd25a75a42d9b0a4a380707ee1))
    - [lock] cleanup signal handling even more… ([`9fb13d2`](https://github.com//Byron/gitoxide/commit/9fb13d27ccce5b0742ee9289fca891dbeb8a65de))
    - [lock] first tests and a lot of refactoring ([`3c34194`](https://github.com//Byron/gitoxide/commit/3c34194b6c0fd5ab22eb91081a563ba3bfa19110))
    - [lock] refactor; Marker is definitely not necessary… ([`6af84c9`](https://github.com//Byron/gitoxide/commit/6af84c92dbe049068be795ef4870fd830baf5384))
    - [lock] test what happens if multiple tempfiles are created ([`17942c7`](https://github.com//Byron/gitoxide/commit/17942c7960f25ad1f8f7fb2c94f251d39bb03c6e))
    - [lock] sketch API ([`f0e1427`](https://github.com//Byron/gitoxide/commit/f0e142734c1b09e6c4175b3c1b232d886449e280))
</details>

## v0.3.0 (2021-06-20)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 ([`92f3a83`](https://github.com//Byron/gitoxide/commit/92f3a830457766c88c68f8424828bfd9b5145f86))
    - [tempfile] refactor ([`f3144a8`](https://github.com//Byron/gitoxide/commit/f3144a897b4e10742fef47fadd350b4e9ddf316a))
    - [tempfile] remaining tests ([`450db66`](https://github.com//Byron/gitoxide/commit/450db6609eb3dad10deed3f9769a21ae8c4b3be8))
    - [tempfile] refactor ([`3bafa7b`](https://github.com//Byron/gitoxide/commit/3bafa7b2a3cf8efd0769564026ce7b757cb8c09b))
    - [tempfile] implement 'closed' version of tempfile ([`d4bb61d`](https://github.com//Byron/gitoxide/commit/d4bb61dbc99b4270464d903978222d31c7e7dc5e))
    - [tempfile] refactor ([`4788222`](https://github.com//Byron/gitoxide/commit/4788222c28605118c03ce9f3a4dfccc26e7f7b60))
    - [tempfile] fix docs ([`3cd6712`](https://github.com//Byron/gitoxide/commit/3cd6712c22dae2e804573bca0b7a687c36066c29))
    - [tempfile] sketch of a closed registration with different types ([`db9bb11`](https://github.com//Byron/gitoxide/commit/db9bb11a3132961029e04f1cf761bfc3c96ec33d))
    - [tempfile] refactor ([`8a0ce64`](https://github.com//Byron/gitoxide/commit/8a0ce64baf5a3d77a08aa68c3245be8e7964be70))
    - [tempfile] typesafe diffentiation between writable tempfiles and closed ones ([`3b424b1`](https://github.com//Byron/gitoxide/commit/3b424b1cc071580303d37b7459e80036635eb4aa))
    - [tempfile] refactor ([`913f301`](https://github.com//Byron/gitoxide/commit/913f3014313fe0c03cd8f0af88944d8d514d89d9))
    - [tempfile] refactor ([`9384617`](https://github.com//Byron/gitoxide/commit/9384617dbe00dd59726cc418f23fb0a6e6dde415))
    - [tempfile] implement 'map' on tempfile to realize that 'close()' can't be done… ([`f4a1d33`](https://github.com//Byron/gitoxide/commit/f4a1d33e994e986604d18a85b7f85e1cea063dcf))
</details>

## v0.2.0 (2021-06-19)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 46 commits contributed to the release over the course of 3 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 ([`7c2eb36`](https://github.com//Byron/gitoxide/commit/7c2eb36274d13646956ac850bee90abbbac91c5b))
    - [tempfile] improve docs ([`d311b08`](https://github.com//Byron/gitoxide/commit/d311b082cdec323eb76363d986064fe771aa2bfd))
    - thanks clippy ([`a0f9803`](https://github.com//Byron/gitoxide/commit/a0f9803533e5684cfed5ab50cd8145d071e978b2))
    - [tempfile] refactor ([`3a0f1ad`](https://github.com//Byron/gitoxide/commit/3a0f1ad0963c77a07f1984c39b127337463c030b))
    - [tempfile] refactor ([`9b8abd0`](https://github.com//Byron/gitoxide/commit/9b8abd0336d6ea1d7c088c78fc09fa935408896f))
    - [tempfile] cleanup control for named and unnamed tempfiles ([`0ef85a2`](https://github.com//Byron/gitoxide/commit/0ef85a247d60332ca232d6d993987c0b89e34466))
    - [tempfile] all remaining remove_dir tests I can think of ([`3e45e5f`](https://github.com//Byron/gitoxide/commit/3e45e5fef4f0bbd8736ae3f197f15813290fe8dc))
    - [tempfile] first bunch of tests for error handling and basic function of rmdir ([`ba41a15`](https://github.com//Byron/gitoxide/commit/ba41a15d874a2709ab92a8680d9e168ece7b4676))
    - [tempfile] quick impl of remove-dir iter without tests ([`bf48913`](https://github.com//Byron/gitoxide/commit/bf48913b3bc1a8c3ebaa230880f573ad01982f08))
    - [tempfile] refactor ([`9226dbe`](https://github.com//Byron/gitoxide/commit/9226dbe18127d7e85ba2779393cb7263e87cfbf8))
    - [tempfile] refactor ([`730b733`](https://github.com//Byron/gitoxide/commit/730b733a1a5b2c3911849eef6ffe0833e12daf73))
    - [tempfile] refactor ([`1da35ce`](https://github.com//Byron/gitoxide/commit/1da35ce045609296c189133ca439a74b550ccc6c))
    - [tempfile] improve Retries documentation; retries docs for remove_dir ([`e665a5f`](https://github.com//Byron/gitoxide/commit/e665a5fcd38c7002545079c63f0bf35dee11876d))
    - [tempfile] sketch how tempfile cleanup should be configured… ([`71acede`](https://github.com//Byron/gitoxide/commit/71acede9cba6fc222d0bde1a3fd0c232d3c877ab))
    - [tempfile] logic fixed, it's working ([`6ad4946`](https://github.com//Byron/gitoxide/commit/6ad4946e2ee603c69dad1da3e1e996cd1d4ca075))
    - [tempfile] better counting, but… ([`972113f`](https://github.com//Byron/gitoxide/commit/972113f1ea72674db61867b14f0eed0de498b310))
    - [tempfile] better retry counts ([`c7a35ca`](https://github.com//Byron/gitoxide/commit/c7a35caa295580a1b9d4f8b77eb8ded9d9c88703))
    - [tempfile] refactor ([`273d722`](https://github.com//Byron/gitoxide/commit/273d72276a73d49a633b9be1c66f1a2357dfcb0f))
    - [tempfile] a better way to count retries… ([`e110b97`](https://github.com//Byron/gitoxide/commit/e110b97b4925a10fa9a62576daf9f078508b6760))
    - [tempfile] trying to implement remove_dir really shows that… ([`1319b90`](https://github.com//Byron/gitoxide/commit/1319b908cc42ef5114d22957ebed9ed2ced11391))
    - [tempfile] docs for create_dir; frame for remove_dir; ([`aa6b6d1`](https://github.com//Byron/gitoxide/commit/aa6b6d14236c817ecc64390b110069c4c1340c03))
    - [tempfile] tests for automatic directory creation ([`3bd5cee`](https://github.com//Byron/gitoxide/commit/3bd5cee0dc0811ff3b1ab3d1a93e7dca8ae06b69))
    - [tempfile] refactor ([`d441312`](https://github.com//Byron/gitoxide/commit/d4413125c432da2e7ef809aca61973f5f55dbd5c))
    - [tempfile] use create_dir::all based on configuration… ([`156c021`](https://github.com//Byron/gitoxide/commit/156c021ac8aaabe8fed60ac4681f365c75e0e165))
    - [tempfile] remove todo ([`5a14ab6`](https://github.com//Byron/gitoxide/commit/5a14ab63555d6e3a58ce32b68d4b47287869b73f))
    - [tempfile] more information about error cases, too ([`7415141`](https://github.com//Byron/gitoxide/commit/74151415f0019a32b4759cf01873acb4102f2d1e))
    - [tempfile] refactor ([`ae0c97a`](https://github.com//Byron/gitoxide/commit/ae0c97a59d9cc56e19d3ce6fcc12b4564a66298a))
    - [tempfile] refactor ([`7c7658d`](https://github.com//Byron/gitoxide/commit/7c7658d3390fdf1b5348a482c71a9fb20a815cca))
    - [tempfile] test for racy directory creation… ([`c9073bf`](https://github.com//Byron/gitoxide/commit/c9073bf0d6dff3165cfd43733a602127b8835727))
    - [tempfile] verify existing files are handled correctly ([`28fee55`](https://github.com//Byron/gitoxide/commit/28fee552718cbbed067b8a16631aaa1080886e00))
    - [tempfile] a test for directory creation limits ([`584b4b7`](https://github.com//Byron/gitoxide/commit/584b4b7a1e6997594f1234d5feab1bc82a83b859))
    - [tempfile] limit retries for directory creation… ([`1536c7a`](https://github.com//Byron/gitoxide/commit/1536c7a58f1da4b80e83f1169b3f865f12a3d9a2))
    - [tempfile] refactor ([`fa7b8e9`](https://github.com//Byron/gitoxide/commit/fa7b8e99d2613297127b044605a2314b878d3ab9))
    - [tempfile] handle interrupts and assure there is an end to it ([`dc0afbd`](https://github.com//Byron/gitoxide/commit/dc0afbdf08c44237b6749426ebacbded6cf8a647))
    - [tempfile] first recursive directory creation ([`b01faa9`](https://github.com//Byron/gitoxide/commit/b01faa9fdc371c1226978e32a3c71dbf3be600ec))
    - [tempfile] refactor ([`7b59392`](https://github.com//Byron/gitoxide/commit/7b59392fec4c80eddd9f82271eb1a5671e44aa5a))
    - [tempfile] another test ([`9e4834d`](https://github.com//Byron/gitoxide/commit/9e4834df1142fd0ffdbf5ffba1aed63bc67b66b3))
    - [tempfile] first sketch of iterator based create directory all… ([`314693c`](https://github.com//Byron/gitoxide/commit/314693c6a4577f0b2b00274a55ec99e87c17918f))
    - [lock] frame for git-lock crate ([`e6bc87d`](https://github.com//Byron/gitoxide/commit/e6bc87d77f9b397b25694f58d347de2fc38bf71d))
    - [tempfile] add journey test to validate operation on process level ([`2d1efd4`](https://github.com//Byron/gitoxide/commit/2d1efd4915d66890b1132d5b39e08027a83047ba))
    - [tempfile] more docs ([`d0c5e6b`](https://github.com//Byron/gitoxide/commit/d0c5e6b96f27d7ae708e7182b4cd5dbaceecd3cd))
    - refactor ([`e0b7f69`](https://github.com//Byron/gitoxide/commit/e0b7f695ee6bd1032544a29d91906f9b75e12d46))
    - [tempfile] clean cargo manifest ([`d43f514`](https://github.com//Byron/gitoxide/commit/d43f51438937d5bdd2bb2e02c355dcd4ee2b8680))
    - [tempfile] fix windows for good ([`3192a47`](https://github.com//Byron/gitoxide/commit/3192a47b730245f2656ccf8cd82394ec31e13126))
    - [tempfile] fix windows build (hopefully) ([`6c1df66`](https://github.com//Byron/gitoxide/commit/6c1df667031084a9e6fe9676534f80aae9a95789))
    - [tempfile] refactor ([`4a45df0`](https://github.com//Byron/gitoxide/commit/4a45df02340b55d34534be89934d2201dda261f9))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.1.0 (2021-06-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [tempfile] prepare release ([`c0f7fde`](https://github.com//Byron/gitoxide/commit/c0f7fde70b28526ad52dce9e2314a25af1531689))
    - [tempfile] an example to show off signal handlers ([`f325e69`](https://github.com//Byron/gitoxide/commit/f325e696c64e3f61f64c0a8d8c4e8af38104a713))
    - [tempfile] remaining docs ([`d334dc0`](https://github.com//Byron/gitoxide/commit/d334dc004d8b8eea5b586c6ada173d28d380ccce))
    - [tempfile] restore original signal handler behaviour. ([`9f91dd8`](https://github.com//Byron/gitoxide/commit/9f91dd8e95e1e51a8e0a4f7ba45628b3d93fc5de))
    - [tempfile] process-id filter on deletion to support forks ([`611056f`](https://github.com//Byron/gitoxide/commit/611056f431dc793684efad668d40b93b1cfec21e))
    - [tempfile] implement handler correctly, probably. ([`8cb0bbc`](https://github.com//Byron/gitoxide/commit/8cb0bbcf2d022401886071f9c91498977cea185c))
    - [tempfile] remove tempfiles on shutdown, but… ([`954b760`](https://github.com//Byron/gitoxide/commit/954b76029e4d9e331738137ec2c9804b0e06a890))
    - [tempfile] switch to dashmap as slab ([`6164719`](https://github.com//Byron/gitoxide/commit/61647195ce8fd0be1b3b63f19e8aaec392f33f19))
    - [tempfile] a more realistic slab test shows the index can get quite high. ([`915f14c`](https://github.com//Byron/gitoxide/commit/915f14c41145dbd3f63bd798e6d0cc18d51fef6f))
    - [tempfile] first step towards clearing tempfiles… ([`b0e0cee`](https://github.com//Byron/gitoxide/commit/b0e0cee866b643f9f9e4ebdc495abed5f5c6abf9))
    - [tempfile] precisely named tempfiles ([`edc74f0`](https://github.com//Byron/gitoxide/commit/edc74f0e8f04f45661d4909bb3e6c62f4ac1b453))
    - [tempfile] `take()` method ([`d377397`](https://github.com//Byron/gitoxide/commit/d3773976b86ad294528997104b9cfa0c803f9d6a))
    - [tempfile] basic operation of a tempfile ([`a692950`](https://github.com//Byron/gitoxide/commit/a692950ae7c32ed9dd040c0aebde494ef3029a30))
    - [tempfile] show that slabs can store a lot actually ([`0cc5b33`](https://github.com//Byron/gitoxide/commit/0cc5b33f0e421ed761e5c350fb4d3ad85ef3e884))
    - [tempfile] initial docs as there is a lot to consider ([`9dffc2b`](https://github.com//Byron/gitoxide/commit/9dffc2b918178650a3b40adfcc35730e48f46951))
    - [tempfile] crate frame ([`1b04c39`](https://github.com//Byron/gitoxide/commit/1b04c39030b436fb6850fbfa0c39a4fed7df727c))
</details>

