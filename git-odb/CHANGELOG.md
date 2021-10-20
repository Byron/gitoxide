# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.23.0 (2021-10-19)

A maintenance release to properly dealing with previously breaking changes in `git-hash`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#222](https://github.com//Byron/gitoxide/issues/222)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#222](https://github.com//Byron/gitoxide/issues/222)**
    - update changelogs prior to release ([`9a493d0`](https://github.com//Byron/gitoxide/commit/9a493d0651b0b6d71cf230dc510a658be7f8cb19))
    - stabilize changelogs ([`920e832`](https://github.com//Byron/gitoxide/commit/920e83219911df1c440d3fe42fd5ec3a295b0bb8))
    - Update changelogs prior to release ([`b3e2252`](https://github.com//Byron/gitoxide/commit/b3e2252f7461a003d9a4612da60ba931dd8c0bef))
</details>

## v0.22.0 (2021-10-15)

### Dependency Upgrade (BREAKING)

* `git-traverse` saw a breaking change moving to v0.9, which triggered this crate to signal a breaking change, too.

### Improvements

* pack-ids as generated when instantiating a compound database are now sequential. That way, they never clash which
  was possible before when it was using a CRC32 over the filename.

  The latter was done to allow deduplicating packs, but it turns out not to be necessary just yet.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 24 commits contributed to the release over the course of 31 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#198](https://github.com//Byron/gitoxide/issues/198), [#67](https://github.com//Byron/gitoxide/issues/67)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com//Byron/gitoxide/issues/198)**
    - Adjust all changelogs to fulfil requirements for publishing ([`04b9ca0`](https://github.com//Byron/gitoxide/commit/04b9ca025a1667529b2221ab4280bd3c8dae01cf))
    - deduplicate conventional message ids ([`e695eda`](https://github.com//Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - regenerate all changelogs to get links ([`0c81769`](https://github.com//Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com//Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - Allow 'refactor' and 'other' in conventional messages if they have breaking changes ([`4eebaac`](https://github.com//Byron/gitoxide/commit/4eebaac669e590beed112b622752997c64772ef1))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com//Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com//Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com//Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - Fixup remaining changelogs… ([`2f75db2`](https://github.com//Byron/gitoxide/commit/2f75db294fcf20c325555822f65629611be52971))
    - Generate changelogs with details ([`e1861ca`](https://github.com//Byron/gitoxide/commit/e1861caa435d312953a9fea7ceff6d2e07b03443))
    - Update all changelogs with details ([`58ab2ae`](https://github.com//Byron/gitoxide/commit/58ab2aee23ba70a536e9487b44fb04c610374d1a))
    - Update changelogs ([`c857d61`](https://github.com//Byron/gitoxide/commit/c857d61ce3ce342012a2c4ba10a8327822aa530e))
    - Avoid adding newlines which make writing unstable ([`6b5c394`](https://github.com//Byron/gitoxide/commit/6b5c394f49282a8d09c2a9ffece840e4683572db))
    - Fix section headline level ([`9d6f263`](https://github.com//Byron/gitoxide/commit/9d6f263beef289d227dec1acc2d4240087cb9be6))
    - Write first version of changlogs thus far… ([`719b6bd`](https://github.com//Byron/gitoxide/commit/719b6bdf543b8269ccafad9ad6b46e0c55efaa38))
    - Parse more user generated section content, adapt existing changelogs to work correctly ([`2f43a54`](https://github.com//Byron/gitoxide/commit/2f43a54298e7ecfff2334627df149fe0882b5d1d))
 * **[#67](https://github.com//Byron/gitoxide/issues/67)**
    - Assure pack-ids are actually unique, the simple way… ([`0509b4f`](https://github.com//Byron/gitoxide/commit/0509b4fb5a78a3e4bfcacbeb661d262f8592884a))
 * **Uncategorized**
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com//Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com//Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - Don't let 'a' leak out of the tempdir to fix #212 ([`682dff8`](https://github.com//Byron/gitoxide/commit/682dff83520fa7a09c88ce83fd1d6439c377a480))
    - More proper fix for #212 ([`dfc1493`](https://github.com//Byron/gitoxide/commit/dfc1493de348c166cd11eb6af3c145c994126472))
    - naive fix for #212 ([`b31863c`](https://github.com//Byron/gitoxide/commit/b31863cc643833db5a7ac2edfe38b942dd15790c))
    - Update changelogs just for fun ([`21541b3`](https://github.com//Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
    - Bump git-traverse v0.9.0, safety bump 8 crates ([`d39fabb`](https://github.com//Byron/gitoxide/commit/d39fabb8757369aa19452a457f610fe21dc13a14))
</details>

## v0.21.3 (2021-09-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-odb v0.21.3 ([`223f930`](https://github.com//Byron/gitoxide/commit/223f93075a28dd49f44505c039cfeae5a7296914))
    - [smart-release #195] fix docs ([`8d7e132`](https://github.com//Byron/gitoxide/commit/8d7e132d055d8c87ea3e45de15593964a61b0608))
</details>

## v0.21.2 (2021-09-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-odb v0.21.2 ([`d443644`](https://github.com//Byron/gitoxide/commit/d44364445cfbae861ce45df8bddec1b34e03f454))
    - Bump git-pack v0.11.0 ([`5ae6ff5`](https://github.com//Byron/gitoxide/commit/5ae6ff52cd2cd1ccd1e26bb987c154eb19603696))
    - Bump git-object v0.14.0 ([`d4fc81f`](https://github.com//Byron/gitoxide/commit/d4fc81f6390443f8c8561d91ac27ea4a6318fb62))
    - [repository #164] Prepare `commit()` for a possible less-allocating future ([`0fd01f7`](https://github.com//Byron/gitoxide/commit/0fd01f7071c785c27c56d2c034aac8dcdf690677))
    - [repository #164] generic write_object() ([`c569f83`](https://github.com//Byron/gitoxide/commit/c569f83363489dde03c8b9cd01e75d35f5e04dbc))
    - [repository #164] Support for refreshing the object database ([`46e10f8`](https://github.com//Byron/gitoxide/commit/46e10f863e1fea419483a7b086022c16cd0ca226))
    - [odb #164] Add refresh() functionality ([`ee16d04`](https://github.com//Byron/gitoxide/commit/ee16d041941a5777c8f6495a28f7633c327cbd6b))
</details>

## v0.21.1 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-odb v0.21.1 ([`cb33c2f`](https://github.com//Byron/gitoxide/commit/cb33c2f71c2e1e228ba0d2299fb531cf2a5f3b23))
    - [odb #190] Read all eligble packed refs, no "pack-" prefix needed ([`ab250f7`](https://github.com//Byron/gitoxide/commit/ab250f76b356c0937ada959591dc4df3872acf8f))
    - Bump git-pack v0.10.0 ([`e5e3c80`](https://github.com//Byron/gitoxide/commit/e5e3c8024e1c2e5e90cee83abbdae41d58eee156))
    - Bump git-hash v0.6.0 ([`6efd90d`](https://github.com//Byron/gitoxide/commit/6efd90db54f7f7441b76159dba3be80c15657a3d))
</details>

## v0.21.0 (2021-08-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 4 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [repository #174] adjust various changelogs ([`081faf5`](https://github.com//Byron/gitoxide/commit/081faf5c3a21b34b7068b44d8206fb5770c392f5))
    - [odb #180] fix docs ([`bd50752`](https://github.com//Byron/gitoxide/commit/bd50752dd9188acd92b8503db53cc2ce8112c61f))
    - [odb #180] refactor ([`eff21da`](https://github.com//Byron/gitoxide/commit/eff21dae1083042412f45cd2f7a0faaf7d6400e6))
    - Bump git-odb v0.21.0 ([`7b9854f`](https://github.com//Byron/gitoxide/commit/7b9854fb35e86958a5ca827ec9a55b1168f38395))
    - [odb #180] add changelog ([`acf1193`](https://github.com//Byron/gitoxide/commit/acf1193e6b72433d4b74ec9fd39de148529224c5))
    - [pack #179] refactor bundle ([`420dca2`](https://github.com//Byron/gitoxide/commit/420dca29bccca6e7d759880d8342f23b33eead0d))
    - [pack #179] refactor ([`ab6554b`](https://github.com//Byron/gitoxide/commit/ab6554b0cd5838f1ea4e82f6b5019798288076fa))
    - [object #177] move mutable objects to crate::* ([`c551c02`](https://github.com//Byron/gitoxide/commit/c551c0236c64f3237cb9be7f35159f753d4b871f))
    - [object #177] migrate immutable::tree to crate::tree ([`fa5cd06`](https://github.com//Byron/gitoxide/commit/fa5cd0648d5c855060ab2b75ee933851987c2dcf))
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments ([`461dc53`](https://github.com//Byron/gitoxide/commit/461dc53ba3bc07d55fdb4aad7570ba9176a8b360))
    - [object #177] rename immutable::* to immutable::*Ref ([`6deb012`](https://github.com//Byron/gitoxide/commit/6deb01291fb382b7fb9206682e319afa81bacc05))
    - Release git-object v0.13.0 ([`708fc5a`](https://github.com//Byron/gitoxide/commit/708fc5abd8af4dd7459f388c7092bf35915c6662))
    - Merge pull request #172 from mellowagain/main ([`61aebbf`](https://github.com//Byron/gitoxide/commit/61aebbfff02eb87e0e8c49438a093a21b1134baf))
    - [actor #173] rename immutable::Signature to SignatureRef! ([`96461ac`](https://github.com//Byron/gitoxide/commit/96461ace776d6b351b313d4f2697f2d95b9e196e))
    - Release git-pack v0.9.0 ([`7fbc961`](https://github.com//Byron/gitoxide/commit/7fbc9617da97d4ba4bb3784f41d4163c0839c03c))
    - [pack #67] refactor ([`14717f6`](https://github.com//Byron/gitoxide/commit/14717f6132672a5d271832a68de0b323b73abb2a))
</details>

## v0.20.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-odb v0.20.2 ([`6fb8bbb`](https://github.com//Byron/gitoxide/commit/6fb8bbb11e87911424c95001ce851bc4665920e9))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com//Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.20.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-odb v0.20.1 ([`ca3f736`](https://github.com//Byron/gitoxide/commit/ca3f736ae3e6a0a5541223364db874a8e31dd3ec))
    - remove dev-dependency cycles by removing their version ([`c40faca`](https://github.com//Byron/gitoxide/commit/c40faca41632cd2a226daf4ddf5293b65d1fdc82))
</details>

## v0.20.0 (2021-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 ([`f123f69`](https://github.com//Byron/gitoxide/commit/f123f69c7a4f9fd1c98bd2f60ebc953a6739fe04))
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 ([`c67291f`](https://github.com//Byron/gitoxide/commit/c67291ff9bcdff9a747d87241f6a71015607af05))
    - Release git-object v0.12.0 ([`7006150`](https://github.com//Byron/gitoxide/commit/7006150ac314d19814608723f69f6e70a72f9262))
    - Release git-actor-0.3.1 ([`727087d`](https://github.com//Byron/gitoxide/commit/727087dca243da4bc40bc87611a2f66234565be7))
</details>

## v0.18.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.18.0 ([`b327590`](https://github.com//Byron/gitoxide/commit/b327590d02fec5536c380b2d39dd7be089ca7c40))
    - (cargo-release) version 0.6.0 ([`d704bca`](https://github.com//Byron/gitoxide/commit/d704bca7de0a6591f35345c842d6418b36ecd206))
</details>

## v0.17.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.17.0 ([`c52a491`](https://github.com//Byron/gitoxide/commit/c52a49176bd294bb36db74b4293cdb684a2ab7f6))
    - (cargo-release) version 0.5.0 ([`c2f94a5`](https://github.com//Byron/gitoxide/commit/c2f94a51bce287be301090450cb00cde57e92f76))
    - (cargo-release) version 0.4.0 ([`d69d0ac`](https://github.com//Byron/gitoxide/commit/d69d0ac21989243fdafa514fa41579fd51bc2558))
    - (cargo-release) version 0.11.0 ([`a5be31c`](https://github.com//Byron/gitoxide/commit/a5be31c4cf7c0b538a1ed4a52ff5c3a992c6feff))
    - (cargo-release) version 0.3.0 ([`64efc05`](https://github.com//Byron/gitoxide/commit/64efc0534ddc372b6e668b23c1e9d276098679c9))
    - (cargo-release) version 0.4.0 ([`70ef344`](https://github.com//Byron/gitoxide/commit/70ef3442775b54ba9e4ee9ebfffb37af9804cc5b))
    - [utils #154] refactor: bool.then(||this) - neat ([`1dec1c4`](https://github.com//Byron/gitoxide/commit/1dec1c49032c8acb449e463fde41f403cb640e45))
</details>

## v0.16.1 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.16.1 ([`8cd173b`](https://github.com//Byron/gitoxide/commit/8cd173b32138a136e6109518c179bf7738fe6866))
</details>

## v0.16.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 113 commits contributed to the release over the course of 83 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 ([`0e9c73a`](https://github.com//Byron/gitoxide/commit/0e9c73abd17e0dd21952275077ae53ad7e7aa1af))
    - (cargo-release) version 0.5.0 ([`ae02dab`](https://github.com//Byron/gitoxide/commit/ae02dabae961089a92a21e6a60a7006de4b56dad))
    - (cargo-release) version 0.16.0 ([`1231dbd`](https://github.com//Byron/gitoxide/commit/1231dbd16dacefb39adec8e067c312d313a82e3c))
    - (cargo-release) version 0.2.0 ([`8ff5115`](https://github.com//Byron/gitoxide/commit/8ff511583e6d859e43ffda0ef75e2fecce3ed03c))
    - clippy on tests and thanks clippy ([`a77a71c`](https://github.com//Byron/gitoxide/commit/a77a71cf02d328a2a964388928d6b2a235a0aa85))
    - thanks clippy ([`e1964e4`](https://github.com//Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - [pack] a way to obtain whole bundles for offset-to-index lookup ([`15fcbe2`](https://github.com//Byron/gitoxide/commit/15fcbe254b75e8f74652711cc339ae5ade74d24c))
    - [pack] refactor ([`64b1dcd`](https://github.com//Byron/gitoxide/commit/64b1dcdb0fb53749ce73017d0dc1e053689d17d4))
    - [pack] bundle::Location with pack offset; order counts by that… ([`f92f285`](https://github.com//Byron/gitoxide/commit/f92f285167c6b5bc4d86f255e360c4534e38bb29))
    - Don't use ASM on windows for Sha1 as it fails to build there. ([`ba1fb7a`](https://github.com//Byron/gitoxide/commit/ba1fb7ab5bc03f5a23ece32ff1e144544e1eaeae))
    - Merge branch 'remove-unnecessary-unsafe' ([`7a3c5c1`](https://github.com//Byron/gitoxide/commit/7a3c5c14dc56d8711548d1b219a969836693cbaa))
    - Remove unnecessary pub(crate) exports ([`3d2456e`](https://github.com//Byron/gitoxide/commit/3d2456e11709f0461b37c6df55ecc3861ca4cab5))
    - Bump thiserror from 1.0.25 to 1.0.26 ([`9682590`](https://github.com//Byron/gitoxide/commit/9682590095dc3a502b0c84ccd206ca4797635092))
    - [actor] fix gix hours ([`b4e95fd`](https://github.com//Byron/gitoxide/commit/b4e95fdbb6664adcb2603d9cb6e6a69182de050f))
    - [actor] git-object uses git-actor ([`d01dd2f`](https://github.com//Byron/gitoxide/commit/d01dd2f9e9e8e2b81cdb1131a436d32b5819b731))
    - thanks clippy ([`3f7e27b`](https://github.com//Byron/gitoxide/commit/3f7e27b91e2c7d66959f5f4c1a667f3315111cd6))
    - (cargo-release) version 0.3.0 ([`6b33678`](https://github.com//Byron/gitoxide/commit/6b33678f83e6d261ca15c4a7634ff5b4e66d81dd))
    - Merge branch 'dependabot/cargo/crc-2.0.0' ([`683c44d`](https://github.com//Byron/gitoxide/commit/683c44db682d8dbef401286963e84cdca145abc8))
    - (cargo-release) version 0.2.0 ([`3286e42`](https://github.com//Byron/gitoxide/commit/3286e42547b59df6365087cbae9ce1c9c959faad))
    - thanks clippy ([`c5b4de8`](https://github.com//Byron/gitoxide/commit/c5b4de8c7675da47b5d6325d2993f4ebce4a8f0c))
    - [git-odb] linked::Store can now check if an object exists ([`bb95c79`](https://github.com//Byron/gitoxide/commit/bb95c7917a272bfe7eb04bea66685d6a1196dc25))
    - refactor ([`a25a774`](https://github.com//Byron/gitoxide/commit/a25a774675e2e9db1c891351077d3af2fd5c72ed))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com//Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - [git-ref] the first failing test ([`7e802a0`](https://github.com//Byron/gitoxide/commit/7e802a0576230dfc666c253d484ea255f265f92f))
    - [git-odb] fix test compiilation ([`639bc10`](https://github.com//Byron/gitoxide/commit/639bc10e1698beb4c9e7902c2545dd0a3e90e698))
    - [git-odb] much better docs; cleanup exposed API ([`3d5b229`](https://github.com//Byron/gitoxide/commit/3d5b229c2605060f2cac9695ff2479777deabdd0))
    - (cargo-release) version 0.2.0 ([`b213628`](https://github.com//Byron/gitoxide/commit/b213628feeb8dfa87dab489c7d3155a60e6a236d))
    - [git-odb] refactor ([`2958145`](https://github.com//Byron/gitoxide/commit/2958145a0ae1ef582bbf88352f5567d5c2b5eaf0))
    - [git-odb] refactor ([`1eab15d`](https://github.com//Byron/gitoxide/commit/1eab15dfb42c819050b0277c4cb6a1045d2fd58d))
    - [git-odb] refactor ([`4967c22`](https://github.com//Byron/gitoxide/commit/4967c22340679e953ec6e6319b671455456f93bc))
    - [git-odb] refactor ([`2e68e0c`](https://github.com//Byron/gitoxide/commit/2e68e0c9296977eaaf239b8f0ede6e285b13d06c))
    - [git-odb] fix docs ([`936cfd3`](https://github.com//Byron/gitoxide/commit/936cfd3af731ed822464765f532dd49a206f596d))
    - [git-pack] compilation ([`b392a55`](https://github.com//Byron/gitoxide/commit/b392a55b97a30b10ac0db94a96230e22ea7ab0dc))
    - [git-pack] refactor ([`ea2b3de`](https://github.com//Byron/gitoxide/commit/ea2b3deab78882943e11270e4166ca7c340b03e1))
    - [git-pack] refactor ([`5ca2547`](https://github.com//Byron/gitoxide/commit/5ca25477c44ff6c606901080e25df57371d9ec9c))
    - [git-pack] refactor ([`157b6ff`](https://github.com//Byron/gitoxide/commit/157b6ff7b55ba2b7f8f90f66864212906426f8d7))
    - (cargo-release) version 0.16.0 ([`769c649`](https://github.com//Byron/gitoxide/commit/769c649c00c009bf5a3f7c0611a7b999618f2938))
    - [git-pack] refactor ([`be6ddaa`](https://github.com//Byron/gitoxide/commit/be6ddaa98fc1dcaf77dc0fd9c9d67754e74927e4))
    - [git-pack] refactor ([`1fab6af`](https://github.com//Byron/gitoxide/commit/1fab6af317fd42662c59f82b476917da29cd3c60))
    - [git-pack] refactor ([`e5b00ee`](https://github.com//Byron/gitoxide/commit/e5b00ee257b712477413f48448b0bccf9a06bfaf))
    - [git-pack] the world compiles again ([`f0c0e36`](https://github.com//Byron/gitoxide/commit/f0c0e36a1fb15d44776678567162ac754fdd26c0))
    - [git-pack] used by git-odb ([`5d6ee07`](https://github.com//Byron/gitoxide/commit/5d6ee07a8dec64fe5f68c14c418d922077fad3df))
    - [git-features] refactor to help understand a zlib-related logic bug ([`ae826e8`](https://github.com//Byron/gitoxide/commit/ae826e8c3240efd14939beedd33a06695a6c112b))
    - [git-features] a first step towards supporting a pure rust zlib backend ([`040cab7`](https://github.com//Byron/gitoxide/commit/040cab7f27de83b283957189244d523d71ca1457))
    - [git-odb] refactor ([`e07478c`](https://github.com//Byron/gitoxide/commit/e07478c7b212e4d1d21ce151d9eb26d0fae422a8))
    - [git-odb] fix docs ([`05347d4`](https://github.com//Byron/gitoxide/commit/05347d4154d43d4657839a9cadcebeb1f44ec728))
    - [git-odb] refactor ([`721303d`](https://github.com//Byron/gitoxide/commit/721303db232f87857aae58e12b342e5fb0139306))
    - [git-odb] refactor ([`ea224e9`](https://github.com//Byron/gitoxide/commit/ea224e9ee5f7efcbf4942a2a6fc7e4d790b2be50))
    - [git-odb] refactor ([`6a1b16a`](https://github.com//Byron/gitoxide/commit/6a1b16ae98edc9a694b945a12a7866eb17fc6be3))
    - [git-odb] refactor ([`bae3980`](https://github.com//Byron/gitoxide/commit/bae3980e01131e7da38146aa510d1243e558a01a))
    - [git-odb] refactor ([`6b7400b`](https://github.com//Byron/gitoxide/commit/6b7400bdcfc793d598f2102576939e55a5a3fc43))
    - [git-odb] refactor ([`19ab0cb`](https://github.com//Byron/gitoxide/commit/19ab0cba168cd037107e5cc16a360884d40bd775))
    - [git-odb] refactor ([`47c4042`](https://github.com//Byron/gitoxide/commit/47c4042f16a0e0e6a536bab7150b7cb21958a7ed))
    - [pack-gen] refactor ([`b5618ca`](https://github.com//Byron/gitoxide/commit/b5618cad9f2a2403058b9b73ff1ada53ba85e8d0))
    - (cargo-release) version 0.15.0 ([`d69d9fb`](https://github.com//Byron/gitoxide/commit/d69d9fb0931f8257cef96ef14a89da9340ad9738))
    - Put prodash behind a feature toggle, too ([`966058d`](https://github.com//Byron/gitoxide/commit/966058d611c548e90c050462de52e36f1925e775))
    - Put 'walkdir' behind a feature flag/make it optional. ([`1a3cc5b`](https://github.com//Byron/gitoxide/commit/1a3cc5bea1868ed3ae015403fbe0cdec788be749))
    - Put 'sha1' behind a feature toggle ([`4f326bc`](https://github.com//Byron/gitoxide/commit/4f326bc261c4e7f0d5510df74ad4215da3580696))
    - Put crc functionality behind a feature toggle ([`458fa6e`](https://github.com//Byron/gitoxide/commit/458fa6ec726ec7901c1f6d970cbb1c1ea975dded))
    - Revert "[pack-gen] quick hack for obtaining the entry size more quickly" ([`4c36f92`](https://github.com//Byron/gitoxide/commit/4c36f92880d52886b1fb2c37cf2f98e6b9a327a0))
    - [pack-gen] quick hack for obtaining the entry size more quickly ([`ad6d007`](https://github.com//Byron/gitoxide/commit/ad6d00701d28befda006f41f85bbbb6fc3508e1e))
    - Revert "[pack-gen] remove tree-diff as traversal option." ([`2907a5f`](https://github.com//Byron/gitoxide/commit/2907a5facb08a7decbdfa652e76eb0ebd5e29dcf))
    - [pack-gen] remove tree-diff as traversal option. ([`8373671`](https://github.com//Byron/gitoxide/commit/8373671fd4f3f7e9d78c480e9f68c0a7ae423c69))
    - [pack-gen] fix docs ([`2548b48`](https://github.com//Byron/gitoxide/commit/2548b4813f52409bc1b214485854e5fceb78b534))
    - [pack-gen] basic progress for entry generation ([`953190d`](https://github.com//Byron/gitoxide/commit/953190d70a5df22b54dc1fffe78d41dc7d81cc61))
    - [pack-gen] the first barely working progress ([`5b89a0e`](https://github.com//Byron/gitoxide/commit/5b89a0e4203d405a50bc2e8de9d87b79e545412d))
    - [pack-gen] tests are green ([`34b6a2e`](https://github.com//Byron/gitoxide/commit/34b6a2e94949b24bf0bbaeb169b4baa0fa45c965))
    - [pack-gen] thanks clippy ([`3f948a4`](https://github.com//Byron/gitoxide/commit/3f948a44857b5ff21c85e71ab0c10538862d3d26))
    - [pack-gen] the basics to get the program going ([`03b67b0`](https://github.com//Byron/gitoxide/commit/03b67b09e4127ae4bd791501d74794d9360f7ef6))
    - [pack-gen] Use more light-weight lookups for all blobs ([`80ce34d`](https://github.com//Byron/gitoxide/commit/80ce34d82aebf9a075dde5e77be8af56d22117c7))
    - [pack-gen] refactor ([`e0caf8d`](https://github.com//Byron/gitoxide/commit/e0caf8df5f2d6009a0ef10aa160c7c0bb5682560))
    - [pack-gen] a way to get the pack location by ID right away ([`5619efb`](https://github.com//Byron/gitoxide/commit/5619efb368176809d550dc9d43d820b05a87a700))
    - [pack-gen] refactor ([`fcb9c5f`](https://github.com//Byron/gitoxide/commit/fcb9c5fad04429b7797d400c2a9661a149b5bf66))
    - [pack-gen] refactor ([`11ce2d8`](https://github.com//Byron/gitoxide/commit/11ce2d84c55ef8ffe5ac0a3cf43a48a74ff3185f))
    - [pack-gen] And the fix - all green ([`202c704`](https://github.com//Byron/gitoxide/commit/202c7046283acd65ae3ae6ab5ff0b20b1020e360))
    - [pack-gen] with the addition of write-oack checks it actually fails ([`a9e46a6`](https://github.com//Byron/gitoxide/commit/a9e46a64fc09dabf1290aeafa309bf86cfd496fe))
    - [pack-gen] it compiles and all tests are green, with tests for all parts ([`b3a0344`](https://github.com//Byron/gitoxide/commit/b3a0344db0f10a6208793087a9a9a9bcf39ab47e))
    - [pack-gen] minor but relevant differences between 'existing' and 'existing_object' ([`5f18124`](https://github.com//Byron/gitoxide/commit/5f18124694dd767e378ff6b4e77c71db642b50a2))
    - [pack-gen] very close to a basic impl of count + entries-gen… ([`c927429`](https://github.com//Byron/gitoxide/commit/c9274295e62f59cd8db06a307cc4a69d096a006e))
    - [pack-gen] Fill the relevant information for later ([`932b439`](https://github.com//Byron/gitoxide/commit/932b43998849e5d755f6fd8d19f1e942080e7bbd))
    - [pack-gen] the first test for object counts ([`67b1512`](https://github.com//Byron/gitoxide/commit/67b1512db8c3bdb2ea946d0de96190146be9ed18))
    - [pack-gen] first sketch of how counting could look like ([`6ef0072`](https://github.com//Byron/gitoxide/commit/6ef00723b134d2ce730a288a89858db2ff568c3b))
    - [pack-gen] prep for counting stage ([`93fd425`](https://github.com//Byron/gitoxide/commit/93fd4251885e6a13f0026b96c6688da0e68f9cbf))
    - [pack-gen] tag handling for diff based traversal ([`e55906c`](https://github.com//Byron/gitoxide/commit/e55906c07d9d6f2fbfa5607a2337e586f94beabe))
    - [pack-gen] tag support for tree traversal ([`28ed260`](https://github.com//Byron/gitoxide/commit/28ed260a73554d261c9b00c8ae9a46e66f123e6f))
    - (cargo-release) version 0.10.0 ([`5d7ee6a`](https://github.com//Byron/gitoxide/commit/5d7ee6a105abbb6efeed8624bade936bb59dbc55))
    - [pack-gen] the first green test for Tag iterators ([`df5ef8a`](https://github.com//Byron/gitoxide/commit/df5ef8a53cb4007058137890b414af510025fccf))
    - [pack-gen] A test to see we can handle tag objects ([`1898319`](https://github.com//Byron/gitoxide/commit/189831944d60217a3cd7383a00550d581259f638))
    - refactor ([`9f0a8cc`](https://github.com//Byron/gitoxide/commit/9f0a8cc1561589088f44a1775832110449a4f1ab))
    - [pack-gen] Finally traversal based pack gen works too ([`086422b`](https://github.com//Byron/gitoxide/commit/086422bbea50bba01060937420ab737e469e11da))
    - [pack-gen] diff-based traversal now finds all reachable objects ([`e819c92`](https://github.com//Byron/gitoxide/commit/e819c92234a1c2b182dd269d0858f003ffcc2cb0))
    - thanks clippy ([`760febf`](https://github.com//Byron/gitoxide/commit/760febf6a025891957b1afea1dd44a4ed0c4b1ca))
    - [pack-gen] add more objects during diff traversal ([`bc2ef19`](https://github.com//Byron/gitoxide/commit/bc2ef193af15a1414d987b9cc780b2cd3a93e9f4))
    - [pack-gen] pickup more trees ([`2da57bd`](https://github.com//Byron/gitoxide/commit/2da57bd02672d1d4effc940bcf81720fc63f02bc))
    - [pack-gen] Specific tests show that something is off in the changes case… ([`b131c9e`](https://github.com//Byron/gitoxide/commit/b131c9e68c7ac062cd9abbd0541afdb9c69e8649))
    - [pack-gen] central object synchronization for diff based algo as well ([`6de3558`](https://github.com//Byron/gitoxide/commit/6de3558e4becbf4d3cf0640b8eceff40b82f55d3))
    - [pack-gen] have to keep track of all seen objects ([`dc645c6`](https://github.com//Byron/gitoxide/commit/dc645c62a1b05e6b160c8355a71452467ccb6d38))
    - [pack-gen] updating tests to verify something shows that objects are duplicated ([`cef1a58`](https://github.com//Byron/gitoxide/commit/cef1a58cf6cc40fd0a53a9c46ef996f753d7d7d4))
    - [pack-gen] tree diff based pack generation passes the trivial test ([`ad0e345`](https://github.com//Byron/gitoxide/commit/ad0e345af0654ce40afce713de9286f06cf1d05c))
    - [pack-gen] refactor ([`cac002a`](https://github.com//Byron/gitoxide/commit/cac002a94427c10a9f87901a861a9d764126f8e5))
    - [git-traverse] accept tree iterators instead of object ids ([`f343dad`](https://github.com//Byron/gitoxide/commit/f343dad60d34dfd88247a14c7e3de906a761cf2d))
    - [pack-gen] Most of changes based pack gen ([`9ade04c`](https://github.com//Byron/gitoxide/commit/9ade04c47b3d4cad29a754f15f21df7e1b266325))
    - [pack-gen] prepare diff based pack-gen ([`fa2ae2c`](https://github.com//Byron/gitoxide/commit/fa2ae2c924b295a4c25f41ba9ecbcf5c45b77e85))
    - [git-diff] refactor ([`087e853`](https://github.com//Byron/gitoxide/commit/087e85367c27bb3684c6ad543c7eae46762e5e44))
    - [git-traverse] refactor ([`85de287`](https://github.com//Byron/gitoxide/commit/85de2874087f64fc166797a3219eeb26be460619))
    - (cargo-release) version 0.3.0 ([`684de4b`](https://github.com//Byron/gitoxide/commit/684de4b376ecd4cc5330f7ac8643352ea9580ed3))
    - [pack-gen] Speed up tree-traversal :D ([`90b4093`](https://github.com//Byron/gitoxide/commit/90b4093aa6076c97f751013de4c25934fef764b8))
    - thanks clippy ([`009a342`](https://github.com//Byron/gitoxide/commit/009a3425d24d4c9f476ff1c32da9b279cb170350))
    - [pack-gen] Probably a valid impl of tree traversal ([`4c72a17`](https://github.com//Byron/gitoxide/commit/4c72a171d50d08d4b35209fcef107b9a85a6c648))
    - [pack-gen] quite a bit closer to tree-traversal for pack gen ([`ecd37ee`](https://github.com//Byron/gitoxide/commit/ecd37eea0154791bf9192d1225828e7d9b5ad530))
    - [pack-gen] refactor ([`325d63e`](https://github.com//Byron/gitoxide/commit/325d63efe6855c8d14d564d8b3cbce9a9e144d14))
    - [pack-gen] a test for upcoming traversal modes ([`8d1e9ac`](https://github.com//Byron/gitoxide/commit/8d1e9ace79bbbe41ea4fac70c13522d7d6091a81))
    - [pack-gen] refactor ([`08b136f`](https://github.com//Byron/gitoxide/commit/08b136f0cf35f8b275feee9830bfab4555d40a99))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

## v0.15.0 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 6 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.15.0 ([`d91b241`](https://github.com//Byron/gitoxide/commit/d91b2412381e3c8c1f24c38469e821c3c3960e34))
    - (cargo-release) version 0.9.0 ([`84897fd`](https://github.com//Byron/gitoxide/commit/84897fd8e6e1b0269da0303d6a0de8f9e0eb58e5))
    - Merge branch 'patch-2' ([`f01dc54`](https://github.com//Byron/gitoxide/commit/f01dc54010683b232c5f5813bd5370e93f1681f5))
    - refactor ([`a9e4feb`](https://github.com//Byron/gitoxide/commit/a9e4feb0a81894568be730603446e2d061dd558f))
    - Fix formatting ([`a341995`](https://github.com//Byron/gitoxide/commit/a341995e6014b6ed0e43ae94fa1152aed6fcfd89))
    - Use Seek to skip large objects during indexing. ([`95e2af7`](https://github.com//Byron/gitoxide/commit/95e2af74574af998294265b6a3de833dbe4dcedb))
    - Remove almost all unsafe code from Tree. ([`42b6033`](https://github.com//Byron/gitoxide/commit/42b6033f3c367ccce37c82356183d165d37ae881))
    - thanks clippy ([`17258cc`](https://github.com//Byron/gitoxide/commit/17258cc58767caa6e71227898decd160ad0cdf13))
    - thanks clippy ([`09decde`](https://github.com//Byron/gitoxide/commit/09decde782e0b9e794a740d4fa654af73398d80a))
    - Convenience methods for iterators ([`aa6c9e6`](https://github.com//Byron/gitoxide/commit/aa6c9e699a09b6b2b4f55aa75a1dd6f648eead09))
    - refactor ([`d9783b9`](https://github.com//Byron/gitoxide/commit/d9783b94b0149c584690a1a50f029c9424de08c3))
    - A sketch of convenient finding of commits ([`db21062`](https://github.com//Byron/gitoxide/commit/db210622b95d5f1f24d815cb35db5d46aa8a09e3))
    - refactor ([`3af7b7b`](https://github.com//Byron/gitoxide/commit/3af7b7b2bc0082298faa7ff2bd4413e80bee1107))
    - sketch of convenience function for `Find` trait. ([`2bf4958`](https://github.com//Byron/gitoxide/commit/2bf4958dd7d1ad0a2f9f8c5754be88c3efb524a4))
    - refactor ([`bd4d21e`](https://github.com//Byron/gitoxide/commit/bd4d21e7003801319a62887e3d39467b2ee1ad0d))
    - refactor ([`8b10434`](https://github.com//Byron/gitoxide/commit/8b1043483cb46fd1b7f47a90c9dce24a65d58d1b))
    - Fix order of operations in git-odb::hash::Write ([`a31d8c7`](https://github.com//Byron/gitoxide/commit/a31d8c75e7821b68b49f017010646a8232ecc6cc))
    - (cargo-release) version 0.14.0 ([`a760f8c`](https://github.com//Byron/gitoxide/commit/a760f8c013e13ba82daa1acf1a4a57e0818a008d))
</details>

## v0.14.0 (2021-05-02)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.14.0 ([`d9514ee`](https://github.com//Byron/gitoxide/commit/d9514eec64579ef77c9f2ac5dfe87cd302180eb9))
    - rename 'Locate' to 'Find' - shorter and just as good ([`60f72f5`](https://github.com//Byron/gitoxide/commit/60f72f573a7696323e09bf4add80d5fbce22c99d))
    - (cargo-release) version 0.13.0 ([`5c791af`](https://github.com//Byron/gitoxide/commit/5c791af217fac6a171d174ad9f4ee5f4d5282892))
    - [traversal] remove git-odb::traversal (now git-traverse::iter) ([`35b74d2`](https://github.com//Byron/gitoxide/commit/35b74d2f046426d99bb5431f8aea42ac453ac101))
</details>

## v0.12.0 (2021-04-30)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 153 commits contributed to the release over the course of 18 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#67](https://github.com//Byron/gitoxide/issues/67)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#67](https://github.com//Byron/gitoxide/issues/67)**
    - The very first version of complete pack writing ([`4d76d53`](https://github.com//Byron/gitoxide/commit/4d76d53aabb956ed7c8a45c883486ec5596bcaa3))
    - A sketch of the pack::generation function signature ([`21b0aab`](https://github.com//Byron/gitoxide/commit/21b0aab81e7304da964dbef90c806134073ccef3))
 * **Uncategorized**
    - prepare test utilities for release… ([`d35e654`](https://github.com//Byron/gitoxide/commit/d35e654747f96cec93bdecd1314ce325129cbc44))
    - (cargo-release) version 0.8.0 ([`a1ce210`](https://github.com//Byron/gitoxide/commit/a1ce210003ff07bf11291018bb182cbc7913647b))
    - (cargo-release) version 0.3.0 ([`e9665c7`](https://github.com//Byron/gitoxide/commit/e9665c784ae7e5cdaf662151395ee2355e9b57b6))
    - [traversal] all the caching ([`0890403`](https://github.com//Byron/gitoxide/commit/0890403cce658ea90c593a6ca21e24f02ddf5a93))
    - [tree-diff] first prototype of traversal experiment ([`ece43b4`](https://github.com//Byron/gitoxide/commit/ece43b4b0bf054d798685461d2b96daaafd8a408))
    - thanks clippy ([`2d5e205`](https://github.com//Byron/gitoxide/commit/2d5e20520499d1a87808db508548b408e3777d0e))
    - [tree-diff] more tests for the tree iterator ([`91b5a02`](https://github.com//Byron/gitoxide/commit/91b5a029337200a2873a21696020dcda08e335cb))
    - [tree-diff] Now the commit graph traversal works with zero-allocations ([`8078910`](https://github.com//Byron/gitoxide/commit/8078910b27149df10b6b236b9311ebee31523710))
    - make it easy to get a commit iterator ([`33213f3`](https://github.com//Byron/gitoxide/commit/33213f30abbb6619d41663a4baf3078af3284085))
    - [tree-diff] refactor into iterator based model ([`29b527a`](https://github.com//Byron/gitoxide/commit/29b527aaea101c9b4e885db1c6d3533ef2310c54))
    - [tree-diff] The least intrusive way to allow dealing with tree iterators ([`d41dd3c`](https://github.com//Byron/gitoxide/commit/d41dd3c38ee34b250a4f5de120d7ae3e04e3212d))
    - refactor ([`a4d5f99`](https://github.com//Byron/gitoxide/commit/a4d5f99c8dc99bf814790928a3bf9649cd99486b))
    - refactor ([`03ee510`](https://github.com//Byron/gitoxide/commit/03ee510a5f9c24b6acddaec1d30ea3ad39174603))
    - Better ergonomics for accessing decoded objects ([`ae3eab6`](https://github.com//Byron/gitoxide/commit/ae3eab6d6e4b96e207372fa8cb82f5ac9833e5e4))
    - refactor ([`c1013dd`](https://github.com//Byron/gitoxide/commit/c1013dddbc221b366b91d186cfd1732f1d72be10))
    - refactor ([`f37c31f`](https://github.com//Byron/gitoxide/commit/f37c31f04bf8cf531284abe380db77d6196bd711))
    - refactor ([`ac70651`](https://github.com//Byron/gitoxide/commit/ac706518fff2e92ade3589dea4a6c81fca57aec2))
    - refactor ([`77764f3`](https://github.com//Byron/gitoxide/commit/77764f3b9c3e8202119bb9327e150089c3ecbb9b))
    - refactor ([`3185cc9`](https://github.com//Byron/gitoxide/commit/3185cc9de1f7d3e52d088b60fcaae0ac91a72fe1))
    - Thanks, cargo audit ([`4f293f5`](https://github.com//Byron/gitoxide/commit/4f293f5036c44a69ccacf102d35202adad83bbe0))
    - refactor ([`edf7d38`](https://github.com//Byron/gitoxide/commit/edf7d382148aa139485c8279c2a50dc6c86d481d))
    - refactor ([`ca98221`](https://github.com//Byron/gitoxide/commit/ca98221d5a512dabf683cc1da56d40a17285f2fb))
    - refactor ([`b4027e3`](https://github.com//Byron/gitoxide/commit/b4027e3df187931a263798b255c80b272910aef7))
    - refacto ([`6e328da`](https://github.com//Byron/gitoxide/commit/6e328da9f8a73ac8e699aea55b1250a433f5ecd9))
    - fix docs ([`a54bab4`](https://github.com//Byron/gitoxide/commit/a54bab40a5881873eb2b1c591fa9f05d8034ac6d))
    - refactor ([`3f2ee4c`](https://github.com//Byron/gitoxide/commit/3f2ee4cda6db14902639f7fc3a9fbee05508086f))
    - refactor ([`d6ab581`](https://github.com//Byron/gitoxide/commit/d6ab581db66c1d09578ed2af9db34929995e2cb9))
    - refactor ([`d490b65`](https://github.com//Byron/gitoxide/commit/d490b65ebbc6666cd59d88f8677dc1c52bfe1e1c))
    - Pack V2 writing (base objects only) seems to work now #(67) ([`e68dd84`](https://github.com//Byron/gitoxide/commit/e68dd84df7d13efcc7882644d3d9347b3722785a))
    - The first more thorough and indirect validation of the newly written pack… ([`d43687e`](https://github.com//Byron/gitoxide/commit/d43687ed9093224e0caba4063063705b9473afd0))
    - refactor ([`08fafaa`](https://github.com//Byron/gitoxide/commit/08fafaa03144fc3ddea9120a4a1943e18c454ae8))
    - test newly written pack data alone ([`01fdd70`](https://github.com//Byron/gitoxide/commit/01fdd70395a662612309ece730c2a75303e2155e))
    - Write pack data entries #(67) ([`722202e`](https://github.com//Byron/gitoxide/commit/722202edce0d5700a9df9eff6208ad5d7c6554fb))
    - refactor ([`eed1e3c`](https://github.com//Byron/gitoxide/commit/eed1e3c21658ee152d224622599cd5a4c65df126))
    - Write pack data header #(67) ([`717726b`](https://github.com//Byron/gitoxide/commit/717726b30e80f0ca56927f31e823ec48470fbeb2))
    - refactor ([`28cbeb3`](https://github.com//Byron/gitoxide/commit/28cbeb3ca1d6c5f6aa7664255d1d44fdb49f116b))
    - refactor ([`4261c7d`](https://github.com//Byron/gitoxide/commit/4261c7dea7666cfc3a867bca2bbdb0827487be00))
    - All logic needed to write a valid pack within an iterator ([`775ab29`](https://github.com//Byron/gitoxide/commit/775ab295531875ec93e57d30422b88e03e48313e))
    - sketch of pack data write API ([`dfeda87`](https://github.com//Byron/gitoxide/commit/dfeda87de13c6f05a39732d3f0518bb76695be9a))
    - refactor ([`f33fa10`](https://github.com//Byron/gitoxide/commit/f33fa10224d46539c94a2042014c14042d7dc968))
    - [experiment/object-access] allow bare repositories ([`401690d`](https://github.com//Byron/gitoxide/commit/401690dbc6c10b2e7144bf3362c4b2e71435e801))
    - thanks clippy ([`c86823a`](https://github.com//Byron/gitoxide/commit/c86823a5cce91a12738c25313ae15eec7751af46))
    - refactor zlib ([`4587b82`](https://github.com//Byron/gitoxide/commit/4587b8244c5ba85aa899e998214119aadb948862))
    - refactor zlib ([`496e6bb`](https://github.com//Byron/gitoxide/commit/496e6bb86ba1bcf66ffaf250b026c12bd3e830c5))
    - refactor ([`3a4469c`](https://github.com//Byron/gitoxide/commit/3a4469c20c44dd649a442c7f6c2902325c744394))
    - First basic pack entry generation (base only) works… ([`75cb32b`](https://github.com//Byron/gitoxide/commit/75cb32baed75c23b47d6422569be630c6fd412f7))
    - refactor ([`d4bf8ae`](https://github.com//Byron/gitoxide/commit/d4bf8aea9b8f811b57b943be16ea4bb2eabccca4))
    - refactor ([`2d89222`](https://github.com//Byron/gitoxide/commit/2d89222b1d48cf63544ceb4bf8d3067a49adb792))
    - refactor ([`eb3a8da`](https://github.com//Byron/gitoxide/commit/eb3a8da7a246355f1ef0de20226abaaf38b01126))
    - Allow calling 'finalize()' on the entries iterator ([`3c617bc`](https://github.com//Byron/gitoxide/commit/3c617bc2ae59adbb12c254308269e745149d462b))
    - refactor ([`b7d0323`](https://github.com//Byron/gitoxide/commit/b7d03235b1eb42e98cfc7620dea9d41b0e87d208))
    - Reduce size of data::Object ([`7aa783a`](https://github.com//Byron/gitoxide/commit/7aa783ab4e06d3e33f918c0cd084dd8d89f3d768))
    - First pack-to-pack copy and crc32 verification ([`37619f0`](https://github.com//Byron/gitoxide/commit/37619f0ea71216ef7a0b9e512e3987fead08c9b9))
    - It's possible to get entry data within pack generation ([`a2a5927`](https://github.com//Byron/gitoxide/commit/a2a59272116029e9328fb2798e5c72e9fc9b3b32))
    - git-odb without cargo warnings due to using the same test twice ([`8945f95`](https://github.com//Byron/gitoxide/commit/8945f95364b489e7a639d74dd0f28b17e82e70f3))
    - A way to obtain entry information using previous lookup information ([`a55d113`](https://github.com//Byron/gitoxide/commit/a55d113ded8d6aeee78f9041f13167dc54243254))
    - refactor ([`95ab11b`](https://github.com//Byron/gitoxide/commit/95ab11bd3014c81ab35437ba9d1c6b84caf6ba76))
    - A probably more secure way of accessing pack data ([`7a01bd8`](https://github.com//Byron/gitoxide/commit/7a01bd8b120fa34566b0f97ca9b35e1d8a97aefa))
    - sketch of pack-entry retrieval api ([`d1e9248`](https://github.com//Byron/gitoxide/commit/d1e92486c9b716c11cf75eccd9829738d3b94ca0))
    - refactor ([`08cf90a`](https://github.com//Byron/gitoxide/commit/08cf90a2e33e3a0cdbb249ffcc46ef3a46685145))
    - Let's be a bit more conservative with this information for now ([`efef417`](https://github.com//Byron/gitoxide/commit/efef417e52caf12a2090b6d4a96e0633e77471dd))
    - Objects know their pack location ([`73f1c66`](https://github.com//Byron/gitoxide/commit/73f1c668b629055d8b0bffc1a6cc31c54037a6da))
    - Chunks based iteration for pack generation ([`23c2694`](https://github.com//Byron/gitoxide/commit/23c2694c86eb397f5063f248c95cd164bae2120a))
    - Some notes about how to treat defaults of file versions ([`cfa5cf6`](https://github.com//Byron/gitoxide/commit/cfa5cf6146d4de028a31b5eb8ad756477e37111b))
    - run git-odb tests in parallel, too; improved threaded error handling ([`40802fd`](https://github.com//Byron/gitoxide/commit/40802fd8bbb15b8a61249522d67f3a5b28da64b3))
    - the first test for pack generation ([`2a2fdde`](https://github.com//Byron/gitoxide/commit/2a2fdde2e5365e83faf99999ea1c640159f5c4b9))
    - refactor ([`385f52d`](https://github.com//Byron/gitoxide/commit/385f52d4ea99230839bb447e2993bad741ce0cae))
    - refactor  Please enter the commit message for your changes. Lines starting ([`f65c68c`](https://github.com//Byron/gitoxide/commit/f65c68c3c7c4c838ea77494ecc0ce17f6d5d719b))
    - fix doc links ([`ec35743`](https://github.com//Byron/gitoxide/commit/ec35743cc4062f2b6dbfc85b7f5aadfa68f598a7))
    - thanks clippy ([`563e445`](https://github.com//Byron/gitoxide/commit/563e4452aae5c6dc1323e0f6759315e73f3a2c89))
    - The first seemingly working iteration over all objects in an odb #(67) ([`6b34f62`](https://github.com//Byron/gitoxide/commit/6b34f62cc4e6f9ee6030590d8b3f185dda3bc568))
    - refactor ([`01d9d91`](https://github.com//Byron/gitoxide/commit/01d9d91d1bce6217b8a48ab1f0a7ba4e17508279))
    - impl size_hint for linked db iter ([`ada259b`](https://github.com//Byron/gitoxide/commit/ada259b4fe441728682521e9138ed9f6ef1c13f4))
    - refactor ([`82c2f42`](https://github.com//Byron/gitoxide/commit/82c2f428e22c3cda79913c9ca2f092c377d692aa))
    - refactor ([`7a6b514`](https://github.com//Byron/gitoxide/commit/7a6b514a5b9b93bf574cd3a114f27ad5967e89ac))
    - First sketch of object iterator for linked::Db ([`a316eed`](https://github.com//Byron/gitoxide/commit/a316eed4bf4634d4776d153528cb28254b847bdd))
    - Set environment in testtools to freeze repositories generation scripts ([`eaad3ab`](https://github.com//Byron/gitoxide/commit/eaad3ab69338115439a553ba1062160dc3a08082))
    - faster repeated tests if fixtures don't change ([`792277f`](https://github.com//Byron/gitoxide/commit/792277f241446086dd6c9b78f688363d4e66e5a7))
    - refactor ([`e1a92ad`](https://github.com//Byron/gitoxide/commit/e1a92adbedcb017a9e35049389ef86fca34fa44c))
    - Allow the use of shared test utilities across crates ([`b117626`](https://github.com//Byron/gitoxide/commit/b117626df6da714c24d2b7914301678e89d2d0cb))
    - refactor ([`40b86a7`](https://github.com//Byron/gitoxide/commit/40b86a78367fbd7cd9c8e5447c9b97fa685cc43e))
    - refactor ([`8b00094`](https://github.com//Byron/gitoxide/commit/8b0009466b820b934a2244a98360007336180246))
    - fix doc links ([`7479071`](https://github.com//Byron/gitoxide/commit/747907145e001a093c8dc84e80d879f4d18c84d5))
    - thanks clippy ([`6f901f5`](https://github.com//Byron/gitoxide/commit/6f901f5daa868c1a0e9cea113abe13beb65d8f35))
    - ancestor iterator is now generic over Locate trait ([`bbfd616`](https://github.com//Byron/gitoxide/commit/bbfd616ae023aae9d92ebd9d873a9be02423e820))
    - [fail] try to abstract ancestor::Iter over Locate trait ([`f8c0375`](https://github.com//Byron/gitoxide/commit/f8c0375bbafffc81938998a8ff8aa2faac9253e1))
    - refactor ([`5ef1f22`](https://github.com//Byron/gitoxide/commit/5ef1f22c1e12ff8d607663d4dfbbbfe426a29e0f))
    - Improve interface for building packs based on Locate trait #(67) ([`5b66b6e`](https://github.com//Byron/gitoxide/commit/5b66b6e729c858068a31e4817db63a5f6ba5032b))
    - A version of the Locate trait we can do today #(67) ([`d752be2`](https://github.com//Byron/gitoxide/commit/d752be2931e3403c16fea8d804c8759c56bb1fd4))
    - [git-odb] Associated types with lifetimes also don't seem to work ([`0e68a9d`](https://github.com//Byron/gitoxide/commit/0e68a9d095eb038da0e9139fe1d649d593d72010))
    - [git-odb] Trying to use offical traits won't work with our kind of object ([`29a5054`](https://github.com//Byron/gitoxide/commit/29a5054740fd0c7958376c603fd6214421f7772f))
    - git-odb::borrowed::Object => git-odb::data::Object ([`747a13e`](https://github.com//Byron/gitoxide/commit/747a13e9a1fe5200c53055dd961507c9fef667e1))
    - An even better name for decode errors ([`f270850`](https://github.com//Byron/gitoxide/commit/f270850ff92eab15258023b8e59346ec200303bd))
    - Make clear it's a decode error we are using there ([`f45cb4b`](https://github.com//Byron/gitoxide/commit/f45cb4b62122559e5701923e0a23dd5791ee2ced))
    - rename git-object::(owned->mutable)|(borrowed|immutable) #(67) ([`91ee558`](https://github.com//Byron/gitoxide/commit/91ee55893bf4b27a47d86d51bae6f99b59b69147))
    - bump git-odb minor version ([`5c833ce`](https://github.com//Byron/gitoxide/commit/5c833ce64babd00b7ced3e3a1c9ed3dbd260c9f4))
    - thanks clippy ([`547af6e`](https://github.com//Byron/gitoxide/commit/547af6e3965112c8eea69cae173a6996249b77c5))
    - Fix test breakage for loose object reading ([`222c7a2`](https://github.com//Byron/gitoxide/commit/222c7a276efdc65da4f9490f53b82e58f8e878c1))
    - fix docs #(67) ([`01db10a`](https://github.com//Byron/gitoxide/commit/01db10a27431ad89a68ed3e4eabae810748a6f29))
    - thanks clippy ([`60a7689`](https://github.com//Byron/gitoxide/commit/60a7689d7493d29103775ce358999314af9257c8))
    - refactor ([`ef674ff`](https://github.com//Byron/gitoxide/commit/ef674ffde5af3c19a9538d99112f414144b921cd))
    - Remove loose::Object entirely #(67) ([`5cf4840`](https://github.com//Byron/gitoxide/commit/5cf4840b10a3fac43266bc9defa72977a004bf8c))
    - Start using loose::Db::locate2 - definitely still bugs in there ([`d6f07b7`](https://github.com//Byron/gitoxide/commit/d6f07b7709fb2291484859477b54371ef3108a57))
    - An alternative version of loose::Db::locate() for use with borrowed::Object ([`5b40a32`](https://github.com//Byron/gitoxide/commit/5b40a32c017f264d80b8babb293470a4a47a45b4))
    - refactor ([`bad3ce4`](https://github.com//Byron/gitoxide/commit/bad3ce417dd7b4cdbcf45c95fbdc44b245b0762f))
    - replace loose::Object::stream() with *::data() #(67) ([`040b347`](https://github.com//Byron/gitoxide/commit/040b347d1b020ef17a8862c4cb792e267d674c8a))
    - sketch loose::Object::data() to start refactoring #(67) ([`ee1701f`](https://github.com//Byron/gitoxide/commit/ee1701f681af4a6acfd6809fe439a3fa1912f259))
    - Sketch of trait for locating objects #(67) ([`31445d7`](https://github.com//Byron/gitoxide/commit/31445d778864c430d363bea86c51286f5f9f69a1))
    - refactor ([`2754dd6`](https://github.com//Byron/gitoxide/commit/2754dd6078608a600ec20a5d1c9307c2d746e6c5))
    - refactor ([`3e908bd`](https://github.com//Byron/gitoxide/commit/3e908bd4b4077c4a5d113cefc113f9d71f249133))
    - refactor ([`409d763`](https://github.com//Byron/gitoxide/commit/409d763d2fca974a647487c72d15f568a9b62ccb))
    - refactor ([`896ab94`](https://github.com//Byron/gitoxide/commit/896ab940bcd475d026e4009b3aa2fa6a025c14bc))
    - Remove unsafe interface for stepped computation #(67) ([`c856613`](https://github.com//Byron/gitoxide/commit/c856613a35aea7dea1d093bfcfe1ddbde93fdf26))
    - Show that with custom iterators, Arc's are natively supported #(67) ([`0c49007`](https://github.com//Byron/gitoxide/commit/0c490073c53cf1f2df9fe2cd7612a1531e1430a7))
    - thanks clippy ([`405dd9d`](https://github.com//Byron/gitoxide/commit/405dd9d4cb7980a4925b19562e02a9fb7f0f5ab6))
    - multi-tip support #(67) ([`2254ecc`](https://github.com//Byron/gitoxide/commit/2254ecc4b1927867f02fe03db4a027d8c1e47ee9))
    - cache support for traversal #(67) ([`1e9300a`](https://github.com//Byron/gitoxide/commit/1e9300ac53b1d3e96352ce466f2c7d27a93ade2a))
    - cycle and duplicate check #(67) ([`334a72d`](https://github.com//Byron/gitoxide/commit/334a72d4a2ec2718d92b9c0843c4f6722a909f5e))
    - a new failing test ([`86b6c24`](https://github.com//Byron/gitoxide/commit/86b6c2497cfa17bf3f822792e3afe406f7968ee7))
    - refactor ([`5408b62`](https://github.com//Byron/gitoxide/commit/5408b6258c5c5aea26c91e4ec7e7d56e8a3cc8f8))
    - The first basic traversal utility #(67) ([`ea6610b`](https://github.com//Byron/gitoxide/commit/ea6610b9157d8d0dabd2ddd07c45dc6651b9cf84))
    - Fix iteration signature due to shadowed naming ([`fe8b459`](https://github.com//Byron/gitoxide/commit/fe8b459fc406d5fee39d7dd333ff0afad78a0c38))
    - thanks clippy ([`a463a43`](https://github.com//Byron/gitoxide/commit/a463a438c69a96ac0f291298113c7814b6d51ec4))
    - Sketch of machinery for producing pack entries #(67) ([`ac8e7fb`](https://github.com//Byron/gitoxide/commit/ac8e7fb6c8ae4ac42f56482d9d7744aa66132702))
    - A step towards using SteppedReduce #(67) ([`0d5595a`](https://github.com//Byron/gitoxide/commit/0d5595a2269314d9aa2a76b2b5d650506a51f58e))
    - (cargo-release) version 0.13.0 ([`ac2eddb`](https://github.com//Byron/gitoxide/commit/ac2eddb06eb3d8a9a3dcdcd796eb54a7e45ab935))
    - Allow parallel reducers to produce something during 'feed()' #(67) ([`6c04fcd`](https://github.com//Byron/gitoxide/commit/6c04fcd643083d9db633edd3bb838b4f5de8f0db))
    - Allow more fine-grained stepping over the pack generator #(67) ([`22eb892`](https://github.com//Byron/gitoxide/commit/22eb892e7e66f6a5e3e35094a657a8469a163e26))
    - Allow to obtain object information without fetching the data #(67) ([`6553850`](https://github.com//Byron/gitoxide/commit/6553850aacbf815989af297bc95fe15d915f62ec))
    - sketch a version to abstract object data retrieval #(67) ([`ad90446`](https://github.com//Byron/gitoxide/commit/ad90446da913f1f0b9833a393c5f33ae2638ae30))
    - Implement `Write` trait for linked::Db ([`21362c3`](https://github.com//Byron/gitoxide/commit/21362c388026837ad78891945cfeac8cea27e0db))
    - Docs for `linked::Db` ([`9d936de`](https://github.com//Byron/gitoxide/commit/9d936dea06b8b28a46e8a0a466ea9f4d618595b1))
    - Support for caches within linked::Db ([`3635a3e`](https://github.com//Byron/gitoxide/commit/3635a3e8629143c6b96ed80eb7d7a10f011afceb))
    - `locate()` for `linked::Db` without cache for now ([`014bc3c`](https://github.com//Byron/gitoxide/commit/014bc3c74a8b566608091f0decfe79439ab5d6f9))
    - refactor ([`7b443d1`](https://github.com//Byron/gitoxide/commit/7b443d1563e8231ca8bf88752eac28f441801d52))
    - refactor ([`d077ead`](https://github.com//Byron/gitoxide/commit/d077ead603ce87f891e83e83cbcffeb4c79dd1f0))
    - linked::Db::init() with a few tests ([`4c77e4c`](https://github.com//Byron/gitoxide/commit/4c77e4c97641ab3b02b56aaa702a7d2ca5bced7c))
    - Frame for linked::Db ([`e64d148`](https://github.com//Byron/gitoxide/commit/e64d148a0984fb6dba3f788f8cc99c37914fd930))
    - Make cycles in alternate object chains fatal ([`67e679a`](https://github.com//Byron/gitoxide/commit/67e679a6d7b56c2754f422e5cce3f8cf0784e506))
    - Resolve alternates as paths, not as repositories ([`73352c3`](https://github.com//Byron/gitoxide/commit/73352c346d4a408eb657f1862996525982c16db6))
    - Remove support for Polonius in preparation for a new repo type ([`871c803`](https://github.com//Byron/gitoxide/commit/871c803d9c5be6e786338b549c243ad50d057df5))
    - (cargo-release) version 0.11.0 ([`fd698e3`](https://github.com//Byron/gitoxide/commit/fd698e334e44d5c478c162f98d09afd9ce7a6895))
    - Introduce pack_id for use in pack cache, preventing (most collisions) ([`ad04ad3`](https://github.com//Byron/gitoxide/commit/ad04ad3b8ac54e78bee307dd44c85c1389edced2))
    - Fix benchmark to get valid test results ([`20abb3a`](https://github.com//Byron/gitoxide/commit/20abb3a4fc9769f23b9a86d2e8d49f53290b36f4))
    - First use of memory-cap based LRU cache for object access ([`b057494`](https://github.com//Byron/gitoxide/commit/b0574945039881c6d5d8be4107c1c987ed3bbaf6))
    - Add hash-map based LRU to allow bigger/more effective object caches ([`5affdd5`](https://github.com//Byron/gitoxide/commit/5affdd5df0c4d01f3130fc1be259c72f601a1f71))
    - Feature toggle for uluru based Lru cache ([`98eec48`](https://github.com//Byron/gitoxide/commit/98eec4837d605a408b026a859e53a7e2eae8e4da))
    - refactor ([`d624d09`](https://github.com//Byron/gitoxide/commit/d624d097784eed216f8d0e94544d8b62ef6c3010))
    - Improve docs to prevent people to 'misuse' the Lru cache. ([`fff62ed`](https://github.com//Byron/gitoxide/commit/fff62ed708153e1c9313930bf36877faad5cd777))
    - LruCache with const-generics ([`93618d1`](https://github.com//Byron/gitoxide/commit/93618d107e9defadb603209251f77948caddc121))
    - [experiment] cached version of compound::locate() ([`ec988dc`](https://github.com//Byron/gitoxide/commit/ec988dc21320b211f3da9327b63101f954db307e))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 8 times to make code idiomatic. 

## v0.10.0 (2021-04-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 23 commits contributed to the release over the course of 4 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#63](https://github.com//Byron/gitoxide/issues/63)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#63](https://github.com//Byron/gitoxide/issues/63)**
    - Impl == and != for common combinations of ObjectId/oid ([`2455178`](https://github.com//Byron/gitoxide/commit/24551781cee4fcf312567ca9270d54a95bc4d7ae))
    - git-protocol uses `oid` type ([`3930a6f`](https://github.com//Byron/gitoxide/commit/3930a6ff508f5bb2249fb2c2f21e00b74fecda22))
    - Use new `oid` where possible in git-odb ([`68a709e`](https://github.com//Byron/gitoxide/commit/68a709e0337d4969138d30a5c25d60b7dbe51a73))
    - refactor; better errors for invalid hash sizes ([`be84b36`](https://github.com//Byron/gitoxide/commit/be84b36129694a2e89d1b81d932f2eba23aedf54))
    - Make ObjectId/oid happen! ([`ca78d15`](https://github.com//Byron/gitoxide/commit/ca78d15373ec988d909be8f240baefe75555e077))
    - Remove all public exports of git-hash types in git-object ([`accf89d`](https://github.com//Byron/gitoxide/commit/accf89d25560e5ded6f44a1c4a898ee65d14f8f6))
    - Remove re-export of git_object::borrowed::Id ([`a3f2816`](https://github.com//Byron/gitoxide/commit/a3f28169c1268c1129852f279631d5a7f7540cdf))
 * **Uncategorized**
    - (cargo-release) version 0.10.0 ([`3161777`](https://github.com//Byron/gitoxide/commit/316177729e42f8d000a40ab01b9b97621e7179e8))
    - (cargo-release) version 0.7.0 ([`b900914`](https://github.com//Byron/gitoxide/commit/b900914a00292217ba7b9bcac260591800395287))
    - (cargo-release) version 0.12.0 ([`3b71e7e`](https://github.com//Byron/gitoxide/commit/3b71e7e8416e550b47e5aed2259c1181497ac9e8))
    - (cargo-release) version 0.2.0 ([`4ec09f4`](https://github.com//Byron/gitoxide/commit/4ec09f4d2239ea1d44f7145027e64191bf2c158c))
    - Greatly reduce compound::Object size ([`afa8156`](https://github.com//Byron/gitoxide/commit/afa8156c37c6ea93bad7553e5a373fc333398d9b))
    - The git-odb compound Object clearly is too large ([`8f0e813`](https://github.com//Byron/gitoxide/commit/8f0e8138ed3313b79b4e358854b9fda5e981f652))
    - git-odb: add link to simplified/polonius version in the docs ([`d53c4b0`](https://github.com//Byron/gitoxide/commit/d53c4b0f91f1b29769c9430f2d1c0bcab1170c75))
    - git-odb: Only check alternates for objects not found in packs or loose ([`b317200`](https://github.com//Byron/gitoxide/commit/b317200b72096573d511d229c6e61e74e7ba14db))
    - git-odb: Avoid double-lookup in packs without polonius ([`eaae9c1`](https://github.com//Byron/gitoxide/commit/eaae9c1bc723209d793eb93f5587fa2604d5cd92))
    - thanks clippy ([`0c5f404`](https://github.com//Byron/gitoxide/commit/0c5f4043da4615820cb180804a81c2d4fe75fe5e))
    - Remove locate(…) -> Option<Result<…>> in favor of Result<Option<…>> ([`40ee743`](https://github.com//Byron/gitoxide/commit/40ee7438a98c4094c0fd04977cd4904668087512))
    - A locate returning Result<Option<Object>> for compound DB ([`a1dafa6`](https://github.com//Byron/gitoxide/commit/a1dafa64b4e26dd1456d38f94d58eaadf19abfd3))
    - Use Result<Option<Object>> in Bundle::locate() ([`2dfef8f`](https://github.com//Byron/gitoxide/commit/2dfef8f71da456c5c494e1530040589582a046b1))
    - A trial for Result<Option<Object>>  for loose object databases ([`3842859`](https://github.com//Byron/gitoxide/commit/3842859c5bddb8b4583443685c26dcae3f8db558))
    - Assure loose objects are actually not found when opening ([`7a4f2bf`](https://github.com//Byron/gitoxide/commit/7a4f2bf2cb31407422be2e563b3df210bbf8bfd0))
    - Add feature toggle for polonius and make it part of the test suite ([`c825c11`](https://github.com//Byron/gitoxide/commit/c825c11e2d17141b38654d30b37e043dfae383f3))
</details>

## v0.9.1 (2021-04-03)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#59](https://github.com//Byron/gitoxide/issues/59)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#59](https://github.com//Byron/gitoxide/issues/59)**
    - Fix initializing pack bundles in compound db ([`5a48e08`](https://github.com//Byron/gitoxide/commit/5a48e085d49a191a85a9b043e34d844389c8342b))
    - Add failing test ([`d629339`](https://github.com//Byron/gitoxide/commit/d629339834479553ceef27c15e5115e820b875ee))
    - Move pack fixtures into place which resembles an actual object db ([`fb5cea4`](https://github.com//Byron/gitoxide/commit/fb5cea4b9a98997f105a6ccb9729371be994af3c))
 * **Uncategorized**
    - (cargo-release) version 0.9.1 ([`e0feb28`](https://github.com//Byron/gitoxide/commit/e0feb28b50ce55be71b24ea5238a760f5b1f8d3b))
</details>

## v0.9.0 (2021-03-29)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 14 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.9.0 ([`efc8983`](https://github.com//Byron/gitoxide/commit/efc898381d830e44487c62e35a665d3ccd0a2d39))
    - thanks clippy ([`0fc239c`](https://github.com//Byron/gitoxide/commit/0fc239cf9b773f72928b7c42344b578c6ff5d19f))
    - refactor ([`f2e9add`](https://github.com//Byron/gitoxide/commit/f2e9add3cb5803426a2e36a3b462f823e8cef44b))
    - upgrade depdendencies ([`e4a7711`](https://github.com//Byron/gitoxide/commit/e4a77112ee4f5d0ab61d0678aab8ee090335740c))
    - Fix compile warnings produced by +nightly build ([`e387d2c`](https://github.com//Byron/gitoxide/commit/e387d2c148e231321f88e5fb1b2988437475d2c0))
    - Conform imports ([`fd73731`](https://github.com//Byron/gitoxide/commit/fd737317379af80f8e0ba9a9a8fc505fb60fd177))
    - Fix error type argument order and spell fields out ([`819568e`](https://github.com//Byron/gitoxide/commit/819568e9c5be14cec1e9e1cdc915b4c286c2ed00))
    - [git-odb] Return error on invalid packs ([`88de64d`](https://github.com//Byron/gitoxide/commit/88de64d433b44996d5f8be733b50e1949c71e42d))
    - [git-odb] Fix Inflate::once ([`36f6bbd`](https://github.com//Byron/gitoxide/commit/36f6bbd451a5474cb6dac0259904e4aed7fd11d9))
    - [git-odb] Remove unnecessary tests ([`ebe41ca`](https://github.com//Byron/gitoxide/commit/ebe41cadc4acb38326e59d193fd3b1e501146943))
    - [gix] Use flate2 by default ([`f1158a1`](https://github.com//Byron/gitoxide/commit/f1158a1a4bc8e13913461db4d4851e32d57816ff))
    - [gix] Add optional zlib feature ([`f1f9665`](https://github.com//Byron/gitoxide/commit/f1f96658a6cd6165ba9c9d7acb809fcaf2c46f9c))
    - [git-odb] Add feature flag for zlib-ng ([`96b3810`](https://github.com//Byron/gitoxide/commit/96b3810995f9e7b0164234dcb9f3b28b0c9b5224))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.8.0 (2021-01-24)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.8.0 ([`1ccfdcd`](https://github.com//Byron/gitoxide/commit/1ccfdcdb96b59c6415e7fbc800371d594b2ef7a1))
</details>

## v0.7.1 (2021-01-24)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 38 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.7.1 ([`2c38ff9`](https://github.com//Byron/gitoxide/commit/2c38ff909cd5ed39995d4ac3b5af3e0da2f3b76d))
    - (cargo-release) version 0.11.0 ([`1aa1f5e`](https://github.com//Byron/gitoxide/commit/1aa1f5e84a07427d5d7f3231735fe9c1923f506f))
    - Require latest version of git-features in git-odb ([`e664e93`](https://github.com//Byron/gitoxide/commit/e664e93e960564c43a5510d32bf5ff45624af8ee))
    - Remove usage of gitfeatures::fs in organize subcommand ([`b567d37`](https://github.com//Byron/gitoxide/commit/b567d3709a74e9fdafef54b0fe58ca82721cd773))
    - refactor; planning ([`5df492c`](https://github.com//Byron/gitoxide/commit/5df492c7d7322bde2b268deaf590f1ba012a6b8e))
    - thanks clippy ([`343ab9a`](https://github.com//Byron/gitoxide/commit/343ab9adb62da1dde495fc209c179137bbe59a10))
    - refactor ([`5b1328f`](https://github.com//Byron/gitoxide/commit/5b1328fc48deab321f81d25b5dc8e9ba55840e2c))
    - Add missing '.' at end of doc comments ([`7136854`](https://github.com//Byron/gitoxide/commit/71368544f97369a4d371d43513607c4805bd0fd0))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.7.0 (2020-12-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - fix git-odb tests ([`35c1209`](https://github.com//Byron/gitoxide/commit/35c1209164b5baaa68d1c566344ac73ee6dfae79))
    - All crates use git-hash::Kind and its types, sometimes through git-object ([`124c171`](https://github.com//Byron/gitoxide/commit/124c171aaf546d8977e9913ff84e65383a80ee98))
    - use git-hash in git-features ([`5b307e0`](https://github.com//Byron/gitoxide/commit/5b307e076f6f5975592c8b177c122c91c1d809c6))
</details>

## v0.6.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 ([`27f5955`](https://github.com//Byron/gitoxide/commit/27f5955e047f35e21a86789eb46bfd89e1c99b44))
    - (cargo-release) version 0.9.0 ([`a89fdb9`](https://github.com//Byron/gitoxide/commit/a89fdb98f64bb0ca070fa79a1f58f1232bb14090))
    - (cargo-release) version 0.5.0 ([`fc7d600`](https://github.com//Byron/gitoxide/commit/fc7d600ac2c438c8b6b91f67cb69b0ac5ec37675))
</details>

## v0.5.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 27 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`c767e07`](https://github.com//Byron/gitoxide/commit/c767e07ccfc58a28e3e8ec22b590afdf0d92b9f2))
    - more docs for owned git-object ([`b79101d`](https://github.com//Byron/gitoxide/commit/b79101d714f59a42a30eb47776486a212ec0f738))
    - thanks clippy ([`ba9b3c2`](https://github.com//Byron/gitoxide/commit/ba9b3c2345887353e02fc081be80733f1c5e22d9))
    - refactor ([`d5d7cf9`](https://github.com//Byron/gitoxide/commit/d5d7cf9d3f42d83652a7b81bc6e1ee6731396d6b))
    - more docs of git-object::owned ([`0620dce`](https://github.com//Byron/gitoxide/commit/0620dce7a3ac368354c73e3927eb96a6e4990f0c))
    - (cargo-release) version 0.8.0 ([`47c00c2`](https://github.com//Byron/gitoxide/commit/47c00c2228cf25c79e1fa3eb4229c7ab24de91e5))
    - cargo clippy Rust 1.48 ([`475a68c`](https://github.com//Byron/gitoxide/commit/475a68ce33b895de911939c51afa159df534f7b8))
    - finish refactoring git-odb ([`ec282ae`](https://github.com//Byron/gitoxide/commit/ec282ae1a3d9f16eb9c89a44e17259112d097a41))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.4.2 (2020-11-18)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 74 commits contributed to the release over the course of 60 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.7.0 ([`7fa7bae`](https://github.com//Byron/gitoxide/commit/7fa7baeb3e7d008a25e4d714eff908e2516c828b))
    - (cargo-release) version 0.4.2 ([`173c957`](https://github.com//Byron/gitoxide/commit/173c957032761705edc61a0ded1f963cac73c320))
    - Minor fixes to git-odb docs ([`3788512`](https://github.com//Byron/gitoxide/commit/37885125d7c4d1dba7aaff37b5d39a7c249bf794))
    - complete docs for git-odb ([`0cf8496`](https://github.com//Byron/gitoxide/commit/0cf84967feed768bc29de29f65f6dc4622008464))
    - prefer empty doc strings for modules over [allow missing docs] ([`9b3f04f`](https://github.com//Byron/gitoxide/commit/9b3f04f4247d6d2a139f813ea2555203d374962a))
    - add remaining doc strings for git-odb ([`428f0ad`](https://github.com//Byron/gitoxide/commit/428f0ad2072148416b54b050add9a50868e7e5d0))
    - Some more docs ([`2d87124`](https://github.com//Byron/gitoxide/commit/2d87124344af845a34d17693f5ef04c9fb3323e1))
    - try to document all the bits an pieces of git-odb ([`1b353fa`](https://github.com//Byron/gitoxide/commit/1b353fa95723a7fe4ddef0a70486a74957e727cd))
    - Finish docs on top-level traversal method ([`2ef1c99`](https://github.com//Byron/gitoxide/commit/2ef1c99a48c39cb9f3362a5ea493b5e90e4593c9))
    - start describing how pack traversal works ([`5e990f2`](https://github.com//Byron/gitoxide/commit/5e990f20dee6005d23ebc5a56389f09d9d7f8782))
    - refactor ([`a681335`](https://github.com//Byron/gitoxide/commit/a681335b51c10ff56ddd2fe80ec24449a771abd2))
    - document pack::index::write ([`f5edc60`](https://github.com//Byron/gitoxide/commit/f5edc602cb3e570ce154a3ba3d692fcbcf8d55c0))
    - dependency update ([`bc336d9`](https://github.com//Byron/gitoxide/commit/bc336d9bb22d13a6d2407b44b297fcb770cdaac6))
    - refactor ([`6b909a2`](https://github.com//Byron/gitoxide/commit/6b909a22cf981b33060cb6f1324ec3231146d159))
    - refactor ([`b511a2b`](https://github.com//Byron/gitoxide/commit/b511a2b1d9b6d55b1937ad3f4bbbb331b5cdd9a3))
    - document index integrity checking ([`9336ab9`](https://github.com//Byron/gitoxide/commit/9336ab9f9675ba5d33eacefc585d745e1b0bcc18))
    - docs for index access ([`996acbf`](https://github.com//Byron/gitoxide/commit/996acbf67fde183a0e5f553ecad9b2361eecf18b))
    - docs for top level pack index module ([`d2dd72f`](https://github.com//Byron/gitoxide/commit/d2dd72fe2d230ecdd88343535ecdbfbd8ae1b143))
    - document pack data verification ([`27962ca`](https://github.com//Byron/gitoxide/commit/27962ca9019d0b4971fa76afedaf1d85f451665b))
    - document pack entry iteration ([`c869ee9`](https://github.com//Byron/gitoxide/commit/c869ee93c6f042ce3de4962229e2caa4377af62b))
    - docs for pack header ([`9505b40`](https://github.com//Byron/gitoxide/commit/9505b401a87c3107ac1e5775ff6c10e8a808ba25))
    - some more pack data file documentation ([`05e05f4`](https://github.com//Byron/gitoxide/commit/05e05f46a38bcc068b564409d92310dd93ca5527))
    - docs for Bundle::write_* ([`ac41253`](https://github.com//Byron/gitoxide/commit/ac41253067803796e5623184d7dee790aa597809))
    - remove special Error with just one variant… ([`d05a416`](https://github.com//Byron/gitoxide/commit/d05a416dc43164f4c9fb2ee00884107fdbd13f90))
    - Docs for Bundle::locate ([`066787c`](https://github.com//Byron/gitoxide/commit/066787c12e3142732d3ba65b233c836f89745543))
    - some more docs for 'pack' module ([`c32850d`](https://github.com//Byron/gitoxide/commit/c32850d4b6f94dd636d09b6222d2aa7ee6a85c82))
    - some more documentation ([`201f67c`](https://github.com//Byron/gitoxide/commit/201f67ce52e39dde3a79ff8a1f05bbaf30deec15))
    - Merge branch 'commit-graph' into main ([`9cb09b2`](https://github.com//Byron/gitoxide/commit/9cb09b248796f0ff5c9d3f3e857de4731324cfd5))
    - specify the hash to create with 'hash::bytes_of_file' ([`c000294`](https://github.com//Byron/gitoxide/commit/c000294423ae0759b978399db3b69ac07c20578d))
    - move 'git_odb::hash::bytes_of_file' into git_features::hash ([`c5f6b45`](https://github.com//Byron/gitoxide/commit/c5f6b4587ee4042a080c0505613b0c72fdfe5273))
    - the daily commit (single handedly) ([`b528c2e`](https://github.com//Byron/gitoxide/commit/b528c2e1bf0a3211491535427c4bd36212711a0f))
    - document `loose::Object` entirely ([`d5eef9c`](https://github.com//Byron/gitoxide/commit/d5eef9cdd06910eeaf1f1c4114b97638a29c7327))
    - thanks clippy ([`b9e0a87`](https://github.com//Byron/gitoxide/commit/b9e0a87996b8f3c4531a392607c353a1f0824ce6))
    - docs for Sink ([`e7a09f0`](https://github.com//Byron/gitoxide/commit/e7a09f0628b44ae0c6b564ef41f044e51866f2df))
    - docs for compound object databases ([`813df71`](https://github.com//Byron/gitoxide/commit/813df7115eb643742158f975975eb7469443cc07))
    - Document borrowed odb objects ([`7626f7f`](https://github.com//Byron/gitoxide/commit/7626f7f3af885f1b95801f9dbc71bee0bc77400e))
    - Document alternates implementation ([`60666e8`](https://github.com//Byron/gitoxide/commit/60666e86316c81f3bb63ee151e457af78dbefc00))
    - docs for git-odb crate (top-level) ([`71af366`](https://github.com//Byron/gitoxide/commit/71af366c84e1bd00125b4582d80799a6d927324a))
    - remove dash in all repository links ([`98c1360`](https://github.com//Byron/gitoxide/commit/98c1360ba4d2fb3443602b7da8775906224feb1d))
    - thanks clippy ([`e355b4a`](https://github.com//Byron/gitoxide/commit/e355b4ad133075152312816816af5ce72cf79cff))
    - refactor ([`5a1cbf2`](https://github.com//Byron/gitoxide/commit/5a1cbf299f2d5c1c07143d14ee3ded95d6a44a20))
    - And octal values unquoting works too ([`5effc7b`](https://github.com//Byron/gitoxide/commit/5effc7b6daf6ff49b6d51af09f8da148602c7322))
    - All explicit escapes ([`1841544`](https://github.com//Byron/gitoxide/commit/18415445caaee6e9e54aabddb88bdcd2f5602508))
    - First bunch of simple unescapes ([`a45c594`](https://github.com//Byron/gitoxide/commit/a45c5941cf426537710842917c0e715cf4c74863))
    - prepare for actual unescaping ([`284da44`](https://github.com//Byron/gitoxide/commit/284da449cae62d12ea4eea8c31f1225699c5e52e))
    - basic infrastructure for unquoting c-style strings ([`f81bb03`](https://github.com//Byron/gitoxide/commit/f81bb038bfc8ea0d9b61012d6effae084c89335a))
    - fix incorrect cycle detection, which worked on MacOS by accident ([`a6e7765`](https://github.com//Byron/gitoxide/commit/a6e77650a886ac33b23af8892182c9832a86e997))
    - Also use alternates for looking up objects… ([`ade929d`](https://github.com//Byron/gitoxide/commit/ade929df38e619850f73389178a2c53e1c43fa45))
    - prepare for unquoting c-strings ([`47e2fa0`](https://github.com//Byron/gitoxide/commit/47e2fa03a1e2fe163c5c019d52bbb0ddbdb80bf0))
    - Read multiple alternates from single file; ignore comments ([`1f8d367`](https://github.com//Byron/gitoxide/commit/1f8d36705c4568b1036b0d62b3a80ae6ec20a99c))
    - support for relateive alternates ([`b20e9ee`](https://github.com//Byron/gitoxide/commit/b20e9eea423ced275781d410217110c85ddb587c))
    - Ignore all cycles and be happy if we have found at least one actual odb ([`1effdfd`](https://github.com//Byron/gitoxide/commit/1effdfda703d5eb9cd1333a7bae21075ef9e53cc))
    - prepare for multi-line parsing and all the bells and whistles ([`08f9ec4`](https://github.com//Byron/gitoxide/commit/08f9ec41feee56fe0ff2b057bb50391100bdb84e))
    - Make compound DB initialization less lazy… ([`6dc57b3`](https://github.com//Byron/gitoxide/commit/6dc57b31d0bc5abfca100ab1d4b5dff68852aad8))
    - Use parallel walkdir (via jwalk) when parallel feature is enabled ([`f444c85`](https://github.com//Byron/gitoxide/commit/f444c859f5b215ea70a46d5493a2babbf7a98235))
    - alternate now handles cycles ([`71167e4`](https://github.com//Byron/gitoxide/commit/71167e4e50efa8a097c3b09a249004e97aeaf2c8))
    - first simple alternate tests ([`7372118`](https://github.com//Byron/gitoxide/commit/73721185cfd646c6e83b2548280fad8f480f8324))
    - test for circular alternates ([`fc92709`](https://github.com//Byron/gitoxide/commit/fc927091d69196a930c0cea4611af8d96b7b84d8))
    - thanks clippy ([`4ddc64f`](https://github.com//Byron/gitoxide/commit/4ddc64fd71d3d1260e001f89c379c46fe157e5ce))
    - Actually resolve alternates when creating a compound DB ([`9be7aed`](https://github.com//Byron/gitoxide/commit/9be7aed7bd4b939d98b9a8d1db8a6ffc85044ca9))
    - refactor ([`c1eff58`](https://github.com//Byron/gitoxide/commit/c1eff58cd28e45a2d5f46481551724b81735ede3))
    - first sketch of alternate resolution ([`6cc8a94`](https://github.com//Byron/gitoxide/commit/6cc8a947df776aeeb031de627f84b7bc85207235))
    - remove quickerror dependency from git-odb ([`7e27495`](https://github.com//Byron/gitoxide/commit/7e2749521b6c873766a2f6f96e6c91a0c6a9dbf3))
    - refactor ([`7874c35`](https://github.com//Byron/gitoxide/commit/7874c35bccb74ae7670335e633efa7eaebc72630))
    - refactor ([`3ec99dc`](https://github.com//Byron/gitoxide/commit/3ec99dc7360c01b4f3c4593ff5049361e7043254))
    - refactor ([`519dd12`](https://github.com//Byron/gitoxide/commit/519dd12f2bf58dd3048bc12e5b058236ad727853))
    - refacator ([`7ac2153`](https://github.com//Byron/gitoxide/commit/7ac21536b3cee6708489011731216b9b831509e4))
    - refactor ([`d4f288c`](https://github.com//Byron/gitoxide/commit/d4f288ceb2436b292993df74ed07d4d7e578edf1))
    - refactor ([`3a8fb61`](https://github.com//Byron/gitoxide/commit/3a8fb61067c210d4db6d515f21b2e28425c52e8c))
    - refactor ([`98b3f4a`](https://github.com//Byron/gitoxide/commit/98b3f4a9cc65e76aa09280065ab1d151f637e692))
    - refactor ([`127b8b2`](https://github.com//Byron/gitoxide/commit/127b8b2949476b38ef6f8ea0fb68bae6d473adcc))
    - refactor ([`669b726`](https://github.com//Byron/gitoxide/commit/669b726da305ce4520c792d62b4344b04fe5f996))
    - refactor ([`7bc321e`](https://github.com//Byron/gitoxide/commit/7bc321e96ecce0aae5063eb7008ecbac7d2ca31c))
    - refactor ([`0752b45`](https://github.com//Byron/gitoxide/commit/0752b45e95dd5378b7fca5b70bd11b9100ba2946))
</details>

## v0.4.1 (2020-09-18)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 39 commits contributed to the release over the course of 5 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.1 ([`60ac8b0`](https://github.com//Byron/gitoxide/commit/60ac8b0a7545fff6ef293fd48716e9a19175517c))
    - refactor ([`ad17bfd`](https://github.com//Byron/gitoxide/commit/ad17bfdc07e1301693fdfa3d09df3b39f675a36f))
    - refactor ([`91d9f78`](https://github.com//Byron/gitoxide/commit/91d9f78a9af04b44b2cead30d6e1cbaaeb76a522))
    - refactor ([`6ebb5d1`](https://github.com//Byron/gitoxide/commit/6ebb5d1839cd5ab4d8aff78acbccebaa66f439c7))
    - refactor ([`8877b77`](https://github.com//Byron/gitoxide/commit/8877b776bda8d1f202e86ac471ea30b595cff41b))
    - refactor ([`4a0d034`](https://github.com//Byron/gitoxide/commit/4a0d0342a20f519f30fe8b84d51ebb2bdea23752))
    - refactor ([`485aa91`](https://github.com//Byron/gitoxide/commit/485aa91c7412c55c0215e33cc4f906dd62e440a8))
    - refactor ([`c1d2f41`](https://github.com//Byron/gitoxide/commit/c1d2f41938211985a6cdb7a0fde6bcb51a7944de))
    - refactor ([`07aff14`](https://github.com//Byron/gitoxide/commit/07aff14a8c2ceca3202b0506b3bd4286550ac3a0))
    - refactor ([`57d463f`](https://github.com//Byron/gitoxide/commit/57d463ffeb5861270abaaf72f662b14c9c262052))
    - refactor ([`c6be43d`](https://github.com//Byron/gitoxide/commit/c6be43de3493566cedd98ce49fb2c8af7714a61c))
    - refactor ([`524d0fe`](https://github.com//Byron/gitoxide/commit/524d0fec17c356c846f0c62f87f2637a7a6fa56b))
    - refactor ([`a8f4cd7`](https://github.com//Byron/gitoxide/commit/a8f4cd7b9c31e59c3329cc649aca8378cd34a597))
    - Checksum verification for compound object ([`3be08b0`](https://github.com//Byron/gitoxide/commit/3be08b09cd71e5e5eb21bdd81d6a07d2c232e6e8))
    - refactor ([`59d989a`](https://github.com//Byron/gitoxide/commit/59d989a9c86789d6572c9a3dfd8a3652bd4a7a1b))
    - More methods for compound object ([`84d2b0e`](https://github.com//Byron/gitoxide/commit/84d2b0ec53f7def1470fbadff45fbe266bceb71a))
    - refactor ([`e5a9343`](https://github.com//Byron/gitoxide/commit/e5a9343f3f5304de4c9f614cdb260cf0fcfbbbfb))
    - refactor ([`6a84f13`](https://github.com//Byron/gitoxide/commit/6a84f137754cddfdcb9b1fec3e353762ebb3ce2b))
    - refactor ([`4e89c3b`](https://github.com//Byron/gitoxide/commit/4e89c3bc0f14cf9581348ae2c1620ade9dc1db8f))
    - Document why we won't use nightly for fixing NLL issue ([`ca29368`](https://github.com//Byron/gitoxide/commit/ca29368b42b902fe7fe14dd4bff1b35e266c96a8))
    - Revert "Fix NLL issue by using nightly" ([`6864a55`](https://github.com//Byron/gitoxide/commit/6864a55001f1d01839f948618355928d666602ee))
    - Fix NLL issue by using nightly ([`8c5bd09`](https://github.com//Byron/gitoxide/commit/8c5bd095539042d7db0e611460803cdbf172beb0))
    - Update tasks, prepare for NLL fix ([`52af8d1`](https://github.com//Byron/gitoxide/commit/52af8d1089dc85cff19aee276bd831393f1b84b3))
    - Thanks clippy ([`6c4d1ec`](https://github.com//Byron/gitoxide/commit/6c4d1ec33d37b99b38698dfd91d38216ab4a2ef2))
    - This works, but locates twice… ([`4e709f6`](https://github.com//Byron/gitoxide/commit/4e709f6029cf98f8c6ff204598706e2b6a1775eb))
    - Also the imperative version doesn't borrowcheck… ([`c5720f1`](https://github.com//Byron/gitoxide/commit/c5720f1e4dc790539980fa81e940be6c6e15b50a))
    - Looks like the functional approach to locate(…) doesn't borrowcheck ([`5df6867`](https://github.com//Byron/gitoxide/commit/5df6867a2b9fa7ba3fe2cdcd3bb9766faba1ae1b))
    - refactor ([`9e68c6b`](https://github.com//Byron/gitoxide/commit/9e68c6bcd1d07ea73730ce5ff59d7883152f894d))
    - refactor ([`f219d5a`](https://github.com//Byron/gitoxide/commit/f219d5a25efb7e258249ca3a4d39382136fe4229))
    - sketch compound::Db::locate; sort packs by size ([`6609a53`](https://github.com//Byron/gitoxide/commit/6609a534f45fc1ffc9d80a60a6a9793cbf54131c))
    - refactor ([`4a09754`](https://github.com//Byron/gitoxide/commit/4a09754f6cd17d7f39f8a71b7de44d517294ffc5))
    - Implement Write in terms of writing to the loose object DB ([`02b88c2`](https://github.com//Byron/gitoxide/commit/02b88c28304ff6d8c1fbad6fdcfa36f3b1f9dafc))
    - First sketch of compound Db ([`9bf2279`](https://github.com//Byron/gitoxide/commit/9bf227914d9281bfbdfc902edc3c1cc21c7fa3cd))
    - refactor ([`203ba99`](https://github.com//Byron/gitoxide/commit/203ba995c9e237ac63bc2ecebda18171e90fcb47))
    - (cargo-release) version 0.6.0 ([`9ef184e`](https://github.com//Byron/gitoxide/commit/9ef184e35712f938fb4f9f6da7390a8777a9284e))
    - Revert "FAIL: try to get rid of tree-traversal Boxed error…" ([`1b42b31`](https://github.com//Byron/gitoxide/commit/1b42b3141dded644a17c8d23057c987e2bac4f80))
    - try to get rid of tree-traversal Boxed error… ([`13159eb`](https://github.com//Byron/gitoxide/commit/13159eb972ed78ce4ebee2313b288023cec91c47))
    - Parameterize traversal error with Processor error ([`1513a13`](https://github.com//Byron/gitoxide/commit/1513a13179bedefd12fc08da07a05c7f07ed4ef6))
    - Switch to prodash 10 and safe a lot of trait bounds in the process ([`e2fb1d9`](https://github.com//Byron/gitoxide/commit/e2fb1d944b4d803a11c91f868b831d406fb5e35f))
</details>

## v0.4.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 30 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 ([`2272fa4`](https://github.com//Byron/gitoxide/commit/2272fa4bcacdaf1898e4cd8b791232fc1321227f))
    - (cargo-release) version 0.4.0 ([`0d7b60e`](https://github.com//Byron/gitoxide/commit/0d7b60e856325009431172e1df742a1cd2165575))
    - (cargo-release) version 0.5.0 ([`82b7313`](https://github.com//Byron/gitoxide/commit/82b73131b79ec3c42a712dad1c0766a72209d737))
    - [clone] All it took was a an intermediary to call 'read' as expected ([`7c8ecb7`](https://github.com//Byron/gitoxide/commit/7c8ecb78e988f7752cea6fe2022ccf9739b86fd4))
    - [clone] minor refactor; it's definitely the read() that doesn't work… ([`406829b`](https://github.com//Byron/gitoxide/commit/406829b951164673c0b8152d1e9de76f1318df0a))
    - [clone] This actually works: first MVP of retrieving packs via clone ([`c06d819`](https://github.com//Byron/gitoxide/commit/c06d8194173f9ec468ddd0faf72dd6d8dbf7d35d))
    - [clone] support for progress that can handle writing pack files ([`46e0055`](https://github.com//Byron/gitoxide/commit/46e0055eab47e402807b15c63b6a4577f5c0b7bb))
    - Use fast-reset for miniz oxide to gain about 4s when resolving the kernel pack ([`e5b6ce4`](https://github.com//Byron/gitoxide/commit/e5b6ce440073c1db32ed4afc8e1db21b32f62863))
    - fix build ([`6178133`](https://github.com//Byron/gitoxide/commit/6178133fd1e08af6abb90ba7d1a4c22970da850c))
    - refactor ([`174baa7`](https://github.com//Byron/gitoxide/commit/174baa7733d34d1dbb2d47f4163ca39fb4a4c473))
    - bump git-features to 0.4 to allow publishes after breaking changes ([`9d6b879`](https://github.com//Byron/gitoxide/commit/9d6b8790e2edd7fa01b3239adff86a7cd2393f10))
    - Allow dual-licensing with Apache 2.0 ([`ea353eb`](https://github.com//Byron/gitoxide/commit/ea353eb02fd4f75508600cc5676107bc7e627f1e))
    - refactor ([`b4a6e16`](https://github.com//Byron/gitoxide/commit/b4a6e16364822c0dccb56f98dbfb0ca4c8007069))
    - remove tree compaction code ([`dfc6c7d`](https://github.com//Byron/gitoxide/commit/dfc6c7dde9014e79eb4a752d81bc3c77ad36c366))
    - See if tree compaction saves considerable amounts of memory ([`0092c25`](https://github.com//Byron/gitoxide/commit/0092c256b3bfaf2818566540e660cdefcf68d246))
</details>

## v0.3.0 (2020-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 413 commits contributed to the release over the course of 31 calendar days.
 - 4 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump minor version to 0.3 ([`4351e28`](https://github.com//Byron/gitoxide/commit/4351e2871c9dcf342b8471fffa74cae338a53269))
    - thanks clippy ([`6725104`](https://github.com//Byron/gitoxide/commit/6725104d2841e6518db641d06e3e107cf4f40f96))
    - Also run file hashing in indexed mode in parallel (like with lookup) ([`8f8d14f`](https://github.com//Byron/gitoxide/commit/8f8d14f4606e99dc710eb352a985db48c00ea4f4))
    - first step towards parallelizing file hashes and traversal! ([`9573836`](https://github.com//Byron/gitoxide/commit/95738369e0d3accf7f6239c8cd966a7f5c36825a))
    - allow hashing to be interrupted ([`df4dfd7`](https://github.com//Byron/gitoxide/commit/df4dfd7ec1be608cec1117f56303c528fb8f7ba7))
    - unify file based file verification of data and index ([`e1b4105`](https://github.com//Byron/gitoxide/commit/e1b4105308963cfe9102c2dee461c7dd948ee942))
    - update to quick-error 2.0 ([`4b1b784`](https://github.com//Byron/gitoxide/commit/4b1b7849b47a54092b49821c39e864c86adda979))
    - Haha, funny, silly me… ([`a4a1244`](https://github.com//Byron/gitoxide/commit/a4a1244af2d386e75ebd55909d4675b475ccd905))
    - better usability for units ([`b226253`](https://github.com//Byron/gitoxide/commit/b226253636d8146a084a7bcd7c0c320e37f9d2fb))
    - better progress for Sha1 of pack and index ([`310a59e`](https://github.com//Byron/gitoxide/commit/310a59ee99ce78a4f14326c0058ea0c543a1d24c))
    - Make obvious that interrupt request was received ([`34b2373`](https://github.com//Byron/gitoxide/commit/34b23737f560fe52d4f98fb886eba754652f9a5e))
    - Conditionally use an eager iterator… ([`e9b5511`](https://github.com//Byron/gitoxide/commit/e9b5511568f4e64968596994855783f19672d678))
    - Reduce progress information for threads ([`e9a1b31`](https://github.com//Byron/gitoxide/commit/e9a1b310fd99675dc87e56c6277d57259a6415a0))
    - Revert "A test to see how much time can be saved by not zeroing zlib buffers" ([`3d51d59`](https://github.com//Byron/gitoxide/commit/3d51d595469d8451867331089e75b808f9361912))
    - A test to see how much time can be saved by not zeroing zlib buffers ([`fd41a51`](https://github.com//Byron/gitoxide/commit/fd41a51de2261f425262ee7dee7a24fd87d87432))
    - Implement optionally keeping the compressed bytes ([`fc26914`](https://github.com//Byron/gitoxide/commit/fc26914a57b6e89c703e1b04d6a4d8d31005ddbc))
    - first step towards more control over allocation in iterator ([`cacf76c`](https://github.com//Byron/gitoxide/commit/cacf76cd69996073894e400e65322d3547493789))
    - never keep decompressed bytes while streaming… ([`65c3856`](https://github.com//Byron/gitoxide/commit/65c38569219134ccd412a8a3ee7ec618d866c941))
    - Only keep base objects, not the deltas (like originally intended) ([`fc8334b`](https://github.com//Byron/gitoxide/commit/fc8334b8d196425f2b766ebb9772a12483ef0f8d))
    - reduce footprint of sha1 when writing the index ([`12aa454`](https://github.com//Byron/gitoxide/commit/12aa4549bee0d9ea00bb0723acefa8187f5119a9))
    - first successful test of moving the streaming iterator into its own thread ([`c9fcb68`](https://github.com//Byron/gitoxide/commit/c9fcb68c644c96a15cb9956a754bec7b65bb5fbd))
    - first sketch of order-destroying eager iterator ([`20fca45`](https://github.com//Byron/gitoxide/commit/20fca4515f6e9ea320d0bf21c15cd6d2c3cff742))
    - add object size progress when resolving with index ([`b2f8c9e`](https://github.com//Byron/gitoxide/commit/b2f8c9e85dfac63f70ca7b0e91af697b801b4131))
    - add decompression progress ([`0e5c534`](https://github.com//Byron/gitoxide/commit/0e5c534d7c6e661a1f6c1cdb59ad1c9ffade642d))
    - Print read throughput automatically ([`0a71b48`](https://github.com//Byron/gitoxide/commit/0a71b482310a129aa8757475290b3b24a200b702))
    - Allow 'read' progress to go out of scope while keeping it accessible! ([`d7a7828`](https://github.com//Byron/gitoxide/commit/d7a782899ca841291e240bad822bb8184d6f5083))
    - Fix throughput display of otherwise stepped progress indicators ([`399f81d`](https://github.com//Byron/gitoxide/commit/399f81daadb8c111b9cad958945924e0eed2c2ad))
    - unify used ranges for line renderer amond pretty and lean interface ([`f59f66e`](https://github.com//Byron/gitoxide/commit/f59f66e189732f567414f68c7463364e510f41c4))
    - Add percentage and throughput to tasks that matter ([`763d7ca`](https://github.com//Byron/gitoxide/commit/763d7caa4c70111b7cb3ef5733d2c3c697758c28))
    - Upgrade to latest iteration of prodash ([`3a4faec`](https://github.com//Byron/gitoxide/commit/3a4faecab56e37670c553e6563f11a46d740c333))
    - First part of migration to prodash 8.0, but… ([`6901a09`](https://github.com//Byron/gitoxide/commit/6901a098641820c8d974ce56a24d6cdca779730d))
    - Fix various issues related to 64bit offset support when writing indices… ([`da31694`](https://github.com//Byron/gitoxide/commit/da31694ee13022bcc52ed06389469d65b4e37daa))
    - fix unit tests: actually sort the directory entries :D ([`b69717a`](https://github.com//Byron/gitoxide/commit/b69717af368510347a550012f4ed97ba24d36ffd))
    - Add convenience method to get a new bundle for the index/data just written ([`a6d74ad`](https://github.com//Byron/gitoxide/commit/a6d74ad7b65cdc293c8504dae73ea1c717e5bfca))
    - bundle write with a given directory ([`7f29c73`](https://github.com//Byron/gitoxide/commit/7f29c73d35b8717c8834beac259ed71eebcc2058))
    - first unit test for bundle writing ([`74bda39`](https://github.com//Byron/gitoxide/commit/74bda3963af7fe4b97e7f04f0bb9e150df8b7fa7))
    - journey tests for restore functionality ([`1aa63e4`](https://github.com//Byron/gitoxide/commit/1aa63e419736960915c03c29827a57c18261e04d))
    - refactor ([`fc42567`](https://github.com//Byron/gitoxide/commit/fc4256788f7c3d3c4a05f240eee4d71a716cafce))
    - refactor ([`cf3ebe0`](https://github.com//Byron/gitoxide/commit/cf3ebe00619d16e957166578038520b2bf080411))
    - refactor ([`72ca435`](https://github.com//Byron/gitoxide/commit/72ca435a90e470797ae59dd10640c36b84bb4f41))
    - more flexible error types for processors - anything goes ([`be3a947`](https://github.com//Byron/gitoxide/commit/be3a947ba6197319fea0b38e48008850cc971bf6))
    - refactor ([`c7dd581`](https://github.com//Byron/gitoxide/commit/c7dd581348a05146d7a79f7622bf30a08d34f474))
    - refactor ([`aae8e79`](https://github.com//Byron/gitoxide/commit/aae8e79a89261a548f088454ca6082a34c2063ce))
    - refactor ([`0e27763`](https://github.com//Byron/gitoxide/commit/0e27763995e135fc1bca56e6084b5c81825dba22))
    - Make lookup based algorithm gracefully interruptible ([`8d2e649`](https://github.com//Byron/gitoxide/commit/8d2e649c754d713e6dd48315cd043204ffda4a7b))
    - Write about user interfaces and the use/non-use of async ([`91ba045`](https://github.com//Byron/gitoxide/commit/91ba0457745f860b7a68cb38b13e69754747e8d9))
    - Use pack hash for index file as well :D ([`2106c64`](https://github.com//Byron/gitoxide/commit/2106c64484c2162ee4e715efc592db14da602327))
    - support for interruptible operations ([`a025593`](https://github.com//Byron/gitoxide/commit/a02559378f9165df97a217f24834a851be719b08))
    - thanks clippy ([`62d2ff3`](https://github.com//Byron/gitoxide/commit/62d2ff383c5f7fe884057c70868569a811a73e00))
    - organize object type comparisons by probability… ([`19a5d94`](https://github.com//Byron/gitoxide/commit/19a5d9465f7962cfcc39ea31a2c84be6235e40ed))
    - count object types as well ([`e04a8d1`](https://github.com//Byron/gitoxide/commit/e04a8d16fda3712663d8d9220f3a017e668b6283))
    - Revert "Less memory for look up mode, faster start" - too slow ([`584350a`](https://github.com//Byron/gitoxide/commit/584350af91f533db4cf980327d530445384c6b5a))
    - Less memory for look up mode, faster start ([`395c7e7`](https://github.com//Byron/gitoxide/commit/395c7e78ef344ee56cf3d4ef49828942a09094bc))
    - remove petgraph entirely ([`70ba33a`](https://github.com//Byron/gitoxide/commit/70ba33a23a3ef887323ee29c248422f1997af6be))
    - compute statistics for indexed pack verify ([`3d31c23`](https://github.com//Byron/gitoxide/commit/3d31c235edaf7f88eb954cffc6864777566b3ef1))
    - prepare for computing indexed statistics ([`082c246`](https://github.com//Byron/gitoxide/commit/082c2467f2ab46aeb285504abcf2d8945dac4ce5))
    - refactor ([`bfbae90`](https://github.com//Byron/gitoxide/commit/bfbae905d3e8a0c5f30779c1723163a947de355e))
    - keep all metadata per object needed to compute the usual statistics ([`961b85e`](https://github.com//Byron/gitoxide/commit/961b85efec1ce84beacaa35720746752f687413a))
    - make 'level' available to support statistics ([`f7ba51c`](https://github.com//Byron/gitoxide/commit/f7ba51c93b04ef2e98f2436cf72e8c28b89b2448))
    - refactor ([`6277318`](https://github.com//Byron/gitoxide/commit/6277318f2ea71451b023a11fc9f74149d11fe9a9))
    - support for error handling in traversal callbacks ([`c1d5bf6`](https://github.com//Byron/gitoxide/commit/c1d5bf628db5f0c79aaf9af9740b990fc78aa4d5))
    - indexed traversal now works, in theory, but needs error handling ([`86f8400`](https://github.com//Byron/gitoxide/commit/86f8400a5e74e75fe7dab24911215a3f820b64b1))
    - support for progress ([`62108fd`](https://github.com//Byron/gitoxide/commit/62108fda164ae35903147eb1808c951bb90dac85))
    - support for thread local storage in callbacks ([`1dad088`](https://github.com//Byron/gitoxide/commit/1dad088a7d0be10a83cae0f119d42501887043e3))
    - support for learning about the objects slice in the pack ([`faec782`](https://github.com//Byron/gitoxide/commit/faec78276c4814edc7bbde150f8379fa73abc364))
    - And even more caapbilities are required to make tree traversal work natively ([`90523bb`](https://github.com//Byron/gitoxide/commit/90523bb983e2cac70dad822531b7d66b7196cefc))
    - refactor ([`2bbfd82`](https://github.com//Byron/gitoxide/commit/2bbfd82f909ebc30cfb276bf40c7dbaa424a62f8))
    - refactor ([`efa7cd8`](https://github.com//Byron/gitoxide/commit/efa7cd843f7e93a8c4beba20597ff6d914bd6a33))
    - first steps towards actually using the new tree traversal during verification ([`785b0ff`](https://github.com//Byron/gitoxide/commit/785b0ff02c0e00f6e7dea3a9c41a32f4129659e6))
    - thanks clippy ([`44b20de`](https://github.com//Byron/gitoxide/commit/44b20deafeac85151d57ecf4c0f5d889e9fe32f7))
    - refactor ([`afe5e44`](https://github.com//Byron/gitoxide/commit/afe5e445617c79b29d519257042b85d9533d40b0))
    - refactor ([`fcc660d`](https://github.com//Byron/gitoxide/commit/fcc660dee6f70b40364c70c73fc6b436929df4cd))
    - reduce memory usage for index considerably ([`aa802be`](https://github.com//Byron/gitoxide/commit/aa802be3402ad26a2907711cd5d1476b0caeec03))
    - and now it works! ([`f14e10e`](https://github.com//Byron/gitoxide/commit/f14e10e9cfe1f4a4b477fcfc9459e49b439b0217))
    - use new traversal in index writing, but it doesn't work yet ([`0dd5570`](https://github.com//Byron/gitoxide/commit/0dd5570a1c615192f3c9382dfb7ffb1d817924db))
    - refactor ([`4ff69c6`](https://github.com//Byron/gitoxide/commit/4ff69c6281ba8d9af29a9f4407e9b2fa72f6550c))
    - refactor ([`6cbb7cc`](https://github.com//Byron/gitoxide/commit/6cbb7ccfc3e1fde4febfe652c25f5566937d3ad2))
    - generalized tree traversal can theoretically work ([`64158e0`](https://github.com//Byron/gitoxide/commit/64158e095f348ffa15139a9fa586074dad4d648b))
    - make traversal part of the tree for greater ease of use ([`6629e30`](https://github.com//Byron/gitoxide/commit/6629e3043786e5caf7d2b6fedc9350cd9e7bc6fb))
    - prepare flexible traversal on decompressed objects ([`7707ea6`](https://github.com//Byron/gitoxide/commit/7707ea6cf99d5ee93e4d6eea57adf00190d79d87))
    - refactor ([`deea36c`](https://github.com//Byron/gitoxide/commit/deea36c090fdd57ef8fc900744bbf17bd6e70097))
    - refactor ([`83a0102`](https://github.com//Byron/gitoxide/commit/83a01024d324123234776c8200ec3a3ae5f3c54e))
    - refactor ([`b77d148`](https://github.com//Byron/gitoxide/commit/b77d148ed1c5aec31cb0493b4f1e0f2d82d7e641))
    - generalize tree iteration ([`fdc06de`](https://github.com//Byron/gitoxide/commit/fdc06de2af8e7c9d2000177ce4f99ac68b5335be))
    - Also index using the new tree impl during verify (prepare replacement) ([`92039b0`](https://github.com//Byron/gitoxide/commit/92039b038653cf97029e06f3f9b80892035d8c87))
    - refactor ([`e3ff6af`](https://github.com//Byron/gitoxide/commit/e3ff6af014cfbdbb53fe9498ff75b7f49fa5beb7))
    - support for building a tree from offsets ([`95858bc`](https://github.com//Byron/gitoxide/commit/95858bcbad01138240512731ec0e6dbdaed6c9fe))
    - refactor ([`8cfe025`](https://github.com//Byron/gitoxide/commit/8cfe0257de05b08a1278f78f6bdf3b5d65447686))
    - refactor ([`bb9e518`](https://github.com//Byron/gitoxide/commit/bb9e518b71ee1b4e1ab24d1369b879e047009294))
    - count sorting into the progress, 7.5 mio entries takes a moment ([`2fc4cd8`](https://github.com//Byron/gitoxide/commit/2fc4cd8dcac50f21491b5d297237acf97b2759fa))
    - Use bigger buffers when reading from disk. ([`e76e4eb`](https://github.com//Byron/gitoxide/commit/e76e4ebb2261351bfe2af42b5782f0058f15edc6))
    - Only keep decompressed bytes of base objects… ([`b39ad89`](https://github.com//Byron/gitoxide/commit/b39ad8976ee853229f87bbf962ada9557c7bbd32))
    - remove memory mode entirely (and some complexity with it) ([`8812e91`](https://github.com//Byron/gitoxide/commit/8812e916a21983868a37c4aade10f79a1dc9b926))
    - turns out you never want to keep deltas in memory ([`657aa2c`](https://github.com//Byron/gitoxide/commit/657aa2c38673cf10174f42bcb97039ac37b2926e))
    - Remove support for keeping compressed memory to reduce the index size ([`1e2ec7e`](https://github.com//Byron/gitoxide/commit/1e2ec7e9d0ef2f2a4908860672080e411e945bff))
    - don't cause re-allocs of the compression buffer ([`2bb6fd2`](https://github.com//Byron/gitoxide/commit/2bb6fd26235825484a8f60a49455fee71f08236c))
    - Revert "FAIL: try to use a customized version of just pieces of Miniz-oxide" ([`ea0fdb3`](https://github.com//Byron/gitoxide/commit/ea0fdb3c9ae42fcbd97f9319e90873c053d4ab71))
    - try to use a customized version of just pieces of Miniz-oxide ([`9945eba`](https://github.com//Byron/gitoxide/commit/9945eba749afb020e0deaaa5bb01fda6ff9ccd84))
    - dependency upgrade + update ([`c6692c6`](https://github.com//Byron/gitoxide/commit/c6692c6d494fe2bf1f9b924cf27da5908b74d62b))
    - refactor ([`133e3ba`](https://github.com//Byron/gitoxide/commit/133e3bafea772028f4bfd0fcc28a3e9bc3507701))
    - Let go of another handbreak - decompression is much faster now ([`ae9dc16`](https://github.com//Byron/gitoxide/commit/ae9dc165b72893216e7337bf0726705adce69cd8))
    - thanks clippy ([`393067d`](https://github.com//Byron/gitoxide/commit/393067ddcf19424381ad2703c9c987d0f99587cd))
    - Use call to produce the resolver, allowing to delay opening a file mapping… ([`dd30e8d`](https://github.com//Byron/gitoxide/commit/dd30e8d3c8b6754bd90e2777ec0153e158d4a708))
    - Fix actual memory violation (thanks to unsafe code) ([`c44c5e1`](https://github.com//Byron/gitoxide/commit/c44c5e1890bc26ced920eb484e8708456d69df15))
    - thanks clippy ([`1083a0b`](https://github.com//Byron/gitoxide/commit/1083a0b75298454d19c2bdabaf0e195c78543792))
    - Reduce memory consumption ([`6d1a7a1`](https://github.com//Byron/gitoxide/commit/6d1a7a1292e8065d0a777cb6acd34776b1e23696))
    - Unfortunately working with an optional for data is unwieldy, let's use default ([`12bbca0`](https://github.com//Byron/gitoxide/commit/12bbca0b2dd780c3f6d4117a6bd0420fec0823bc))
    - Tree can now be used as sole data structure, collecting all results ([`3e52d6f`](https://github.com//Byron/gitoxide/commit/3e52d6f89cb0ff0ab7e5a7fdb5aa892b498eef29))
    - preparation for allowing reuse of the tree data structure ([`f565512`](https://github.com//Byron/gitoxide/commit/f565512c6d37c0532d0d138dd1db0456903a0d2a))
    - refactor ([`9c4bc0a`](https://github.com//Byron/gitoxide/commit/9c4bc0a98bd024ca0a6e3d3f86f491dd92b880ac))
    - And it works! The new algorithm is sleeker, and really wants to be backported… ([`8e025b1`](https://github.com//Byron/gitoxide/commit/8e025b1177db12e0e4f2387e44e58815e703a054))
    - thanks, clippy… ([`079ce9c`](https://github.com//Byron/gitoxide/commit/079ce9c07409ceb9acfc0eae900e73a4ae51fc58))
    - Basis for re-implementing core algorithm using new Tree data structure ([`be6caf4`](https://github.com//Byron/gitoxide/commit/be6caf4caf73fb61f23a4ea42617c3ca61b44569))
    - refactor ([`290c29a`](https://github.com//Byron/gitoxide/commit/290c29ade648c7bb850c2e0629f8cc10967758fb))
    - incorporate proper filtering of bases ([`0880998`](https://github.com//Byron/gitoxide/commit/08809986ac50081d91a9dbe8fd28c3452bf54e69))
    - overhauled iterator logic, still missing 'is_root' filter ([`2bfbae1`](https://github.com//Byron/gitoxide/commit/2bfbae145e7d2256d41ed0a69e03d1e002534a49))
    - First impl of the Iterator shows it's 'unknown' what a root node is ([`3f32938`](https://github.com//Byron/gitoxide/commit/3f329380f6d13ab6ab991a5bb82e4cb38b37a52f))
    - sketch on how children access could look like ([`16a35df`](https://github.com//Byron/gitoxide/commit/16a35dfcee905a672b2c1a0741320a51b3cf67d7))
    - How a referenced version would look like… ([`e36021d`](https://github.com//Byron/gitoxide/commit/e36021df6b6b872be1249dbddd96a2d678c3bcc3))
    - refactor ([`62a01fe`](https://github.com//Byron/gitoxide/commit/62a01fee56b45ef83b4e3efb018af8ebb1db22ac))
    - more experimentation towards a safe tree data structure… ([`d907ce8`](https://github.com//Byron/gitoxide/commit/d907ce8f34ff488bc6a70f17d3c99df82b7ef41b))
    - first stab at new Tree datastructure… ([`85d7579`](https://github.com//Byron/gitoxide/commit/85d7579bf9c2f82f941b983ea4d54e16c6661c9b))
    - Safety for handling base pack offsets ([`17d8375`](https://github.com//Byron/gitoxide/commit/17d837514ad0f28771d67a64f74a30ef460fc3d1))
    - …but there seem to be issues with the kernel pack… ([`cc147bc`](https://github.com//Byron/gitoxide/commit/cc147bc60066c4ef31353a499958edadc960a9c4))
    - quick and dirty impl of gitoxide layer for bundle writing, aka index-pack ([`e78386b`](https://github.com//Byron/gitoxide/commit/e78386b824010c5ca8efca87604c339d40b545ae))
    - cargo clippy ([`586ba7a`](https://github.com//Byron/gitoxide/commit/586ba7af016f9a510b4ffeecc1aff6de0a569627))
    - implement in-memory mode; refactor ([`0c195b9`](https://github.com//Byron/gitoxide/commit/0c195b92b59892c9f5369e28acd8f99d25f42c0c))
    - refactor ([`c9d9298`](https://github.com//Byron/gitoxide/commit/c9d92980fd15e8e3568c82243d5eedb5e6e13f10))
    - Use monomorphic calls only at the expense of code siz ([`40b28d1`](https://github.com//Byron/gitoxide/commit/40b28d18736b09bf3af1a70e9854e98e94bd09fc))
    - refactor ([`150d0bc`](https://github.com//Byron/gitoxide/commit/150d0bcc3ab39061be1add3f98da299e95edbbd5))
    - Also implement the 'no output directory' branch ([`5a3240f`](https://github.com//Byron/gitoxide/commit/5a3240fae2211924ac2eb03c9f57d2234de4f26f))
    - refactor ([`68e52f8`](https://github.com//Byron/gitoxide/commit/68e52f8ce144f2daf2db407e66b3684a7d96d58d))
    - For the first time, writing an index could work with persistence ([`16e045c`](https://github.com//Byron/gitoxide/commit/16e045c3cd0f6e003a6b6e547360acdf99a06585))
    - Don't write pack to file if everything is kept in memory ([`f3ddda6`](https://github.com//Byron/gitoxide/commit/f3ddda6434824845e9abffab1d851c067428d8c7))
    - Allow data file to be optional in preparation for in-memory operation ([`95af105`](https://github.com//Byron/gitoxide/commit/95af105298e1073b71e3edcbbe3c9f3179ecf78e))
    - refactor ([`413968d`](https://github.com//Byron/gitoxide/commit/413968dfee5e5a66ed9e63823f6bda5a5a22753e))
    - refactor ([`5d27cdb`](https://github.com//Byron/gitoxide/commit/5d27cdb98b85baa5c544bc326ad50d1d7664116a))
    - optional pack lookup depending on the settings ([`2b509de`](https://github.com//Byron/gitoxide/commit/2b509deefe7e09e59bd69937044337c8ac327f5f))
    - Write-through the pack file as we receive it and move it into place ([`6180e39`](https://github.com//Byron/gitoxide/commit/6180e3995426e99400364b04af36e0265ad779aa))
    - receive progress information when reading packs in bundle ([`759091d`](https://github.com//Byron/gitoxide/commit/759091d3c6696b427d7b5aab1b6da05a0d268c04))
    - start supporting writing packs to disk right away ([`f2203e0`](https://github.com//Byron/gitoxide/commit/f2203e0ebefaf254008f4ad4628218c42f1a2208))
    - refactor ([`75c333c`](https://github.com//Byron/gitoxide/commit/75c333c121c402201ed4abf82ea7f14481d3f55b))
    - prepare for implementing the bundle with various write modes ([`de420e4`](https://github.com//Byron/gitoxide/commit/de420e4515c6e4953a6e8cf6c632e3561873caca))
    - bundle thread progress underneath reducer progress ([`76b1b2b`](https://github.com//Byron/gitoxide/commit/76b1b2b3015183129638b1f122a54fb8df8a1ac7))
    - prevent deadlock, interestingly ([`ca02901`](https://github.com//Byron/gitoxide/commit/ca02901ad0eff63c3d9105a385c0ada6179ae71a))
    - refactor ([`ea254c0`](https://github.com//Byron/gitoxide/commit/ea254c095465c880383d47a5284994a5a68a8769))
    - rough progress for writing the index ([`f1a7f9b`](https://github.com//Byron/gitoxide/commit/f1a7f9b9ec71f2ae2de2c9bbe57f5118c76fa3dd))
    - initial batch of progress usage for index creation… ([`b10e5c6`](https://github.com//Byron/gitoxide/commit/b10e5c664be9bd1bdb2b72b858ebaf35c1ed4cb4))
    - refactor ([`77b3c21`](https://github.com//Byron/gitoxide/commit/77b3c213922c2f264722fc2423dbc22d0988c507))
    - refactor ([`fb23d15`](https://github.com//Byron/gitoxide/commit/fb23d156c276484038761394a054a96d6f9ed087))
    - refactor ([`7da7e08`](https://github.com//Byron/gitoxide/commit/7da7e080241c36b3a743e9bc01b61db5758246e5))
    - refactor ([`5a3ad3a`](https://github.com//Byron/gitoxide/commit/5a3ad3a59da297c56ea47450b2c90dd24f542d40))
    - refactor ([`785a23d`](https://github.com//Byron/gitoxide/commit/785a23d9b0ef3529ca4f655ed122a5e0c783b945))
    - header encoding works now! As well as index writing :)! ([`024b854`](https://github.com//Byron/gitoxide/commit/024b854b07720f219fe12eefa94a166820523c9c))
    - initial version of a complete header encoding impl, but… ([`ce6b46b`](https://github.com//Byron/gitoxide/commit/ce6b46b1bdcdf5ff5047d3288dc6fddb5bf62f77))
    - looks like CRCs are not correct ([`3c4e4a0`](https://github.com//Byron/gitoxide/commit/3c4e4a0a61fe552913ec72c569d9a2095646b69a))
    - cargo clippy ([`a5596fb`](https://github.com//Byron/gitoxide/commit/a5596fb71fd268b6faaa3b19c8b78d3608070012))
    - Fanout writing works now… ([`93a7ba9`](https://github.com//Byron/gitoxide/commit/93a7ba913fa29f734b98fe5723d01e2a7593ae2c))
    - It's a good idea to remove old code from time to time… ([`9e47f1b`](https://github.com//Byron/gitoxide/commit/9e47f1b04a5bbd4c0f13da5d55ae6302ae941d35))
    - fanout table, but slowly I get it :D ([`cfd8a25`](https://github.com//Byron/gitoxide/commit/cfd8a25f9125c48afe4b66eab6b6ecf71097c486))
    - Fix decompression; fanout table is still wrong though ([`77fac1a`](https://github.com//Byron/gitoxide/commit/77fac1a01d8c15f9f772c3e14a430a890ff50899))
    - Despite writing the CRC32 now, it doesn't work yet ([`ecd12b9`](https://github.com//Byron/gitoxide/commit/ecd12b90aadd6bf6cdf551802918823670a45466))
    - first stab at streaming pack header encoding ([`3c6e78b`](https://github.com//Byron/gitoxide/commit/3c6e78bec9cbd4df842919cc8dc3c575414ed002))
    - refactor ([`5925d46`](https://github.com//Byron/gitoxide/commit/5925d4615216dea70b5bc737b70f898e81e540e2))
    - Simplify offset handling in favor of allocating less ([`ce4ec62`](https://github.com//Byron/gitoxide/commit/ce4ec62e66a7fd9ff720633f531156ed51d610fe))
    - Only allocate memory for offsets if needed ([`72e0642`](https://github.com//Byron/gitoxide/commit/72e06421ae386dd15b34ce6dcf5e1cf666e70c3a))
    - First complete implementation of index writing… ([`826f996`](https://github.com//Byron/gitoxide/commit/826f996b9a9d877b84d286e18f5501eaec73d6f1))
    - Reduce contention by using the shared cache only once ([`c370e13`](https://github.com//Byron/gitoxide/commit/c370e133f4626a59eadbc8b70b4b5df39a34ad71))
    - Optimize CRC handling - no need to assign it after the fact ([`ffcc03d`](https://github.com//Byron/gitoxide/commit/ffcc03de5768c26c25dafbfbe523ca3bd4422336))
    - Assure we can deltas store theyr resolved buffer ([`d2a81d9`](https://github.com//Byron/gitoxide/commit/d2a81d912cdd9ec22ed8351b2a8395d85de46aa5))
    - And it does seem to work! Awesome! ([`71cd982`](https://github.com//Byron/gitoxide/commit/71cd9824bece6215745b02d9df001ae202fe2597))
    - Delta-application could work if we handle our buffer better ([`ac6100b`](https://github.com//Byron/gitoxide/commit/ac6100b094842f0a472be9789c024fc45939ff06))
    - refactor ([`400a2a9`](https://github.com//Byron/gitoxide/commit/400a2a91edd72394de0aba55628154c16bca98bc))
    - One step before applying deltas ([`a074193`](https://github.com//Byron/gitoxide/commit/a07419303b3b9a24acb580a8653da952a5fa9964))
    - prepare for delta application ([`9a9fb7a`](https://github.com//Byron/gitoxide/commit/9a9fb7a53fbda1b77d013f9806bd383a06135741))
    - cargo clippy ([`d69c973`](https://github.com//Byron/gitoxide/commit/d69c973626fc554d34326b7ba37243b5389d2193))
    - parse pack header before trying to decompress :D ([`9d1b44a`](https://github.com//Byron/gitoxide/commit/9d1b44ad98bb4cac55749ce25af5e444bc14d4ab))
    - refactor ([`772e9ce`](https://github.com//Byron/gitoxide/commit/772e9cef82b1d58a1d7c9ad23dda570ec97bcc0b))
    - consumer can resolve entries ([`13adce6`](https://github.com//Byron/gitoxide/commit/13adce6e18a4efb9da30dfc86c22a74dbc9026aa))
    - refactor ([`c87f770`](https://github.com//Byron/gitoxide/commit/c87f77036eb8f8b095997afcf5200b165d9ddf2f))
    - refactor ([`d9d406d`](https://github.com//Byron/gitoxide/commit/d9d406d77531bdfe5b33ee8ed17bccd431e85f9b))
    - first version of resolver to copy from a memory map ([`506b8fd`](https://github.com//Byron/gitoxide/commit/506b8fd94478ab259d18f4226c4b25bd080f775d))
    - rethink resolver into something even simpler ([`4388c6c`](https://github.com//Byron/gitoxide/commit/4388c6c1ccbfbee7d5abb064eab3569a1aebf6a0))
    - use parking_lot where possible ([`367874e`](https://github.com//Byron/gitoxide/commit/367874e91a2ca79d17c90d8ebaace1ee23efb4d9))
    - Consumers can fail gracefully ([`9082080`](https://github.com//Byron/gitoxide/commit/9082080a43e4db43378abc5555ad6f8084fdc111))
    - refactor ([`1b4cad0`](https://github.com//Byron/gitoxide/commit/1b4cad01c6fb99e06db51415557c555ffb06b9f7))
    - refactor ([`4ce13bb`](https://github.com//Byron/gitoxide/commit/4ce13bbe65403a2a9a320fb439ae797b19921862))
    - support for decompression in case compressed bytes are stored ([`c1fcf28`](https://github.com//Byron/gitoxide/commit/c1fcf28f1069b605191652a2bd1556445e3b9833))
    - computing hashes for bases from decompressed in-memory store works ([`7c19fe6`](https://github.com//Byron/gitoxide/commit/7c19fe6aec0cdb425d77cf13349e2f7f687c63e3))
    - show that all data can be passed for processing in threads ([`a95ce9c`](https://github.com//Byron/gitoxide/commit/a95ce9c83920f29689ded1e374a224bef2d2b7cb))
    - A cache usable from threads ([`1d4879a`](https://github.com//Byron/gitoxide/commit/1d4879aee75a2c2ccbefdd48a2c2d339db38a23b))
    - re-associate CRC32 with the correctly sorted ID output ([`037e1e5`](https://github.com//Byron/gitoxide/commit/037e1e5a92c430689674e2cb7e96f9738a92fde5))
    - refactor ([`b3a365d`](https://github.com//Byron/gitoxide/commit/b3a365d179301f315d24884717c2dc09e34c3087))
    - refactor ([`97eb524`](https://github.com//Byron/gitoxide/commit/97eb524ffa4cbf04113d3a622aca3a76606f0d96))
    - Use chunked input and calculate 'optimal' chunk and thread sizes ([`0cc74d7`](https://github.com//Byron/gitoxide/commit/0cc74d7982577866c6fa6d7b0f56073979142bf0))
    - generalize chunk iterator ([`905e85e`](https://github.com//Byron/gitoxide/commit/905e85e0910650b139a845c7e7bae97a7ae5b215))
    - first rough cut of in_parallel invocation ([`8f16081`](https://github.com//Byron/gitoxide/commit/8f160810f6baf0fca5590001dd89895fccae0bbe))
    - prepare for parallelization ([`cb36596`](https://github.com//Byron/gitoxide/commit/cb36596d3059700deaf87a26df344f5dbb87f1f4))
    - Simplify indexing step ([`070899c`](https://github.com//Byron/gitoxide/commit/070899cd8cb86ac3761255ccba72225ffd6c518e))
    - resolver look ups may now turn out empty… ([`a991923`](https://github.com//Byron/gitoxide/commit/a9919230896b9129fe91b5e12dc6e0f03547b5e9))
    - Allow us to stop searching for bases early when resolving ([`e7874da`](https://github.com//Byron/gitoxide/commit/e7874dad3982829d82d3e708926e2965eca3ef4e))
    - This should be the interface for building indices from packs directly ([`f5295d0`](https://github.com//Byron/gitoxide/commit/f5295d09592753089569543b843a352fd91df201))
    - Got a good idea on how this will work! ([`7bb229f`](https://github.com//Byron/gitoxide/commit/7bb229fbb17fb8cc8251c49b681511519a9a6b9c))
    - keep track of the pack trailer ([`cdba61e`](https://github.com//Byron/gitoxide/commit/cdba61ea90a5e4f4e64ca2fe7777da540dbbf09c))
    - Now I understand why there is a separate resolution phase… ([`1c2bcbd`](https://github.com//Byron/gitoxide/commit/1c2bcbd3f510dddfc43e3f02a7987890306d8db7))
    - Fix tests ([`b9866b6`](https://github.com//Byron/gitoxide/commit/b9866b683687e210279b88b5409f01d52659f550))
    - prepare a way to gradually implement V2 index writing ([`92a4986`](https://github.com//Byron/gitoxide/commit/92a4986fcec21870abfbe8a7886fa428d5d47941))
    - refactor ([`feba75b`](https://github.com//Byron/gitoxide/commit/feba75b9f04459abc341bd2482a393a69602b054))
    - We can now restore (possibly half-written) packs ([`b1daa46`](https://github.com//Byron/gitoxide/commit/b1daa465c40ea8c7c9de69a18e467d69459d911e))
    - prepare ability to restore pack files ([`76583e5`](https://github.com//Byron/gitoxide/commit/76583e58ad8a4a4269fb857364b213ae12d4ea9b))
    - Support for pack trailer verification when iterating ([`f37f131`](https://github.com//Byron/gitoxide/commit/f37f131cf6904780147371746ff5bf56dbc21356))
    - Also read the pack trailer during iteration ([`98a8e17`](https://github.com//Byron/gitoxide/commit/98a8e17e791b6bcd92149d7ff68cbc9d9ceee087))
    - Only take as many objects as we are allowed (without 'take(…)') ([`86f5853`](https://github.com//Byron/gitoxide/commit/86f585344f968ba86a19b58129fe3bd2a058730c))
    - refactor ([`e15bde4`](https://github.com//Byron/gitoxide/commit/e15bde409cd1eae30a3a4b45624f52025144a10a))
    - Shift thin pack resolution to another work bucket; test for index writing ([`2592361`](https://github.com//Byron/gitoxide/commit/25923611663a244908198c4dc656ac73cc16c841))
    - refactor; better tests ([`12d14bf`](https://github.com//Byron/gitoxide/commit/12d14bfe2aa089723a395287c5100aad6e838935))
    - refactor ([`bd66a85`](https://github.com//Byron/gitoxide/commit/bd66a8592d3d2f5c6a7393c261f19023d14d2f37))
    - Now keeping track of read bytes works ([`d32d921`](https://github.com//Byron/gitoxide/commit/d32d9210133ab339cece3b8811958eadb8428587))
    - An attempt to intercept bytes read from bufread - FAIL ([`8db04f6`](https://github.com//Byron/gitoxide/commit/8db04f66fe4a4c5d0dba1c2a0c82723b4487f5bf))
    - refactor ([`2d817d7`](https://github.com//Byron/gitoxide/commit/2d817d7b3fcb939067b7b94fa7aeac20382effc8))
    - refactor ([`893f65b`](https://github.com//Byron/gitoxide/commit/893f65b63b424922b8cdc496a9e798acc498c1c6))
    - refactor ([`12816bc`](https://github.com//Byron/gitoxide/commit/12816bc715a0d0bad338a00c394c4cc503b20c3e))
    - refactor ([`56f763a`](https://github.com//Byron/gitoxide/commit/56f763a44538e053b4f674543720720fcc1af5d4))
    - Associate HashKind with the kind of pack ([`d66d139`](https://github.com//Byron/gitoxide/commit/d66d1391a3edee0572e07cb421527a57d90de9d9))
    - Move all pack-related file handling to bundle; big refactor ([`f8b6e75`](https://github.com//Byron/gitoxide/commit/f8b6e7524b6d73406dc6ff7b8e9c7e22322efd78))
    - first step towards putting the index file into position ([`d994c74`](https://github.com//Byron/gitoxide/commit/d994c74d7cd9c9c004bf27f0b2ac23558ce9c50d))
    - initial interface trial for writing pack index files ([`936bdcc`](https://github.com//Byron/gitoxide/commit/936bdcc29e5531026c1b0e83d9084501fc6ded9c))
    - refactor; more thorough tests ([`82d87ce`](https://github.com//Byron/gitoxide/commit/82d87ce35b1e68a07057807d28afffa7acc03b7f))
    - cargo clippy ([`b768b56`](https://github.com//Byron/gitoxide/commit/b768b56db4274b7cc313e8a6c09f3c46a48a2829))
    - At least make it configurable if to keep decompressed bytes or not ([`28ebcae`](https://github.com//Byron/gitoxide/commit/28ebcae69e95c768e4d9567ec6cc8adacd8d520b))
    - And streaming iteration works, even though we are forced to allocate… ([`27d624d`](https://github.com//Byron/gitoxide/commit/27d624d920a0ea92cf506363a505517676ced770))
    - Yes, this really cannot work: StreamingIterator ([`b4df430`](https://github.com//Byron/gitoxide/commit/b4df430b96561c63d20bb5de442582eca79768f1))
    - In the moment we tried to actually return Entry<'a>, it didn't let me :D ([`8367955`](https://github.com//Byron/gitoxide/commit/836795514f19a9d43039be228c5183061db4a404))
    - First steps towards making the InflateReader reusable ([`83a97d4`](https://github.com//Byron/gitoxide/commit/83a97d462e16d6e28151c2bf6eb7b201f4982dce))
    - Better error handling in iterator, fuse yourself ([`5ebacc4`](https://github.com//Byron/gitoxide/commit/5ebacc491a5148d31bf5ebe2746ea3d5c562b407))
    - The next() impl shows that we should be less lenient ([`4521cab`](https://github.com//Byron/gitoxide/commit/4521cab497757c34501b8eefd3b2d7d36b4df32b))
    - Provide entries which borrow from iterator ([`86eea13`](https://github.com//Byron/gitoxide/commit/86eea1326a48cf55c8a17505d2cf7c44a110a878))
    - Provide a lifetime for iterator (and possibly its entries) ([`7852bd1`](https://github.com//Byron/gitoxide/commit/7852bd193ad5659f07fc8759ca3597b037ad0255))
    - first version of expected iterated data types ([`d5e7d31`](https://github.com//Byron/gitoxide/commit/d5e7d311f38ffff0a31f85feaab692f078a75bb5))
    - improved iterator constructors ([`fb71f04`](https://github.com//Byron/gitoxide/commit/fb71f0463519c886d2e5ab30a32d546e70fb0606))
    - better handling of pack headers ([`0030bdb`](https://github.com//Byron/gitoxide/commit/0030bdbe3d476f6dac9c98f273d72666e2a9b7eb))
    - frame for a pack iterator ([`07d1096`](https://github.com//Byron/gitoxide/commit/07d109652a6ccb93d166296cd1f91babbd1ae0aa))
    - some more tests ([`9095728`](https://github.com//Byron/gitoxide/commit/9095728dff0f5ae221dcf3345e81cfb54300e03d))
    - verification for pack objects ([`17bd95e`](https://github.com//Byron/gitoxide/commit/17bd95ec43ca2814165823026fd85a776208fe21))
    - refactor ([`3ee947e`](https://github.com//Byron/gitoxide/commit/3ee947e241404cdac3225824b1434c2b270236da))
    - 'stream()' now assures all data is decompressed ([`32e994c`](https://github.com//Byron/gitoxide/commit/32e994c60f58f1be839c6dc07d819ac31f30af1d))
    - it looks like something is wrong with the object stream implementation ([`d187b5a`](https://github.com//Byron/gitoxide/commit/d187b5a769b62ec706c1265e0db8403327d8e92d))
    - Loose object verifycation - but it doesn't seem to work as expected ([`9dd5676`](https://github.com//Byron/gitoxide/commit/9dd56761ae75eac691449cd86a1be04c11c0fecb))
    - refactor ([`37cfd9b`](https://github.com//Byron/gitoxide/commit/37cfd9ba14726d6fd38b5ba6eabb3b17be263779))
    - refactor ([`8e3b9fc`](https://github.com//Byron/gitoxide/commit/8e3b9fc23a139c8307e052afa8d1d6f6f562ca1d))
    - prepare full 'verify' implementation ([`ee45c7f`](https://github.com//Byron/gitoxide/commit/ee45c7f47b95fc406cc5922a322c8fd6c0f52775))
    - refactor ([`0a33b24`](https://github.com//Byron/gitoxide/commit/0a33b24f5b61ccdf1358f1e9adcf0f6fd4099c1c))
    - Always compress values when using a sink when exploding packs ([`70562fa`](https://github.com//Byron/gitoxide/commit/70562fa123faf51bd72a4aedb12acb0d3247e4e2))
    - support for compression even when using sink ([`105c845`](https://github.com//Byron/gitoxide/commit/105c84551361bd93ec549a07ab377a7f1ae97332))
    - Another stab at fixing stress tests :) ([`7db6a33`](https://github.com//Byron/gitoxide/commit/7db6a33bc8bdaccf9091acc2ca48eb26f8a8c1fa))
    - fix stress test; improve progress messages ([`37ccd92`](https://github.com//Byron/gitoxide/commit/37ccd92bbc4eb9917c1916e39f626ecddbf85064))
    - Ignore decode errors (if configured) at the right spot ([`e53141d`](https://github.com//Byron/gitoxide/commit/e53141dd5e319d29de15ab73a783ce21158ed54a))
    - tests for relaxed error handling ([`93c0e26`](https://github.com//Byron/gitoxide/commit/93c0e2664ccc259747543845186c4211ae139008))
    - Nice error message on failure ([`adbc82c`](https://github.com//Byron/gitoxide/commit/adbc82c31450681fcb38233eeb8095efc5e52a18))
    - inform about deleted files using progress ([`a3ee516`](https://github.com//Byron/gitoxide/commit/a3ee5160093c9326006fcedbf1f507d8978a97c2))
    - Fix error display - certainly something to watch out for ([`38eff2c`](https://github.com//Byron/gitoxide/commit/38eff2c3f0bb6170a253b4c96f01077c1358bc40))
    - The first 'explode' implementation… ([`0d31ad1`](https://github.com//Byron/gitoxide/commit/0d31ad1b61997fa0d0692c5919fb8032ffaaa35b))
    - Support for skipping various safety checks during traversal ([`0416666`](https://github.com//Byron/gitoxide/commit/0416666d3492ddd031188f750371248f5f67d598))
    - prepare for configuration of safety checks ([`06638d0`](https://github.com//Byron/gitoxide/commit/06638d0f9ce50782e2897d76c742c526758889d1))
    - cargo clippy ([`95e02c9`](https://github.com//Byron/gitoxide/commit/95e02c951ace19f6ace49a9190607674d98c970d))
    - Restore original verification functionality ([`0e3c1b9`](https://github.com//Byron/gitoxide/commit/0e3c1b9bb9841ae4bb0ef1df2e72e950f7a7fd33))
    - nearly there! Interesting that anyhow errors must be sync! ([`eaee77e`](https://github.com//Byron/gitoxide/commit/eaee77ea4ce10f5c85b42a33452eef996adac3bf))
    - finally it compiles with returning Boxed errors! Ouch… ([`1fc8252`](https://github.com//Byron/gitoxide/commit/1fc8252a24b75faa88065838a3e9ffa13e6f7f54))
    - First sketch of new verify expressed in terms of traversal ([`4cb570f`](https://github.com//Byron/gitoxide/commit/4cb570f96ddd7ee2faa62e54927afd78ba7822af))
    - refactor ([`f2832a8`](https://github.com//Byron/gitoxide/commit/f2832a840d0bc69e7ee0817e3617ac0b3d40e4fd))
    - Finally a progress can be passed to the delegate… ([`a9f4de0`](https://github.com//Byron/gitoxide/commit/a9f4de0783a87b0693f87da98283e30ec72f3737))
    - refactor ([`bbb3e1e`](https://github.com//Byron/gitoxide/commit/bbb3e1efd309bbcdb3adda84308a3fc644389e43))
    - Pass all arguments (but progress) to processor ([`1e87922`](https://github.com//Byron/gitoxide/commit/1e87922299762dc0b2cf0800e1ff1e0a61467ce5))
    - Call a bare version of the traversal processor ([`95a5cea`](https://github.com//Byron/gitoxide/commit/95a5cead30fa4e7904b28158a747ac28adadf01e))
    - preparation for abstracting the 'process object (stateful)' function ([`fe400f5`](https://github.com//Byron/gitoxide/commit/fe400f572accb396def704f7853d5e81a42839de))
    - discard idea of making traversal even more generic ([`1525f36`](https://github.com//Byron/gitoxide/commit/1525f36d29574699d2fcb16b70678121030fd109))
    - Initial step towards separating verification from traversal ([`d14b4fc`](https://github.com//Byron/gitoxide/commit/d14b4fc7fd09bf1a96b16d583c1a8df102517650))
    - refactor ([`bae7781`](https://github.com//Byron/gitoxide/commit/bae7781ab549f0daa73980a29d18d64320601470))
    - rename verify-pack to pack-verify (keeping it more formal) ([`ec8c48a`](https://github.com//Byron/gitoxide/commit/ec8c48a8fcbcd748c9c764734d881b5f0217e1e4))
    - refactor ([`f580441`](https://github.com//Byron/gitoxide/commit/f5804410eb80fa406294fb83e161b09a4f3bf1a2))
    - Fast implementation for buffered input ([`c50b150`](https://github.com//Byron/gitoxide/commit/c50b150adc7c5379017237c0914c294aad1fdc7c))
    - Respect object size to be 64 bit where applicable… ([`61c8aba`](https://github.com//Byron/gitoxide/commit/61c8aba769a52d11de549505f6b4cbca1d949758))
    - better errors for writing disk objects ([`f7bc137`](https://github.com//Byron/gitoxide/commit/f7bc1372d6b445f5c078632c4f3ad7786f98e6a9))
    - Try to use HashKind where possible ([`b32e01d`](https://github.com//Byron/gitoxide/commit/b32e01dbfd257d123a461380df5dcfcb88c77e1e))
    - refactor ([`a3777ed`](https://github.com//Byron/gitoxide/commit/a3777edb2612b50de7a12da4ecbf707638d23ac3))
    - clippy happy ([`a938c70`](https://github.com//Byron/gitoxide/commit/a938c7002b4c4905694d97dc682dd77cd6780cff))
    - And writing of loose objects works ([`bbfe7bf`](https://github.com//Byron/gitoxide/commit/bbfe7bf2be3ab04dd27b9a23381ced9838fc292e))
    - This seems to be a working deflate write implementation ([`0acce38`](https://github.com//Byron/gitoxide/commit/0acce381912059c06df55955e45245c5eeb6d4b3))
    - The first succesful inflate run for small input ([`94e1c5a`](https://github.com//Byron/gitoxide/commit/94e1c5a69d22ee56d697e6e59a6a367ceb5e0c6f))
    - what seems to be a reasonable write implementation for deflate ([`45a28d2`](https://github.com//Byron/gitoxide/commit/45a28d259c2f08b8c3c96b0bf0092261d0bd17a3))
    - Another test to understand the deflate streamer better ([`4256038`](https://github.com//Byron/gitoxide/commit/4256038e65bc68222c60c1b25273a3d066991970))
    - refactor ([`dd463df`](https://github.com//Byron/gitoxide/commit/dd463df3b4a48d117596d78e050bf425db350b27))
    - refactor ([`0b42237`](https://github.com//Byron/gitoxide/commit/0b42237ead90a1582b5ddb936d30fbf75da8b6b1))
    - refactor ([`5b0bb84`](https://github.com//Byron/gitoxide/commit/5b0bb841bbc5f2e267238e4cdec69029a8344a31))
    - Put down a few tests to understand how deflate wants to be fed ([`178a018`](https://github.com//Byron/gitoxide/commit/178a01814b344c7b2ae7a7470c33808dca7e3a38))
    - refactor ([`0d8d7fe`](https://github.com//Byron/gitoxide/commit/0d8d7fee0fde7f24c91fb147745ed8474f40e834))
    - Improve looks of documentation ([`11a32eb`](https://github.com//Byron/gitoxide/commit/11a32ebc2209d1a05eb4c4ec5131e85dfb43d9f6))
    - Fix tests for now… ([`79ab945`](https://github.com//Byron/gitoxide/commit/79ab9453264562488e8c5bc6ead7dd1c1fe46cba))
    - refactor ([`0cd7bb7`](https://github.com//Byron/gitoxide/commit/0cd7bb74e379483116afb1ab618081ef1bfef67a))
    - Complete and unoptimized disk writer for objects, but… ([`9d0c3f1`](https://github.com//Byron/gitoxide/commit/9d0c3f16413d437fa893524c1cdf4a899fc3c921))
    - refactor ([`62e75bc`](https://github.com//Byron/gitoxide/commit/62e75bca7de17782dc5b7cbae29c8ce8e63b8d02))
    - Make use of HashKind in Write trait ([`0304dd0`](https://github.com//Byron/gitoxide/commit/0304dd0f44cd55af07796c3aacca0f116ffd181b))
    - Make our Sink API similar to std::io::sink() ([`a03ae0f`](https://github.com//Byron/gitoxide/commit/a03ae0f064cbf63bc4cb352ccec25333ec1843e6))
    - Finish Sink implementation ([`84f7908`](https://github.com//Byron/gitoxide/commit/84f7908b1883ed6c484ca4e522ac530c8cc161d5))
    - first steps towards serialization tests for sink ([`e8d52c6`](https://github.com//Byron/gitoxide/commit/e8d52c6997997688220959b096d46aaa641d14a1))
    - Introduce hash kind, as this should be specified when writing an object ([`f5d0acf`](https://github.com//Byron/gitoxide/commit/f5d0acf61ac5dd815bc5ece4462eb9a43dd9c44a))
    - A simple trait for writing owned objects and streams ([`68b7d7d`](https://github.com//Byron/gitoxide/commit/68b7d7defdb07b3a100bc16a9167ee957647f5cb))
    - (cargo-release) version 0.2.0 ([`76fe0ab`](https://github.com//Byron/gitoxide/commit/76fe0ab5f0b58504a5ea5adb74b349b9d588e51e))
    - (cargo-release) version 0.2.0 ([`0bb8314`](https://github.com//Byron/gitoxide/commit/0bb831480d8657e1bb29ee7009aeac673471403e))
    - Use 'optimized' chunk size for 'less-time' algorithm ([`c8c23c0`](https://github.com//Byron/gitoxide/commit/c8c23c0fb9ab0174dd33299ddd3f257f7b2dde78))
    - incorporate dynamic chunking into 'less-time' algorithm ([`295aa2f`](https://github.com//Byron/gitoxide/commit/295aa2f01dc596a8880cd2f68a8d83bc6913ce48))
    - integrate new chunk size code into lookup code ([`a8422cf`](https://github.com//Byron/gitoxide/commit/a8422cf0b0c9ff4d3275cc17a68a74811b5bd01f))
    - Simplify progress code using `inc()` ([`9e8df59`](https://github.com//Byron/gitoxide/commit/9e8df59d9a6349c49dd80447cbdbde95090e1f04))
    - Add 'inc()' convenience methods to progress ([`2e46c9b`](https://github.com//Byron/gitoxide/commit/2e46c9b72a2a5b90bcdac249de07ffbc124cfb04))
    - Run clippy first; pacify clippy ([`0a5b883`](https://github.com//Byron/gitoxide/commit/0a5b883c22f2df8a6d51f75c5e09bdfdf276fee4))
    - use faster algorithm by default ([`bb45c3d`](https://github.com//Byron/gitoxide/commit/bb45c3d8a2aabf87231981000240f0444abf6fc4))
    - Properly compute delta chain length by default ([`a93b894`](https://github.com//Byron/gitoxide/commit/a93b89464e4484bc7100d5934f14a7321f3ca7a4))
    - remove hits_to_live ([`3a3fae9`](https://github.com//Byron/gitoxide/commit/3a3fae9a8f637481d526d28a695c3f411c1a89a8))
    - attempt to auto-remove unusable deltas… ([`5dd8243`](https://github.com//Byron/gitoxide/commit/5dd8243ceafbb2a89964708f5f9b2783953677aa))
    - Now with cache (and due to that, incorrect statistics for now) ([`efd28d2`](https://github.com//Byron/gitoxide/commit/efd28d21acd97709f68ff9404131123cda527cbd))
    - Make chunk statistics independent of traversal method ([`6225f36`](https://github.com//Byron/gitoxide/commit/6225f36cc4735dd41b0c01d7c7ce6ed61f384e9a))
    - First working version of alternate object traversal, without cache ([`51b5eb6`](https://github.com//Byron/gitoxide/commit/51b5eb6c3a91e323c92e3e8f4069a12cda904354))
    - initial state for indexed lookup ([`acbcd79`](https://github.com//Byron/gitoxide/commit/acbcd79942e9783ca60ac41010a73ef98031d3e9))
    - refactor; tests now fail with more than just not-implemented ([`310a2f7`](https://github.com//Byron/gitoxide/commit/310a2f7f5498ed48777eec53b830b9f7dece33c3))
    - speedup entry sorting a little; use less memory ([`b4df372`](https://github.com//Byron/gitoxide/commit/b4df37258734e55d4679870c639f993305ada73c))
    - better index entries sorting progress ([`b4d7038`](https://github.com//Byron/gitoxide/commit/b4d7038ae729c2631277b0d5ca842a20c609abe9))
    - prepare sharing even more code ([`61c76cf`](https://github.com//Byron/gitoxide/commit/61c76cf6f856f79fd2c77e8ed9cf8940b29d6a50))
    - Make use of shared reducer in upcoming indexed verify implementation ([`290eae1`](https://github.com//Byron/gitoxide/commit/290eae115a1df277c4331bb7f2994265da117656))
    - Use shared reduce implementation in lookup based algorithm ([`10fc88d`](https://github.com//Byron/gitoxide/commit/10fc88d492821cf67de4cea9beefef4b77d4452b))
    - prepare for integration of general reducer ([`c37832e`](https://github.com//Byron/gitoxide/commit/c37832eb8a6b08cf965c287a104bfdead02776d2))
    - refactor; enable testing of reverse-delta lookup ([`512daf9`](https://github.com//Byron/gitoxide/commit/512daf94038f675353271c930694e0577ac746b4))
    - Revert "Move deallocation off into own thread" - not worth it! ([`051da15`](https://github.com//Byron/gitoxide/commit/051da1572a8ed8a99108a337f802ae5f7cc9491e))
    - Move deallocation off into own thread ([`90230f1`](https://github.com//Byron/gitoxide/commit/90230f1c0cdd1c9091a3f5e6d9393e05b6c0abb5))
    - Implement more cache-friendly pack offset v2 retrieval ([`00cf84b`](https://github.com//Byron/gitoxide/commit/00cf84baeee9932196288c8641f18621610d47a9))
    - refactor ([`3c25c67`](https://github.com//Byron/gitoxide/commit/3c25c6778b3d4fbba9906e0f5b37acbce6c69c61))
    - initial refactor of DeltaTree, but… ([`6384649`](https://github.com//Byron/gitoxide/commit/63846499367a3f106cf668cb84606ca355ad7a3d))
    - measuring performance of sorting index offsets is quite revealing ([`4b16336`](https://github.com//Byron/gitoxide/commit/4b163366cbf5b8e314e1913e24a1d19179e25611))
    - Properly handle the BufReader to make indexing work; FAST ([`57e95cf`](https://github.com//Byron/gitoxide/commit/57e95cf79c78285283be88ca9e7baf56c1ad58c0))
    - Avoid seek in favor of skimming a file read in bursts ([`01ae405`](https://github.com//Byron/gitoxide/commit/01ae4053ee57f35875d843f00d390acc19e56849))
    - Some performance information in progress ([`20aef2c`](https://github.com//Byron/gitoxide/commit/20aef2cf0e0212d5d79a6a4b7ece328adffbdf23))
    - Nodes now provide access to the pack offset ([`61c1497`](https://github.com//Byron/gitoxide/commit/61c1497547ee2789f5a90735b72b06186030c3d3))
    - Basic tree access for the entry graph ([`c5e5c77`](https://github.com//Byron/gitoxide/commit/c5e5c77aea3981d4f3b0ad528ae25eccdc58ae85))
    - Fix clippy ([`ec40e09`](https://github.com//Byron/gitoxide/commit/ec40e093d72f93d86168f39ebaca5b122ca0bec3))
    - hookup new indexing step ([`313064f`](https://github.com//Byron/gitoxide/commit/313064f1875fea6165f9d7feeb31ce0183959044))
    - frame for running the new streaming code on bigger packs ([`e0b34eb`](https://github.com//Byron/gitoxide/commit/e0b34eb87bbf29b31c87d298cdb68e6e0fa5349b))
    - refactor ([`fdfab40`](https://github.com//Byron/gitoxide/commit/fdfab408c38087c5afcdd028e988089c56311baf))
    - refactor ([`1fbeb35`](https://github.com//Byron/gitoxide/commit/1fbeb35cb1a0e66d7e12d678f351fecedc7978dd))
    - refactor ([`385e935`](https://github.com//Byron/gitoxide/commit/385e9356f49fb9e1e87f13137ee270b34527fc0b))
    - Now it works :D ([`008b4de`](https://github.com//Byron/gitoxide/commit/008b4defcfbccdd61bc7f5f2c9a8e939f817095d))
    - Initial (failing) implementation of building an index tree ([`25dc83d`](https://github.com//Byron/gitoxide/commit/25dc83d660c832cc68306395c7bd303ae806ac07))
    - Easy access to sorted offsets in pack index files ([`d93540f`](https://github.com//Byron/gitoxide/commit/d93540fe2a6d4bb70248e82d039d6a2665354ef3))
    - refactor ([`cb8d561`](https://github.com//Byron/gitoxide/commit/cb8d56101bdc4cd7e3fa95ac79f82c1cda99871c))
    - refactor ([`c7ae705`](https://github.com//Byron/gitoxide/commit/c7ae7056eb6a33656b0db31bf1c1012b7ffa2ca8))
    - refactor ([`2fc449c`](https://github.com//Byron/gitoxide/commit/2fc449cdefab1d10a446f83bd2462d1034808d97))
    - Change course and do pack streaming first ([`bcb275e`](https://github.com//Byron/gitoxide/commit/bcb275e91cfd6f0a71b3cb59a2b706b60608a594))
    - roundtrip Rust repo in stress test; accept more diverse trees when parsing ([`0347cdb`](https://github.com//Byron/gitoxide/commit/0347cdbf473d80c016745ffbaf582832fe2eba2a))
    - Allow some very special trees not to be round-trippable ([`8fe1358`](https://github.com//Byron/gitoxide/commit/8fe1358aa9375bbe63f1ee64174b9e663d140a05))
    - Consume PGP signature in tags fully ([`ffd6c31`](https://github.com//Byron/gitoxide/commit/ffd6c31aa3adecc2dea6357373d88a495d63ba0d))
    - make tagger signature optional ([`3358f9a`](https://github.com//Byron/gitoxide/commit/3358f9ae539c7b7878d87a209d678d2f08f94b1b))
    - remove now unused pgp_signature field - it's in extra-headers ([`c8c937c`](https://github.com//Byron/gitoxide/commit/c8c937c505e455572544a1a9da1b991ef4662b97))
    - proper support for extra-headers ([`d0feb2b`](https://github.com//Byron/gitoxide/commit/d0feb2b5b30f9719bf3b40ac5b74f8a5a8515bc9))
    - Switch to latest quick-error ([`9760856`](https://github.com//Byron/gitoxide/commit/976085614ee13a19fc1347209259a3dcf36ef95b))
    - Fully implement --encode and --re-encode flags ([`a7cfac8`](https://github.com//Byron/gitoxide/commit/a7cfac83ddd859d9c2c25e457c0d7043738792dc))
    - refactor ([`56b66ac`](https://github.com//Byron/gitoxide/commit/56b66ac069f24635a8fa74b4b2231dfb0a82a1ef))
    - prepare for re-encoding each pack object ([`afae684`](https://github.com//Byron/gitoxide/commit/afae684c72e5dc4b718976056dd5d34ed61de72a))
    - fix build with rustc 1.45 ([`8c2a1ee`](https://github.com//Byron/gitoxide/commit/8c2a1ee853c5354117fc0a1b6719108785633915))
    - refactor ([`ec5e50f`](https://github.com//Byron/gitoxide/commit/ec5e50f607d59302d6db3944f6ea7b667f820927))
    - prepare for writing out owned trees ([`2b6eced`](https://github.com//Byron/gitoxide/commit/2b6eced325057a884d56ed9db755a8699cbf8d00))
    - Use borrowed::Id in trees for full type safety ([`5d57c1f`](https://github.com//Byron/gitoxide/commit/5d57c1f7e3b9a84f7b46a4378015572155f3104b))
    - refactor ([`f7b8826`](https://github.com//Byron/gitoxide/commit/f7b8826ba144f54f3a3fe6096a5daafd29e25002))
    - fix odb test ([`a792f44`](https://github.com//Byron/gitoxide/commit/a792f44fec60d63aaa16538bf06fb29277e78433))
    - Prepare for allowing an owned, processed version of multi-line headers ([`f966e7f`](https://github.com//Byron/gitoxide/commit/f966e7f26cbbe99e5508215adaacf073e108bf48))
    - Use borrowed::Id everywhere ([`9f876f0`](https://github.com//Byron/gitoxide/commit/9f876f04feaa3fd3bba9729fff7667708dc0c4be))
    - move git_object::Id into git_object::owned::Id - much better already! ([`50c7136`](https://github.com//Byron/gitoxide/commit/50c71368a69f57b0a43061df105685e992ed384a))
    - basic integration of borrowed Id; translate between owned and borrowed ([`84ff638`](https://github.com//Byron/gitoxide/commit/84ff638a183567593ace8056de2a856304d29d1d))
    - prepare to allow Id be owned and borrwed; abstract over hash type ([`d883c31`](https://github.com//Byron/gitoxide/commit/d883c31dd14f253a3af153616007c9231fdf265a))
    - introduce the notion of IdRef ([`7007361`](https://github.com//Byron/gitoxide/commit/700736197b903cb6fe9ed60718e49e4be44199a7))
    - Use statically known borrowed arrays for perfect type safety! ([`3ead048`](https://github.com//Byron/gitoxide/commit/3ead048bb999e6266831df2ca6c2022013529ab2))
    - refactor ([`766f3e4`](https://github.com//Byron/gitoxide/commit/766f3e491dc6ebcca20753cda3487545268721eb))
    - refactor ([`bca1f16`](https://github.com//Byron/gitoxide/commit/bca1f16a6f3da497e3488e333d5ebc99e39ee689))
    - 'data -> 'a as it's shorter and also more idiomatic ([`71821e9`](https://github.com//Byron/gitoxide/commit/71821e938887f448f1458642eda2bac365f2aa85))
    - refactor ([`dedd4dc`](https://github.com//Byron/gitoxide/commit/dedd4dc91c26dfef368307345bb9e8d49637207c))
    - refactor ([`de0bc3c`](https://github.com//Byron/gitoxide/commit/de0bc3cb4a32bf4cd02ce7c8420bc008e469b779))
    - refactor ([`e5391d3`](https://github.com//Byron/gitoxide/commit/e5391d36d192c3c12426102f734d2e227c568a08))
    - refactor ([`163909b`](https://github.com//Byron/gitoxide/commit/163909b593e21860d0a292c6e45daee93fb270fb))
    - refactor ([`49f64db`](https://github.com//Byron/gitoxide/commit/49f64db88fc0643e0bff215efdb9b1b429b648ba))
    - refactor ([`9f825b8`](https://github.com//Byron/gitoxide/commit/9f825b849d14494a2d58a09eb6499fd86fd05af3))
    - refactor ([`2fbc2e1`](https://github.com//Byron/gitoxide/commit/2fbc2e1f76f972758b0c880d3eabdf75586749e7))
    - fix naming change, which was introduced accidentally ([`fbb9f98`](https://github.com//Byron/gitoxide/commit/fbb9f98508ec722e192466e28ded47aef2fb78b3))
    - make it easier to validate bundles, for completeness ([`8ea05de`](https://github.com//Byron/gitoxide/commit/8ea05de8d1ae49e09465a66354cf69dd7c7a2e05))
    - refactor ([`34e85f2`](https://github.com//Byron/gitoxide/commit/34e85f2242b12ec1560b8e50bc9ab447cd1805fc))
    - refactor ([`b3bde87`](https://github.com//Byron/gitoxide/commit/b3bde870054cb4001c1e2ea8a81b1a4b1d83405b))
    - refactor ([`0b540c2`](https://github.com//Byron/gitoxide/commit/0b540c236b3459b340f13908ca52c82e40378e13))
    - refactor ([`2888f1b`](https://github.com//Byron/gitoxide/commit/2888f1b10a2baf40155544e667ddd461f3ddc938))
    - refactor ([`0817b24`](https://github.com//Byron/gitoxide/commit/0817b24fae6106db2d9e3fcfcdcb10b9a182911d))
    - refactor ([`dcacd3b`](https://github.com//Byron/gitoxide/commit/dcacd3b06d7a4532c600dfdf62e03561e8ed55ef))
    - refactor ([`b113da9`](https://github.com//Byron/gitoxide/commit/b113da945715f9611eb0fb79925d1239eaf1569c))
    - refactor ([`6659174`](https://github.com//Byron/gitoxide/commit/66591745f08d15f3756a352f4041c807ea92fc6f))
    - refactor ([`bed5dc8`](https://github.com//Byron/gitoxide/commit/bed5dc80c5b307c6d35f7b4405693dce1f7f6d71))
    - refactor ([`4867740`](https://github.com//Byron/gitoxide/commit/486774096a86f6eb001d812e6ac9ab0b29791148))
    - refactor ([`f6cc80e`](https://github.com//Byron/gitoxide/commit/f6cc80e5f0ae966c83345be64219fa7ebe0e1db2))
    - refactor ([`8b416d4`](https://github.com//Byron/gitoxide/commit/8b416d4b8417c04ea5d3527a88190d867dc8b7c2))
    - refactor ([`23e05d7`](https://github.com//Byron/gitoxide/commit/23e05d78c73b2bfce3025b3e34746d48026b34ed))
    - refactor ([`d3b36f4`](https://github.com//Byron/gitoxide/commit/d3b36f4ad8a5cb9266542ee997941c879121be96))
    - More tests for various object types ([`f4703e0`](https://github.com//Byron/gitoxide/commit/f4703e047834d13748f21db861fd0a753d5b1233))
    - refactor ([`86fa00f`](https://github.com//Byron/gitoxide/commit/86fa00f0967dba5453f7226125123ef398e48790))
    - Basic decode implementation ([`7ff02cb`](https://github.com//Byron/gitoxide/commit/7ff02cb84469f5aa4a3be1489927344b45385a45))
    - Support for in-pack object lookup in Bundle::locate ([`7e3d6be`](https://github.com//Byron/gitoxide/commit/7e3d6be5136d9c3816bedd3b9797186457aeb476))
    - First dummy implementation of borrowing a buffer provided by the user ([`9c31fcb`](https://github.com//Byron/gitoxide/commit/9c31fcb7c25be5c75e3dad1e940683b8ae42b935))
    - Make it easy to learn that objects couldn't be located by using options ([`a916f36`](https://github.com//Byron/gitoxide/commit/a916f367d329927369b127a5f2fba63e8d4d9d88))
    - mild refactor - need combined pack + index ([`6bf8ed4`](https://github.com//Byron/gitoxide/commit/6bf8ed470803ab58737b119d892b7eabb77fd8b9))
    - Respect thread limit in 'in_parallel' ([`babfd84`](https://github.com//Byron/gitoxide/commit/babfd84cba77ef7a0f541ba921b31ebd3f3c50e3))
    - apply cargo diet ([`79b9b73`](https://github.com//Byron/gitoxide/commit/79b9b7398be608de1f439f56b057dc08d421081f))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

## v0.1.0 (2020-07-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 167 commits contributed to the release over the course of 90 calendar days.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - add missing license description ([`2b80181`](https://github.com//Byron/gitoxide/commit/2b80181ad428a9bf267a9660886f347a850fc76f))
    - Make crates publishable ([`5688a34`](https://github.com//Byron/gitoxide/commit/5688a3427ff3673e1422d43106f4d685fa837aed))
    - cargo clippy (from CI) ([`0a28857`](https://github.com//Byron/gitoxide/commit/0a288579545345c0dffdfa814b052959baec0a34))
    - Proper implementation of line renderer into 'lean' CLI ([`e98e7c2`](https://github.com//Byron/gitoxide/commit/e98e7c280d73e9d9ebd13202afb93a56cb2f7c9c))
    - Handle windows newlines in test suite for packs as well. ([`ebd5176`](https://github.com//Byron/gitoxide/commit/ebd517633f099582dc2633e71f7bb7890acd14d1))
    - Add metadata to allow docs.rs build all featueres ([`10f9386`](https://github.com//Byron/gitoxide/commit/10f9386a12decc1f13999aee72be484c8f6d48ce))
    - update tasks ([`269280a`](https://github.com//Byron/gitoxide/commit/269280a0e00c9eee54d591b96c3ed0e9d4202489))
    - Allow to limit the logging depth for less cluttered output ([`fce7035`](https://github.com//Byron/gitoxide/commit/fce703531d7006f7d961d6ffa66f51f6c9bc0efc))
    - Looks like this performs much better already, but… ideally subprogress isn't shown ([`3b96d18`](https://github.com//Byron/gitoxide/commit/3b96d18483a845f7692f94cc40c28871fd96e479))
    - finally speed up logging progress properly - needs input throttling ([`1a550c6`](https://github.com//Byron/gitoxide/commit/1a550c6458b10fad2e42b641899216c5517c6e26))
    - provide average throughput per second ([`5b23d17`](https://github.com//Byron/gitoxide/commit/5b23d171102ad859258b9673bf35561ef9e8f246))
    - git-odb with serde support ([`0da930c`](https://github.com//Byron/gitoxide/commit/0da930cf23f215cc1e2bda8f7340a5d69370735a))
    - Remove dependency to git-object from git-features - it better remains free ([`67c3a6a`](https://github.com//Byron/gitoxide/commit/67c3a6ab4cc32358a1406c2f863e26a4c2929867))
    - commit to using bstr whenever something is not data bytes; remove miniserde ([`3183d1b`](https://github.com//Byron/gitoxide/commit/3183d1b02c2d7bb3c750f8472c29bb161641ca7f))
    - Prepare centralization of bstr as optional component ([`aa857d9`](https://github.com//Byron/gitoxide/commit/aa857d9df32dfc75f151154ca430ddfee907deed))
    - \#[forbid(unsafe)] for all crates ([`afda803`](https://github.com//Byron/gitoxide/commit/afda8039259b7a30cfed5dbcdd9caf4773b4c234))
    - Allow for more screen space when formatting ([`6794300`](https://github.com//Byron/gitoxide/commit/67943002e7f4215b5383bd0538786ce2857f011e))
    - prepare next task ([`74bcbb5`](https://github.com//Byron/gitoxide/commit/74bcbb506585aa9e0955253d07bab111f83f014e))
    - display object throughput per second, even though it won't be visible in TUI… ([`53b4513`](https://github.com//Byron/gitoxide/commit/53b4513f6a8bb2f2e5b07fa72a3085e620cee24c))
    - disable LRU cache if we have to get statistics ([`befba3b`](https://github.com//Byron/gitoxide/commit/befba3b769195fb592d714afe12194a61ae4a330))
    - wonderful statistics on compression efficiency! ([`1bb09c5`](https://github.com//Byron/gitoxide/commit/1bb09c509dae4e493ab05022bbf51c0b1786d479))
    - count objects per chain level ([`209d53f`](https://github.com//Byron/gitoxide/commit/209d53f531ec9bcffbb04ba060447bee59ad26f6))
    - pass average stats through to the top level ([`5b4979c`](https://github.com//Byron/gitoxide/commit/5b4979c1dfeb9a29974dd4e6529ae5da074d0b1a))
    - refactor ([`4dd9fd4`](https://github.com//Byron/gitoxide/commit/4dd9fd4a2c48380bda9a865ef704e7fdfa7e5b89))
    - closer to actually producing statistics ([`5f087ec`](https://github.com//Byron/gitoxide/commit/5f087ec30a50775ad8bb67f21e352fe9ee1ccc9f))
    - refactor ([`7add82c`](https://github.com//Byron/gitoxide/commit/7add82c39169e3c2fff76c48cdd318fe6040d7bc))
    - Also average statistics on chunk level ([`3b927e5`](https://github.com//Byron/gitoxide/commit/3b927e50173e3feae72cde8a226cee524275403a))
    - Provide more detailed information when decoding an entry ([`80c5da9`](https://github.com//Byron/gitoxide/commit/80c5da9bd88e1f329292f3f93ba53c8ff8324a20))
    - no need to say 'begin' before doing something, it's primarily for logging ([`13eba3a`](https://github.com//Byron/gitoxide/commit/13eba3a3484068939436996352fe5585aa221bca))
    - throughput for pack ([`81f5c33`](https://github.com//Byron/gitoxide/commit/81f5c335b224dd85062c9208cee2bb288ad3e833))
    - print performance stats at the end of hashing the index ([`9c94417`](https://github.com//Byron/gitoxide/commit/9c9441709c9a759a3a0916402921c7beeb735d75))
    - assure hashing progress is dropped when done ([`db6e067`](https://github.com//Byron/gitoxide/commit/db6e067c5dd90311d174881546d8df8f521eb552))
    - First implementation of logging per thread ([`477dd90`](https://github.com//Byron/gitoxide/commit/477dd90ce5e102875b19489bf8ae9877522ef9c8))
    - Support for providing progress to threads ([`2815858`](https://github.com//Byron/gitoxide/commit/2815858adf7ac0f7b4cbc88cf05df0ea6aef4116))
    - properly count objects ([`d398e7e`](https://github.com//Byron/gitoxide/commit/d398e7e68ad893d21a088ec6ac727dc8577317fc))
    - first very basic progress implementation ([`b820717`](https://github.com//Byron/gitoxide/commit/b8207177daee8a9ffa23c7c052cf9ca651b15804))
    - Pass progress everywhere, for now just to discard it ([`da3ae1c`](https://github.com//Byron/gitoxide/commit/da3ae1c82cd726b8fae9b8d26069719930e9ba99))
    - Control which hashing crates to use from the top-level as well. ([`dfe9b20`](https://github.com//Byron/gitoxide/commit/dfe9b203b2e877a7e345b4f2942bf5a1582ab43e))
    - Use git-features to toggle 'parallel' mode from the 'gitoxide' level ([`d944fbf`](https://github.com//Byron/gitoxide/commit/d944fbf181acc5fb83a841613174702af1e074d6))
    - sketch out `Progress` trait; don't forget to write docs at some point ([`534b3c7`](https://github.com//Byron/gitoxide/commit/534b3c73101fd1b885de630523ab706bd06a327b))
    - refactor ([`baeb4ef`](https://github.com//Byron/gitoxide/commit/baeb4ef1a9680c212ce9d1010e2c34eedafcd246))
    - refactor ([`e12bfd6`](https://github.com//Byron/gitoxide/commit/e12bfd645bb2f707a1b5077190d9f37393a8e315))
    - Make `in_parallel` trait bound more loose: Clone instead of copy ([`3e91b05`](https://github.com//Byron/gitoxide/commit/3e91b0512919c02899324564b8f571ce534955d9))
    - Using all cores actually does speed things up ([`ed944b9`](https://github.com//Byron/gitoxide/commit/ed944b9480ae647c7e75a7a07a9c59885725b3a0))
    - Also run index+pack validation in parallel; only parallelize bigger packs ([`dc15b26`](https://github.com//Byron/gitoxide/commit/dc15b2652cc6b9e94f80aebfeec8f879ae5a529f))
    - avoid running anything in parallel for small packs ([`c2df183`](https://github.com//Byron/gitoxide/commit/c2df183943e8b533c4cd5f5833f61ad94942943d))
    - Don't send every single entry, instead send reasonably sized chunks ([`56298a6`](https://github.com//Byron/gitoxide/commit/56298a62ea8cc9c6fef7f682ffb8ddda5404ca9b))
    - refactor (down to 6 minutes for big pack verification) ([`4157b51`](https://github.com//Byron/gitoxide/commit/4157b5196936e9f5f884a645f7e1c37ba6b13b52))
    - first working version of actually parallel `in_parallel` ([`145ee39`](https://github.com//Byron/gitoxide/commit/145ee399e2c057aec3330e26bafb7910ca7dc56d))
    - first implementation of 'parallel' without threads. How will scoped fare? ([`735744e`](https://github.com//Byron/gitoxide/commit/735744e1960a3055b836767c85613ba9d147cdd4))
    - A sketch of a minimal helper for parallel work ([`377252a`](https://github.com//Byron/gitoxide/commit/377252a3b4869952059e832ce32656e2cf2a674c))
    - refactor ([`be4795f`](https://github.com//Byron/gitoxide/commit/be4795f00d7b693cb52f93857ac3b4b65340053f))
    - refactor ([`3e2efff`](https://github.com//Byron/gitoxide/commit/3e2efffc945a0737c2d8b820a93b013e6ffa45e2))
    - bigger LRU caches are better, but with this one we can't go too large ([`5e1f7ae`](https://github.com//Byron/gitoxide/commit/5e1f7aedc970552d3ec4ab3358757af790ce6628))
    - First implementation of an LRU cache - it gets hit, let's see how it fares! ([`5a21031`](https://github.com//Byron/gitoxide/commit/5a21031415a6e2ca43cb828492fd5517d2a98e9e))
    - also set the cache with bases and deltas ([`915a3fb`](https://github.com//Byron/gitoxide/commit/915a3fb21c950dd35a97a735375a144bbc59e3b1))
    - first sketch of cache implementation - get() is there, next is put() ([`ce54756`](https://github.com//Byron/gitoxide/commit/ce547565de23e89212bf6197178191ddf5b11fd3))
    - Allow delta base resolution to fail (similar to how lookups can fail) ([`b721424`](https://github.com//Byron/gitoxide/commit/b7214241dfbb85c3115e230fa502f790133e2192))
    - Allow in-pack lookups for V1 packs ([`2e51bbb`](https://github.com//Byron/gitoxide/commit/2e51bbbab4a47001ef725d0bf8bf5714d0c37e70))
    - Add CRC32 reading at index ([`268f855`](https://github.com//Byron/gitoxide/commit/268f855da9db5f694bedb073493778147d646271))
    - Pack offset by index ([`69e35b1`](https://github.com//Byron/gitoxide/commit/69e35b1d8f24f366d675484a1bddbebd37b72e22))
    - V2 pack lookup ([`9e56902`](https://github.com//Byron/gitoxide/commit/9e56902bdb7702181809c6a4c2280750ddd64044))
    - test V1 lookup ([`e9c7127`](https://github.com//Byron/gitoxide/commit/e9c71271fa51d5420fcb205d2d3deb6d012f0d41))
    - Add CRC32 check during pack verification ([`04ff1a0`](https://github.com//Byron/gitoxide/commit/04ff1a0bf9aa164e9cff262ec521eab76c2e4688))
    - prepare for CRC32 check - needs understanding of size of bytes in packed object ([`3ab2df1`](https://github.com//Byron/gitoxide/commit/3ab2df1e00eb41e8a222b208131f63ba3e065df5))
    - refactor ([`dd2d623`](https://github.com//Byron/gitoxide/commit/dd2d6238771ff86df6a412a6d817aa92a5e5ed43))
    - Finally delta-objects can be read as expected. ([`81f2f54`](https://github.com//Byron/gitoxide/commit/81f2f547bad33c414f6e12d16df4922274b06758))
    - definitely an improvement to the way add-deltas are applied… ([`c6cdb12`](https://github.com//Byron/gitoxide/commit/c6cdb12b47f6d5f4e3f02895acb2de08a7df00cc))
    - Fix one issue with Trees being declared as tags ([`ada66cd`](https://github.com//Byron/gitoxide/commit/ada66cdbab0fec4765428ce815c0868d34d5babf))
    - validate sha1 of pack objects, some work, some don't for some reason… ([`aa8799a`](https://github.com//Byron/gitoxide/commit/aa8799a01b92c3c3b7d4347f745921bbb685c454))
    - Capability to write loose object headers, fast ([`de0aeff`](https://github.com//Byron/gitoxide/commit/de0aeff518ebd218b73bf472558f278f6bcdc17c))
    - refactor ([`5364bbe`](https://github.com//Byron/gitoxide/commit/5364bbe0415c37f684066e22eb017fe5d7ca7c64))
    - Fix another implicit assumption that doesn't hold: deltas are NOT… ([`093637d`](https://github.com//Byron/gitoxide/commit/093637da964b807fa767009732e9b93002e35fab))
    - Finish delta-application to take into account the biggest possible result… ([`0ee2b69`](https://github.com//Byron/gitoxide/commit/0ee2b696014012864b0645bd1b9da508cb1e465c))
    - first stab at dealing with bigger-than-expected intermediate result sizes… ([`8027ff4`](https://github.com//Byron/gitoxide/commit/8027ff4de7ffe6126cf1ade4938baa08899cb938))
    - First simple implementation of fetching all objects in a pack (without validation) ([`053045b`](https://github.com//Byron/gitoxide/commit/053045bb23e2a85e2a1d16eeb65c399dfabba5b4))
    - Support for verifying pack files and index files ([`b09b4e1`](https://github.com//Byron/gitoxide/commit/b09b4e1f35c3802dfd3418bda42b96828acd9ec8))
    - simple index file verification (internal) ([`1d27050`](https://github.com//Byron/gitoxide/commit/1d27050f21ee1c8f492d38e14c294fa31a7b48a1))
    - refactor ([`4023b02`](https://github.com//Byron/gitoxide/commit/4023b0260b0b139853f8dc1b9260045a8dac6e47))
    - refactor ([`855a769`](https://github.com//Byron/gitoxide/commit/855a769026f81739f28b38507c0bef7b59e97a8b))
    - refact[r ([`c84410b`](https://github.com//Byron/gitoxide/commit/c84410b2b0e66c10c30fc70c3674c971b270204d))
    - refactor ([`c24c79d`](https://github.com//Byron/gitoxide/commit/c24c79d65b947625a5a9ab73dbd3afdef060fa12))
    - test --no-default-features for git-odb ([`2394bca`](https://github.com//Byron/gitoxide/commit/2394bca4a76247c420fe06c59d0d76819c6e978b))
    - refactor; prevent trailing bytes to become part of the digets ([`043813c`](https://github.com//Byron/gitoxide/commit/043813cd2e49b358e17ad78d975ef255924c78fa))
    - try a version that doesn't rely on memory mapped files for throughput… ([`d59ddfc`](https://github.com//Byron/gitoxide/commit/d59ddfcf50edd0bfc8252e6c7a68c86fe27b5a9f))
    - try to speed it up with prefetching - not really :D ([`8485185`](https://github.com//Byron/gitoxide/commit/8485185bcb7895461dc4347f25b9f0b0bab54594))
    - simplify folder names ([`36fde1f`](https://github.com//Byron/gitoxide/commit/36fde1f90e9034060b5ede8a923365474659085e))
    - Fix LSB parsing code with python based code written 6 years ago :D ([`c12fdad`](https://github.com//Byron/gitoxide/commit/c12fdadbf839ce6f8a638fe25667d870a8f6b808))
    - improved packed header parsing… it works a little better now it seems, but… ([`ca779ed`](https://github.com//Byron/gitoxide/commit/ca779edc457f1f1baed05e8c64bb2994f6b12945))
    - refactor; and figured out what the header parsing issue is ([`d364049`](https://github.com//Byron/gitoxide/commit/d3640493e509b782589b4c0680962e6e1f2ae665))
    - some more tests ([`85e541f`](https://github.com//Byron/gitoxide/commit/85e541f36fd7795c53d0dc3d07d5b76a6725c889))
    - refactor; better error handling ([`031df11`](https://github.com//Byron/gitoxide/commit/031df11a3c3767330c9f13cab0e55c2559a72e9b))
    - first very rough version of full-object decompression without allocation ([`7c704a7`](https://github.com//Byron/gitoxide/commit/7c704a71e51607149a7a6a1293a401f4c7ecb610))
    - refactor ([`dcb1997`](https://github.com//Byron/gitoxide/commit/dcb19971841d3330df63c67f73793f0a45b6c74f))
    - refactor ([`baaf06e`](https://github.com//Byron/gitoxide/commit/baaf06e36605f9b79ef09dd7cbdbb42fb16b64be))
    - refactor ([`3edaaec`](https://github.com//Byron/gitoxide/commit/3edaaec2fad6594049a0f10a4bf921dc3c485ac0))
    - Finish Object Reader implementation, now for in-memory objects, too ([`35e69b8`](https://github.com//Byron/gitoxide/commit/35e69b87521eef89705012a7170517670ee20e7c))
    - a simpler implementation to skip the header ([`47ca6ab`](https://github.com//Byron/gitoxide/commit/47ca6ab2ff0cbf8801d0a82cebbbeb8c4f62cdae))
    - Allow skipping the header when decompressing files (streaming) ([`ff35032`](https://github.com//Byron/gitoxide/commit/ff350323e4a424df8c17a9dca53cc8967e45e960))
    - first step towards supporting skipping the header in the stream ([`8e45f53`](https://github.com//Byron/gitoxide/commit/8e45f5370516b0df9df4e984d29161d399697fdd))
    - Fix stream decoding - it seems to work, but we need to deal with the header ([`f10ed75`](https://github.com//Byron/gitoxide/commit/f10ed75a74c183edeb2a5bd665e5649a5b282e93))
    - tests for streamed reading of bigger objects (FAIL) ([`b4a6b72`](https://github.com//Byron/gitoxide/commit/b4a6b7233ff4f4154d1dd46a29a88787746899f8))
    - refactor ([`80aad4b`](https://github.com//Byron/gitoxide/commit/80aad4b97b76b26050c87eac483b8af1fcfb61ed))
    - Add missing parts to implement Read, need refactoring to make it work though ([`13d4cdb`](https://github.com//Byron/gitoxide/commit/13d4cdb32fe197d1517270183d9547ddf1aa381e))
    - First step towards streaming of ZLIB deflated content ([`a870f7a`](https://github.com//Byron/gitoxide/commit/a870f7a5bca9f57374e7c9582866473fbbce6e5e))
    - cleanup ([`a2f0a5d`](https://github.com//Byron/gitoxide/commit/a2f0a5dec0b183712e03397e8b4340fed77ce008))
    - fix clippy ([`a9c5da7`](https://github.com//Byron/gitoxide/commit/a9c5da7132eeaa6806b8190985a7aa25f9ef89d8))
    - Make decompression of bigger objects work (on the fly) ([`7e4f5a9`](https://github.com//Byron/gitoxide/commit/7e4f5a9594b31c67a49a1d2d6a063241ab8821d9))
    - It becomes obvious that this way of decompressing things won't work ([`1818bda`](https://github.com//Byron/gitoxide/commit/1818bda0acc83354b093c39831e2844d48eb5637))
    - Don't do so much logic if we already decompressed everything ([`26cb36c`](https://github.com//Byron/gitoxide/commit/26cb36ce3717a63ca7934e7bbc35052208227056))
    - refactor ([`423b885`](https://github.com//Byron/gitoxide/commit/423b8857f1dc580d64ec4075f955d34524979269))
    - more convenient access to our four object types ([`ecda6d2`](https://github.com//Byron/gitoxide/commit/ecda6d23561dc176f7d7ad2565da8105efac614f))
    - It's proably OK to make parsed pack entries avaialble, why not ([`8a64e10`](https://github.com//Byron/gitoxide/commit/8a64e10ae5206e10f487fbde88412037c165e583))
    - refactor ([`13f0e77`](https://github.com//Byron/gitoxide/commit/13f0e77c0d67f8078bfaf96c3bb735f8c3161a3f))
    - Memory size checks for objects ([`ab51616`](https://github.com//Byron/gitoxide/commit/ab51616bb250a62b5367e861c25c1d90ec60f720))
    - Reduce loose Object memory footprint ([`38a81b0`](https://github.com//Byron/gitoxide/commit/38a81b0fc3ef1bff54779f0cf531ea2e0f82ebd8))
    - first Blob test for blobs that are already in memory ([`f503324`](https://github.com//Byron/gitoxide/commit/f503324b33fd7289782fe642b1f566e9d101ceab))
    - Make single-field objects blob and tree more explicit ([`1aef68f`](https://github.com//Byron/gitoxide/commit/1aef68f7e979324eb94966d44c160ffe537ee4a8))
    - add Blob type to parsed objects ([`d3e8e4b`](https://github.com//Byron/gitoxide/commit/d3e8e4b24ecda84665b994ccad768774efdcdc90))
    - See 'parsed' blobs as in-memory representations… ([`6a6e105`](https://github.com//Byron/gitoxide/commit/6a6e105b3e438380b55f9e9566f0acd76c5efffd))
    - Make clear that not all objects can be parsed at the expense of convenience ([`ce3031d`](https://github.com//Byron/gitoxide/commit/ce3031da8ba1eb3e66d72474a8efc65c2990bc99))
    - don't conflate errors with 'there is no suitable object' to parse ([`b9b796f`](https://github.com//Byron/gitoxide/commit/b9b796f69ced726167d72615e5628263a3158a35))
    - fix imports ([`10f2967`](https://github.com//Byron/gitoxide/commit/10f29675442c76b38e0a8deb757930a13af3a3bb))
    - try pub use with rename. Not bad in the docs, but maybe a bit confusing ([`526f3f8`](https://github.com//Byron/gitoxide/commit/526f3f8d3ca9fe9672b0518f1bc3b921f695c0d8))
    - refactor ([`b9a1647`](https://github.com//Byron/gitoxide/commit/b9a16473ed028abc59fc5126db9530f2107498d8))
    - Integrate Commit object into Loose DB ([`7e9fe50`](https://github.com//Byron/gitoxide/commit/7e9fe505f08def0378c967514a9389da9e46301d))
    - test for parsing trees from loose dbs ([`4f48249`](https://github.com//Byron/gitoxide/commit/4f4824971d62d165fd4c2bea869fd88986dc259f))
    - refactor ([`9f9ccad`](https://github.com//Byron/gitoxide/commit/9f9ccad37fea96954a2df9e314b6c154466dc0ca))
    - refactor ([`427c480`](https://github.com//Byron/gitoxide/commit/427c48007016e95b13d8750df8b6ac1620f465ac))
    - refactor loose db ([`6ea4f53`](https://github.com//Byron/gitoxide/commit/6ea4f5331f8d4279025e3f912315af50f0eedbdc))
    - handle commits without newlines; make tag newlines optional ([`c0b54be`](https://github.com//Byron/gitoxide/commit/c0b54bef5a2bcfce9b6deb90cdd27c7e0cc85810))
    - Make Commit available in borrowed object ([`b2d1b5d`](https://github.com//Byron/gitoxide/commit/b2d1b5d684bdfda5f922b466cc13d4ce2d635cf8))
    - avoid unnecessary allocation when creating SHA1 paths in loose ODB ([`09d8d3a`](https://github.com//Byron/gitoxide/commit/09d8d3a12e161a7f6afb522dbe8900a9c09bce06))
    - first silly attempt to randomly remove an allocation ([`4ff2168`](https://github.com//Byron/gitoxide/commit/4ff21686c32a6edc84ea041c3040f33ae24f9519))
    - document existing use of unsafe, deny everywhere else ([`41f4bce`](https://github.com//Byron/gitoxide/commit/41f4bce9d9a492f8e20a6eb5b3eaf5adc6d78329))
    - cleanup integer parsing in loose object database ([`ecdce1a`](https://github.com//Byron/gitoxide/commit/ecdce1a05d8c732afd53c6da6067bf591f96fa6a))
    - the defining property is actually that the object is borrowing data ([`e0125fd`](https://github.com//Byron/gitoxide/commit/e0125fdb0a41ed139364084f6d679932f08b7b4f))
    - fix cargo fmt ([`642dd13`](https://github.com//Byron/gitoxide/commit/642dd13afa77ea9c0f4a20d59f54b84bf6ca3333))
    - cleanup; all tests work! ([`7c96603`](https://github.com//Byron/gitoxide/commit/7c9660354484869681356a8c4ef8057313e864f2))
    - first version of tag message parsing - it's actually changed now ([`74b2328`](https://github.com//Byron/gitoxide/commit/74b2328fcbbcffab9981c23e903c4f4c5d085aff))
    - remove itertools in favor of vendoring the little code we need ([`8340508`](https://github.com//Byron/gitoxide/commit/834050878b43bae677287767332adc746a8aa2ed))
    - optimize macro usage ([`0c9960b`](https://github.com//Byron/gitoxide/commit/0c9960b1a9404ec8db62ffeeedb3e482eba81c77))
    - optimize dependencies ([`3ea2195`](https://github.com//Byron/gitoxide/commit/3ea2195090728f17ae425e4816405f10b7eb8a14))
    - Use git-object in git-odb ([`07f7c31`](https://github.com//Byron/gitoxide/commit/07f7c318d55603e3731f08cb04d3da8ac2fcfea6))
    - Add the latest nom, hoping it will be come out of alpha… ([`85958f1`](https://github.com//Byron/gitoxide/commit/85958f1771b521f905528ca426404b846244e122))
    - refactor; use pretty-assertions for massively more readable test-failures ([`ea8d311`](https://github.com//Byron/gitoxide/commit/ea8d3113c32fff85c02d8ff2217adc6b42153137))
    - Switch everything parsed to BStr ([`62ae90a`](https://github.com//Byron/gitoxide/commit/62ae90a37d0dea33a23eb7d026cdf9b719692078))
    - refactor ([`9a86f63`](https://github.com//Byron/gitoxide/commit/9a86f6352ccd5178198ad87df44d88358b475d1a))
    - Use btoi to parse all integers, directly from ascii-bytes ([`4f6ef42`](https://github.com//Byron/gitoxide/commit/4f6ef42a0b871096f81bd0cb9759aa651a1943d0))
    - refactor ([`2990902`](https://github.com//Byron/gitoxide/commit/299090296fb3a2074c74289c4645b79d3f736ed0))
    - move parsing tests close to actual parsing ([`3ca2c59`](https://github.com//Byron/gitoxide/commit/3ca2c592d91c9aa8fab8ed749871d6d96f2ef4e2))
    - move examples into demos, having their very own dependencies; optimize tests ([`b757712`](https://github.com//Byron/gitoxide/commit/b757712f82de1d75ed813e744f979c1c652350e6))
    - fix (untested) extraction of delta object information ([`55a56b7`](https://github.com//Byron/gitoxide/commit/55a56b70b7b5a80089fde2edfff3ab3743d61cdd))
    - parallelize git-conut, optimize for speed ([`debd044`](https://github.com//Byron/gitoxide/commit/debd0445ba482d7b4424e53c45c0b6acf8b1de37))
    - refactor ([`9fc9fc0`](https://github.com//Byron/gitoxide/commit/9fc9fc0e706eaced2a4f04ae082f9f5acdde1fc0))
    - Fix big-pack 64 bit offset handling in index v2 ([`3b485b5`](https://github.com//Byron/gitoxide/commit/3b485b57062765b7ea476feaed328f4f94fc3478))
    - make refactor ([`cd6a18a`](https://github.com//Byron/gitoxide/commit/cd6a18ace5c07b542475518f6cfb506d34547013))
    - cargo clippy first pass ([`8b0a2a8`](https://github.com//Byron/gitoxide/commit/8b0a2a8b0665cb4bd7c32e46bec9dc33114e4985))
    - Finally remove failure and equip example with anyhow ([`f5e4ec5`](https://github.com//Byron/gitoxide/commit/f5e4ec5804efec4966ab1ca7fbf6e1a757f2f8c2))
    - remove failure from Index ([`55034a7`](https://github.com//Byron/gitoxide/commit/55034a7a22404b2d6a117c7242852480f42b84ab))
    - And one more module without failure ([`d0575bf`](https://github.com//Byron/gitoxide/commit/d0575bf39e6eebd0337bb8712eda1141b5766e92))
    - A big step towards removing failure ([`d862bd8`](https://github.com//Byron/gitoxide/commit/d862bd87a4d5bcafce83eed6c49c15a093972416))
    - refactor ([`87c8a2e`](https://github.com//Byron/gitoxide/commit/87c8a2e288140b04e163fe85266d040d039ec69c))
    - git-core: get rid of failure crate in favor of quick-error ([`91c8fc1`](https://github.com//Byron/gitoxide/commit/91c8fc1f0c50af55d7cb233bbe813c6d12fe11bc))
    - Get rid of nightly requirement, just parse tags differently soon ([`f037c4d`](https://github.com//Byron/gitoxide/commit/f037c4d982f2158cf173dce898c8dda1aea14106))
    - cargo fmt ([`2aa0857`](https://github.com//Byron/gitoxide/commit/2aa085752aa3e99b51034a3dec882aea8c27ad94))
    - reorganize repository a bit; use different contact email address ([`cb9fa28`](https://github.com//Byron/gitoxide/commit/cb9fa2848476e30767deb9d9807c649e0bc366da))
</details>

