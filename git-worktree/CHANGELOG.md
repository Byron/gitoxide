# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

An initial release with the ability to checkout indices with simple files only.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 94 commits contributed to the release over the course of 59 calendar days.
 - 84 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 5 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#331](https://github.com/Byron/gitoxide/issues/331), [#333](https://github.com/Byron/gitoxide/issues/333), [#366](https://github.com/Byron/gitoxide/issues/366)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - Use hash_hasher based hash state for better keys/less collisions ([`814de07`](https://github.com/Byron/gitoxide/commit/814de079f4226f42efa49ad334a348bce67184e4))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - refactor ([`f86eacc`](https://github.com/Byron/gitoxide/commit/f86eacc5cfaf6d88ead4f8dbd65989d32674c213))
    - use io-close instead of close-file - works ([`279461b`](https://github.com/Byron/gitoxide/commit/279461ba1741ace0399127ca9089230082bbf3e0))
    - better error handling on close ([`a28c9b3`](https://github.com/Byron/gitoxide/commit/a28c9b32466a431450a504e313d2e49926e36a98))
    - try close_file crate and see tests fail for some reason ([`c7e1400`](https://github.com/Byron/gitoxide/commit/c7e140094a3a5947cf59107d5a621245ea2ecbeb))
    - more multi-threaded test stability ([`be5a19e`](https://github.com/Byron/gitoxide/commit/be5a19e0eb2e895d03b80afc24c7b8d2d436458d))
    - avoid racyness in worktree tests ([`c8a1319`](https://github.com/Byron/gitoxide/commit/c8a13198a12939befa473b30131e5a763c6fc28c))
    - stabilize assertions in parallel mode ([`21d6f88`](https://github.com/Byron/gitoxide/commit/21d6f880293de4e8ffc6a8472eb1b54d8b1b105a))
    - a reducer which produces progress reporting each time it feeds ([`e83079d`](https://github.com/Byron/gitoxide/commit/e83079d219c96692725ab8af1c0e656cb331ecd8))
    - call chunk processing in threaded processor ([`6bfd865`](https://github.com/Byron/gitoxide/commit/6bfd865a0578eeacd8d19eaa89d8914ac947c62a))
    - conversions from Rc to arc for Handle ([`c19331e`](https://github.com/Byron/gitoxide/commit/c19331e001e587e4fca74f3e9fec28a7df922c0a))
    - basic parallelization, without proper reducer, just so it compiles ([`5f29c0f`](https://github.com/Byron/gitoxide/commit/5f29c0f66d0aa6c045bfdf6f39a806ce8c4a5100))
    - decouple amount of bytes written from progress ([`9ecdade`](https://github.com/Byron/gitoxide/commit/9ecdade0f117b966c98f48d1879bdba21ccaafd7))
    - parallel and non-parallel tests ([`1cd7eb3`](https://github.com/Byron/gitoxide/commit/1cd7eb3f720e8b66792c942a99d7d9d85069ec03))
    - switch index checkout to chunk-based operation ([`e5f6943`](https://github.com/Byron/gitoxide/commit/e5f69433e4a6cc7866b666e0baccfa32efb92a7f))
    - proper handling of interruptions during checkout ([`7575a58`](https://github.com/Byron/gitoxide/commit/7575a5854ebe61a5941177efb470143192223ef3))
    - add thread-count and chunk-size computation; interrupt capability ([`8cbe85d`](https://github.com/Byron/gitoxide/commit/8cbe85d135898826a91939726465a9e295c1e24b))
    - refactor ([`542f49b`](https://github.com/Byron/gitoxide/commit/542f49beb811f7f9bf9dff3cd19694498f6cf9e2))
    - refactor ([`c3c31af`](https://github.com/Byron/gitoxide/commit/c3c31afb9dee5040abef7a8d6f8e1e2cba29e2d7))
    - fix windows test expecations for good ([`81bcb8d`](https://github.com/Byron/gitoxide/commit/81bcb8d281099e952a5e3c075d9578f15f2f2a0d))
    - try to fix windows once again ([`ff95265`](https://github.com/Byron/gitoxide/commit/ff95265a35fb9f340c3a9fa78f8beba24d6734ff))
    - some more debugging on windows ([`0c18443`](https://github.com/Byron/gitoxide/commit/0c18443f5195e10c99504c4f527c1882fcf84e45))
    - debug mode for windows ([`8f3bc5a`](https://github.com/Byron/gitoxide/commit/8f3bc5a3195770753b0b6445259ce20ab609b393))
    - See if we can remove symlinks this way on windows ([`0bc9489`](https://github.com/Byron/gitoxide/commit/0bc94891c92f324d3940e064e8918b117db4641d))
    - delete directories recursively on overwrite-existing ([`ea561e6`](https://github.com/Byron/gitoxide/commit/ea561e6f7d398991f214957dbd92e1b6a81e9ab0))
    - better symlink checking on ubuntu ([`facad25`](https://github.com/Byron/gitoxide/commit/facad25c08b82a975eda70493d4818ca7c560aa8))
    - overwrite-existing support with tests ([`49d1d34`](https://github.com/Byron/gitoxide/commit/49d1d34dff76d8b1e5e7fa9d08e6ead4e8bca018))
    - Fix dir-cache to properly handle its valiity which fixes test ([`52c0058`](https://github.com/Byron/gitoxide/commit/52c0058531df1a0f3fc755c5c51e71d34841cb77))
    - delayed symlink creation for everyone, but… ([`ab5cd3d`](https://github.com/Byron/gitoxide/commit/ab5cd3d383c3c6cb31a7b8d387daedacb9e3838f))
    - delayed symlink creation for windows, but… ([`77b053d`](https://github.com/Byron/gitoxide/commit/77b053dfd38e30a8ab397704059283a4766b9601))
    - prepare for first overwrite test… ([`cd6e086`](https://github.com/Byron/gitoxide/commit/cd6e08644df3a2b52aa70a2f37e988ec10b280f0))
    - fix case-insensitive tests ([`ccd25cb`](https://github.com/Byron/gitoxide/commit/ccd25cb5929554c69ea1250c6d2762fdd6ef5bbd))
    - Allow symlinks to dirs to be returned, too ([`d3d7a7c`](https://github.com/Byron/gitoxide/commit/d3d7a7c3c67868ba0fda6b04e6874aa2f91f638b))
    - try to fix tests on linux ([`9f9d36d`](https://github.com/Byron/gitoxide/commit/9f9d36d7d7bba443fba5917e9920911596fd64f6))
    - a stab at making file writes safer… ([`805c0da`](https://github.com/Byron/gitoxide/commit/805c0da62204b8c4675c9c098e10eb0fe2bc12a9))
    - mior refactor and notes towards parallelization ([`99de1ef`](https://github.com/Byron/gitoxide/commit/99de1ef494719cb4d46e3414474e619225fe7bd4))
    - return proper errors during checkout object lookup ([`f9beac0`](https://github.com/Byron/gitoxide/commit/f9beac0471a38cb4c3b070ecb576ed1a39456bd6))
    - switch worktree to thiserror ([`bacc654`](https://github.com/Byron/gitoxide/commit/bacc65481d4ff5ecfbdf3755383b60f354deaf47))
    - sub-command to print multi-index entries ([`6c10e09`](https://github.com/Byron/gitoxide/commit/6c10e097a432d81b930008abc00c6821ed7ac9be))
    - bring back more detailed errors in case of keep-going ([`8198817`](https://github.com/Byron/gitoxide/commit/8198817507a5e9c6e6fb847a45ac47bd38de68f6))
    - use progress to print errors right when they happen ([`af03686`](https://github.com/Byron/gitoxide/commit/af03686b5abf9548300a83329500b27acd66e16a))
    - implement 'keep-going' for index checkout ([`ecebc55`](https://github.com/Byron/gitoxide/commit/ecebc55f8321c67f57111f8d0002e75388dd3734))
    - Support for forceful removal of symlinks or files during dir creation ([`749c310`](https://github.com/Byron/gitoxide/commit/749c3100d785f7ac373bdb109fda21f2ac62d5c0))
    - forbid symlinks and files in the path ([`de58f50`](https://github.com/Byron/gitoxide/commit/de58f50748bd70e39d29e503a7f4b1e6c9b20093))
    - avoid popping the entire cached path ([`a3501df`](https://github.com/Byron/gitoxide/commit/a3501df6eb8d2fd3176434c80c443316e91dabb6))
    - basic impl of the dir cache which already avoids unnecessary allocations ([`cb36d56`](https://github.com/Byron/gitoxide/commit/cb36d5691294971e1b0e097ed11908768283731a))
    - sketch out dir cache and realize that git uses chdir ([`f4621cc`](https://github.com/Byron/gitoxide/commit/f4621cc4dd48fcd4b1aba294c811bc92f2715981))
    - allow writing empty files during checkout but also query the odb ([`5388d80`](https://github.com/Byron/gitoxide/commit/5388d8091ef02cf927478a1492847ae1666040d4))
    - basic version of index checkout via command-line ([`f23b8d2`](https://github.com/Byron/gitoxide/commit/f23b8d2f1c4b767d337ec51888afaa8b3719798c))
    - basic progress reporting for checkout ([`039e822`](https://github.com/Byron/gitoxide/commit/039e822bb4e56e49432db5c53081e0eb39588d66))
    - support for unicode-precomposition for gix apps ([`e90c123`](https://github.com/Byron/gitoxide/commit/e90c123675a98ab62fc6bb22019f889cee8b7301))
    - fix symlink creation on windows, hopefully ([`4b1650b`](https://github.com/Byron/gitoxide/commit/4b1650ba1988f52a7a91ce4f5327eca350f32520))
    - gather more information about test failure on windows ([`be5e3fb`](https://github.com/Byron/gitoxide/commit/be5e3fb3a19f86e37244b17055bf31cc455e78e8))
    - hopefully fix symlink creation on windows ([`acb8acd`](https://github.com/Byron/gitoxide/commit/acb8acd905c4a7ec0fbc831b159f626962c0a37d))
    - refactor ([`48dc401`](https://github.com/Byron/gitoxide/commit/48dc40195fd3d41d1fa5cd6326422ae18266dd7d))
    - also validate symlink collisions ([`322c316`](https://github.com/Byron/gitoxide/commit/322c3161947cd5c10e3122c097d5a888726d42c1))
    - fix compile warnings ([`58145bc`](https://github.com/Byron/gitoxide/commit/58145bc0fc329c370638a336215679fa727a9f0f))
    - try to fix windows ([`5c1e727`](https://github.com/Byron/gitoxide/commit/5c1e727a1af4b9a0b5b7dcfca0d1ef5a533a66b6))
    - finally an understanding on collision checking ([`0454e4a`](https://github.com/Byron/gitoxide/commit/0454e4a6f039541255728c4c8e076578236f0d86))
    - Add check_stat and trust_ctime options to index checkout ([`1a502c7`](https://github.com/Byron/gitoxide/commit/1a502c7e456a191d8639b799648ea33eb5a7dac2))
    - validate that colliding files are checked out ([`09fecd9`](https://github.com/Byron/gitoxide/commit/09fecd9687cf3271f7138bca9214ba99c17b5ef7))
    - support for executable bit check ([`267e3a7`](https://github.com/Byron/gitoxide/commit/267e3a7f4718c8f724e3e4488dd24dcebfc69413))
    - probe precompose unicode ([`0c1c006`](https://github.com/Byron/gitoxide/commit/0c1c00689000dfc943ed25cd52eac42e3642a78c))
    - refactor ([`fc816bd`](https://github.com/Byron/gitoxide/commit/fc816bd12f142d1df4d10429ee5b56e9eb5fbf4d))
    - determine filesystem case ([`f8e1de0`](https://github.com/Byron/gitoxide/commit/f8e1de0dc031ad73084b2da6a6d39960b9b78b4b))
    - basic test for filesystem probing ([`adbed12`](https://github.com/Byron/gitoxide/commit/adbed121f969a05b622d0325b434b3c6d44ae248))
    - symlink probing ([`1bfbf1d`](https://github.com/Byron/gitoxide/commit/1bfbf1d120e31474367cd2008e1715c50af19071))
    - make clear that we are currently only dealing with checkout during clone ([`178beb4`](https://github.com/Byron/gitoxide/commit/178beb42eaf1112143299eafa7fc93106eb9fc5b))
    - refactor for checkout to use fs::Context ([`8914fcc`](https://github.com/Byron/gitoxide/commit/8914fcc114cdf920f2f4162e71d4d390007f6f3b))
    - document-features support for git-index and git-worktree ([`1367cf5`](https://github.com/Byron/gitoxide/commit/1367cf5bc5908639e67e12f78f57835c5fd68a90))
    - Support for 'serde1' feature in git-worktree ([`f11929c`](https://github.com/Byron/gitoxide/commit/f11929c9652b2f414029f2ad02dacee238a138d1))
    - sketch filesystem context, without probing for now ([`de3749e`](https://github.com/Byron/gitoxide/commit/de3749e1426d48a1d31a0ddc1fddfdb394a01078))
    - refactor ([`004394a`](https://github.com/Byron/gitoxide/commit/004394ad04a965b631c5d75a7eced632540d9e1e))
    - restructure tests ([`831c429`](https://github.com/Byron/gitoxide/commit/831c4294c87aae0594e1238177dd71efb997cbde))
    - make fmt ([`636fa8a`](https://github.com/Byron/gitoxide/commit/636fa8a97ce56982c76dffc64ee084e31d39afad))
    - strucural refactor ([`cdca1df`](https://github.com/Byron/gitoxide/commit/cdca1dfec590d24dd42f34294e21f4bdf61d36ad))
    - Allow mutation of entries during iteration, while obtaining their path ([`d0c4563`](https://github.com/Byron/gitoxide/commit/d0c4563f71ea18aaf8ae21dd8646ab256a550594))
    - refactor ([`72af261`](https://github.com/Byron/gitoxide/commit/72af261603ee38651e15015547871d0510ce6370))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - Fix build ([`f6d9693`](https://github.com/Byron/gitoxide/commit/f6d969370b8ef05b3b29983dcd9f6fa11d6225f2))
 * **[#333](https://github.com/Byron/gitoxide/issues/333)**
    - Use git_features::path everywhere where there is a path conversion ([`2e1437c`](https://github.com/Byron/gitoxide/commit/2e1437cb0b5dc77f2317881767f71eaf9b009ebf))
 * **[#366](https://github.com/Byron/gitoxide/issues/366)**
    - the first possibly working version of loading a mailmap with multiple sources ([`98d745e`](https://github.com/Byron/gitoxide/commit/98d745e8080975a91cff1ce75e187258c851d3f4))
 * **Uncategorized**
    - Merge branch 'for-onefetch' ([`8e5cb65`](https://github.com/Byron/gitoxide/commit/8e5cb65da75036a13ed469334e7ae6c527d9fff6))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - make fmt ([`7cf3545`](https://github.com/Byron/gitoxide/commit/7cf354509b545f7e7c99e159b5989ddfbe86273d))
    - thanks clippy ([`07a4094`](https://github.com/Byron/gitoxide/commit/07a4094965ac1b4eb223da8e5ca5cc4a86c5f596))
    - thanks clippy ([`0e2a243`](https://github.com/Byron/gitoxide/commit/0e2a2438da35c0abb412682b103e5be171b1c3ad))
    - thanks clippy ([`3229240`](https://github.com/Byron/gitoxide/commit/322924037a1710f35e4134e5a35c82b3d4266a1f))
    - thanks clippy ([`a8e9497`](https://github.com/Byron/gitoxide/commit/a8e9497caebf1c0e9faac537717cd86378f1acf6))
    - thanks clippy ([`e04cba8`](https://github.com/Byron/gitoxide/commit/e04cba8837340d1ca0f102a340e52e8610fb0750))
    - Refactored code and tests ([`a4b880c`](https://github.com/Byron/gitoxide/commit/a4b880cf17665b61e3f7f193de57704b1db5318f))
    - Refactored tests ([`25a9dc1`](https://github.com/Byron/gitoxide/commit/25a9dc16dbb26e9aa0f3379b2af53cc0baa96663))
    - Reduce io calls ([`e838eaa`](https://github.com/Byron/gitoxide/commit/e838eaa5721d8b1b13155aa81234c9c44d9b15fe))
    - Refactor errors and remove unwraps ([`eaee855`](https://github.com/Byron/gitoxide/commit/eaee85595dc658549e62e3292b025ec016e70abd))
    - Implemented git-worktree ([`4177d72`](https://github.com/Byron/gitoxide/commit/4177d72c95bd94cf6a49e917dc21918044e8250b))
</details>

## 0.0.0 (2022-01-08)

Reserve the name for a necessary crate of the `gitoxide` project.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#293](https://github.com/Byron/gitoxide/issues/293)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - update changelog ([`b3ee7c6`](https://github.com/Byron/gitoxide/commit/b3ee7c6f7553de6bff4934bbdf38f6c6ea2cf349))
    - preempt the eventual need for a worktree implementation ([`bce67d8`](https://github.com/Byron/gitoxide/commit/bce67d8ec58f78a1fce1c76f7b93d9650f9f550e))
 * **Uncategorized**
    - Release git-worktree v0.0.0 ([`ddb1bf4`](https://github.com/Byron/gitoxide/commit/ddb1bf49e3b5b663fcf166d8cbce416e78d9fc18))
</details>

