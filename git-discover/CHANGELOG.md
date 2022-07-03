# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.2.0 (2022-06-13)

<csr-id-6106521581029c5c24b23a47bb91c1921edfa0af/>

### Other

 - <csr-id-6106521581029c5c24b23a47bb91c1921edfa0af/> avoid shortening absolute paths

### New Features (BREAKING)

 - <csr-id-266d4379e9132fd7dd21e6c8fccb36e125069d6e/> Make `realpath()` easier to use by introducing `realpath_opt()`.
   That way there is consistency about how many symlinks to follow.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 16 calendar days.
 - 16 days passed between releases.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - make fmt ([`c665aef`](https://github.com/Byron/gitoxide/commit/c665aef4270c5ee54da89ee015cc0affd6337608))
    - refactor ([`ec37cb8`](https://github.com/Byron/gitoxide/commit/ec37cb8005fa272aed2e23e65adc291875b1fd68))
    - refactor ([`b27a8c2`](https://github.com/Byron/gitoxide/commit/b27a8c243cdc14730478c2a94cafdc8ccf5c60d3))
    - refactor ([`06e96a4`](https://github.com/Byron/gitoxide/commit/06e96a435d820a1ef1e567bf93e7b9ca5fa74829))
    - refactor ([`b555bda`](https://github.com/Byron/gitoxide/commit/b555bdae9964628b6d43e00dd7d7ee8fe674d309))
    - Unify the way `dir_made_absolute` is computed ([`48417fc`](https://github.com/Byron/gitoxide/commit/48417fc8d8d3ec0005ef7b6ef35ced09d92282c2))
    - Merge branch 'main' into davidkna-envopen ([`bc0abc6`](https://github.com/Byron/gitoxide/commit/bc0abc643d3329f885f250b6880560dec861150f))
    - refactor ([`7b307f5`](https://github.com/Byron/gitoxide/commit/7b307f5acb7214f4ff674e00167933cdcccc353a))
    - Make `realpath()` easier to use by introducing `realpath_opt()`. ([`266d437`](https://github.com/Byron/gitoxide/commit/266d4379e9132fd7dd21e6c8fccb36e125069d6e))
    - avoid shortening absolute paths ([`6106521`](https://github.com/Byron/gitoxide/commit/6106521581029c5c24b23a47bb91c1921edfa0af))
    - Add discovery opt env-overrides & env discovery helpers ([`e521d39`](https://github.com/Byron/gitoxide/commit/e521d39e1b0f4849280bae1527bf28977eec5093))
</details>

## 0.1.3 (2022-05-27)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#426](https://github.com/Byron/gitoxide/issues/426), [#429](https://github.com/Byron/gitoxide/issues/429)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#426](https://github.com/Byron/gitoxide/issues/426)**
    - allow tests to deal with shared drives by adjusting trust expectation ([`85ab096`](https://github.com/Byron/gitoxide/commit/85ab0964a1a35556a5efe392576c9b028e601c6c))
    - cleanup ([`e029eb4`](https://github.com/Byron/gitoxide/commit/e029eb4589942e06a53bc0f6ed472d1d33c4a7e6))
 * **[#429](https://github.com/Byron/gitoxide/issues/429)**
    - Adjust changelogs prior to release ([`7397805`](https://github.com/Byron/gitoxide/commit/7397805fd032a752d6c2f2c2c28ac11ddecc7193))
 * **Uncategorized**
    - Release git-sec v0.1.2, git-discover v0.1.3, cargo-smart-release v0.10.2 ([`6cd365e`](https://github.com/Byron/gitoxide/commit/6cd365e2cf6851f5cdecc22f3b1667440ad011b0))
    - Merge branch 'davidkna-admin-sec' ([`3d0e2c2`](https://github.com/Byron/gitoxide/commit/3d0e2c2d4ebdbe3dff01846aac3375128353a2e1))
</details>

## 0.1.2 (2022-05-23)

<csr-id-136eb37b00c9e7ba0fd0bc2a75dee2ac1b06516d/>

### Features

- `discover` now avoid crossing file-system boundaries by default on unix.

### Other

 - <csr-id-136eb37b00c9e7ba0fd0bc2a75dee2ac1b06516d/> adopt git-for-windows exception rules

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-path v0.1.3, git-discover v0.1.2, git-repository v0.18.1, cargo-smart-release v0.10.1 ([`b7399cc`](https://github.com/Byron/gitoxide/commit/b7399cc44ee419355a649a7b0ba7b352cd48b400))
    - prepare for smart-release release ([`2f74cb0`](https://github.com/Byron/gitoxide/commit/2f74cb05e9b2399355af07517fe3c14e4e8724c5))
    - adopt git-for-windows exception rules ([`136eb37`](https://github.com/Byron/gitoxide/commit/136eb37b00c9e7ba0fd0bc2a75dee2ac1b06516d))
    - Merge branch 'davidkna-discover-x-fs' ([`9abaeda`](https://github.com/Byron/gitoxide/commit/9abaeda2d22e2dbb1db1632c6eb637f1458d06e1))
    - refactor ([`aac5169`](https://github.com/Byron/gitoxide/commit/aac5169aa7c753eb3510e61bc01b47a4b7b01c6f))
    - use `defer` to make dmg unmounting more reliable ([`dbc5caa`](https://github.com/Byron/gitoxide/commit/dbc5caa99b050ac7a96cd5e7a73786072c695530))
    - refactor ([`886e26e`](https://github.com/Byron/gitoxide/commit/886e26ee686776ad4caf1864f24cb358432d5c49))
    - Assure cross-fs check doesn't break if cursor is currently empty ([`4546889`](https://github.com/Byron/gitoxide/commit/4546889adb87c0d9d1c5d22785240033dc3b0dc2))
</details>

## 0.1.1 (2022-05-21)

<csr-id-e63e722791a7795cd99048bed834459595c60abc/>
<csr-id-277c41f1185398e14f92247aada8422f6b08afd1/>

### Other

 - <csr-id-e63e722791a7795cd99048bed834459595c60abc/> add ceiling_dirs option to upwards discovery

### Other

 - <csr-id-277c41f1185398e14f92247aada8422f6b08afd1/> add cross_fs option to upwards discovery

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 19 commits contributed to the release over the course of 2 calendar days.
 - 3 days passed between releases.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-path v0.1.2, git-sec v0.1.1, git-config v0.4.0, git-discover v0.1.1, git-pack v0.19.1, git-repository v0.18.0, cargo-smart-release v0.10.0, safety bump 2 crates ([`ceb6dff`](https://github.com/Byron/gitoxide/commit/ceb6dff13362a2b4318a551893217c1d11643b9f))
    - add cross_fs option to upwards discovery ([`277c41f`](https://github.com/Byron/gitoxide/commit/277c41f1185398e14f92247aada8422f6b08afd1))
    - Re-enable discovery test on windows thanks to use of `realpath()` ([`1f4ae9e`](https://github.com/Byron/gitoxide/commit/1f4ae9e74843d108b261cbfe10e158f74286088a))
    - Merge branch 'svetli-n-git_includeif' ([`cf24fbe`](https://github.com/Byron/gitoxide/commit/cf24fbe4b62d67b06138243d470dcc1805ebd55b))
    - Fix windows test failure due to //? by ignoring it there. ([`c5fd322`](https://github.com/Byron/gitoxide/commit/c5fd3223db73ee61844477aff0e8a2438d9b2e39))
    - fix windows tests (broke thanks to \\?\), maybe ([`e458b59`](https://github.com/Byron/gitoxide/commit/e458b5946ebf01d9f901769b8547df85ef14afcb))
    - Assure ceiling dirs are comparable after absolutize ([`32a157b`](https://github.com/Byron/gitoxide/commit/32a157b513191c60795924765825dc7dfb0b38c1))
    - thanks clippy ([`4979d20`](https://github.com/Byron/gitoxide/commit/4979d2071cbd1a98f3d81aacd60dd99f07d3f746))
    - refactor ([`895b772`](https://github.com/Byron/gitoxide/commit/895b772b5855818ad2227cac8dda0be00f9d5189))
    - Control if at least one ceiling dir has to match using an option ([`ca1f3eb`](https://github.com/Byron/gitoxide/commit/ca1f3ebb1306075767597c75bb288a3a0b4ebb41))
    - Revert "remove implicit canonicalization and improve tests" ([`821f3f3`](https://github.com/Byron/gitoxide/commit/821f3f383a21ddfd274cb6bdcd2228717cd21942))
    - refactor ([`6bbc53b`](https://github.com/Byron/gitoxide/commit/6bbc53be867e5292a98c422526185c6a4736a6a5))
    - fix `special_relative_base` test ([`ae226ba`](https://github.com/Byron/gitoxide/commit/ae226ba9c08b04621ce3b42e6972e102d8af2b9c))
    - remove implicit canonicalization and improve tests ([`cdbb4c9`](https://github.com/Byron/gitoxide/commit/cdbb4c9db56ef6eb258a9a76691614d4b45c8d7a))
    - refactor ([`429446c`](https://github.com/Byron/gitoxide/commit/429446c87473dca98bf802fa3de020cb58625f63))
    - refactor ([`9673aae`](https://github.com/Byron/gitoxide/commit/9673aae3da57bc12d45389de3034ab0c2c1c27ab))
    - some more assertions and ceiling dirs ([`a30bcb8`](https://github.com/Byron/gitoxide/commit/a30bcb82413103c69827151f25be0b14b1f04f37))
    - add ceiling_dirs option to upwards discovery ([`e63e722`](https://github.com/Byron/gitoxide/commit/e63e722791a7795cd99048bed834459595c60abc))
    - declare `git-discover` usable as it's fully documented ([`e439015`](https://github.com/Byron/gitoxide/commit/e439015288dad806494e0a2ed8d44fb2247de372))
</details>

## 0.1.0 (2022-05-18)

### New Features

 - <csr-id-050f795bfb0fe11655cd7e45c10d87c89ba82625/> allow discovery of linked worktree git dirs
   This also works if the work-tree can't be found but it is otherwise
   a valid git dir.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 38 commits contributed to the release over the course of 12 calendar days.
 - 12 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#301](https://github.com/Byron/gitoxide/issues/301)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - a test to assure non-existing worktree directories don't hinder discovery ([`131481c`](https://github.com/Byron/gitoxide/commit/131481cac26959d46f62d40adcfc895faa4a1698))
    - allow discovery of linked worktree git dirs ([`050f795`](https://github.com/Byron/gitoxide/commit/050f795bfb0fe11655cd7e45c10d87c89ba82625))
    - path shortening now calculates the actual path length, not just component count ([`c0d24e0`](https://github.com/Byron/gitoxide/commit/c0d24e0fb422f990213606d781ba056c238bd70b))
    - adjust to changes in git-path ([`a627ddc`](https://github.com/Byron/gitoxide/commit/a627ddc48be034f8a42d86edf2a9295299297d59))
    - adapt to changes in git-path ([`c258d77`](https://github.com/Byron/gitoxide/commit/c258d77660755559eaa5389bfabe0cebdf478a80))
    - refactor ([`81a542f`](https://github.com/Byron/gitoxide/commit/81a542fbb680a19322877cf3740ae2ae66b95011))
    - rely on `absolutize_components()` ([`e844006`](https://github.com/Byron/gitoxide/commit/e84400660dad6281fe3869ad649470f2adf31979))
    - brutally fix path handling of common dirs ([`e120232`](https://github.com/Byron/gitoxide/commit/e120232252875cd3fdacb9b7df90c3db58e7e24e))
    - Another test to assure invalid worktree dirs aren't git dirs ([`e6e3608`](https://github.com/Byron/gitoxide/commit/e6e3608490bda2577c23f93d905a51c33fa4f777))
    - adjust for different errors on windows when handling errors opening files… ([`9625829`](https://github.com/Byron/gitoxide/commit/962582996bb8d53739393acfcd150e9aa5132bae))
    - fix tests on windows ([`8080ad2`](https://github.com/Byron/gitoxide/commit/8080ad2baedf83237a0d9a01550582fc8592e568))
    - better tests for worktree discovery ([`9d0f134`](https://github.com/Byron/gitoxide/commit/9d0f134b3186635cbc253464fc7cec2ff522a4e2))
    - The first working version of worktree detection ([`dc73e96`](https://github.com/Byron/gitoxide/commit/dc73e96fdd21713083d9fe0464d1a5689405c116))
    - fix windows tests ([`b025008`](https://github.com/Byron/gitoxide/commit/b0250080c5a1e8ba3fbdb3f70038521b75634306))
    - refactor ([`6b73e0d`](https://github.com/Byron/gitoxide/commit/6b73e0dc60488fcadf21366322e75f711bada6f4))
    - support for gitdir file parsing with all of gits safety ([`1223fc7`](https://github.com/Byron/gitoxide/commit/1223fc7777f57d73c3693407f6a1e41393919a99))
    - basic parsing for git-dir files ([`e11c677`](https://github.com/Byron/gitoxide/commit/e11c67770c301942188f204dbb2cd61880087959))
    - fix test on windows maybe ([`9583048`](https://github.com/Byron/gitoxide/commit/95830480928421de1e38c99a7e9a65629ec1cc41))
    - Try to fix winodws build ([`f2c5ed3`](https://github.com/Byron/gitoxide/commit/f2c5ed3326a3076cc25afbaeec9a540d823e0606))
    - refactor `repositoryKind` adjusted to handle linked worktrees ([`84677cb`](https://github.com/Byron/gitoxide/commit/84677cb09634e1d18ce20850bb7c6c9d63a13818))
    - refactor ([`eead214`](https://github.com/Byron/gitoxide/commit/eead2144e10b96e997cd702cb651f4c92693df95))
    - avoid running failing test on _any_ windows ([`9351a3d`](https://github.com/Byron/gitoxide/commit/9351a3db2a7e9da13be3071c1d2d5aa0cb7a35b6))
    - Fix tests on windows maybe ([`8084581`](https://github.com/Byron/gitoxide/commit/8084581619c7261f779ae42856be756eac7fbfa4))
    - don't archive repositories with worktrees ([`389ef66`](https://github.com/Byron/gitoxide/commit/389ef663ce0417f3840f90dedfc91a57eae6fd0c))
    - prepare test to provide worktrees of different kinds as well ([`a4cec4a`](https://github.com/Byron/gitoxide/commit/a4cec4ab8ba80a8f439b283d369d48cdafbe7b74))
    - Add required support for reading path files ([`bc08511`](https://github.com/Byron/gitoxide/commit/bc085116241e8824c623efc9893ded220fe9a5a2))
    - Remove handling of environment variables ([`4e914f8`](https://github.com/Byron/gitoxide/commit/4e914f89bbf04dbe55eae01f7be09899296ad8b0))
    - Add git-discover to check-size ([`85b29a6`](https://github.com/Byron/gitoxide/commit/85b29a6ba6b3ee4b925fd10c688b2aabcf4273f2))
    - fix docs ([`1e3acd0`](https://github.com/Byron/gitoxide/commit/1e3acd08b9df9fe0cc36bb6a4d4bac57c365443d))
    - refactor ([`00a988e`](https://github.com/Byron/gitoxide/commit/00a988e3c2c964447f675164a6126bf6cb470c6b))
    - Migrate all relevant code from git-repository to git-discover ([`631e70e`](https://github.com/Byron/gitoxide/commit/631e70e6f210df40eb789023970ec17095ec3556))
 * **Uncategorized**
    - Release git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0 ([`349c590`](https://github.com/Byron/gitoxide/commit/349c5904b0dac350838a896759d51576b66880a7))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - make fmt ([`e043807`](https://github.com/Byron/gitoxide/commit/e043807abf364ca46d00760e2f281528efe20c75))
    - thanks clippy ([`7617da0`](https://github.com/Byron/gitoxide/commit/7617da002ef5906f858c0cc2c349f442bafd6239))
    - thanks clippy ([`a084951`](https://github.com/Byron/gitoxide/commit/a084951c72818d7cb2061053078793213890c899))
    - thanks clippy ([`3c9da80`](https://github.com/Byron/gitoxide/commit/3c9da80a39bc6638a38868f71d80e2a8ad337a4a))
</details>

## 0.0.0 (2022-05-06)

The first release, an empty crate.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#301](https://github.com/Byron/gitoxide/issues/301)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - Add empty git-discover crate ([`6565f16`](https://github.com/Byron/gitoxide/commit/6565f163ba464f5adf63d214143a4e12f48b00fc))
 * **Uncategorized**
    - Release git-discover v0.0.0 ([`2b5cf2b`](https://github.com/Byron/gitoxide/commit/2b5cf2b95a723d885584ac967cf956c28ea35f47))
</details>

