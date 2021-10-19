# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

A maintenance release due to properly dealing with previously breaking changes in `git-hash`.

## v0.5.3 (2021-10-15)

This release contains no functional changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 5 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#198](https://github.com//Byron/gitoxide/issues/198)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com//Byron/gitoxide/issues/198)**
    - Adjust all changelogs to fulfil requirements for publishing ([`04b9ca0`](https://github.com//Byron/gitoxide/commit/04b9ca025a1667529b2221ab4280bd3c8dae01cf))
    - deduplicate conventional message ids ([`e695eda`](https://github.com//Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - regenerate all changelogs to get links ([`0c81769`](https://github.com//Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com//Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - Allow 'refactor' and 'other' in conventional messages if they have breaking changes ([`4eebaac`](https://github.com//Byron/gitoxide/commit/4eebaac669e590beed112b622752997c64772ef1))
    - new changelogs for actor and features crates ([`e0d437c`](https://github.com//Byron/gitoxide/commit/e0d437c4cfa06e0792609f41ed5876c390634921))
 * **Uncategorized**
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com//Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com//Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - make fmt, but now it picked up some parts that usually don't get altered… ([`01f7b72`](https://github.com//Byron/gitoxide/commit/01f7b729337bd2c99498321c479a9a13b1858e3e))
</details>

## v0.5.2 (2021-09-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-actor v0.5.2 ([`32a8fde`](https://github.com//Byron/gitoxide/commit/32a8fde43ac3db3486710c3f01df07b664fcb9b0))
    - [repository #164] Prepare `commit()` for a possible less-allocating future ([`0fd01f7`](https://github.com//Byron/gitoxide/commit/0fd01f7071c785c27c56d2c034aac8dcdf690677))
    - [object #164] Allow referenced objects to be serialized as well ([`a98d298`](https://github.com//Byron/gitoxide/commit/a98d2985dae2259d72bb91a01548906862fee9f7))
</details>

## v0.5.1 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 6 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-actor v0.5.1 ([`0758045`](https://github.com//Byron/gitoxide/commit/0758045a43d15238eb6537fb3b60d2c1fdf7674e))
    - [repository #190] produce nice reflog messages ([`e7a8b62`](https://github.com//Byron/gitoxide/commit/e7a8b62eb24f840f639aa436b4e79a4a567d3d05))
    - [repository #190] A way to write objects and the empty tree specifically ([`7c559d6`](https://github.com//Byron/gitoxide/commit/7c559d6e1b68bc89220bca426257f383bce586ae))
    - [various #190] rename 'local-offset' to 'local-time-support' ([`3a7d379`](https://github.com//Byron/gitoxide/commit/3a7d3793a235ac872437f3bfedb9dd8fde9b31b1))
    - [repository #190] Make local-offset available on demand only… ([`1927be7`](https://github.com//Byron/gitoxide/commit/1927be7764f6af04ecc715dd52c631a3c8e16577))
    - [actor #190] methods to get an actor signature at the current time ([`6d0bedd`](https://github.com//Byron/gitoxide/commit/6d0beddb20092a80b113a39c862d6b680d79deb6))
</details>

## v0.5.0 (2021-08-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [object #177] fix docs ([`2fd23ed`](https://github.com//Byron/gitoxide/commit/2fd23ed9ad556b8e46cf650e23f0c6726e304708))
    - Merge pull request #172 from mellowagain/main ([`61aebbf`](https://github.com//Byron/gitoxide/commit/61aebbfff02eb87e0e8c49438a093a21b1134baf))
    - Release git-actor v0.5.0 ([`a684b0f`](https://github.com//Byron/gitoxide/commit/a684b0ff96ebfc5e4b3ce78452dc21ce856a6869))
    - [actor #175] refactor ([`ec88c59`](https://github.com//Byron/gitoxide/commit/ec88c5905194150cc94db4d4c20e9f4e2f6595c3))
</details>

## v0.4.0 (2021-08-25)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 6 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-actor v0.4.0 ([`16358c9`](https://github.com//Byron/gitoxide/commit/16358c9bf03604857d51bfa4dbfd2fc8c5210da7))
    - [actor #173] refactor ([`08a1849`](https://github.com//Byron/gitoxide/commit/08a18498d62f1d5bdabbb4712b08f3d17d63e16c))
    - [actor #173] rename immutable::Signature to SignatureRef! ([`96461ac`](https://github.com//Byron/gitoxide/commit/96461ace776d6b351b313d4f2697f2d95b9e196e))
    - Upgrade to nom-7 ([`f0aa3e1`](https://github.com//Byron/gitoxide/commit/f0aa3e1b5b407b2afd187c9cb622676fcddaf706))
    - Merge branch 'main' into 162-repo-design-sketch ([`e63b634`](https://github.com//Byron/gitoxide/commit/e63b63412c02db469fbdb17da82cd1e9fda1ef0f))
    - [smart-release #165] Use generic edit-reference functionality ([`be3e57f`](https://github.com//Byron/gitoxide/commit/be3e57f6221dc87505ba1aad1166e28c328c3b54))
</details>

## v0.3.3 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-actor v0.3.3 ([`3ead949`](https://github.com//Byron/gitoxide/commit/3ead9498db4168fb93f857324224c7dce340bc29))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com//Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.3.2 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-actor v0.3.2 ([`8f96eca`](https://github.com//Byron/gitoxide/commit/8f96ecae72e6363f1edacde0d3b861836d1c5730))
    - remove dev-dependency cycles by removing their version ([`c40faca`](https://github.com//Byron/gitoxide/commit/c40faca41632cd2a226daf4ddf5293b65d1fdc82))
</details>

## v0.3.1 (2021-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-actor-0.3.1 ([`727087d`](https://github.com//Byron/gitoxide/commit/727087dca243da4bc40bc87611a2f66234565be7))
    - [utils #154] commit manifest changes; create tags ([`95dcd9d`](https://github.com//Byron/gitoxide/commit/95dcd9d7d060101596c51116218102cc8049d0dd))
</details>

## v0.3.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 ([`64efc05`](https://github.com//Byron/gitoxide/commit/64efc0534ddc372b6e668b23c1e9d276098679c9))
    - (cargo-release) version 0.4.0 ([`70ef344`](https://github.com//Byron/gitoxide/commit/70ef3442775b54ba9e4ee9ebfffb37af9804cc5b))
</details>

## v0.2.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 45 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 ([`8ff5115`](https://github.com//Byron/gitoxide/commit/8ff511583e6d859e43ffda0ef75e2fecce3ed03c))
    - clippy on tests and thanks clippy ([`a77a71c`](https://github.com//Byron/gitoxide/commit/a77a71cf02d328a2a964388928d6b2a235a0aa85))
    - Merge branch 'remove-unnecessary-unsafe' ([`7a3c5c1`](https://github.com//Byron/gitoxide/commit/7a3c5c14dc56d8711548d1b219a969836693cbaa))
    - Remove unnecessary pub(crate) exports ([`3d2456e`](https://github.com//Byron/gitoxide/commit/3d2456e11709f0461b37c6df55ecc3861ca4cab5))
    - [ref] packed refs header line parsing ([`fde5543`](https://github.com//Byron/gitoxide/commit/fde5543ad22395e27266db02a5442a33d16e68c5))
    - [ref] log line writing ([`3da8fcf`](https://github.com//Byron/gitoxide/commit/3da8fcf0bfb77b80c06a3358416f10d6f393db8b))
    - Merge branch 'negotiate-fallible' ([`27c8abe`](https://github.com//Byron/gitoxide/commit/27c8abe1948bc10c779efa33d4bc0b92741f6444))
    - [actor] refactor ([`bccb738`](https://github.com//Byron/gitoxide/commit/bccb738edfc2e6923643a2e73f93b6acfdd7cf5c))
    - [actor] don't leak btoi errors… ([`e6c7fc1`](https://github.com//Byron/gitoxide/commit/e6c7fc18954a5a5ad12b3da6c290f8cb9a74c19c))
    - [actor] FAIL an attempt to remove btoi errors ([`3f99cf5`](https://github.com//Byron/gitoxide/commit/3f99cf531caacb93a3ce81b16d61be18e5d8a017))
    - [actor] pure nom error handling… ([`78cbe18`](https://github.com//Byron/gitoxide/commit/78cbe18888ec654f3410fc655d9beaaf63f68003))
    - [object] refactor ([`1ddb5c0`](https://github.com//Byron/gitoxide/commit/1ddb5c07b75aa2b9a9536125fbba1fc862b7fe34))
    - [actor] make signature parsing public, exposing nom :/ ([`a627972`](https://github.com//Byron/gitoxide/commit/a627972ecc53d38210c826f851ea9c5fec17b9cb))
    - [actor] cleanup error interaction with nom… ([`2dd7197`](https://github.com//Byron/gitoxide/commit/2dd7197248d58a7a89f5ff368072c511fce127e3))
</details>

## v0.1.1 (2021-06-25)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.1.1 ([`e9cdc95`](https://github.com//Byron/gitoxide/commit/e9cdc958e7ce2290e2d7958cdb5aa9323ef35d37))
    - [actor] fix dependencies ([`3ff918e`](https://github.com//Byron/gitoxide/commit/3ff918efa0b94dd20f781a3d038a0449cd9c7a59))
</details>

## v0.1.0 (2021-06-25)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - thanks clippy ([`94fb007`](https://github.com//Byron/gitoxide/commit/94fb0071193b0e3428fd1747422ea1675dd5974b))
    - [actor] refactor ([`986d09a`](https://github.com//Byron/gitoxide/commit/986d09a4c894966bc7d918c7aaf7da6e211bfbbd))
    - [actor] refactor ([`591a741`](https://github.com//Byron/gitoxide/commit/591a74153b3fbbe6ffdc2dc06834f581dc632b3e))
    - [refs] git-actor crate to share types between git-ref and git-object ([`13edbf7`](https://github.com//Byron/gitoxide/commit/13edbf7c5d7668991cad6c49b0bbd3e396a267c4))
</details>

