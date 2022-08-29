# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.8.0 (2022-08-27)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### New Features

 - <csr-id-231785644194cd3be0b0dab06224a39ecf0ed714/> Provide `GIT_VERSION` information along with a way to skip a test based on it.
 - <csr-id-654b521323a5822cbb86e57bee159d90576fa5ff/> expose `on_ci` in the top-level.
 - <csr-id-449b6c1555fc2832c712ba51cd41ab9ed79e0b15/> Allow to re-execute scripts into temp directories.
   This is important in cases where the files created by the script
   contain absolute mentions of locations. That way, when copying
   files over, the test might accidentally return to the original
   read-only location, and write into it making future test runs fail.
 - <csr-id-f1635c3ee36678cff9f26135946c281bf4a75331/> publicly accessible `Result` type

### Bug Fixes

 - <csr-id-004dab17deab4c360adb5ac428f6b4951c974fe3/> `_with_args(…)` functions now allow non-static strings

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 68 commits contributed to the release over the course of 135 calendar days.
 - 145 days passed between releases.
 - 7 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 10 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#331](https://github.com/Byron/gitoxide/issues/331), [#382](https://github.com/Byron/gitoxide/issues/382), [#384](https://github.com/Byron/gitoxide/issues/384), [#391](https://github.com/Byron/gitoxide/issues/391), [#393](https://github.com/Byron/gitoxide/issues/393), [#427](https://github.com/Byron/gitoxide/issues/427), [#488](https://github.com/Byron/gitoxide/issues/488), [#XXX](https://github.com/Byron/gitoxide/issues/XXX)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - upgrade dependencies ([`b039d39`](https://github.com/Byron/gitoxide/commit/b039d39613bb14d49670c4d8b586f76ffb420d03))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - Allow to re-execute scripts into temp directories. ([`449b6c1`](https://github.com/Byron/gitoxide/commit/449b6c1555fc2832c712ba51cd41ab9ed79e0b15))
    - don't print archive message if archive is excluded ([`c6bd30e`](https://github.com/Byron/gitoxide/commit/c6bd30e81997931d1f65a62924d20fe5e74b8521))
    - Support unique directories for different platforms ([`0b385b3`](https://github.com/Byron/gitoxide/commit/0b385b31cf95d500f9ec2d05be0894956e40e4a1))
    - Use git exclude information to determine if archives should be generated ([`4a3dccc`](https://github.com/Byron/gitoxide/commit/4a3dccc7fc7a5e190d88af8c7eb0713edbada55f))
    - Add TODO ([`778fd77`](https://github.com/Byron/gitoxide/commit/778fd7703920e6a2693beb59aad611f3c9fab106))
    - publicly accessible `Result` type ([`f1635c3`](https://github.com/Byron/gitoxide/commit/f1635c3ee36678cff9f26135946c281bf4a75331))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - expose `on_ci` in the top-level. ([`654b521`](https://github.com/Byron/gitoxide/commit/654b521323a5822cbb86e57bee159d90576fa5ff))
    - move `Env` test utility into `git-testtools` ([`bd3f4d0`](https://github.com/Byron/gitoxide/commit/bd3f4d014dd7df7a1e25defa8eea7253eec1560a))
 * **[#382](https://github.com/Byron/gitoxide/issues/382)**
    - Simplify state tests ([`fc61c0d`](https://github.com/Byron/gitoxide/commit/fc61c0d4f7cb3cd9073418e4d8edc55cd14f5fb3))
 * **[#384](https://github.com/Byron/gitoxide/issues/384)**
    - enforce signal handler setup to cleanup tempfiles on abort ([`1caf3ae`](https://github.com/Byron/gitoxide/commit/1caf3ae2cabee776dc45a687f00ce386c27ab87d))
    - No need to isolate archives by crate name ([`19d46f3`](https://github.com/Byron/gitoxide/commit/19d46f35440419b9911b6e2bca2cfc975865dce9))
    - provide some more information when using archives; debug windows more ([`4f5b1fd`](https://github.com/Byron/gitoxide/commit/4f5b1fd5e6440208c460388a9d69d664d6d8d0d7))
    - protect test generation from multi-process races ([`1aec924`](https://github.com/Byron/gitoxide/commit/1aec924f009fd16b953cd1313b9408558b1c7aeb))
    - definitely don't follow symlnks ([`1343448`](https://github.com/Byron/gitoxide/commit/13434481c44efbc170cb74dd9057807c3ee58e01))
    - make sure existing files aren't written into ([`9b5a8a2`](https://github.com/Byron/gitoxide/commit/9b5a8a243d49b6567d1db31050d3bf3123dd54d3))
    - extraction of tar archives with identity check ([`07c1f07`](https://github.com/Byron/gitoxide/commit/07c1f0752fefbd3e49ef414bced2ca6bbc844448))
    - assure there are no archive file-name clashes across crates ([`c30bebf`](https://github.com/Byron/gitoxide/commit/c30bebf4f0272fe728e18b1932e419128f63ed44))
    - actual compression of archives ([`5dd3d82`](https://github.com/Byron/gitoxide/commit/5dd3d82aa68c9024cd1742043a3c56cd6b0665fd))
    - simple creation of test-archives ([`f1e107a`](https://github.com/Byron/gitoxide/commit/f1e107aa864107e02309b15b41da8d8f962e19a6))
    - make sure archives are handled by git-lfs ([`f744a6c`](https://github.com/Byron/gitoxide/commit/f744a6cc8b453ea349664540af4be0566e376528))
    - frame for extracting and generating archives ([`92c7044`](https://github.com/Byron/gitoxide/commit/92c7044cfbc3054b237ea7c79da981bb91908812))
    - further partition generated test directories by script name ([`e141ddb`](https://github.com/Byron/gitoxide/commit/e141ddbdd2e0677e921856b30096733530fde569))
    - auto-set commit.gpgsign=false when executing git ([`c23feb6`](https://github.com/Byron/gitoxide/commit/c23feb64ad157180cfba8a11c882b829733ea8f6))
 * **[#391](https://github.com/Byron/gitoxide/issues/391)**
    - also write a failure marker if archive creation failed ([`7f88c7f`](https://github.com/Byron/gitoxide/commit/7f88c7f9d908df39ad4e710402783dca35eb758f))
    - auto-clean test fixtures on re-run if they failed previously ([`3617ff4`](https://github.com/Byron/gitoxide/commit/3617ff411224a691057eb1c39c4144b932b33f51))
 * **[#393](https://github.com/Byron/gitoxide/issues/393)**
    - Add support for disabling archive usage ([`624ad2e`](https://github.com/Byron/gitoxide/commit/624ad2ef42172556efe942129f6f46dd627250d5))
 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - make fmt ([`4b320e7`](https://github.com/Byron/gitoxide/commit/4b320e773368ac5e8c38dd8a779ef3d6d2d024ec))
 * **[#488](https://github.com/Byron/gitoxide/issues/488)**
    - Provide `GIT_VERSION` information along with a way to skip a test based on it. ([`2317856`](https://github.com/Byron/gitoxide/commit/231785644194cd3be0b0dab06224a39ecf0ed714))
 * **[#XXX](https://github.com/Byron/gitoxide/issues/XXX)**
    - `_with_args(…)` functions now allow non-static strings ([`004dab1`](https://github.com/Byron/gitoxide/commit/004dab17deab4c360adb5ac428f6b4951c974fe3))
 * **Uncategorized**
    - Release git-attributes v0.3.3, git-ref v0.15.3, git-index v0.4.3, git-worktree v0.4.3, git-testtools v0.8.0 ([`baad4ce`](https://github.com/Byron/gitoxide/commit/baad4ce51fe0e8c0c1de1b08148d8303878ca37b))
    - prepare changelogs prior to release of git-testtools ([`7668e38`](https://github.com/Byron/gitoxide/commit/7668e38fab8891ed7e73fae3a6f5a8772e0f0d0b))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge branch 'example-new-repo' ([`946dd3a`](https://github.com/Byron/gitoxide/commit/946dd3a80522ef437e09528a93aa1433f01b0ee8))
    - thanks clippy ([`9aa8277`](https://github.com/Byron/gitoxide/commit/9aa827785c25e63dd1b351a7cc553f140fb93c2e))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - fix CI for good ([`e0c0b8c`](https://github.com/Byron/gitoxide/commit/e0c0b8c7c1898b2bc11a915e8e4fb8426295ccbb))
    - Merge branch 'write-index-files' into write-index-v2 ([`cddc2ca`](https://github.com/Byron/gitoxide/commit/cddc2ca06f63f66e887ff821452d1f56fb08fe6a))
    - Merge branch 'write-index-files' into rev-parse-delegate ([`370110d`](https://github.com/Byron/gitoxide/commit/370110d3356528af38150c2280ed505354ceca5b))
    - Merge branch 'main' into rev-parse-delegate ([`4ae2bed`](https://github.com/Byron/gitoxide/commit/4ae2bedfc25d1881d58ebdc54aca0936c68d4859))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Add docs related to archives. ([`f409a2a`](https://github.com/Byron/gitoxide/commit/f409a2ae88f2b0d80c7d160563c07935993203a6))
    - Add documentation to test-tools. ([`074b283`](https://github.com/Byron/gitoxide/commit/074b2833d15c8483bd89e4bde4486c0c7df14637))
    - make fmt ([`47724c0`](https://github.com/Byron/gitoxide/commit/47724c0edb382c036a3fc99884becfd2b0740d4b))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - thanks clippy ([`49f5a54`](https://github.com/Byron/gitoxide/commit/49f5a5415c119267ea37e20fb198df80f621cbde))
    - Release git-path v0.3.0, safety bump 14 crates ([`400c9be`](https://github.com/Byron/gitoxide/commit/400c9bec49e4ec5351dc9357b246e7677a63ea35))
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - Release git-sec v0.1.2, git-discover v0.1.3, cargo-smart-release v0.10.2 ([`6cd365e`](https://github.com/Byron/gitoxide/commit/6cd365e2cf6851f5cdecc22f3b1667440ad011b0))
    - Merge branch 'davidkna-admin-sec' ([`3d0e2c2`](https://github.com/Byron/gitoxide/commit/3d0e2c2d4ebdbe3dff01846aac3375128353a2e1))
    - Release git-path v0.1.3, git-discover v0.1.2, git-repository v0.18.1, cargo-smart-release v0.10.1 ([`b7399cc`](https://github.com/Byron/gitoxide/commit/b7399cc44ee419355a649a7b0ba7b352cd48b400))
    - Merge branch 'davidkna-discover-x-fs' ([`9abaeda`](https://github.com/Byron/gitoxide/commit/9abaeda2d22e2dbb1db1632c6eb637f1458d06e1))
    - Release git-path v0.1.2, git-sec v0.1.1, git-config v0.4.0, git-discover v0.1.1, git-pack v0.19.1, git-repository v0.18.0, cargo-smart-release v0.10.0, safety bump 2 crates ([`ceb6dff`](https://github.com/Byron/gitoxide/commit/ceb6dff13362a2b4318a551893217c1d11643b9f))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - make fmt ([`e043807`](https://github.com/Byron/gitoxide/commit/e043807abf364ca46d00760e2f281528efe20c75))
    - Merge branch 'main' into refs-and-worktrees ([`9cf0c7b`](https://github.com/Byron/gitoxide/commit/9cf0c7bd0cc5419137db5796f3a5b91bdf3dcc94))
    - thanks clippy ([`60cf67c`](https://github.com/Byron/gitoxide/commit/60cf67cb081b91932d9943b9c525cac2c0cf0782))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - Merge branch 'git_includeif' of https://github.com/svetli-n/gitoxide into svetli-n-git_includeif ([`0e01da7`](https://github.com/Byron/gitoxide/commit/0e01da74dffedaa46190db6a7b60a2aaff190d81))
    - Merge branch 'main' into worktree-stack ([`8674c11`](https://github.com/Byron/gitoxide/commit/8674c11973e5282d087e35a71c70e418b6cc75be))
    - set the time to wait for lock to longest expected runtime of fixture scripts ([`eea3988`](https://github.com/Byron/gitoxide/commit/eea3988462a61e8a64d646a15d062d13fdbfb615))
    - More robust archive creation on windows ([`e7b2d8f`](https://github.com/Byron/gitoxide/commit/e7b2d8f446b41b26b518abf7d1b048605ef2bbe8))
    - thanks clippy ([`658862e`](https://github.com/Byron/gitoxide/commit/658862eeb042073632f5a3f203e264a47151d454))
    - thanks clippy ([`c8d218c`](https://github.com/Byron/gitoxide/commit/c8d218c6399f52fb1a57eca22005196d1c686774))
</details>

## v0.6.0 (2022-04-04)

<csr-id-1d5ab44145ccbc2064ee8cc7acebb62db82c45aa/>

### Test

 - <csr-id-1d5ab44145ccbc2064ee8cc7acebb62db82c45aa/> ensure tests use 'merge.ff false' and recreate fixtures on each run

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 26 commits contributed to the release over the course of 210 calendar days.
 - 223 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 7 unique issues were worked on: [#198](https://github.com/Byron/gitoxide/issues/198), [#266](https://github.com/Byron/gitoxide/issues/266), [#293](https://github.com/Byron/gitoxide/issues/293), [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#364](https://github.com/Byron/gitoxide/issues/364), [#366](https://github.com/Byron/gitoxide/issues/366)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - fix windows tests by transforming line endings ([`e276d77`](https://github.com/Byron/gitoxide/commit/e276d777eb7a88dc424badbf88a929b5f567e5de))
 * **[#266](https://github.com/Byron/gitoxide/issues/266)**
    - a failing test to show the handle-stability doesn't quite work yet ([`5562e88`](https://github.com/Byron/gitoxide/commit/5562e8888cd8ac8fc3d89a41f8e8cc5cec7b8ca6))
    - refactor ([`c499843`](https://github.com/Byron/gitoxide/commit/c499843485a8af102cb4d3594c4e6014976c5aa0))
 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - REUC reading works ([`29c1af9`](https://github.com/Byron/gitoxide/commit/29c1af9b2d7b9879a806fc84cfc89ed6c0d7f083))
    - use parking_lot mutex to avoid poison errors ([`d8ca74f`](https://github.com/Byron/gitoxide/commit/d8ca74f358e802916353f545b90127f9a7bb5137))
    - base setup for index testing ([`aa60fdf`](https://github.com/Byron/gitoxide/commit/aa60fdf3d86e08877c88f9e4973f546642ed1370))
 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - upgrade parking_lot and cargo_toml ([`f95c1a0`](https://github.com/Byron/gitoxide/commit/f95c1a0d9c19bcc6feb9b8739a09d86f9970a0e0))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - refactor ([`9ea1e44`](https://github.com/Byron/gitoxide/commit/9ea1e4474a3ce803da7a56e1fc1748f65c11a876))
 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - add test-tools changelog prior to release ([`1ebc16a`](https://github.com/Byron/gitoxide/commit/1ebc16a6ac9ef188c188a52737820773aa949cee))
 * **[#366](https://github.com/Byron/gitoxide/issues/366)**
    - quickfix for unintentionally using 'unicode' feature of bytecode ([`fb5593a`](https://github.com/Byron/gitoxide/commit/fb5593a7272498ae042b6c8c7605faa3d253fa10))
 * **Uncategorized**
    - Release git-testtools v0.6.0 ([`45386a0`](https://github.com/Byron/gitoxide/commit/45386a0b135656681dbdf8c47ad888b50e68f151))
    - Merge branch 'for-onefetch' ([`8e5cb65`](https://github.com/Byron/gitoxide/commit/8e5cb65da75036a13ed469334e7ae6c527d9fff6))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - Merge branch 'main' into mailmap ([`b2df941`](https://github.com/Byron/gitoxide/commit/b2df941feaf5ae9fa170fa49270189f3527f2eab))
    - thanks clippy ([`1038dab`](https://github.com/Byron/gitoxide/commit/1038dab842b32ec1359a53236b241a91427ccb65))
    - add `fixture_bytes` to test tools ([`85e3820`](https://github.com/Byron/gitoxide/commit/85e3820caa106a32c3406fd1e9e4c67fb0033bc5))
    - Commit to using 'unicode' feature of bstr as git-object wants it too ([`471fa62`](https://github.com/Byron/gitoxide/commit/471fa62b142ba744541d7472464d62826f5c6b93))
    - Merge branch 'AP2008-implement-worktree' ([`f32c669`](https://github.com/Byron/gitoxide/commit/f32c669bc519d59a1f1d90d61cc48a422c86aede))
    - Release git-hash v0.9.2, git-object v0.17.1, git-pack v0.16.1 ([`0db19b8`](https://github.com/Byron/gitoxide/commit/0db19b8deaf11a4d4cbc03fa3ae40eea104bc302))
    - Merge branch 'index-information' ([`025f157`](https://github.com/Byron/gitoxide/commit/025f157de10a509a4b36a9aed41de80487e8c15c))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - Release git-bitmap v0.0.1, git-hash v0.9.0, git-features v0.19.0, git-index v0.1.0, safety bump 9 crates ([`4624725`](https://github.com/Byron/gitoxide/commit/4624725f54a34dd6b35d3632fb3516965922f60a))
    - ensure tests use 'merge.ff false' and recreate fixtures on each run ([`1d5ab44`](https://github.com/Byron/gitoxide/commit/1d5ab44145ccbc2064ee8cc7acebb62db82c45aa))
    - Release git-hash v0.8.0, git-features v0.17.0, git-actor v0.6.0, git-object v0.15.0, git-diff v0.11.0, git-traverse v0.10.0, git-pack v0.13.0, git-odb v0.23.0, git-packetline v0.12.0, git-transport v0.13.0, git-protocol v0.12.0, git-ref v0.9.0, git-repository v0.11.0, git-commitgraph v0.6.0, gitoxide-core v0.12.0, gitoxide v0.10.0, cargo-smart-release v0.5.0, safety bump 16 crates ([`0e02953`](https://github.com/Byron/gitoxide/commit/0e029537a7f6242d02ccf7e63d8d92f5246e6c5e))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - Bump git-hash v0.6.0 ([`6efd90d`](https://github.com/Byron/gitoxide/commit/6efd90db54f7f7441b76159dba3be80c15657a3d))
</details>

## v0.5.0 (2021-08-24)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 8 calendar days.
 - 12 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-testtools v0.5.0 ([`86e0a92`](https://github.com/Byron/gitoxide/commit/86e0a92c7dc3b69a766aeac1b675b148d61a7ec5))
    - Upgrade to nom-7 ([`f0aa3e1`](https://github.com/Byron/gitoxide/commit/f0aa3e1b5b407b2afd187c9cb622676fcddaf706))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.4.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 57 calendar days.
 - 64 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 ([`70ef344`](https://github.com/Byron/gitoxide/commit/70ef3442775b54ba9e4ee9ebfffb37af9804cc5b))
    - (cargo-release) version 0.5.0 ([`ae02dab`](https://github.com/Byron/gitoxide/commit/ae02dabae961089a92a21e6a60a7006de4b56dad))
    - [pack] refactor ([`9ee1e22`](https://github.com/Byron/gitoxide/commit/9ee1e22fa5c5d97ff626f0dfc44706272433bfef))
    - [ref] packed refs header line parsing ([`fde5543`](https://github.com/Byron/gitoxide/commit/fde5543ad22395e27266db02a5442a33d16e68c5))
    - [tools] fix create writable fixture ([`bf7783d`](https://github.com/Byron/gitoxide/commit/bf7783dd9ccc9ac433b978b9dded0d38f7351494))
    - [ref] on the way towards realistic transactions… ([`c808cb1`](https://github.com/Byron/gitoxide/commit/c808cb17b2fea12e018fabb789862e9b7703e49b))
    - [ref] on the way to setup the first transaction test ([`29c0b51`](https://github.com/Byron/gitoxide/commit/29c0b51625e2c7e3a8d60075bb925126a024dc83))
    - Bump once_cell from 1.7.2 to 1.8.0 ([`bd323d9`](https://github.com/Byron/gitoxide/commit/bd323d911b6becf8b379343c6ef56ec46e28fa28))
</details>

## v0.3.0 (2021-06-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 12 calendar days.
 - 38 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 ([`6b33678`](https://github.com/Byron/gitoxide/commit/6b33678f83e6d261ca15c4a7634ff5b4e66d81dd))
    - Merge branch 'dependabot/cargo/crc-2.0.0' ([`683c44d`](https://github.com/Byron/gitoxide/commit/683c44db682d8dbef401286963e84cdca145abc8))
    - (cargo-release) version 0.2.0 ([`3286e42`](https://github.com/Byron/gitoxide/commit/3286e42547b59df6365087cbae9ce1c9c959faad))
    - Manually fix crc in tooling ([`48fa9bc`](https://github.com/Byron/gitoxide/commit/48fa9bc80876a0186f43add6c6d3477385241f5e))
    - Bump crc from 1.8.1 to 2.0.0 ([`07f08ac`](https://github.com/Byron/gitoxide/commit/07f08ac1ea04ec278993ad1a5fc1d4f243bf8eb7))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com/Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - [git-ref] the first failing test ([`7e802a0`](https://github.com/Byron/gitoxide/commit/7e802a0576230dfc666c253d484ea255f265f92f))
</details>

## v0.1.0 (2021-04-30)

<csr-id-29bf8ca8399b6d4941aa242b9f08c74e59a179bb/>

### Other

 - <csr-id-29bf8ca8399b6d4941aa242b9f08c74e59a179bb/> try to disable GPG signing with environment variables…
   …but it's not picked up at all even though it's definitely present.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 276 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - prepare test utilities for release… ([`d35e654`](https://github.com/Byron/gitoxide/commit/d35e654747f96cec93bdecd1314ce325129cbc44))
    - [tree-diff] Beginning of more nested test-suite… ([`b8a90e7`](https://github.com/Byron/gitoxide/commit/b8a90e7c9347b0eefdbef6f4c724cc0561cd79c9))
    - fix debug assert, thanks gitpython ([`fe954b9`](https://github.com/Byron/gitoxide/commit/fe954b9f6d26bd8629f24a01bd2a06f9800deed0))
    - Revert "FAIL: try to disable GPG signing with environment variables…" ([`e326352`](https://github.com/Byron/gitoxide/commit/e326352eec7bd1aae13f770328979e5730ffc32b))
    - try to disable GPG signing with environment variables… ([`29bf8ca`](https://github.com/Byron/gitoxide/commit/29bf8ca8399b6d4941aa242b9f08c74e59a179bb))
    - Thanks, cargo audit ([`4f293f5`](https://github.com/Byron/gitoxide/commit/4f293f5036c44a69ccacf102d35202adad83bbe0))
    - thanks clippy ([`002792a`](https://github.com/Byron/gitoxide/commit/002792a8bc2512c92c16fd28662c26c9b3a12572))
    - Set environment in testtools to freeze repositories generation scripts ([`eaad3ab`](https://github.com/Byron/gitoxide/commit/eaad3ab69338115439a553ba1062160dc3a08082))
    - faster repeated tests if fixtures don't change ([`792277f`](https://github.com/Byron/gitoxide/commit/792277f241446086dd6c9b78f688363d4e66e5a7))
    - Allow the use of shared test utilities across crates ([`b117626`](https://github.com/Byron/gitoxide/commit/b117626df6da714c24d2b7914301678e89d2d0cb))
    - The first test with the new and nice and cheap journey test tool ([`d3c99e1`](https://github.com/Byron/gitoxide/commit/d3c99e1cf3125ab107e12718b39ac9b7c9a9165c))
</details>

