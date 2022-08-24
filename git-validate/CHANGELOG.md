# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.5.5 (2022-08-24)

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 3 calendar days.
 - 97 days passed between releases.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge branch 'example-new-repo' ([`946dd3a`](https://github.com/Byron/gitoxide/commit/946dd3a80522ef437e09528a93aa1433f01b0ee8))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
</details>

## 0.5.4 (2022-05-18)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 8 calendar days.
 - 215 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#301](https://github.com/Byron/gitoxide/issues/301)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - assure validation of typical worktree related reference names ([`de169ae`](https://github.com/Byron/gitoxide/commit/de169ae8912b5d06b42237fa06e7dd221e58a1f6))
 * **Uncategorized**
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Merge branch 'main' into refs-and-worktrees ([`9cf0c7b`](https://github.com/Byron/gitoxide/commit/9cf0c7bd0cc5419137db5796f3a5b91bdf3dcc94))
</details>

## v0.5.3 (2021-10-15)

This release contains no functional changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 19 commits contributed to the release over the course of 23 calendar days.
 - 59 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#198](https://github.com/Byron/gitoxide/issues/198), [#205](https://github.com/Byron/gitoxide/issues/205)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Adjust all changelogs to fulfil requirements for publishing ([`04b9ca0`](https://github.com/Byron/gitoxide/commit/04b9ca025a1667529b2221ab4280bd3c8dae01cf))
    - deduplicate conventional message ids ([`e695eda`](https://github.com/Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - regenerate all changelogs to get links ([`0c81769`](https://github.com/Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com/Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - respect release-wide ignore list to allow removing entire conventional headlines ([`145103d`](https://github.com/Byron/gitoxide/commit/145103d4aa715386da9d4953f7f85fadc49fff9a))
    - Only write headlines that we can parse back… ([`d44369a`](https://github.com/Byron/gitoxide/commit/d44369ab5d849720dda9a9c0edc1ba1a3c1a78b5))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com/Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com/Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com/Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - Generate changelogs with details ([`e1861ca`](https://github.com/Byron/gitoxide/commit/e1861caa435d312953a9fea7ceff6d2e07b03443))
    - Update all changelogs with details ([`58ab2ae`](https://github.com/Byron/gitoxide/commit/58ab2aee23ba70a536e9487b44fb04c610374d1a))
    - Update changelogs ([`c857d61`](https://github.com/Byron/gitoxide/commit/c857d61ce3ce342012a2c4ba10a8327822aa530e))
    - Avoid adding newlines which make writing unstable ([`6b5c394`](https://github.com/Byron/gitoxide/commit/6b5c394f49282a8d09c2a9ffece840e4683572db))
    - Fix section headline level ([`9d6f263`](https://github.com/Byron/gitoxide/commit/9d6f263beef289d227dec1acc2d4240087cb9be6))
    - Write first version of changlogs thus far… ([`719b6bd`](https://github.com/Byron/gitoxide/commit/719b6bdf543b8269ccafad9ad6b46e0c55efaa38))
 * **[#205](https://github.com/Byron/gitoxide/issues/205)**
    - validate assumption about '(null)' as ref-name ([`2576168`](https://github.com/Byron/gitoxide/commit/2576168aa04d6451fc29c5dadfa698a9a50dac32))
 * **Uncategorized**
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com/Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - Update changelogs just for fun ([`21541b3`](https://github.com/Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
</details>

## v0.5.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 3 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-validate v0.5.2 ([`7bcbf9d`](https://github.com/Byron/gitoxide/commit/7bcbf9d24da16a87a23817a94ae6821f506eb1d5))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.5.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-validate v0.5.1 ([`fdd844a`](https://github.com/Byron/gitoxide/commit/fdd844a80e49f884e30d8a244e50284a9fc7080d))
    - remove dev-dependency cycles by removing their version ([`c40faca`](https://github.com/Byron/gitoxide/commit/c40faca41632cd2a226daf4ddf5293b65d1fdc82))
</details>

## v0.5.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`bf15c2a`](https://github.com/Byron/gitoxide/commit/bf15c2a2f285046b094093760c1969007ee75e25))
    - (cargo-release) version 0.4.0 ([`70ef344`](https://github.com/Byron/gitoxide/commit/70ef3442775b54ba9e4ee9ebfffb37af9804cc5b))
</details>

## v0.4.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 76 calendar days.
 - 76 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 ([`0d5c8b9`](https://github.com/Byron/gitoxide/commit/0d5c8b96dfdfb96e4fc82623f756f6c7f7046e90))
    - [ref #152] all tests and impl for refname expansion ([`9cef2f2`](https://github.com/Byron/gitoxide/commit/9cef2f2f166514048fae52ceec5a86a2849be286))
    - [ref #152] refactor ([`431dd86`](https://github.com/Byron/gitoxide/commit/431dd8655397b0ae88a5144d5c8553ba63e46c8f))
    - clippy on tests and thanks clippy ([`a77a71c`](https://github.com/Byron/gitoxide/commit/a77a71cf02d328a2a964388928d6b2a235a0aa85))
    - [validate] assure we can't accidentally write windows paths ([`02f127b`](https://github.com/Byron/gitoxide/commit/02f127ba3a525c6f2f02dc6aaad2f16ec76a182a))
    - [ref] on the way towards realistic transactions… ([`c808cb1`](https://github.com/Byron/gitoxide/commit/c808cb17b2fea12e018fabb789862e9b7703e49b))
    - (cargo-release) version 0.3.0 ([`87db688`](https://github.com/Byron/gitoxide/commit/87db688f23475d7232731429d770848aea228492))
    - [validate] disallow missing docs, fill in the remaining ones. ([`a593e79`](https://github.com/Byron/gitoxide/commit/a593e796cdba02700cb5747cf208c2f59f771f10))
    - [tempfile] crate frame ([`1b04c39`](https://github.com/Byron/gitoxide/commit/1b04c39030b436fb6850fbfa0c39a4fed7df727c))
    - (cargo-release) version 0.3.0 ([`6b33678`](https://github.com/Byron/gitoxide/commit/6b33678f83e6d261ca15c4a7634ff5b4e66d81dd))
    - Merge branch 'dependabot/cargo/crc-2.0.0' ([`683c44d`](https://github.com/Byron/gitoxide/commit/683c44db682d8dbef401286963e84cdca145abc8))
    - (cargo-release) version 0.2.0 ([`3286e42`](https://github.com/Byron/gitoxide/commit/3286e42547b59df6365087cbae9ce1c9c959faad))
    - [git-ref] find_one_existing(…) for convenience ([`7a443ff`](https://github.com/Byron/gitoxide/commit/7a443ffc148ae8161ba93351ffd16631f79e095c))
    - [git-ref] the first green find_one test ([`30177e8`](https://github.com/Byron/gitoxide/commit/30177e81451bd4fb51dd3297502fa3c63f67286e))
    - (cargo-release) version 0.2.0 ([`1327894`](https://github.com/Byron/gitoxide/commit/132789475400abe660b30ef6d2c5ff57821dd2c4))
    - [git-ref] refactor ([`0758867`](https://github.com/Byron/gitoxide/commit/0758867a8886be466bfb30aaffd628964b047a3d))
    - thanks clippy ([`474b73b`](https://github.com/Byron/gitoxide/commit/474b73bbe8679d0c58a4a7567d06e623ac4c1788))
    - [git-ref] all validation tests green ([`5312310`](https://github.com/Byron/gitoxide/commit/53123100201cae11e2dab1536d30c12f123ea710))
    - [git-ref] more tests green ([`4f5a1d0`](https://github.com/Byron/gitoxide/commit/4f5a1d0c55f790cdf6d6bd4e5b1957f5c1e0b290))
    - [git-ref] more tests for invalid ref paths ([`db3f1b1`](https://github.com/Byron/gitoxide/commit/db3f1b1b0326448514d566cde7ed9e9d210fa8ba))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.1.0 (2021-05-26)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [git-ref] use git-validate crate ([`6b4f937`](https://github.com/Byron/gitoxide/commit/6b4f937f13ad62bc2c7e5b0fc14416feb9c313ba))
    - [git-ref] migrate tag::name validation to git-validate ([`1ec4a54`](https://github.com/Byron/gitoxide/commit/1ec4a54ed67c89515cf5d2725eb394aac8395d7c))
    - [git-ref] setup git-validate crate for sharing of this kind of code ([`530d392`](https://github.com/Byron/gitoxide/commit/530d3920bb5ebbd1d0b0af05eda6bdbf5a5cb45e))
</details>

