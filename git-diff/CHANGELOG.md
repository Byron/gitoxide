# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.10.0 (2021-10-12)

It looks like there were no functional changes despite the minor version bump.
Please consider it a fluke that will be fixed with `cargo smart-release` automating
version number generation.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 28 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#198](https://github.com//Byron/gitoxide/issues/198)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com//Byron/gitoxide/issues/198)**
    - deduplicate conventional message ids ([`2fc45b7`](https://github.com//Byron/gitoxide/commit/2fc45b71116b19dd6d908686518a8620b78b8fb7))
    - regenerate all changelogs to get links ([`d654788`](https://github.com//Byron/gitoxide/commit/d65478880a170235e4f838156862ed035894fd5b))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com//Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - Allow 'refactor' and 'other' in conventional messages if they have breaking changes ([`4eebaac`](https://github.com//Byron/gitoxide/commit/4eebaac669e590beed112b622752997c64772ef1))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com//Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com//Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com//Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - Generate changelogs with details ([`e1861ca`](https://github.com//Byron/gitoxide/commit/e1861caa435d312953a9fea7ceff6d2e07b03443))
    - Update all changelogs with details ([`58ab2ae`](https://github.com//Byron/gitoxide/commit/58ab2aee23ba70a536e9487b44fb04c610374d1a))
    - Update changelogs ([`c857d61`](https://github.com//Byron/gitoxide/commit/c857d61ce3ce342012a2c4ba10a8327822aa530e))
    - Avoid adding newlines which make writing unstable ([`6b5c394`](https://github.com//Byron/gitoxide/commit/6b5c394f49282a8d09c2a9ffece840e4683572db))
    - Fix section headline level ([`9d6f263`](https://github.com//Byron/gitoxide/commit/9d6f263beef289d227dec1acc2d4240087cb9be6))
    - Write first version of changlogs thus far… ([`719b6bd`](https://github.com//Byron/gitoxide/commit/719b6bdf543b8269ccafad9ad6b46e0c55efaa38))
 * **Uncategorized**
    - make fmt, but now it picked up some parts that usually don't get altered… ([`01f7b72`](https://github.com//Byron/gitoxide/commit/01f7b729337bd2c99498321c479a9a13b1858e3e))
    - Update changelogs just for fun ([`21541b3`](https://github.com//Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
    - Bump git-traverse v0.9.0, safety bump 8 crates ([`d39fabb`](https://github.com//Byron/gitoxide/commit/d39fabb8757369aa19452a457f610fe21dc13a14))
</details>

## v0.9.2 (2021-09-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-diff v0.9.2 ([`17c411f`](https://github.com//Byron/gitoxide/commit/17c411f7679f4386eb3225c56dac80084787ed2b))
    - Bump git-object v0.14.0 ([`d4fc81f`](https://github.com//Byron/gitoxide/commit/d4fc81f6390443f8c8561d91ac27ea4a6318fb62))
</details>

## v0.9.1 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-diff v0.9.1 ([`cedae8d`](https://github.com//Byron/gitoxide/commit/cedae8d61f44a2de46edbac8afe19b7d8fa15cbf))
    - [repository #190] first shot at ancestor iteration… ([`85f1a48`](https://github.com//Byron/gitoxide/commit/85f1a48ea39f3b224e8d0ba3728dd75e03a6edc3))
    - Bump git-hash v0.6.0 ([`6efd90d`](https://github.com//Byron/gitoxide/commit/6efd90db54f7f7441b76159dba3be80c15657a3d))
</details>

## v0.9.0 (2021-08-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [pack #179] refactor ([`ab6554b`](https://github.com//Byron/gitoxide/commit/ab6554b0cd5838f1ea4e82f6b5019798288076fa))
    - Bump git-diff v0.9.0 ([`2e2e798`](https://github.com//Byron/gitoxide/commit/2e2e7983178b3af7e5684995de68ed5d020927ec))
    - [object #177] dissolve 'immutable' module ([`70e11c2`](https://github.com//Byron/gitoxide/commit/70e11c21b0637cd250f54381d5490e9976880ad9))
    - [object #177] migrate immutable::tree to crate::tree ([`fa5cd06`](https://github.com//Byron/gitoxide/commit/fa5cd0648d5c855060ab2b75ee933851987c2dcf))
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments ([`461dc53`](https://github.com//Byron/gitoxide/commit/461dc53ba3bc07d55fdb4aad7570ba9176a8b360))
    - Release git-object v0.13.0 ([`708fc5a`](https://github.com//Byron/gitoxide/commit/708fc5abd8af4dd7459f388c7092bf35915c6662))
</details>

## v0.8.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-diff v0.8.2 ([`3ad0829`](https://github.com//Byron/gitoxide/commit/3ad082939c52cfd6d679ebefcbaea4b16b12cfdb))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com//Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.8.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-diff v0.8.1 ([`41b218f`](https://github.com//Byron/gitoxide/commit/41b218f456ceea448d3b6a524e05970c478bdf6b))
    - remove dev-dependency cycles by removing their version ([`c40faca`](https://github.com//Byron/gitoxide/commit/c40faca41632cd2a226daf4ddf5293b65d1fdc82))
</details>

## v0.8.0 (2021-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 ([`f123f69`](https://github.com//Byron/gitoxide/commit/f123f69c7a4f9fd1c98bd2f60ebc953a6739fe04))
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 ([`c67291f`](https://github.com//Byron/gitoxide/commit/c67291ff9bcdff9a747d87241f6a71015607af05))
    - Release git-object v0.12.0 ([`7006150`](https://github.com//Byron/gitoxide/commit/7006150ac314d19814608723f69f6e70a72f9262))
    - (cargo-release) version 0.18.0 ([`b327590`](https://github.com//Byron/gitoxide/commit/b327590d02fec5536c380b2d39dd7be089ca7c40))
</details>

## v0.6.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 ([`4b71e15`](https://github.com//Byron/gitoxide/commit/4b71e15c3ba4a17ff2da5a5ef79986a2832fa3f2))
    - (cargo-release) version 0.5.0 ([`e21142b`](https://github.com//Byron/gitoxide/commit/e21142ba1a113b2afc4725d4d4225dff519c513a))
    - (cargo-release) version 0.17.0 ([`c52a491`](https://github.com//Byron/gitoxide/commit/c52a49176bd294bb36db74b4293cdb684a2ab7f6))
</details>

## v0.5.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`1687e59`](https://github.com//Byron/gitoxide/commit/1687e599be98d97925fbab594f31cf5558e9d2b1))
    - (cargo-release) version 0.4.0 ([`28e58f6`](https://github.com//Byron/gitoxide/commit/28e58f6b43a44e010da749a5618df02441f0d2e8))
    - (cargo-release) version 0.11.0 ([`a5be31c`](https://github.com//Byron/gitoxide/commit/a5be31c4cf7c0b538a1ed4a52ff5c3a992c6feff))
    - Revert "break more dev-depedency cycles up to git-odb" ([`22337ce`](https://github.com//Byron/gitoxide/commit/22337ce23995eee474e7dfb2e37fb56814522942))
</details>

## v0.4.1 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.1 ([`9790c15`](https://github.com//Byron/gitoxide/commit/9790c1590ec7180b76241b9f5ad7711d13abc7cc))
    - break more dev-depedency cycles up to git-odb ([`7ee278b`](https://github.com//Byron/gitoxide/commit/7ee278bf5b04adc5e4ab82cb83a3519f93587176))
</details>

## v0.4.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 83 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`ae02dab`](https://github.com//Byron/gitoxide/commit/ae02dabae961089a92a21e6a60a7006de4b56dad))
    - clippy on tests and thanks clippy ([`a77a71c`](https://github.com//Byron/gitoxide/commit/a77a71cf02d328a2a964388928d6b2a235a0aa85))
    - refactor ([`a92f1e6`](https://github.com//Byron/gitoxide/commit/a92f1e68beb0f946d0e117934b244d5aa1b6b5fc))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com//Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - [git-ref] the first failing test ([`7e802a0`](https://github.com//Byron/gitoxide/commit/7e802a0576230dfc666c253d484ea255f265f92f))
    - [git-odb] refactor ([`2958145`](https://github.com//Byron/gitoxide/commit/2958145a0ae1ef582bbf88352f5567d5c2b5eaf0))
    - (cargo-release) version 0.16.0 ([`769c649`](https://github.com//Byron/gitoxide/commit/769c649c00c009bf5a3f7c0611a7b999618f2938))
    - [git-odb] refactor ([`721303d`](https://github.com//Byron/gitoxide/commit/721303db232f87857aae58e12b342e5fb0139306))
    - [git-odb] refactor ([`ea224e9`](https://github.com//Byron/gitoxide/commit/ea224e9ee5f7efcbf4942a2a6fc7e4d790b2be50))
    - [git-odb] refactor ([`6a1b16a`](https://github.com//Byron/gitoxide/commit/6a1b16ae98edc9a694b945a12a7866eb17fc6be3))
    - (cargo-release) version 0.10.0 ([`5d7ee6a`](https://github.com//Byron/gitoxide/commit/5d7ee6a105abbb6efeed8624bade936bb59dbc55))
    - [git-traverse] fix potential lifetime issue ([`fcf2e8f`](https://github.com//Byron/gitoxide/commit/fcf2e8fb5356e5d4fb541347a9ca37306362815a))
    - [git-diff] refactor ([`fa8b4e8`](https://github.com//Byron/gitoxide/commit/fa8b4e8549c5992b8e622979aba3d11a6197bcc3))
    - [git-diff] refactor ([`9373cd6`](https://github.com//Byron/gitoxide/commit/9373cd6281b679d556255893ab0252e33bb86e77))
    - [git-diff] refactor ([`087e853`](https://github.com//Byron/gitoxide/commit/087e85367c27bb3684c6ad543c7eae46762e5e44))
    - (cargo-release) version 0.4.0 ([`c85d59a`](https://github.com//Byron/gitoxide/commit/c85d59a9a63d3cb503d906dcbeff2e585e4397e4))
    - [git-diff] enforce greater restraint when using path-ids ([`ad89320`](https://github.com//Byron/gitoxide/commit/ad893203912d60f382dab66bcd38e2fc312b7246))
    - (cargo-release) version 0.3.0 ([`684de4b`](https://github.com//Byron/gitoxide/commit/684de4b376ecd4cc5330f7ac8643352ea9580ed3))
</details>

## v0.3.0 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 6 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.15.0 ([`d91b241`](https://github.com//Byron/gitoxide/commit/d91b2412381e3c8c1f24c38469e821c3c3960e34))
    - (cargo-release) version 0.3.0 ([`3f2f8de`](https://github.com//Byron/gitoxide/commit/3f2f8de01088f8bf09ff04443534db513c522f6c))
    - (cargo-release) version 0.2.0 ([`3fb8377`](https://github.com//Byron/gitoxide/commit/3fb8377ff36422fe7607fb9172edf8bd5a4db995))
    - (cargo-release) version 0.9.0 ([`84897fd`](https://github.com//Byron/gitoxide/commit/84897fd8e6e1b0269da0303d6a0de8f9e0eb58e5))
    - refactor ([`082f8d0`](https://github.com//Byron/gitoxide/commit/082f8d0a4219246050d4594ba8cf769c8f5cdc90))
    - [traverse-tree] one test to pin implementation down a little ([`f0aeee1`](https://github.com//Byron/gitoxide/commit/f0aeee1ca3d9c0fd1290c1912226c7dae396e10b))
    - refactor ([`cceff1c`](https://github.com//Byron/gitoxide/commit/cceff1cf5297a6e507f8b44672181ba2600c748c))
</details>

## v0.2.0 (2021-05-02)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.14.0 ([`d9514ee`](https://github.com//Byron/gitoxide/commit/d9514eec64579ef77c9f2ac5dfe87cd302180eb9))
    - (cargo-release) version 0.2.0 ([`ca48e06`](https://github.com//Byron/gitoxide/commit/ca48e06b19076db961d81f8759ae564d5a5b7f6c))
    - And it's a wrap for git-diff docs for now ([`9e09dd5`](https://github.com//Byron/gitoxide/commit/9e09dd560a23d52d0469ce4fc13de01f7efce227))
    - refactor ([`6e6453d`](https://github.com//Byron/gitoxide/commit/6e6453d9e044499c9ee0a85d79dd75906adb9fb8))
    - [traversal] Add remaining missing docs ([`2f573f3`](https://github.com//Byron/gitoxide/commit/2f573f39c47879f7f318be9efa357e10a9e14ed2))
    - refactor ([`c0318cf`](https://github.com//Byron/gitoxide/commit/c0318cfa13dc32cf6c01879feae60158bc46d708))
    - git-diff docs ([`76af15b`](https://github.com//Byron/gitoxide/commit/76af15b708842fd0adaef6f685fd40101e8f7d72))
    - rename 'Locate' to 'Find' - shorter and just as good ([`60f72f5`](https://github.com//Byron/gitoxide/commit/60f72f573a7696323e09bf4add80d5fbce22c99d))
    - (cargo-release) version 0.13.0 ([`5c791af`](https://github.com//Byron/gitoxide/commit/5c791af217fac6a171d174ad9f4ee5f4d5282892))
    - [traversal] experiment uses git-traverse ([`3609356`](https://github.com//Byron/gitoxide/commit/360935640cbae5b691dcd976422bf00f9768e1c0))
    - [changes] more flexible handle of state ([`11db16b`](https://github.com//Byron/gitoxide/commit/11db16b585e7551fa0d85644ee085b95a9dc2c1e))
    - a new crate: git-traverse ([`1a9af50`](https://github.com//Byron/gitoxide/commit/1a9af50f1fca0e7e939f339b885c66dcb95e44e5))
</details>

## v0.1.0 (2021-04-30)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 65 commits contributed to the release over the course of 4 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - git-diff - fix include directive ([`c684382`](https://github.com//Byron/gitoxide/commit/c684382f5cac8c667a0a19b9b2cc95bd32d025d5))
    - prepare test utilities for release… ([`d35e654`](https://github.com//Byron/gitoxide/commit/d35e654747f96cec93bdecd1314ce325129cbc44))
    - (cargo-release) version 0.8.0 ([`a1ce210`](https://github.com//Byron/gitoxide/commit/a1ce210003ff07bf11291018bb182cbc7913647b))
    - (cargo-release) version 0.3.0 ([`e9665c7`](https://github.com//Byron/gitoxide/commit/e9665c784ae7e5cdaf662151395ee2355e9b57b6))
    - (cargo-release) version 0.1.0 ([`cb7b667`](https://github.com//Byron/gitoxide/commit/cb7b667255eafb6e378569892f47574533a698dc))
    - [traversal] run libgit2 parallel first to have a chance to get data more quickly ([`0a3564d`](https://github.com//Byron/gitoxide/commit/0a3564d5e949e328ee2923ee1b96a5d369102f9b))
    - [traversal] add CommitIter::tree_id() convenience method ([`6affd9d`](https://github.com//Byron/gitoxide/commit/6affd9d90d56d89774fcd4843638309a198815bf))
    - [tree-diff] another test, but no new outcome except that it seems to work ([`e295b53`](https://github.com//Byron/gitoxide/commit/e295b539df0bb3e4ae7093f09d6dcda8326029c5))
    - [tree-diff] And another test that showed something was indeed wrong ([`362680f`](https://github.com//Byron/gitoxide/commit/362680ff77f00dd305939090cb903003ff7be679))
    - refactor ([`85c5781`](https://github.com//Byron/gitoxide/commit/85c5781def8b45b01d4e46af97bbf24e1aa6da88))
    - refactor ([`109c4e0`](https://github.com//Byron/gitoxide/commit/109c4e0bf2ecb307da882d42584f769da19db02d))
    - refactor ([`e7a7ee8`](https://github.com//Byron/gitoxide/commit/e7a7ee81b0b40336671b28b7eecbac6ce40c4c23))
    - [tree-diff] Beginning of more nested test-suite… ([`b8a90e7`](https://github.com//Byron/gitoxide/commit/b8a90e7c9347b0eefdbef6f4c724cc0561cd79c9))
    - [tree-diff] the last todo, gone by test ([`d7418f3`](https://github.com//Byron/gitoxide/commit/d7418f3342319a31b3f591ecfe1d5d9b1b198e9c))
    - [tree-diff] consider that windows does do symlinks differently ([`b1b6e00`](https://github.com//Byron/gitoxide/commit/b1b6e0014dd02b66db538a262c4a0f7f891870e5))
    - [tree-diff] another green test ([`2627df0`](https://github.com//Byron/gitoxide/commit/2627df0bbb9da9eb8a3d1bdbe725fe35bf24071e))
    - [tree-diff] be independent on commit hashes ([`05e8e4a`](https://github.com//Byron/gitoxide/commit/05e8e4a060d8e47e6d98e188d8a93b01947f8035))
    - [tree-diff] another green test ([`1bfa9da`](https://github.com//Byron/gitoxide/commit/1bfa9daa95bf5a5643f3b70fdb8031e757ae1506))
    - [tree-diff] another green test ([`9ca57fa`](https://github.com//Byron/gitoxide/commit/9ca57fa9bc7a52170d109b323b1b1a74172604c1))
    - [tree-diff] a new failing test ([`c6eb677`](https://github.com//Byron/gitoxide/commit/c6eb6773f6768f3b24a4267ba2e0d3e6ce0aaa14))
    - tree-diff] another test ([`1eb961c`](https://github.com//Byron/gitoxide/commit/1eb961c8f22e8dc4a1988da09ce6521ca26fbfb4))
    - [tree-diff] less todos (that break tests if present) ([`03f87fe`](https://github.com//Byron/gitoxide/commit/03f87fe4bfa002aec57a074c64835ceab120fee9))
    - [tree-diff] another test ([`b23012e`](https://github.com//Byron/gitoxide/commit/b23012ebb943a0382b5cc3c2757a763f9183dda8))
    - [tree-diff] looks like windows now does line ending conversions for us ([`ff32a8f`](https://github.com//Byron/gitoxide/commit/ff32a8f98d96a7fa28c7e0f4021d4a7ed7e30787))
    - [tree-diff] another green test ([`ec681da`](https://github.com//Byron/gitoxide/commit/ec681da870e1677efc6c97dba35c1ccf21ea4724))
    - refactor ([`d550936`](https://github.com//Byron/gitoxide/commit/d5509369f509feddb1c3c10bae8b65c5dd3da35f))
    - [tree-diff] one more test green + refactor ([`bc5549d`](https://github.com//Byron/gitoxide/commit/bc5549db2ad16222761219d8652caf64867a889f))
    - [tree-diff] ManuallyDrop turns of drop behaviour, and I think it's Ok… ([`b885805`](https://github.com//Byron/gitoxide/commit/b885805e9d9cf5a02635b86cd5f86db5bbf57a4e))
    - [tree-diff] [FAIL] try to use peekable()… ([`0dcdc0e`](https://github.com//Byron/gitoxide/commit/0dcdc0efd59dda8a14db38c8a064d7caca9d1e0d))
    - [tree-diff] a step towards catching up with rhs ([`bbe7beb`](https://github.com//Byron/gitoxide/commit/bbe7beb606071610f1506ab1f29456eb79f56f8b))
    - [tree-diff] more tests (none of which hits new code paths) ([`791c429`](https://github.com//Byron/gitoxide/commit/791c4291926fd3aa2ab413d1058b2257976e8d87))
    - [tree-diff] deletion of directory and replacing it with a file ([`28e3fdd`](https://github.com//Byron/gitoxide/commit/28e3fdd54036dcd4a227062e9db01017196c20e0))
    - [tree-diff] test modification within a directory ([`ff82a82`](https://github.com//Byron/gitoxide/commit/ff82a82c1bd1b884afddea5baffb7448437561d1))
    - thanks clippy ([`c223e31`](https://github.com//Byron/gitoxide/commit/c223e31074d989024e22e8331eeb4280fb01cfab))
    - [tree-diff] The first example of recursion works ([`f86566c`](https://github.com//Byron/gitoxide/commit/f86566c646d8c9a1bb0304508faecc0e2eb163d8))
    - step towards zero-alloc traversal ([`f554c77`](https://github.com//Byron/gitoxide/commit/f554c77b8371deb987e2365381b85dd6d4325b74))
    - refactor ([`ca13594`](https://github.com//Byron/gitoxide/commit/ca1359414c6dc0ca3f9052299c7f088d83b38777))
    - refactor ([`aa1897d`](https://github.com//Byron/gitoxide/commit/aa1897d870df3fb76193f7e4f33e135760732288))
    - refactor ([`a717dba`](https://github.com//Byron/gitoxide/commit/a717dbaaafcbb0869bd189f1b625e5ff84a9ae72))
    - refactor ([`8087ca3`](https://github.com//Byron/gitoxide/commit/8087ca3e856f2c5a9c409a94ff8b54fcf295c894))
    - refactor ([`46583c1`](https://github.com//Byron/gitoxide/commit/46583c1fff415f742466b93c0821b21e7c9e7e1c))
    - refactor ([`fdc8c79`](https://github.com//Byron/gitoxide/commit/fdc8c7975a67b332eff995ca8046cafdb3bbeae2))
    - [tree-diff] refactor into iterator based model ([`29b527a`](https://github.com//Byron/gitoxide/commit/29b527aaea101c9b4e885db1c6d3533ef2310c54))
    - refactor ([`9ce9832`](https://github.com//Byron/gitoxide/commit/9ce98322bc578832495082e8a9c147d12542262b))
    - [tree-diff] A step closer to handling additions in a directory ([`a11f210`](https://github.com//Byron/gitoxide/commit/a11f210bec2c6c55bcf8cebe00e116e835306360))
    - [tree-diff] actually windows might have a point, let's see ([`0020a7c`](https://github.com//Byron/gitoxide/commit/0020a7cc368ffc5b62d6618f94a4cdec36c6d512))
    - [tree-diff] detect modifications ([`b87f2b4`](https://github.com//Byron/gitoxide/commit/b87f2b46783152964c24d6e7566a1787be60a932))
    - [tree-diff] See if this works on windows ([`95db1de`](https://github.com//Byron/gitoxide/commit/95db1de95a585ac1fa8a185b201300e86e5f34da))
    - [tree-diff] the first succeeding test - additions ([`619d4f0`](https://github.com//Byron/gitoxide/commit/619d4f05516ca0e54016b7ee8ab0433d6839ef7f))
    - refactor ([`a4d5f99`](https://github.com//Byron/gitoxide/commit/a4d5f99c8dc99bf814790928a3bf9649cd99486b))
    - refactor ([`11018e1`](https://github.com//Byron/gitoxide/commit/11018e1453ba6b130403f6d9f699881a93955c06))
    - [tree-diff] The first proper test with an API I like ([`ae6944e`](https://github.com//Byron/gitoxide/commit/ae6944eaf874a7d52f1f061e5d0d0a4d642c20b5))
    - refactor ([`633cba7`](https://github.com//Byron/gitoxide/commit/633cba7c1ff1f63c32613bedf963d1bd89afaee1))
    - refactor ([`3c10d06`](https://github.com//Byron/gitoxide/commit/3c10d0613ec00606a678c65e05ab1fda0ef742f7))
    - delegate-based tree diff traversal for maximum flexibility and performance ([`cbacca0`](https://github.com//Byron/gitoxide/commit/cbacca0be8bc8cb968b26438fc2caf48a447c542))
    - Maybe avoid even more allocations? At the expense of usability. ([`230ef04`](https://github.com//Byron/gitoxide/commit/230ef0447a56e9acd28efc6b71c5406e1b43653c))
    - probably a good idea to just use a graph for now to avoid a huge trap ([`6b43cdc`](https://github.com//Byron/gitoxide/commit/6b43cdca4749840fd179492bf9b7d7b9fb595814))
    - Sketch of how changes could actually be returned. ([`a48db50`](https://github.com//Byron/gitoxide/commit/a48db50049657f8299423c8eaacc1d44da0a5b34))
    - refactor ([`03ee510`](https://github.com//Byron/gitoxide/commit/03ee510a5f9c24b6acddaec1d30ea3ad39174603))
    - Second sketch of 'fluid' diff API that hopefullly makes clear how it works ([`ef6d469`](https://github.com//Byron/gitoxide/commit/ef6d469dfe22b8cdc816960b1be717483e3cdf8f))
    - First sketch of diff API ([`fc3f2b7`](https://github.com//Byron/gitoxide/commit/fc3f2b7066538e31f8d4bb1053d70dcabd5fbab1))
    - Better ergonomics for accessing decoded objects ([`ae3eab6`](https://github.com//Byron/gitoxide/commit/ae3eab6d6e4b96e207372fa8cb82f5ac9833e5e4))
    - Make sure releases of 'git-diff' don't get too big ([`378dde7`](https://github.com//Byron/gitoxide/commit/378dde703978812c6ffa39b51a4a7edd19a903ba))
    - Frame for testing tree(&tree) diffing ([`28c78f5`](https://github.com//Byron/gitoxide/commit/28c78f558e625f1d61bfa455f43bf6701e71703b))
    - More explicit expectations towards entries in mutable Trees ([`d94f84c`](https://github.com//Byron/gitoxide/commit/d94f84ceac637d2b6495be01dfc8eeb2494922f2))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.0.0 (2021-04-26)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add git-diff crate ([`42fdd8d`](https://github.com//Byron/gitoxide/commit/42fdd8d94b6fb65c1900cfef4f44dad619f7f09d))
</details>

