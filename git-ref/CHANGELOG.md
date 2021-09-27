# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 9 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 145 commits contributed to the release over the course of 11 calendar days.
 - 39 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 8 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#164**
    - remove old and unnecessary experiment (aba8e5603833c85302db0b610802286a03a084df)
    - path::is (1f4e45a26a3d2727f00c3f248452dd41fc8a95be)
    - rename path::is_git to path::is (ac3b9efb7b90958274ce55800959d930f8641115)
    - path::discover (1958e8aa65eb97f9755f065d713f0a48c5e41b1b)
    - Avoid duplicate module paths in 'tree' and 'commit' (2f2d856efe733d3cf81110c0e0607d2e7c40d968)
    - top-level of 'path' module (066f59b23a125b1ce9a015437a3f4468e5791da0)
    - object_id (329d183ad4e256a4f9cdeb34589b5f3432495f79)
    - rename ObjectIdExt::ancestors_iter() to *::ancestors() (a19567eceab0dd7f5478b83c2ff9ce79754db308)
    - repository (1a1959f487d69ffdd5394775b707139c44dbd11d)
    - ext::tree (5e091fb2b4fd33879c176e6dadd3c9805d99af50)
    - easy::object::peel (e3760679547e0dc1bf31761acdb6e63b04a50919)
    - easy::object::errors (de004b318fdc6923711dd001bff5f4bcbba4270e)
    - rename `easy::Object::to_(commit|tag)_iter()`… (61793ff42f5c2f9ddf302901adea2dac6149eac8)
    - easy::object, sans a few child-modules (f582439a3efe5c234f54c488792395e9de09a032)
    - update 'platform' information to reflect the current usage (42080aefe3b286afb58235c1c22491579ab73919)
    - rename easy::reference::log::State to easy::reference::Logs (03fe8a7ebd34608d725d4585da5c1630123762ec)
    - rename `*::State` into `*::Platform` (0cd585e20a5abd323a34ec32d92fbd48531b3b18)
 * **#192**
    - smart-release: assure the current package version is actually breaking (fb750b65ca64c894ffb79cd0049f10a8db255ab6)
    - smart-release: better verbosity handling when comparing to crates-index (f6f2d1b2c1c50d36ee046ed58ffffed0444cd25a)
    - smart-release(feat): turn off safety bump with its own flag (a040f7d882eb5f6db0d54ba7e32437da3579a075)
    - smart-release(refactor): (443f000015de2117eae08fedf7d23f0d1ac6abff)
 * **#196**
    - Revert "traverse(chore): try git-cliff…" (cd293aee7cf7fefba9e1f61108eba5400e48b9a7)
    - try git-cliff… (cbc5b8171cdef5933d684c481300d9fcff43cf4b)
 * **#197**
    - smart-release: improved safety bump log message (9b78c344ee287c4c2908ccbe64bd64c2c9648459)
    - smart-release: commit message reveals safety bumps (b1a39046056bf4a862cebe69f44f3ea1e53a2069)
    - smart-release: released crates only receive minor bumps… (ecf38b8c013e46a33aa0b2c1b4e9cf547c8393c4)
    - smart-release(chore): update changelog (342b443a4f49736a10c2b311d69841dbf581ceec)
    - smart-release(test): way more tests to nail current log output (0d30094f4d397f932288f8c04ffd01f956113dc8)
    - smart-release: dependency upgrade works (a56bd7b134d315e22e5c8d01ca2d927de75955a9)
    - smart-release: calculate new version of dependent (c50704a0595884c3fb20629aba0f22bf99893cbf)
    - smart-release(fix): don't claim "conservative" updates for major version change (681d743e5579197d7262c40237dda0116fc4af1c)
    - smart-release: assure we can find non-sequential connections (798b650ad848001b10018087ed6c5d8a4055ece8)
    - smart-release: all logic to calculate dependent version bumps (7ca029c73eee51302d6828c6f9e8862d3fd4fbd4)
    - smart-release: an algorithm to collect dependencies by 'growing' (73794a4e382404cb7b684c9054278fb4ff8a84ce)
    - smart-release: foundation for bumping versions (d1145d1a6219ddafa7a41c82d6149b289f033640)
 * **#198**
    - Update changelogs (b30db3ba52d250ccc129208963ccc33eab6dc195)
    - introduce notion of essential sections in a changelog… (be891e20cb0152af52ceec47400cf3401e2112fb)
    - Preview changelog support for smart-release as well (b9e6de124eab5e961c1effe797a5e54e23228284)
    - Detect changes after merge; add flag for controlling changelog preview (6beb7345f0329592081c2955cf7ad2c9adf0e68a)
    - A lot of logic to handle messaging around changelog generation and halting… (28f6e181ff0e14e52704544bc6ed5f41bd7fb234)
    - Unconditional changelog creation in smart-release (48b52281f789a93415fefe38d661228ab582a107)
    - rename --skip-* flags to --no-* for consistency (3c0a6389fe5ff981dadca20e8a4a4a0d2ef66e13)
    - fix windows tests by transforming line endings (e276d777eb7a88dc424badbf88a929b5f567e5de)
    - Avoid adding newlines which make writing unstable (6b5c394f49282a8d09c2a9ffece840e4683572db)
    - Fix section headline level (9d6f263beef289d227dec1acc2d4240087cb9be6)
    - Write first version of changlogs thus far… (719b6bdf543b8269ccafad9ad6b46e0c55efaa38)
    - Make use of fixed git-conventional (b7b92b6c72051d462ab01c7645ea09d7d21cb918)
    - update git-conventional dependency (2d369e863b15269ba8714b025fe596f69e5b1217)
    - first test and sketch for stripping of additional title values (55b7fe8c9391e3a9562e084ae7524bb9f83ec36c)
    - Basic message parsing, either conventional or not, without additions (b3b6a2dc07c2eff38556ee66b9290b0c66b463ed)
    - Sketch Message fields from which change logs can be built (b167d39ecf0cd306dcf4d2c00413251cbfd02ed6)
    - feat: `BodyRef::without_trailer()` for more obvious access than `*body` or `body.as_ref()` (f0ea526775793c9104e4ae27dd5d92b5a1202c5f)
    - refactor (ef3fc6d92c1d751d0032e072834f41d37cbb9200)
    - feat: `CommitRef::message_trailers()` as shortcut… (5324391f581c5ad2c09604f0beeac7df852bfb33)
    - more tests for trailers iterator (c3b0161eb76aaf806a7d82232ec7ac1a304052a3)
    - feat: `BodyRef::trailers()` allows iterating trailer tokens and values (175e1cbdfebfc6f01784fffdaf0859cd6c23377e)
    - Some tests and sketch for BodyRef parsing (3953c245461941c636ce5d755e6a469f7fa3eabe)
    - feat: CommitRef::summary() and `MessageRef::body()` methods (1714d05df812aa373da485492b342e58e9e7c17d)
    - refactor (7055dc81e9db448da89ab2ee0ba2ffe07cd00cc2)
    - Another test for footer separation, simple version (b4391862b67a09330476a82d520bfc3a698a4fbe)
    - Return to safety (35313b9f7c92edd1afdbe22d1f592baabc0abc9c)
    - omg nom parsing works… (cd11704dd0d469cd23d7ee6905d6219b26ba4563)
    - FAIL: not really successful to continue down the 'fold' road (d9afc22f161fb60195571be09d2d816d67050575)
    - three tests failing with nom (13646e8dfe97d8503d0cef935c4c303f82271aa4)
    - Revert " FAIL: try to use nom-way of the previous body parsing…" (d1e6f621c2898ad9f03b2ee712019e6a10b44035)
    - FAIL: try to use nom-way of the previous body parsing… (909f6682ac1de6be0eb8b66015b3f250daca17cd)
    - sketch nom version of the message parser… (1ec47ded5793cac1f2633262d59bfbae4a0e14be)
    - Research commit message trailers just to learn that I want to skip them (c332b8fb335f6c4081add894c3fcdcab298fc828)
    - Fix build (d0a956fdb5a822dbd116792bfbe70d1532a95ec9)
    - refactor!: Use git_object::commit::MessageRef::summary()… (13e7c3ad5e079fe778d07d115c9e41c4c6eb038f)
    - feat(commit): A summary for commit messages suitable for logs (cd3fc99968baa827302aa9c4f5d181bc9c4f9084)
    - More message parsing tests with windows line separators (001e8c2a4ede5fc025572a4c39a771cc854f8b18)
    - A manual message parse impl and more tests (f4b8a0da787f9a16eebd2a36b342f5a2a66edabd)
    - More message parsing tests, now with legit failure… (625be8dbd4204ea1a7131ade9b17f63dcc7e30d7)
    - feat(commit): Add `message()` method and `MessageRef` type… (6150b2db18034d5912029deac51d1affb38ae7b2)
    - Sketch data for parsed messages (32dd280eaada635994e11b4f2722a4efc59faa8f)
    - smart-release: add git-conventional (0c355ed24eb230e9834e797d5c8dc72ae21f0c46)
    - smart-release: consider nom for custom parsing, but… (5fc33266b2626a07b19d2f5bd075e2c600204a3d)
    - smart-release: refactor (17322fa378fdecad80ad1349292aaaee8bcd00f6)
    - smart-release: refactor (ac0696b8226a1478fa90b932306f35e5dbf464b1)
    - smart-release: refactor (87ebacc65f56f8765eb787fea1bd27f2c99dfd97)
    - smart-release: a seemingly slow version of path lookup, but… (41afad3386461b658ee859225785b6de86d13cfb)
    - smart-release: fast filter by single-component path (ae7def47388aeb56c7df4a73fd13ff508cee7017)
    - smart-release: prepare for fast lookup of paths (fbf267eeb424bf90649be278ee847fe3f2a3db80)
    - configure caches with env vars using `apply_environment()` (d422b9a31a37a03551bec4382039aaf3a7e49902)
    - refactor (e7c061b10c263001eb4abf03098d6694b770f828)
    - set package cache via RepositoryAccessExt (66292fd1076c2c9db4694c5ded09799a0be11a03)
    - smart-release(feat): Add GITOXIDE_PACK_CACHE_MEMORY_IN_BYTES=536870912 to control pack-cache size… (5aadf75a0d93d1a990ad0305c38366c5c22bdcb2)
    - allow disabling the pack cache with GITOXIDE_DISABLE_PACK_CACHE (d79a1b75304e397c16b5af7055906591a187ddfd)
    - prepare for configurable pack cache (7d2b6b66e09ff39727fccd68d190679b52d90126)
    - object-cache to allow for a speed boost… (06996e032b1e451a674395ebaca94434fac46f05)
    - smart-release: actually build the segment vec, without pruning for now (422701be4ed6d2a61361af9b6eb0f4f470d1d782)
    - smart-release: build commit history for later use in changelog generation (daec7167df524b329daad7dabb1b9920b6ef8936)
    - Allow object access during commit ancestor traversal… (4fe4786797d240a59d29dbf2c6310490a381c8b6)
    - smart-release: sketch history acquisition (debe0094826f83839f907523715def929133fd58)
    - various small API changes (89f15051763a03627f332c46beedfc53b8b9b15b)
    - add panicking `Target::id()` and `TargetRef::id()` (4ed4b2da50557aff540685441f4b5c7d5e582977)
    - add 'Head::peeled()' method (56e39fac54bfa3871c42bbf76a9f7c49486b85be)
    - move easy::head::peel::Error -> easy::head::peel::to_id::Error (f644d0ede7a2e8d344a81c7003c3877eed64a6b0)
    - smart-release: some performance logging (1954b467cf1e97e22629c55487b4a66cb1380a89)
    - smart-release: build ref lookup table (9062a472ac63887900562ed341c7b68665b8587a)
 * **#200**
    - feat: Lift io::Errors to response::Error::UploadPack(…)… (f293b633d16c0f7393d0ede64e12f14e47d0296b)
    - feat: Add --reference/-r flag to gixp pack-receive (637d12cf368e044f59ccde37c6365d9528d2c43f)
    - feat: add git_packetline::read::Error to represent ERR lines (454c840c652ecb5774d2e3d37be14db42749ea63)
 * **#205**
    - '(null)' symref targets are turned into direct refs instead… (c77bd7a01820110154f2c66cd954c1ccfff173c1)
    - fetch::Ref::Symbolic::target is now an option… (da68bfb8104ecf58e73e3f99d87f81c90712a2ca)
    - validate assumption about '(null)' as ref-name (2576168aa04d6451fc29c5dadfa698a9a50dac32)
 * **#67**
    - describe variants (899c57927ce4ba2e1b5d8182f9e731c2a9094cba)
    - parse entry mode into number instead of comparing it to byte strings (83d591d536d1a43e8523082824ec0b95cca55d34)
    - ObjectID specific hashers, using the fact that object ids are hashes (f9232acf8e52f8cd95520d122469e136eb07b39f)
    - Tree parsing now probably is twice as fast… (d1e2b8910b454dd798be8a9a43871f0b0644d503)
    - Use a custom hasher for 'seen' objects hashset… (70179e2cf8d15ba4e1cf8e94a9915bf5b02cf755)
    - don't include submodules in count… (faf6f813927720c5adf62102f9ce46606ff2617c)
    - control pack and object cache size in megabytes (60c9fad8002b4e3f6b9607bba6361871752f4d3d)
    - Use 'cache::Object' trait where it matters (71c628d46088ab455b54eb2330d24dcff96c911d)
 * **Uncategorized**
    - Merge branch 'changelog-generation' (bf0106ea21734d4e59d190b424c22743c22da966)
    - thanks clippy (b856da409e6a8fdc81ea32ebb4a534b0e70baebc)
    - don't claim to change manifest version if it's the same one (11eebdcc572a72b2e66a9db3cae0a01f12a81619)
    - thanks clippy (d78d3828c7f80963c0b8803cb64e0ae5e08d0ba3)
    - Bump git-traverse v0.9.0, safety bump 8 crates (d39fabb8757369aa19452a457f610fe21dc13a14)
    - thanks clippy (2b5542761ab160cd9460b133928efd6f0cb55e75)
    - thanks clippy (4ea11264296063278977c5539e2d68367475464a)
    - thanks clippy (a554b9d356d4e44c9504f7b35aa2c4f9c660df9b)
    - Bump git-repository v0.10.0 (5a10dde1bcbc03157f3ba45104638a8b5b296cb9)
    - thanks clippy (d15fded08224c45dcbd34cf742398e2594f39964)
    - thanks clippy (e56af5a0846652f177a88771d495bff5973abde3)
    - Note about git-subtree… (4b48a1404444731fead6b4d3a691a06f377f789e)
    - thanks clippy (ae7826e1cf79fce6ad12f407162f58b3bfb02b16)
    - [repository #164] docs for easy::reference::log (7de7c7eb51b7d709fd140dbf789e31e97161bfa7)
    - [repository #164] docs for easy::reference::iter (d86c71363a5a73dd8986566a9687e2b4756972cb)
    - [repository #164] refactor (437e63b4e841ef478c12a91bf3e2dce63d5b1041)
    - [repository #164] docs for top-level of easy::reference (9e465e03dc636c360128c93864749c4a3f8a99e5)
    - [repository #164] fix build (1db554216e99c5df62a2fc7fa3f8693fdc35b3eb)
    - [repository #164] docs for easy::oid (b66b6fe759eeb55cb875fcb65aa58b62c6963ca8)
    - thanks clippy (b02edb5b1e9b7c8f8bd1b4a8e2d60667da629839)
    - [repository #164] docs for easy::commit and easy::odb (abf37e54e5a4584f521988e27dd02f6d6badc4ef)
    - [repository #164] Documentation for `easy::borrow` (3e612f441e1e837d7ba3d3ddd40b4a8c2ba05c61)
    - [repository #164] docs for easy::head::* (516fde7ffb505603479b4de2a78200da480b66ed)
    - [repository #164] refactor (65b0e0fbe7ab7cb405fd267802e7ad3de36d98f7)
    - [repository #164] docs for `easy::ext::ReferenceAccessExt` (ab4910f1b4bf98569a04596b43aba862caca029b)
    - [repository #164] docs for easy::ext::RepositoryAccessExt (9041d474f178f45c86d628a7140c64810365b97d)
    - [repository #164] another test and fix for `commit()` (8d676d77cb69df203d3fcbf8c1a34f212035605f)
    - [repository #164] easy::ext::ObjectAccessExt docs (c4984af4f6343a17290f6c85f8385e77354875bb)
    - [repository #164] (4111d22ebe4cc9ddd726cce566e5872708067440)
    - Release git-repository v0.9.1 (262c1229d6d2d55c70fe0e199ab15d10954d967b)
</details>

## v0.7.3 (2021-09-10)

- Compatibility with Rust 1.55. It informed about the incorrect usage of `std::io::ErrorKind::Other` which this crate also dependent on causing
  runtime bugs. The latter were detected by tests and this release contains the fix to not rely on certain behaviour anymore.

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release.
 - 4 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#198**
    - loose reference iteration with non-dir prefixes… (293bfc0278c5983c0beaec93253fb51f00d81156)
    - Add 'references().all().peeled().'… (650241251a420602f74037babfc24c9f64df78d8)
    - smart-release: filter refs correctly, but… (2b4a61589a7cba3f7600710e21304e731ae3b36a)
    - smart-release: find tag references by name… (72e175209441b12f3d4630e5118e21a3156146df)
    - commit traversal along the first parent… (7bce49c1d27cb279b61ff51de0038e01fcf3561e)
 * **Uncategorized**
    - Release git-ref v0.7.3 (b0a98157ab3b240af027acb9965c981a543e55fa)
    - try not to force native insutrction sets (53ea9c83170affc3cdb36700b0485d832a36f983)
    - Release git-tempfile v1.0.2 (310ea7336e78fbedb2cefa1ecb773b25e6a77e0a)
    - Update changelogs once more… (d57d279dc603cf450c151bbb6dc6c6505cc6da10)
    - thanks clippy (68ea77dcdd5eb8033618e7af2e3eb0989007b89b)
    - [repository] don't enforce feature flags that may fail on windows by default (afdec2e89eee0397b16602fdff16d3997ef370d0)
    - Dependency update (d2f23f80d67b2207fa72d43a967ef1cc9ddd381c)
    - thanks clippy (7899ef10f2f4a6df43beed598ddf396991dcd9e5)
    - Update changelogs retro-actively… (78cfe0ac341c6c2257743d913884b50042078e6c)
    - Release gitoxide v0.8.4 (effb2a612d5912ea7bd9e7c65465ca3da3797a7a)
    - Release gitoxide-core v0.10.5 (590e59b2b41a419574443e6b850bdb119a172279)
</details>

## v0.7.2 (2021-09-10)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 26 commits contributed to the release.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#198**
    - git-ref(docs): improve changelog format (90e6128727932f917c485f411e623fc6a9c2ad4d)
    - smart-release: sketch first step of info generation (ff894e5b0257722c31578772ed694324194c0741)
    - smart-release: changelog gets crates to work on (78d31d9de2710b4369862c1226f18d4a2d79a9c4)
    - smart-release: handle unborn heads (0e02831fff83f6d6b0ea8889d54196e54e4e4aff)
    - smart-release: fmt (d66c5aea01a7d1df2cc539c52b789ad39a058ad2)
    - smart-release: refactor (d4ffb4f2ac935f6345bdc7d03cc1878007609503)
    - smart-release: refactor (9fc15f92ddec4ccfd0803d2b1231ed08d424cf33)
    - smart-release: refactor (9e430df135e87ee9e9673e7d52f072f39abaf4d9)
    - smart-release: initial test for changelog (a33dd5d21039441556ab89c997195f1bcc5bc543)
    - smart-release: very basic support for changelog command… (1a683a91a2850d663cf87fb326e5ab66ae86fc96)
    - smart-release: add 'cargo changelog' sub-command binary (3677b782f8bc63a38d4d49b8555b5a6b9a618f84)
    - smart-release(test): add changelog to most tests (cdf41998360527161a1b04821bab377489f6c5f0)
 * **Uncategorized**
    - Release git-ref v0.7.2 (e940e9a21938035eb8791bba19cc16814a0fb4e7)
    - Release git-protocol v0.10.4 (898ee08befa1eb7dd22980063c7633f83d0a8958)
    - Release git-odb v0.21.3 (223f93075a28dd49f44505c039cfeae5a7296914)
    - Release git-tempfile v1.0.1 (295eb374d104ac2775b9f864ef3234e2c5832b54)
    - [smart-release] auto-detect changes in production crates as well (24bc1bd678602e6b1af771b0b47eb3a39f8aa3a7)
    - [smart-release #195] update test output to match CI… (f86438609a1f99173efbe6b1fe91229433c1fc76)
    - [smart-release #195] better error for untracked files. (f5266f9756b1dbb9dc9846ba6efb863bdc12ae35)
    - [#195] Provide nix-shell target for macos… (5e75e8c6690f851bbbca5888d3f7c9880316a620)
    - [tempfile #195] adapt to Rust 1.55 (d9e71acc5d619b5e78673da4fbb5a531c97ad6dd)
    - [#195] Fix previously incorrect usage of io::Kind::Other… (4dae07dc7f562395a174be6cb2220e754ff902f7)
    - thanks clippy (4701296bd5e2c4ad2f80f4e1de498db49f93385a)
    - [smart-release #195] fix docs (8d7e132d055d8c87ea3e45de15593964a61b0608)
    - improved changelog… (8b82f7d44c7eb63b7922ddc31ada9cefdce776b0)
    - [smart-release #195] assure dependent packages are not packages to be published (6792ebc9d09aec81ebc81b3b0fa58ca7c6ce4fcc)
</details>

## v0.7.1 (2021-09-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.7.1 (d34191dfd3ac3b34a3ae0d772c8b4302e5115fd6)
    - Bump git-object v0.14.0 (d4fc81f6390443f8c8561d91ac27ea4a6318fb62)
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

 - 76 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.7.0 (24300b169bde2269bcabf8eac31d84e196572bf1)
    - Merge branch 'repository-integration' (49f5453629646ac24d752f53c532e5f67eb09374)
    - [features #189] simple UTC-offset support for git-features (b58134bbd132f9e685d1adf7859ec5219c16dd25)
    - [odb #190] Read all eligble packed refs, no "pack-" prefix needed (ab250f76b356c0937ada959591dc4df3872acf8f)
    - [features #???] WIP local time (1388ebf0925eb326ec3045d7f83bd5beda22a6fe)
    - [repository #190] test for oid.ancestors().all() (fdc3678c63fa128ac754b3fa9ae3d88a4a221d0d)
    - [#189] remove special handling of time from deny.toml (72050ef6c425769ee8e23adddf9fb43782adb811)
    - [repository #190] fix build, lets just make traversal available by default (6da35994cf2a3c9ab741733af53761c9a2cebeed)
    - [#189] Upgrade to prodash 16… (8e98418652926860f58906a6f21a3210e2f0183f)
    - Bump git-pack v0.10.0 (e5e3c8024e1c2e5e90cee83abbdae41d58eee156)
    - [repository #185] rustfmt (dfbb015a89db47c79015135870013ecc384c4aea)
    - [repository #190] access to repository directories (f4d1ec4ac0be8aa46d97eb92fb8a8f3fb8da94fb)
    - [config #185] refactor (509c938dd061060141756ee791cdcb6017934fe2)
    - [repository #190] first shot at ancestor iteration… (85f1a48ea39f3b224e8d0ba3728dd75e03a6edc3)
    - [config #185] Count lines correctly on windows… (57203ce5d5e3c481b69c3ca173e4b00f11aaf7d7)
    - [repository #190] refactor (e7188e047529cb0f4b20b3876f36b4592e9d2dc4)
    - [config #185] add test for handling windows formatted files… (2a2a89f68cc45e27a1cf0d33fc644ebabc762302)
    - [ref #190] refactor (010be48d2cd2dfebf7a1b6302e94b5f2e95fedc6)
    - [repository #185] remove quick-error infavor of thiserror (212c44c84b903681f6d35d934ee5f7ad6e1da791)
    - [ref #190] fix tests (e426e15188d8ec38ee0029f1d080dbab9afd8642)
    - [repository #185] on the way to removing quick-error (6ecd431661e7ddc2f97e5a78a7932d2a7f1f27f0)
    - [repository #190] fix tests; needs inbound transaction handling… (e5a5c09bb108741fff416672566e381f50f02b38)
    - [config #185] flyby refactor (9b9ffa3c1d5ccbea22aa38b740daa8a349494395)
    - [ref #190] don't provide namespace support for loose and packed refs… (c663da16646bc3371e5a31f5c488a775aac4f795)
    - [repository #185] support for initializing bare repositories (9e8a39e3cbd620bd48f379743df0d5783c33a86f)
    - [ref #190] find() with namespace support (1240c21a353c7df736f40b6639076af94eae0f15)
    - [repository #185] use git-config to handle bare repos more properly (8a5aac55cf62bdd7287a363fa29f12aa39d4c583)
    - [ref #190] prepare test for namespaced find(…) (5fcd0e4c3c803a372360ef4cc2a7663b17ccebdb)
    - [repository #185] sketch of how to open a repository… (48207b54b97ac1b6354f6b53c13ccc4d1d8ea98f)
    - [repository #190] leverage git-ref namespace support (1aa9c113488175f03758f8a64338a33b3417dd87)
    - [repository #185] refactor (63089ff356ea0f62963ae213ea0dbb09f891ada6)
    - [ref #190] iteration with namespace support (d5987d41753cf083573d86e8d5bc86c7a825605c)
    - [repository #185] refactor (7604935b12eacb26a98bedc5f77636b5583629a5)
    - [ref #190] refactor (3c7968c7fe8ac166b01f5338b23f817899dc085e)
    - [repository #185] refactor repository initialization… (5ff7eaa86bddfa94aec97355a5d6adb117045693)
    - [#190] disable caching to see if this fixes windows (0660a6f8fcb5a51a4661dd8b3e2e43a07b5e1d3a)
    - Notes about 'git-notes' and 'git-trailers' (459dd37a1b7461e4e554764fc780f49ff221f2c2)
    - [repository #190] prepare for namespacing support on file store level (d2d1db647e6ad0dd92b88ce57df866f5195b8dd6)
    - Release gitoxide-core v0.10.3 (e1326808a24fa7e797106cbd4bf3f34aba59b148)
    - [repository #190] refactor (609c249916ca64f4beecdab86eb4562adbd1ca4f)
    - Release git-protocol v0.10.2 (54a44009e3507ee1c53a51a5f3b6735b6115a887)
    - [ref #190] refactor (1ef6cb344176aeafcc61a1f1af503a3f8afd1f77)
    - Release git-transport v0.11.1 (0952976eac1dac9b8f351ecc9867746b650377f9)
    - [repository #190] fix build (f5e118c8871e45ed3db9da9cd6bc63a5ea99621e)
    - Release git-config v0.1.5 (150ed760c8b357e5c40ec0bd8d0cd849b39c34c0)
    - [repository #190] note a known limitation about finding references in namespaces… (d3357318cf100fc3e0751e5b6de3922b1c209ddb)
    - Release git-commitgraph v0.4.3 (7dfe16bdebaf971b7101331ad037d1ca8ab491d2)
    - [ref #190] more assetions to understand 'find(…)' for namespaced refs… (f58a0ff8be6144d1dcb97f9b8030e1ee36ce41de)
    - [various #184] configure docs.rs build features (cc502492c512293e93e95610ca80a71896076ded)
    - [repository #190] transparent namespace support (d14f073707c2f4641a271ba7965ec8281638e8df)
    - Release git-repository v0.8.1 (b269a1264f830bafcfe74f0f3ce01448c894146e)
    - [#190] run tests faster (at the cost of compile time) (a22c95bac4947c849ad1372a9588ea945d14fd49)
    - [repository #164] make EasyArcExclusive available (2fa3dcb40a34a7ec19382e5f6a71348ecf7a7c36)
    - [#190] faster builds with debug=false and dependency caching (0b0fea4f6315373f1c1c103fa50ef6f798e9d7fd)
    - Release cargo-smart-release v0.3.0 (82f0cec9c8f0f5610ddbd6cd1ac0716a9633d7c6)
    - [ref #190] Make References sortable (16b2232c70ad331e17e76ccca3b950963906aa81)
    - Release git-repository v0.8.0 (15ae2b8a43fd35615cf57e8088166cdd2a7cc47d)
    - [repository #190] turns out we need bstr with unicode support (3d8796e670f9bb5d2ed22fb3b75130a599737341)
    - [repository #174] keep assets (e0fca771f5ee068b0a9a0975930317d0883701cc)
    - [repository #190] public bstr re-export (3b7ffde385b1984393ee65a7505ad7221fecd0dc)
    - [repository #174] remove arc_lock code entirely (dcbe742eb5244f0b5c6563cf59962183b708f54f)
    - [repository #190] cleanup usage of bstr… (e4411ff43b24af79fefeaa4411e004dc504a4e2a)
    - Release git-repository v0.8.0 (1c9c5f1600363aa42e5310096804bb3dd17f789c)
    - [ref #190] more conversion trait impls (1795a333c05c60a1a2f3164d5c4c78289eb7050c)
    - Release git-protocol v0.10.1 (cec8ee3709ed401303cdd412a53e73f91eced619)
    - [repository #190] prefixed reference iteration (a6e19c9a49bdc6a7c5cabef0a8d93bfd48a74fcd)
    - [repository #174] conditionally compile future parking_lot version… (5375fc872b9af2526683326f58e9c3d7f20ef166)
    - [repository #190] implementation of reference iteration (all() for now)… (2c0939a146b5973de26bd03987e075a34a84bc88)
    - [protocol #174] fix tests… (cdc16fc0ef42df4a17ec4fde4be4511ee2cdaed6)
    - [repository #190] refactor (8c532a4c78452dd11115cf36a906a27741858774)
    - [smart-release #174] add asciinema recording of failed release (6668527ee961df214bda41619d6fb76540b0dda1)
    - [repository #190] prepare reference iteration (427f14622fb98e0397de2cae4d36a29f5915d375)
    - Release git-repository v0.8.0 (e191eab555de2c932830f143cbbda71690ec9874)
    - Bump git-hash v0.6.0 (6efd90db54f7f7441b76159dba3be80c15657a3d)
    - Release git-repository v0.8.0 (403ef0aec7d106302d7baa12166fd1ce985c9c02)
    - [repository #190] obtain the kind fo hash used in a repo (a985491bcea5f76942b863de8a9a89dd235dd0c9)
</details>

## v0.6.1

### Bugfixes

* splits of edits to symbolic references will now 'move' the desired previous values down to the
  referents while resorting to not having any requirements in the symbolic ref instead.

## v0.5.4 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.5.4 (bc5d860a616fd5a4371792a8ecde6e6356e217f8)
    - [smart-release #162] FAIL: one level down, using the cache isn't really working… (65db0104146248b273081fc6616a6ed484aa948e)
    - [ref] Out of bounds check to prevent legitimate panic (303608cbc1ade71c635dd1bbbe60988d09184351)
    - Apply nightly rustfmt rules. (5e0edbadb39673d4de640f112fa306349fb11814)
</details>

## v0.5.3 (2021-08-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.5.3 (e6a8020ff9b85c6dfedd80525c571514e039edae)
    - [ref #157] Support for unsorted packed refs and those without header (272468892c02133efd68d15ffc5cacb4d5c5cd78)
</details>

## v0.5.2 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.5.2 (50dcca97e207ec608e506adcef90dd0599b4441d)
    - remove dev-dependency cycles by removing their version (c40faca41632cd2a226daf4ddf5293b65d1fdc82)
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 (f123f69c7a4f9fd1c98bd2f60ebc953a6739fe04)
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 (c67291ff9bcdff9a747d87241f6a71015607af05)
    - Release git-object v0.12.0 (7006150ac314d19814608723f69f6e70a72f9262)
    - Release git-actor-0.3.1 (727087dca243da4bc40bc87611a2f66234565be7)
    - [utils #154] commit manifest changes; create tags (95dcd9d7d060101596c51116218102cc8049d0dd)
    - (cargo-release) version 0.3.0 (263088b3faaccd9edae8c21dfc7d39b191d76207)
    - (cargo-release) version 0.18.0 (b327590d02fec5536c380b2d39dd7be089ca7c40)
    - (cargo-release) version 0.17.0 (c52a49176bd294bb36db74b4293cdb684a2ab7f6)
    - (cargo-release) version 0.6.0 (d58f37e3b5a000fbe069aa869bd84f66d5c3210b)
    - (cargo-release) version 0.11.0 (a5be31c4cf7c0b538a1ed4a52ff5c3a992c6feff)
    - (cargo-release) version 0.5.0 (bf15c2a2f285046b094093760c1969007ee75e25)
    - (cargo-release) version 0.3.0 (64efc0534ddc372b6e668b23c1e9d276098679c9)
    - (cargo-release) version 0.4.0 (70ef3442775b54ba9e4ee9ebfffb37af9804cc5b)
    - Revert "[ref] break dev-dependency cycle" (436e89b18cb157b3d30bd24b8d1acef25631ec2a)
</details>

## v0.5.1 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.1 (6f61fcaf9528f2ba6752ce94524b59ff505cc518)
    - [ref] break dev-dependency cycle (d5af42898487a82f2fbd000fac2f0db9505a587c)
</details>

## v0.4.1 (2020-12-19)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 46 commits contributed to the release over the course of 88 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.1 (25d2c2e6ae70f46869ab0dabdda2b9f7840539d3)
    - Document `git-ref` (91dce23c8faf74511c33e5cfa07d2f293b1cd0a2)
    - remove dash in all repository links (98c1360ba4d2fb3443602b7da8775906224feb1d)
    - increase git-odb crate size limit (75bcc85ec0fffcab3a2c8d06962ba99ab6e041e7)
    - [commitgraph] Ditch pre-generated test repos. (1ce84689ee89eb0f9e4f57cdba3a5ccac4a1a12d)
    - prepare for unquoting c-strings (47e2fa03a1e2fe163c5c019d52bbb0ddbdb80bf0)
    - [commitgraph] Include in `make check` target. (724f39113837e8ee2321312b9b7421a48fc47b99)
    - Read multiple alternates from single file; ignore comments (1f8d36705c4568b1036b0d62b3a80ae6ec20a99c)
    - [commitgraph] Remove `Kind` enum. (3c927610eb717645e7f83a257184e44f76918571)
    - support for relateive alternates (b20e9eea423ced275781d410217110c85ddb587c)
    - [commitgraph] Take `info` dir as arg, not `objects` dir. (36953e0ec6119e1a01ae9b8e46e40bbd083e732c)
    - Ignore all cycles and be happy if we have found at least one actual odb (1effdfda703d5eb9cd1333a7bae21075ef9e53cc)
    - [commitgraph] implement basic, low-level read API (d1f0e9cbd259b460a7d12ae068fb95ede0000cb2)
    - prepare for multi-line parsing and all the bells and whistles (08f9ec41feee56fe0ff2b057bb50391100bdb84e)
    - Revert "FAIL: try to get rid of tree-traversal Boxed error…" (1b42b3141dded644a17c8d23057c987e2bac4f80)
    - Make compound DB initialization less lazy… (6dc57b31d0bc5abfca100ab1d4b5dff68852aad8)
    - try to get rid of tree-traversal Boxed error… (13159eb972ed78ce4ebee2313b288023cec91c47)
    - Use parallel walkdir (via jwalk) when parallel feature is enabled (f444c859f5b215ea70a46d5493a2babbf7a98235)
    - Parameterize traversal error with Processor error (1513a13179bedefd12fc08da07a05c7f07ed4ef6)
    - alternate now handles cycles (71167e4e50efa8a097c3b09a249004e97aeaf2c8)
    - Switch to prodash 10 and safe a lot of trait bounds in the process (e2fb1d944b4d803a11c91f868b831d406fb5e35f)
    - first simple alternate tests (73721185cfd646c6e83b2548280fad8f480f8324)
    - Prepare next iteration (4f656b269918dd7d62851c986b7c40a898cd6a5e)
    - test for circular alternates (fc927091d69196a930c0cea4611af8d96b7b84d8)
    - Provide terminal dimensions to better use horizontal space (11f6b8497a5089377e605f4cbe1cd317ef677d59)
    - dependency update (6b0796a59707efde5d8ab21854a4b798cc95ae4c)
    - asciinema link for pack-receive (79ac34c1301c44eedb3d0c25233fdc78709e5669)
    - thanks clippy (4ddc64fd71d3d1260e001f89c379c46fe157e5ce)
    - asciinema link for remote-ref-list (aafd5f852284a3799fa189591e95389e9bd78dbc)
    - Actually resolve alternates when creating a compound DB (9be7aed7bd4b939d98b9a8d1db8a6ffc85044ca9)
    - (cargo-release) version 0.4.0 (f667785e9e9db16e1afb498c06ff89246e4f6aef)
    - refactor (c1eff58cd28e45a2d5f46481551724b81735ede3)
    - (cargo-release) version 0.4.0 (92e8b273654c3dedce60de244944683c7cf153e7)
    - first sketch of alternate resolution (6cc8a947df776aeeb031de627f84b7bc85207235)
    - (cargo-release) version 0.4.0 (2b1bca83c453544972e370dc0adff57cb7590b42)
    - take not of a few more obscure features (8f9570c602503f8689240a790766712dc1c4ec71)
    - refactor (7c3c80acf487296014ae9f2f9b88865c6aa6d98e)
    - (cargo-release) version 0.4.3 (5b47a1a051243ec2aa407297a19d41b30447bfab)
    - refactor (8930610c3ad73d2c1294880c3081f0662525f339)
    - Enforce using the correct version of clap (fd6457f3a006873506543846d9400b4a66833e48)
    - refactor (e4bcfe6406b14feffa63598c7cdcc8ecc73222bd)
    - remove quickerror dependency from git-odb (7e2749521b6c873766a2f6f96e6c91a0c6a9dbf3)
    - refactor (6a84f137754cddfdcb9b1fec3e353762ebb3ce2b)
    - refactor (7874c35bccb74ae7670335e633efa7eaebc72630)
    - refactor (4e89c3bc0f14cf9581348ae2c1620ade9dc1db8f)
    - refactor (3ec99dc7360c01b4f3c4593ff5049361e7043254)
</details>

## v0.4.0 (2020-09-12)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 83 commits contributed to the release over the course of 29 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 (f9dd225afc4aafde1a8b8148943f56f2c547a9ea)
    - Document why we won't use nightly for fixing NLL issue (ca29368b42b902fe7fe14dd4bff1b35e266c96a8)
    - refactor (519dd12f2bf58dd3048bc12e5b058236ad727853)
    - (cargo-release) version 0.5.0 (82b73131b79ec3c42a712dad1c0766a72209d737)
    - Revert "Fix NLL issue by using nightly" (6864a55001f1d01839f948618355928d666602ee)
    - refacator (7ac21536b3cee6708489011731216b9b831509e4)
    - thanks clippy (e5d80b19b83dc03d49606b7ccba20ff0c39bc5d9)
    - Fix NLL issue by using nightly (8c5bd095539042d7db0e611460803cdbf172beb0)
    - refactor (d4f288ceb2436b292993df74ed07d4d7e578edf1)
    - [clone] make cloning the linux kernel work (e78052649c734f16f4d154edcbf54f4cc4484f5e)
    - Update tasks, prepare for NLL fix (52af8d1089dc85cff19aee276bd831393f1b84b3)
    - refactor (3a8fb61067c210d4db6d515f21b2e28425c52e8c)
    - dependency update (446d4a59f7d0d8c0eaa2529b7a38197022c7384a)
    - Thanks clippy (6c4d1ec33d37b99b38698dfd91d38216ab4a2ef2)
    - dependency update (4a762f6c2d9a94b1424c0b1c5f8a38f8df2fbbb6)
    - refactor (dc022ce94505ce091e52fd64076bba01f0fe0eb0)
    - This works, but locates twice… (4e709f6029cf98f8c6ff204598706e2b6a1775eb)
    - Fixes for clap beta-2 (3fcdc5d21554417a4a47f464c68cd6bca06beb0d)
    - [clone] refs can now be written into a specified directory (fb1f04837be994fa5bcb9aa24f25b5f4f72e4ce0)
    - Also the imperative version doesn't borrowcheck… (c5720f1e4dc790539980fa81e940be6c6e15b50a)
    - refactor (98b3f4a9cc65e76aa09280065ab1d151f637e692)
    - [clone] First version of writing references, but… (445be27cf81663ba4fe941c00262448444efbac2)
    - Looks like the functional approach to locate(…) doesn't borrowcheck (5df6867a2b9fa7ba3fe2cdcd3bb9766faba1ae1b)
    - dependency update (e897b50f457d8eaccbda4860671b4266bdbe7a41)
    - [clone] add remaining journey tests (354e63fcfd9cac304f09d12387066b94d1334fe8)
    - refactor (9e68c6bcd1d07ea73730ce5ff59d7883152f894d)
    - refactor (127b8b2949476b38ef6f8ea0fb68bae6d473adcc)
    - [clone] v2 tests for pack receive (25cdd6345aa34124966c86b2a2e08d4af58b16a2)
    - refactor (f219d5a25efb7e258249ca3a4d39382136fe4229)
    - refactor (669b726da305ce4520c792d62b4344b04fe5f996)
    - [clone] better JSON output for pack-receive (bc6b8e86f258835b6da60ea7e749fe01243a4010)
    - sketch compound::Db::locate; sort packs by size (6609a534f45fc1ffc9d80a60a6a9793cbf54131c)
    - refactor (7bc321e96ecce0aae5063eb7008ecbac7d2ca31c)
    - [clone] initial implementation of Json format for pack-receive (9090ac6c6acdb5e050c597a279a963b48c08871a)
    - refactor (4a09754f6cd17d7f39f8a71b7de44d517294ffc5)
    - lower velocity (69f7930e00e78ff561ab5f599f9832ba7699da55)
    - [clone] nicer pack-receive output for humans (09c6c576ddb4c791b1b5f9b1812485e73a080f93)
    - Implement Write in terms of writing to the loose object DB (02b88c28304ff6d8c1fbad6fdcfa36f3b1f9dafc)
    - refactor (0752b45e95dd5378b7fca5b70bd11b9100ba2946)
    - [clone] first journey test for pack-receive (46a3511aead043bc45256ce603285ff4d0fff60a)
    - First sketch of compound Db (9bf227914d9281bfbdfc902edc3c1cc21c7fa3cd)
    - (cargo-release) version 0.4.1 (64fff36dcfdade887b7f417605d81b9d5750f000)
    - [clone] Assure we don't hang due to unprocessed headers when peeking lines! (d9ced2711dba702d73b28f0e1b9399cd7eab5183)
    - refactor (203ba995c9e237ac63bc2ecebda18171e90fcb47)
    - (cargo-release) version 0.4.1 (105c50132c8ad1f15ace0821278a11b06c81103c)
    - [clone] more correct handling of 'no-done'/done when sending wants/haves… (50f4516adfa458f4b16e301340a39b3c34ddbef0)
    - (cargo-release) version 0.2.1 (ebf341936b22e899de88293ef377b438353d1821)
    - (cargo-release) version 0.4.1 (60ac8b0a7545fff6ef293fd48716e9a19175517c)
    - [clone] Don't hide nested pack-decoding information (4d4be975707d017a67a0b2c081a07c4092b2801d)
    - (cargo-release) version 0.6.0 (9ef184e35712f938fb4f9f6da7390a8777a9284e)
    - refactor (ad17bfdc07e1301693fdfa3d09df3b39f675a36f)
    - [clone] Don't try to explicitly close the connection… (17200b3c494a24de19b7c6ec3191e61551a54380)
    - (cargo-release) version 0.1.1 (bb38c6b66e8de2b6743bb873c94afb187c8c8dd3)
    - refactor (91d9f78a9af04b44b2cead30d6e1cbaaeb76a522)
    - [clone] Fix encoding of V1 capabilities in first want (b68a5c57a6bd35391d8efb6436bb36e032851b49)
    - (cargo-release) version 0.2.1 (abc218c442cea95884d8b987faf0f29fc25384b1)
    - refactor (6ebb5d1839cd5ab4d8aff78acbccebaa66f439c7)
    - [clone] When unpacking peeled refs, use the object that refers to the tag… (fe8bb3985bd5529a36c71fa170ca48df91060491)
    - Support V2 shallow-info section (6679c918628979efc73e68c60e0968058cd220db)
    - [clone] All it took was a an intermediary to call 'read' as expected (7c8ecb78e988f7752cea6fe2022ccf9739b86fd4)
    - Tests for V2 shallow section parsing (5bf58ab344cb6b670ae535c7f7bca8a7f99a726c)
    - [clone] minor refactor; it's definitely the read() that doesn't work… (406829b951164673c0b8152d1e9de76f1318df0a)
    - Support for the 'deepen-relative' argument (b86fed6e415183f52bb34c68d8b503566740f671)
    - [clone] none the wiser - it really looks like everything is alright… (3b8d613c6de349defce9af06d56f73ac2c0d0d25)
    - Assure peek behaves exactly as we want it to with ERR lines (bbdaee5ff7abe364e4eb1bcbfce7fe7068935166)
    - [clone] it looks like in order to figure out the issue, it needs tests higher up… (edf1540d2014eb26cd5b98aa1baaa1e0c99662bd)
    - V1 parsing of shallow and unshallow lines… (8bcf535a8b07d9b1d53fb84c73ba55c76a318daf)
    - [clone] Don't send V2 capabilities that don't have a value… (9c9a4ee2a9c93612fd80844e8d2338461ee82ccc)
    - remove unused fixtures (6ae69f5f57ab371684e8c35cc77803aea05edd7b)
    - [clone] Handle remote progress name prefixing (more) correctly (51d4d15028a4162fae2d4e68a8fbb34c6ba93cc7)
    - Fix wants/haves separator handling for stateful V1 (16295757a33cdbdb8c69ba6c487ae8b298f612cd)
    - [clone] This actually works: first MVP of retrieving packs via clone (c06d8194173f9ec468ddd0faf72dd6d8dbf7d35d)
    - Make really clear that V2 is stateless no matter what the transport supports :D (c296845201b379273ff8077489ace9ed33f416b7)
    - [clone] First step towards implementing a working pack receiving… (264ec821ca92a08d1756222abab11ffebb6dc0ff)
    - Assure the first 'want' in V1 is always first (e729ec8f075a6c3122b42e367486a15c5367960f)
    - Use git attributes to prevent crlf conversion of fixtures on windows (80ca8b24b5565d82bc1f8e7d92c942f985e6ea3b)
    - Properly handle statelessness in V2 protocol (1b49f1ef6d7a40e2dec07f9c08036b1b1d460f6b)
    - [clone] increase git transport size limit (422993d457cafa19408fd6aa9fd2074c4ecbca74)
    - add some samples for deepen clones (61bc41a6f97decd3bdd96f874001ffb45251aca4)
    - [clone] Support for reading multi-step negoritaions, but… (507d342dfe2a714a4dd0bc100d96ed9e64a58243)
    - upgrade futures-lite (1d830330101b797f840f9f4a61fe4f28058fdb4c)
    - Allow dual-licensing with Apache 2.0 (ea353eb02fd4f75508600cc5676107bc7e627f1e)
    - refactor (63c129292288cc626b09ad29e9ef5f1a1d8339e4)
</details>

## v0.3.0 (2020-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - update release script to match dependency order (e8df6c1ffb7afa27aff9abbe11c7e4b80d19b61e)
    - bump minor version to 0.3 (4351e2871c9dcf342b8471fffa74cae338a53269)
    - update to quick-error 2.0 (4b1b7849b47a54092b49821c39e864c86adda979)
</details>

## v0.2.0 (2020-07-23)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 6 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 (d350a13784685ea82b84646b18736986aeb68146)
    - Switch to latest quick-error (976085614ee13a19fc1347209259a3dcf36ef95b)
    - assert we don't exeed package sizes (df66d74aa2a8cb62d8a03383135f08c8e8c579a8)
</details>

## v0.1.0 (2020-07-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - incorporate git-ref crate into releases (e66c9ed041c7ebede869e899ecd4398fee47028b)
    - refactor (6ad93041813f78548c3bd813b8685a60d857336f)
    - refactor (1fd90f739f4d8bb7c4f27103d2bb92e3f58b6f68)
    - test for common ascii control characters (ae0c885518d9ce4de05adbb048c0188f9ca934c3)
    - all test for valid ref name except for ascii control chars (a157acfb1f68ec6af6bb0b76f52aa8c7f72d43bf)
    - add new 'git-ref' crate; place ref name validation code there (1a0e84e627b17be1b1fb53b4dc98ab78e9cfb9a7)
</details>

## v0.5.0 (2021-08-10)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 15 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 406 commits contributed to the release over the course of 78 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - fix release order to match actual dependencies (65ff8c1c106182820dc6e4a308f71708e657f07f)
    - (cargo-release) version 0.5.0 (ae02dabae961089a92a21e6a60a7006de4b56dad)
    - (cargo-release) version 0.4.0 (0d5c8b96dfdfb96e4fc82623f756f6c7f7046e90)
    - (cargo-release) version 0.16.0 (1231dbd16dacefb39adec8e067c312d313a82e3c)
    - (cargo-release) version 0.2.0 (20d8e27dd4e93ae2234a3fe19b5f1511365eee2e)
    - (cargo-release) version 0.5.0 (0e11e98f0562c7baa9c90e18db6240731d165217)
    - (cargo-release) version 0.2.0 (8ff511583e6d859e43ffda0ef75e2fecce3ed03c)
    - [ref] refactor (501182b106b70af73db4f23cc01291d30481f76e)
    - [ref #152] remaining tests for transaction namespacing (63d80c0d0fbcf4fd1b7c3db652f622b59bc6fd18)
    - [ref #152] first succeeding test for namespace rewriting (758c8f60ca6567cd0a12892490ce27f88d1140df)
    - [ref #152] first failing test for namespaced updates (a81f1d44a83474152d53140f8d9fdd0ace8060ac)
    - [ref #152] refactor (f9c63fbe70ceb10bc3ef3edee008f72c3494b18c)
    - [ref #152] namespace prefix stripping and fixed test expectations (bce135b7c58ba5f709aad2daab0e1668a834a4cd)
    - [ref #152] a test for namespaced iteration (2338c6e96e3dbd0759c122e264044c195f16a269)
    - [ref #152] packed-refs are optional for generalized iteration, too (88525a9f028e94c8647ad5f2f7067b5b4e01c0a3)
    - [ref #152] FAIL: cleanup iter API by allowing Option<packed::Buffer> (1836243b6ec42eaf162463cded4a613c8984ac3a)
    - [ref #152] prepare namespaced iteration tests (cf5abc96115f4bab0ee52f58295f06f689173bf8)
    - [ref #152] no silent failure if path conversion isn't possible (8df04d8973fc62eae0e8d98c8116351907dd282f)
    - [ref #152] introduce Namespace type (67d5c8526d8356bcee81b690a38559a01128863b)
    - [ref #152] sketch API for namespaces (138be9588576eca84921cedcf5f697b5c98e85a7)
    - [ref #152] docs (8d6c8564faeccafc1430a2184a4060d953349e3f)
    - [ref #152] refactor (bfb82fb13350d986c93cc6dc67d6f86506dd80a5)
    - [ref #152] all tests and impl for refname expansion (9cef2f2f166514048fae52ceec5a86a2849be286)
    - [ref #152] refactor (431dd8655397b0ae88a5144d5c8553ba63e46c8f)
    - [ref #152] basic test setup for namespace expansion (e8523996b73fb93218c651b6f6041935833293d0)
    - clippy on tests and thanks clippy (a77a71cf02d328a2a964388928d6b2a235a0aa85)
    - thanks clippy (e1964e43979b3e32a5d4bfbe377a842d2c0b10ea)
    - [ref #140] finish implementation of tag peeling, with test (c06e72916e9622df62579baa6817603af0c7c747)
    - [ref #140] refactor (edcc3951bd0fc98589207a1b1f8941d6bb9652ab)
    - [ref #140] sketch ref tag peeling (ef90652dfcd84b2fc140c38e1364b42578fdfbde)
    - [ref #140] refactor (8e1a7305e869979751230f23c614f276ebce3f1d)
    - [ref #139] add missing docs (5422ec8923a5f3c284f7094894a952a392812e63)
    - [ref #139] my first empty test but where else to document this :)? (0f00065fa3360a55cc52926bfaa94d72598933b5)
    - [ref #139] refactor (a8f5d8dbaecaa26509d568a36acbf350ee86a03c)
    - [ref #139] peeling for all refs to be written to a pack (cc891a1809a6678f168b08766f67644742386a5d)
    - [ref #139] refactor (7e1581788356889a936f4a778119b0bce36d3041)
    - [ref #139] Allow packed-refs creation in the presence of updates (0cf7314df7a6ab79478525544e0ed28d07cf3642)
    - [ref #139] impl of loose ref deletion, but it doens't work yet… (f6631ad537b4c7fd6dec2a511214552e606462d4)
    - [ref #139] a failing test for pruning loose refs into packed refs (437c610eeb3b4a5874f001ba6fbbd42c7dc1188e)
    - [ref #139] refactor (62558cb562747d3c6f2b4e1b62dd44e4f1e95019)
    - [ref #139] a first sketch to resolve object chains for packed ref peeling (54bc1161128f0c719622935728a870820918038b)
    - [ref #139] Allow 'git pack-ref --no-purge' essentially (c32d8b7a599c0ee0d8936a0c5aee658b5d986453)
    - [ref #139] refactor (e5fbc4c92f0ea74afdff45c243a762e7a978d749)
    - [ref #139] refactor (4e1b95e40e94b0c9398c40985e092bd1d8607a4c)
    - [ref #139] refactor (42215a15ce53bd78fe1d8d9b15d7a08919f5f980)
    - [ref #139] a complete test for the first packed-refs mode (f332dcf2b1beda319871f7b0de585c8a1d9b813f)
    - [ref #138] delete packed-refs when it's empty after rewrite (8b7c359db1c81ae69321c9c2637d0af8b303d9bb)
    - [ref #138] refactor (3fc0014dbf3c6a0d0c3e34d39c3068c71f867fd1)
    - [ref #138] no need for preprocessing, input is already checked (a6fca6e0f81cdccfd7284d70ad4218e94b6cbe24)
    - [ref #138] less is more… (6f3971325380dee93370a2d6a05d43adec94181b)
    - thanks clippy (169a39d72106c24dac78af2198e54ca6e09b743e)
    - [ref] the first green packed deletion… (76a23b0e3e508a3445a9e1c77045e59bb7bbef69)
    - [ref] refactor (packed refs aren't changed in memory) (0a7e8ce1be7c7e6cb8a7646a8dacc7e95acf5efd)
    - [ref] basic packed transaction commit impl, but it doesn't work yet (1913099eeb84e78d9b4373e6ba9823a493d82343)
    - [ref] fix order of operations when committing the transaction (be5774a3d5e8fa20eadc6ef6f0bbfceab35f1827)
    - [ref] refactor (69d53f99097220cf3a5e3e5afa855d1847715007)
    - [ref] first revised sketch of packed-refs writing (f942c7622cf09d3c6937c7fa78089991d58482a0)
    - [ref] work on first naive transaction, but… (b08cc4a47ecf8ad5f4b56ffdaf678946549b0ae9)
    - [ref] tests incorporating packed-ref deletion (399096e0f611a649fb99facc0925adc1c306cbfe)
    - [ref] validate packed refs are taken into consideration during create/update (25999b4cebcb925bf0f0d4f451c7ca557f03dbc2)
    - [ref] allow creating new packed-refs files as well; prepare test arena (8494c7452f68bb3ebe7bc9115b7feb36871a406a)
    - [ref] refactor (e379177a1937fdc23cba843d2dc6fecd3dfd2ab2)
    - [ref] refactor (a844146a799e07c3d95c4224b4a114b77cd94832)
    - [ref] refactor (bd94ea55c1b598e507b5717ee5a5d6f14830c3bb)
    - [ref] actually make use of packed refs in file transactions (7746238207b637d4f241a05af7814916736cce24)
    - [ref] refactor (7a7b0dcd8b9156a5c67bbdcdebb6a2a2e2757a7e)
    - [ref] refactor (74ed358c7ef6147095e8df9eb29b34ab55c850f4)
    - [ref] first basic sketch of packed-ref transaction (8aac30c77b03aa6c020d46c79f54d031043351df)
    - [ref] on the way to requiring a packed transaction for file transactions (85f30ac10fa740293d72f558dbd48a14aee82fde)
    - [ref] prepare existing refs to take packed-refs into account… (5849b44c87c8b9ca68d7d30623540d8d441b6a3f)
    - [ref] remove one todo, add another… (46c47ab440df49d0f3a5324b243cdcf5a2898e03)
    - [ref] all todos done (763257327632b39a5ec777df4f07da9f87005a36)
    - [ref] refactor (fb37e9612c03cf1fcf5cdef9241a35242b9ff1d0)
    - [ref] refactor (23ea139e0af622e8d40774fa2a890ef3525a991a)
    - [ref] rev-iter for overlay references (8b28d4a326a2ee43bd00e475a0376eb577145a8b)
    - [ref] refactor (a80b8c18eb5cfc77ca5e071e9163df0a89a35fd4)
    - [ref] tests for remaining todos (0ef6b3dbdc7f8c67e69eeb453122ce2722d171fa)
    - [ref] remove loose::Reference backref to simplify everything (9f1d960ae07d368f3ab208cf886ea1af99dfe25f)
    - Revert "[ref] back-reference of packed refs to their packed buffer" (464aefe563c045b30ead0144b97a41d7b353235e)
    - Revert "[ref] FAIL: let's not add more back-refs, let's add less" (eaf4e9a1582fcd3c1d1da9eba3fb4c7046a5cdb9)
    - [ref] FAIL: let's not add more back-refs, let's add less (8e90d7545d4bda92e339387acfa1c882e2a99264)
    - [ref] back-reference of packed refs to their packed buffer (da860efa8fb42f9f755cd9070732fc4403843cc9)
    - [ref] refactor (61972a298bfcbad7efe23a480895fc26bb53bf24)
    - [ref] refactor (f03c6144f395fd8713157a4a3137c6c0dacd41da)
    - thanks clippy (08f8bc4c09ad85df0ea75916f8bd9beb061069ea)
    - [ref] probably fix windows (6eb2532724d6be1b25b68b10b58cd504ff1a7af9)
    - [ref] refactor (3df606aa33ab8c161a7b36b79a9661eefac218e7)
    - [ref] test for peel one level of packed ref (3d8602f2fff98e3a1078c24e65cd887bebc7fa78)
    - [ref] assure packed-refs have a consistent target after peeling. (29a352a24c0e2685d06672967e4898abfa1c2f8c)
    - thanks clippy (321908e12a885978dc4fa3fa1f71cebc8efdf741)
    - [ref] improve import paths (2dbe785d80d56b2d9f5a617b57a02926dba70434)
    - [ref] refactor (49fc212e9e82382d06da16dc9b84e3952a73ddce)
    - [ref] prepare to create loose:Reference (8ed3916564917fd99a74dda06d35f4390e918fa5)
    - [ref] refactor (f2225253de054ce8cfa8f8ce33a93c3ac613dc85)
    - [ref] finally peeling works again (d5bd75acdf48f7a274dbb88441f003d5d287e3b8)
    - [ref] packed-refs are now enforcing valid names (5d9291976370edae3a8429e745174147c1fadf90)
    - [ref] prepare peel test; realize another refactoring requirement (62f71552da037c126058b7bcaa9e6bab8e2c168b)
    - [ref] refactor (ae4d5da10fc6e0ec5015539a1285f1a3dbbc9628)
    - [ref] refactor (e26c72fb1bf9392932ffe42843f3dec52c7bbd7d)
    - [ref] refactor (f4bb7a02d8e8b820f30894ac74613bee10532c79)
    - [ref] another test to run into one more todo (13502f5bb7b1df7abd1d2de4f9e93a9e5439b84f)
    - [ref] some TODOs to not forget (4d6a75cc6835cbd1f6ab321e158310c97def2a71)
    - [ref] and it compiles again, may todos left (16618b916ff67316717d95575fc1344d956d2c49)
    - [ref] all required Reference methods are defined, but… (3c976a65cad62e4e04c686b1e8f645bf300ccf41)
    - [ref] refactor (65f7a7db56d6db974db197101b6306dbb7483ff5)
    - [ref] changing the ref type means a lot of breakage and some unsolved problems (407dc4d79a4281fc3ec09456bb6f969f42bbabd7)
    - [ref] refactor to be able to use loose_then_packed::Reference for top-level find (2c4e45a5bf997530d84a214714ff25fdbbcafd16)
    - [ref] figure out how peeling works with packed-refs… (2801f7aa137c6167bd392ca585f1aad378cae0b4)
    - Revert "[ref] FAIL: actually it's enough to give access to 'packed' when peeling only" (8dc62955f1a8b92f08924f155c932d0dfbf415ef)
    - [ref] FAIL: actually it's enough to give access to 'packed' when peeling only (5173a97531f213573da12d0d9dda8e0bc808c013)
    - [ref] put packed-ref lookups into the correct spot (6d11e22c723f03155f12878ac7b94ef959f633a4)
    - [ref] remove over-complicated refs store trait which… (1cc876cde25820a7a8afa8d867dec59e6079d72e)
    - [ref] refactor (62e682c269c48a9eb2c25f4bb6421b8647fb3fab)
    - [ref] API sketch for allowing packed-refs to be used in find() (ca736ab2ee8eab337683ff66e6e07d4488ff15da)
    - [ref] fix windows build (f99851bc3195aca958409bd5773e6210037b07f8)
    - [ref] assure names are using forward slashes in file-based refs (ff695e4dae73d1497290d1efcc77b0cf1b265617)
    - [ref] prefix iteration for all references (228ca00a91069ebe32dddbae3d716cc6bb59542e)
    - [ref] improve structure; fix docs (aa6052a41e44a13ea31c9ec585663b0904cdd929)
    - [ref] overlay really seems to work (d2ec30af1be4bc54d69ef7d794c1bf372c80463b)
    - [ref] more detailed overlay test (d747d730afd4db6c0c20c3c63cc09824fbd6e223)
    - thanks clippy (636e1fd85ceb3a1dc3cf5d3c7224f6f36d8eb695)
    - [ref] fix windows build… (65e6953d1a9e751cb4644056aabd7c6edfbf7978)
    - [ref] first successful test for overlay iterator (5f924885f343d8a60737de74c651e8e5c11a8d48)
    - [ref] conversion for packed refs (929bb0f75715a547993e8ce9c885d7de1a030013)
    - [ref] loose refs iteration in overlay iterator (0b0f64d16acb97d2282b982647362b164ac280ad)
    - [ref] leverage sorted file iteration (036257eee036c2d5edea2ac8b16aad6bae8ba7fd)
    - [ref] add setup for parallel file traversal tests (1306647447f712805b3d8c8ca38e90fb4f94ca67)
    - [ref] reproducible loose ref iteration with built-in sorting (e13874807ccc3cbc2b4aacccf63ac5c3dd21c445)
    - [ref] sketch remaining overlay types, now on to 'next()' (6792cf1362ed21948d9b5f8b252b1c08ca8ca7ca)
    - [ref] a way to obtain valid ref names along with their path for overlay iteration (bbaa1eb10b3d2fd0de6afde61e5b6378be2e110c)
    - [ref] first steps towards test and impl for overlay iterator (f5d07b67af4fdf68f3109a8bc1481474cd5c3807)
    - [ref] add missing docs (e6052a5a36b27bbcf79c05cd517eab9ec7507d8d)
    - [ref] all remaining tests (ee9bc211e857ed2bbf9eb5fc6e46f5e126b11ab2)
    - [ref] first successful test for prefix filtering in packed refs (430549da137c5469a0ee17eca8d52a6f3ed8b04b)
    - [ref] run all performance tests (3635b25deee7ded4307458abcf83d0c1181030f4)
    - [ref] simple performance tests to get an idea of what it can do… (06bedcd7a79c64ece443a34cc21a9ca32ac38ca9)
    - [ref] perf 'test' for ref iteration (922d129ff3b741a3091cf899a8e1400e98417093)
    - thanks clippy (a39a68a3d51bf0185df86ca34f90b9755f31f2b5)
    - [ref] rename find_one to 'find' in git-ref… (ae7746a0815bb94659de67383ba372ac522d53b8)
    - [ref] refactor (758c0907df8dc6987f374e326304e0f9fad29812)
    - [ref] finish packed find() lookup testing (5f67c19a1f4f62419bfc7d6e52c56aa5be40b723)
    - [ref] refactor (953939c2ce7922efd6df4654dc329743d3052492)
    - [ref] prevent unnecessary rounds for full names that aren't found (fb765de831aa704b04b6a23c6a1d4ff183d784e0)
    - [ref] Assure ref-misses misses aren't parse-errors (d9d13602c83d0725d23d3abb3d2d5bf30355e1d9)
    - [ref] basic lookup rule impl; needs more test cases (3226f775129231b4bc4735baf9e14a187665ace3)
    - Remove unnecessary unsafe code (83e207a44aece0ff4870e57990bd5aaf43f38e22)
    - [ref] fix compile warning on windows (c32877415aba8df6d5a37cfd799b218e3a29b18a)
    - Merge branch 'parser-into-iter-without-alloc' (a799ca8d6c2e51303512160ddef7477e176ab01b)
    - [ref] a test case specifically for lookup rules (ab3a34f481ebe335578e3a7dbff325087b4ba647)
    - Implement Parser::into_iter without extra allocation (aa79924b36c0d717cc65d7471fedd27eb41e83a5)
    - dependency update (059fa3318e3e76c407e456d28a28cb834d532719)
    - [ref] improve parse failure handling in packed-ref lookup (ba62aab4308d44092d151d11d9be44ba6bfddb02)
    - Remove unnecessary pub(crate) exports (3d2456e11709f0461b37c6df55ecc3861ca4cab5)
    - [ref] refactor (959abc70c754cf4cd812f6014c29fd2f6d1a7fc4)
    - [ref] prepare for proper full-name conversion (0e6d3f29a6abe54b04424697009bb8524faaca7e)
    - [ref] searching fully qualified reference names actually works. (9b2579c3713b3bd185895318868378b8831dbc96)
    - [ref] prepare find() impl… (b26dd1ed253d8714cf4f9a77c0c29f67cc952c76)
    - [ref] assure packed-refs buffers are sorted (a797493c93aa2d1b6e46442f714c8d5b98032456)
    - [ref] refactor (897a49a9973ccb225dbc9b75be624b7e4c9ec608)
    - [ref] windows fix; now maybe? (0e1a20424a25902e80ad8dd6b6a413cb00f77904)
    - [ref] windows pathname replacement: \ -> /… (94a1e02d3e03f29d56b83e92c176c8d245ff44fc)
    - [ref] fix one test failure on windows (21f10319d4047401bb6b11dec975c9386788773b)
    - [ref] rough frame for finding packed refs (a24a54fb2b2620a0c86c2b9bc2a094412ed73fb8)
    - [ref] learn more about the windows issue… (dde6276a52b0f067bfeb8bb355a05696df6f134f)
    - [ref] refactor (c150abaa86ebcbd10ccee4359b45b4a0b802b68e)
    - [ref] prefixed loose ref iteration (49ce1e2184841ecd9c54573ba026341f4fecc0b5)
    - [ref] refactor; tests for prefix iteration (63566eb81cdd14a98f25491fbb7f363a2fb6a0c7)
    - [ref] loose ref iteration with broken ref support (2d1234f9f8ae55c13af18ef5978e4ef9634e1606)
    - [ref] maybe fix windows (6fc778455c374fa289d15e64d1d67ad9310e0d0a)
    - [ref] first rough implementation of loose ref iteration (918af425298a1fdbb8e7dd6328daefe9eaa10cef)
    - [ref] packed-refs iteration… (ea97e063bfa5cbafac521dbd7f8becd357083356)
    - [ref] docs for packed refs iterator (02690bc96903071108ffc54594bd4c31ebd054d1)
    - [ref] fix 'small' build (5fd10fe1e901a0c8d9627f76c4a040922847cd15)
    - [ref] packed-refs iteration works, incl. decent error handling (e5a6b9d2f637ee746ccaf67354f64c3999cf971a)
    - [ref] the first packed-refs iterator tests (f6d769ec5948fefe363ffa436e326e5fae820a66)
    - [ref] refactor (207a799c1fcf490425f2e5dcf8274da83125af6f)
    - [ref] flexible and simple support for different hash lengths (9c2edd537fb86d2d7db874ec976d0cb1b8ec7c2e)
    - Revert "[ref] parameterize all uses of hash length…" (21f187e6b7011bb59ed935fc1a2d0a5557890ffe)
    - [ref] sketch of iterator (6c05243b53a74c770fc41e50a7df55f01ba21b3d)
    - [ref] refactor (79184cfe1035ad8665972c796c27448dc1fe3430)
    - [ref] parameterize all uses of hash length… (5c7285e7233390fd7589188084fcd05febcbbac2)
    - [ref] less lenient packed-ref header parsing (45b41e0f522ac491e49be5e36a1744c9d07a4286)
    - thanks clippy (33f1b00e134222641a71521561db4671a4285462)
    - [ref] refactor (de526b31dbd84ddf05cbc5d447862fa0559a7561)
    - [ref] first working packed ref line parsing (bc60229403ae075b66bb457a80695e2ab959448c)
    - [ref] first test for line (and peeled ref) parsin (7af27c5676c986b05953995d216b78389e986ee0)
    - [ref] refactor (b74913ef90c6d827dff50ca5df13c826be4fc86d)
    - [ref] refactor (d0eb8196e3faed6c013f2e746ba50bba1330d87e)
    - [ref] packed refs header line parsing (fde5543ad22395e27266db02a5442a33d16e68c5)
    - [ref] first rough steps to testing parsing a little (57659e92de9a525a72dc3cba50b844bef7e021a1)
    - [ref] sketch packed refs, but… (8951b3fd96735adc2eed5b0035bc0a97759e2207)
    - [ref] refactor + docs review (4b9b034e3600cc3dc6dc35a257231914802a60fb)
    - [ref] the last TODO is gone (01dc422cef924f26943dbc5b41b45098853d4868)
    - [ref] down to the last todo (23cea99f645dfc27a89296f7bbd30c1b22015dba)
    - [ref] two more tests but only one todo down (bf947d65b508511d90299e93f285989c1a3eafd1)
    - [ref] the drop test (e472bde7bf24eaeefa93a3dbc269cea41f6ddcc8)
    - [ref] refactor (059f836f490261cf5257349e0a7bfb69d9b68d89)
    - [ref] refactor (7faf6f24f90854bd885e59c517b73db8ba5082af)
    - [ref] adjust expectation to not do any special HEAD business (49d294a292709882179cf3b7934ec1885c60ccaa)
    - Revert "[ref] FAIL: realize that HEAD-reverse-lookup isn't done in transactions…" (8b0d7b62ff2ee96692d3014299fad67e0c82f3a1)
    - [ref] FAIL: realize that HEAD-reverse-lookup isn't done in transactions… (6098ba0f4288b379f84f48bb2d3245309a70ce7c)
    - [ref] test to validate HEAD update as special case of… (276aa9a89b41df43ad47f2096b4d89bdf697acea)
    - [ref] refactor (861483a4e7b7d61447d6bbfa91937ddfdf69ba02)
    - [ref] validate non-empty directories (8fb625d577fad376b28f5f568b8455aa901c2f0a)
    - [ref] moving a ref onto empty directories works now… (a237f77ee0eb395bf89f7ed1b7496bf33c2d30af)
    - [ref] refactor (ed40a87e14d38b7f8b9a3a605b70a0fb1dc92220)
    - [ref] another complex test works (ebdbfae9e26aa11f7afda7f60f0fbf6757dabb76)
    - [ref] fix build (b4dcdfc9b2f2edcbcf9fb144d1f97e9a841463ad)
    - [ref] try fix windows, once again (95e74dd9f1510fd288f281beea3f560319ad323d)
    - [ref] refactor (a261b82c1ee7ebdbbc82ce1c8286ca6a0f221cea)
    - [ref] probably fix windows (a8b7c8d2fef9438a23a96c35497d34e816af96c7)
    - [ref] allow reflogs to be created in place of empty directory trees (80a6e0e1ff2321d9162e799d5fc0f457e7de4ade)
    - [tempfile] a way to delete empty dirs recursively (6025aa08d93cd5124c8df38c51b795b9c7d1c911)
    - [ref] refactor (21920ec173da4642ad335fcd5fbc3b85c940061e)
    - [ref] refactor directory handling (45dbf2253d13ee8eba7654ef294614c3b9651a9d)
    - [ref] refactor (92867c58467e66d1b6b13d2ca4375d268fbafde5)
    - [ref] handle existng empty directories more gracefully… (0849c70596ed7674e7e18cd444b6cd99d37da4ff)
    - thanks clippy (d967e30f1652f29c3c13ea0014d8d3910a4f7245)
    - [ref] handle create-or-append when writing valid reflog files… (9175085248855a7ffa0d4e006740eafc0f4e1c92)
    - [ref] refactor (1ee341922d4a8343bc5146378da4353a99b28a73)
    - [ref] auto-creation logic for reflogs (80f71dc85836b640b264f146d37fc74a0bd99fd9)
    - [ref] reflog creation test is quite complete (b67e79c861f644756e9bd12cc3a28bd6355250d3)
    - [ref] allow commiter to be passed for use in reflog (80f5627d6fe5aef8d0a82cdad1746d5d2509f2c3)
    - [ref] tests for converting reflock paths into log paths (1f2e75439d2ff5b7db40a979fde289e68c578d81)
    - [ref] refactor (a29fcf1d61ec9f387a401a1a4a903256b6413536)
    - [ref] frame for reflog creation or update (81cb79017ca5a2f18531bc6caedc28de94a0a064)
    - [ref] refactor (a76929b45b4f82488b1e713d1012e1d431257fcd)
    - [ref] disambiguate create-or-update logic (585f369ea7bb7ee3d8f5103583628e3d68ef3de5)
    - [ref] write out Create-or-Update logic to see that's its probably not going to cut it. (54d084ffe0d684ab4879973293f2efad4966c632)
    - [ref] show how the original name can be displayed for lock failures… (07f0c2dc9b3949566b3c3d0a15302c416ae9ccb7)
    - [ref] write peeled previous OID through to parent refs (3355dd8295886b0dbeeaa802cbf32ea6e3264de6)
    - [ref] fix child link transformation (5d9a685fedd4d5614dd338d4b9baa37f11649cb0)
    - [ref] refactor (2f92f360e581a1a7b7bad389c915545cd6a5b31a)
    - [ref] sketch of inverting parent links for later oid lookup (a050f1856f69b710f6e63898d11fa52cafd254c7)
    - [ref] refactor (1e88948455111c01f2a8f9d24a4fcf835553e55b)
    - [ref] add reflog message to change… (b31e103f2492b0507e2e1eab3a26ddc025dd470f)
    - [ref] sketch more detailed test for updating reflogs (5a657cdd0a342aa8b5a57398718bf27ef136997a)
    - thanks clippy (eb8ea22a97f132169e81d71ca2ca64ef52463fe3)
    - [ref] the last deletion test (258a494562d8266561540e07c01d1e87466470d9)
    - [ref] refactor (db76cfd5585a5fa54739ce003837a8750dea9f99)
    - [ref] deletion won't have problems with broken refs (286b5c1a5529c58c35b8ff0504f9e784f7be10e1)
    - thanks clippy (e5da69e642c16ddaf39b59e6e0de6b3c4153acff)
    - [ref] add failing deletion test for broken refs (578413f5848cb8ab3b14fe149be3db12705182c3)
    - [ref] another del test (d935d6f67fff1d7b02f6b0805a3e6efb9f429fc1)
    - [ref] another deletion test (8b756e094bd4ecf47415d8eb8c7adf44b8a89039)
    - [ref] another deletion test (69ede1b90e6573df86829437f3c3adf3924b31cf)
    - [ref] refactor (d05a6467c185d0f4dcb030e4bf751070a9b3d5bf)
    - [ref] Make sure edit preprocessing happens in the right order (2d5f9aaa68b065f84df3a2db3707cf9cf10b0321)
    - [ref] refactor (dd9c99b9d1c0c6222f5a12f280c8ed0eb0c3daf2)
    - [ref] refactor (97fc864fb4dd2903eb9f7dd671422dfbeaa304f3)
    - thanks clippy (f436f18be3b4aafe40cb0e36432d22666795ecc6)
    - [ref] splitting handles reference cycles (09b4fc1e6f01a9124f6563fa614b42356560e4b4)
    - [ref] splitting actually works! (a9f824bc95f157146f22b468d4a9d8dddc9f31a5)
    - [ref] first stab at splitting refs, needs more elaboration to fulfil expectations (66b1f3725cd710d991625bcd2c1994545b33aa53)
    - [ref] refactor (eb0328fb67ad677d8875bef5deb7efea2c55ae67)
    - [ref] first part of ref splitting is tested (ce7f83b7e58762866e141d1b71e1ea68153fd075)
    - [ref] refactor; prep slitting tests (7ffc619a7c06f0d47572fac9f91444c3663ac316)
    - [ref] refactor (683651d2a7cc9b589b4490a1767677f3d7fb5e3e)
    - [ref] first sketch of generalized splitting of edits (1f2efdcf9151f161a325680737f1992edf46228c)
    - [ref] working on splits really shows that we want more than one enum maybe… (1b62838d00ec35cb45d43e5e9e5ce6573f1db2a7)
    - [ref] need ref splitting for the first time. (f52989f325d50db66c0ffe75a964feaba075dc19)
    - [ref] better deletion tests; more useful return value (96848f68a70a6721c9fc4c7d36763a3015527728)
    - thanks clippy (ef9bfd2806b0407ccbc7391e086592f4bf7a7424)
    - [ref] another deletion test succeeds (60379001d2729627c042f304217d6459f99f01bf)
    - [ref] refactor, not quite sure about delete mode… (683991a4edbc53c583603af94fbec625a211b52d)
    - [ref] another test; failing for now (1908b693b75e8cb204dc5026ea2f311b88bddfc4)
    - [ref] another test green (104598eb71e830a5feed763dea1dc1fd03be6eff)
    - [ref] first succeeding deletion test (3445d7dfcade73bec8ba68d58d034608169e7758)
    - [ref] refactor (d2e2e8f49b3668235cf808b08f85bd89a592105f)
    - [ref] first deletion tests (e41f8c8a48328fb0fe154e5212f1b1e41195d3c1)
    - [ref] write more details on how prepare and commit should work overall. (a7d988b8feb2aba87a19f3484470d8f77786ffd4)
    - [ref] refactor; get closer to what git does… (488f31160300bccaba6a510869c7c3e53d52d27b)
    - [ref] refactor (58a5653a6647931bf90f88ff2d83c6b0322ad9b1)
    - [ref] first very basic ref writing (7ebed3ff14e6944ba18be0c9876b10c42c2d840c)
    - [ref] remove complexity in the name of performance, fix windows… (77c3f24a935800d7643dc61466385a76a58bf365)
    - [ref] (probably) fix windows (7c1eead4b589975fb1dcfe63fb2071bb6d8ab611)
    - thanks clippy (6865549cf6df08999618bfa6cd658d44b8aba9c7)
    - [ref] slowly getting there (650692443459b253a56fb5bda78bd3a4a0de07f9)
    - [ref] a way to determine if a reflog exists. (e6fbba87942b9138261ee70d8fa8408422149521)
    - [ref] reference::log_iter_rev() (1f7af5dcf093a9169ce353c0b1d354ed7acda4a5)
    - [ref] reference.log_iter() works, but… (c298473f0f353f9f59d39ab530c133e13cfb47ec)
    - [ref] [FAIL] try to forward iterator creation to reference… (ef1737c7e67038c0541a619e77c0ea5451bcca28)
    - [ref] refactor (129bccf8dfaaab1c487c49fe35a2877ff900d06e)
    - [ref] refactor (96dd98b800b9e808853fc954ac78b8778bf18f23)
    - [ref] refactor (a7dd9940a0a6e1f8685f5bb785d8c05023027393)
    - [ref] refactor (34601272230c37aad803409e89dc6b270de1f02d)
    - [ref] store ref log reverse iterator (34d795700e89a264dcf3a40a6dec63cdc5998814)
    - [ref] store can provide reflog forward iter… (9adb9ca2b2b63f9fc4b57e45732389077778c324)
    - [ref] more assertions (80006772e0ef9d9f9fc4d274f460194712138327)
    - [ref] a fully implemented first test with assertions (29a58937a3e8d4fae861952d6bc34565da8c3e8c)
    - [ref] sketch more tests that will be needed (01690be8acf6a5f18b55db941f05644650f062f0)
    - [ref] add control over handling lock failures during transaction (7c4057aa4bd5e65195c80d0319798615b9571c0d)
    - [ref] generic operation on input edits, split-suitable now (7f4f63763249a614936be3baa702b93558a4d494)
    - [ref] try using borrow on a slice intead of iterator… (b2371d93408613ab0e07048398bd95e60da603e1)
    - [ref] duplicate ref edit checks… (3ec0182376fad623814408703f1d47736eea6349)
    - [ref] a more fleshed out API for file transactions (918123f7f951d7f773dd8b38a184de2f2c3e25b9)
    - [ref] on the way towards realistic transactions… (c808cb17b2fea12e018fabb789862e9b7703e49b)
    - [ref] on the way to setup the first transaction test (29c0b51625e2c7e3a8d60075bb925126a024dc83)
    - [ref] file store can ignore all writes; sketch transaction API (52a81e98f38657023d3eb384fd6db69917dd57ca)
    - [ref] refactor (6a84790b13e445d5a1b85fd3cae2ec0feed4ff02)
    - [ref] log line writing (3da8fcf0bfb77b80c06a3358416f10d6f393db8b)
    - [ref] Line::from_bytes(…); iter uses that now (7895995cf91fbaeb798c4277699e02107cb63909)
    - [ref] test for small buffer sizes (61837723f7c1f3150d7f853c055248116bba9633)
    - [ref] handle multiple buffer reloads (4559c7a184b9cdbd174785b84b41a218c683c94f)
    - [ref] refactor (65e333de6194b48b558d02b503502bd7ab267945)
    - [ref] refactor (2b416ee7e788faadf280553464fd77f2c91e2d0a)
    - [ref] refactor (82b18e50f3c31fac10dc5a752ab9b0c134607e37)
    - [ref] multi-line reverse iteration works, without window shift for now (f1e38618371408d844144a736c3082d57b2d1015)
    - [ref] first reverse iter test succeeding (88756015d8fc77ddb3b12fcdd1df85a709f8189a)
    - [ref] let's not forget to simply not try to return borrowed things from iterators (bcc934dea0aa71502945a20d5987dec4eeb34aea)
    - [ref] FAIL: try it with included buffer (189080e8bc2d999ee4f1a76ed9b537cfda7ad82c)
    - [ref] FAIL another attempt this time without iterator… (5e73dc2fa1a77b5bcf2319ed244004ac3ec86506)
    - [ref] FAIL at attempt to to have self-referential iterators :D… (bc4012eb8a1b0c27dd2b54d169c2058478449b0a)
    - [ref] first test for reverse iterator and more boilerplate (40db35547b855066b3584d8e81f62c8978ac5840)
    - [ref] refactor (4daddb13a7f7139b8e0e7c6817854dad00429dbc)
    - [ref] sketch of reverse iterator (c581d169c2e21e568bce3d7bc8469836aa9d1e2c)
    - [ref] thanks clippy (4ba3b08e69002ae20545e9d27c3130a672fa9ae6)
    - [ref] significantly simplify error messages… (b15cb16f022045207a9419266d3fe972fbd663e1)
    - [ref] don't include terminators to get slightly nicer error messges (09bbc6d0b32b835d1a4ba2dca7e24522b94cee22)
    - [ref] another test for iter::forward() (1d843029dbaa7d06f9338fa6eb90f583a4225094)
    - [ref] a forward iterator with a single test (917040cb58d9dda18835c255bff3a9d692cfe1de)
    - [ref] log line docs (10ab8e0e4bcccc4e79203f06e16835b8e5d9504b)
    - [ref] refactor (cd89e21280463deb1fd22ef20d2c54926bbb9b6c)
    - [ref] more context for line parsing (ddb5f9d256cf0be36943e11a9df18b938551be87)
    - [ref] refactor (a08fb776a523040445006c81a890ef11f496f650)
    - [ref] be truly zero copy and delay work to when it's first asked for (b4e594bdeb06329beacd61b03ab90057284bcb54)
    - [object] Generalize nom error handling and use nom-errors instead of custom ones (47c8a97194c9e401ee311234a269f8b8f3650ba1)
    - [protocol] adjust description of fetch::Error to match io::Error sources (23dafc6e24377ad00b70c0235fd7a8ff107eee0a)
    - [actor] refactor (bccb738edfc2e6923643a2e73f93b6acfdd7cf5c)
    - [protocol] fallible negotiation (e269a2cde18f604a36b33efb7e53f31ea5c45e2d)
    - [actor] don't leak btoi errors… (e6c7fc18954a5a5ad12b3da6c290f8cb9a74c19c)
    - Revert "[ref] Try using BorrowMut to avoid blanket trait impls, but…" (8212536376341e673a6ef05221d20815659d92d3)
    - [actor] FAIL an attempt to remove btoi errors (3f99cf531caacb93a3ce81b16d61be18e5d8a017)
    - [ref] Try using BorrowMut to avoid blanket trait impls, but… (4bb9bbad5b4e0c2e64a48a8e4a70a1b3af1ca3e3)
    - [actor] pure nom error handling… (78cbe18888ec654f3410fc655d9beaaf63f68003)
    - [ref] refactor (869448833d9de5c0859e6fab267b48d19f1a9119)
    - [protocol] refactor (11b2fd1250ff902dd084d8664f06732d4b69b4b3)
    - [ref] getting there! (bd73d8ee04f7baa9aeb05857484da6cb63175ebb)
    - [protocol] refactor (967946a65f67cb1fc5d7bf6944a7e900ff3521c7)
    - [ref] a step forward to nom error handling, but… (426ae5b7db6cb943fdf6ee48e2be531157341e49)
    - [protocol] refactor (8dc425ff91d00d3315903f429d4009df6410ba77)
    - [ref] try really hard to use generic verbose nom errors but… (10316252fa5dc02effe5596165268f8d806c55f8)
    - [protocol] assure we don't coerce refs into UTF-8 representation (5ceb64dfed67b942100e2e36715903492d870c71)
    - [ref] tests and impl for happy cases (7be82f09ce3c2421ba922e3f8bc1238ca5d494ab)
    - [protocol] support ref-in-want (b6df400dccd66ad2f01c80d2fa05b8f9bb130b23)
    - [ref] the first test for log line parsing; make serde1 work (cba3cdc75280b247e59af878d1afe286638b95b7)
    - [refs] try to get structure in place for reflog parsing (727c66a2560c00cc8e01fbe47503ffbb67147c59)
    - [refs] sketch more of transactions so it has all it needs (8f9a0157e876fadfe16a2cc58445543d1c10a21b)
    - [refs] allow writing any valid ref value instead of limiting ourselves to object ids (114fce8368fe858bc64696b4d7253c425367560a)
    - [refs] finish transaction sketch (or so it seems) (976a0799a7862de7b85d45cb080102f41fc33d07)
    - [refs] this gets more and more interesting (e05649577a6cd5e2958884b10f7f75d48aa91a94)
    - [refs] finish research for transactions and their flags (2eb3bccadf338c07493e40cb8c5f357eb2502a5f)
    - [refs] sketch some parts of a transaction based on git source (d9a5d328f575dfd86e414091688a545f931059e3)
    - (cargo-release) version 0.3.0 (87db688f23475d7232731429d770848aea228492)
    - (cargo-release) version 0.3.0 (6b33678f83e6d261ca15c4a7634ff5b4e66d81dd)
    - (cargo-release) version 0.2.0 (3286e42547b59df6365087cbae9ce1c9c959faad)
    - [git-refs] a way to build a big packed-refs file (51135291b60d38bdf50d24569596c421bcb4f0b9)
    - (cargo-release) version 0.4.0 (866f86f59e66652968dcafc1a57912f9849cb21d)
    - [git-repository] traversal program uses new facilities, and it's cumbersome (29ea2de9ad48036f78d3776d8526d959f68bf287)
    - [git-repository] most of the git repository discovery (72a49c816253520230a04290619f469df608be19)
    - [git-ref] refactor (0c795c50834bcf52324ede46ec11eea26acb1107)
    - [git-ref] fix docs (4fbc476b2361afef25cff208ecfa66ac2ccb077a)
    - [git-ref] docs complete (93a1f4e3fe48082abf5b0baa17a976808789ec20)
    - [git-ref] nicer semantics for peel_in_place_to_id() (d3250a7b5d0e16f8f1b38d10334282fe60f9d5ce)
    - Revert "[git-ref] refactor (Option<Result… -> Result<Option…" (d4046e94eb22d9e9b65ffa9861400c4fde4d0bd7)
    - [git-ref] refactor (Option<Result… -> Result<Option… (774e86ce78159f7e07ec552c1847658b6f9ac288)
    - [git-ref] refactor (928b63789237b808b296c60c989b853b78d39f0e)
    - [git-ref] more docs (f962c74215965f14e8f136ab0a4eddfbba97e8c2)
    - [git-ref] refactor (415f15aa5751ee1a58d9e6723a9da9f3407a4d66)
    - [git-ref] a bunch of docs (7cfc5ab3c3b969e968b894161f73f3c69fe8e4c9)
    - thanks clippy (93915fa6f1c00260e4f263ac4837c2ae7916b764)
    - [git-ref] peel to id done (f74771c8caccb090066b5209721b8973c047f00c)
    - [git-ref] first working peel-to-id() (3574f8717700ae3b33e167be2442c69f604f287c)
    - [git-ref] frame for peel_to_id (3710b6cfe5cf2e5e6f9199255ebb4ca68a195be5)
    - [git-ref] peeling without an iterator, fine (b118946ef68425ffa0a606d67df7b5d3b2d851df)
    - [git-ref] first stab at reference iteration… (806d10ef735caf3575b84de0cca5b55374140571)
    - [git-ref] refactor (c363269e118a2dc53ce29ba245c079cecf061b7e)
    - [git-ref] find_one_existing(…) for convenience (7a443ffc148ae8161ba93351ffd16631f79e095c)
    - [git-ref] some find failure cases (d85505195541f3123527a337c9935e25bfc40ec4)
    - [git-ref] handle all find_one cases as per docs (3c0acc6545ede1a3fef25ace2b7dbf79debdc754)
    - [git-ref] more ways of finding reference (b3c4e928c6fb01e029f509e8b24516cd6c24e48f)
    - [git-ref] the first green find_one test (30177e81451bd4fb51dd3297502fa3c63f67286e)
    - thanks clippy (8f0e9ed9220a874e8437ede6e129d345e9c8f737)
    - [git-ref] first basic impl shows validation needs a little adjustment (8b901c750f97a950cb162c9195770aee451d2e7e)
    - [git-ref] a sketch of find_one - easiest for the caller for sure (ec96256c4be9ff6de15bb698f2d3b9559619a042)
    - [git-ref] refactor (5bac5851367d77ead43feceefdb2bfaf24a1561e)
    - [git-ref] frame for loose store reference lookup (30b0d54ed04916a858af3101345c677dbf48594d)
    - (cargo-release) version 0.2.0 (132789475400abe660b30ef6d2c5ff57821dd2c4)
    - [git-ref] use git-validate crate (6b4f937f13ad62bc2c7e5b0fc14416feb9c313ba)
    - [git-ref] Setup more tests to realize we really want validate::tag (54ee5b5eace8c35bc33ef1261778ba0fcee2ef37)
    - [git-ref] frame for validation (9656ac620a1a085122676052b9a0b32d9c4f6661)
    - [git-ref] failure tests (567e86caf83c73497b021d636ea440cc817f10ba)
    - [git-ref] more tests (048fb775764004ec5bb39bf243a102233dd9946c)
    - [git-ref] refactor (77d0cc088d6de8c37fec9ae0136c9f85bfdbc643)
    - [git-ref] don't support serde for now (2a6295bbd8a30d84c0d6544ca83e79146aff088e)
    - [git-ref] refactor (02e545ba6fe801f43e0a76e43e8bcfaaf77bd5f5)
    - [git-ref] first basic 'ref: ' parsing (60fa3bac9bfff7b5e3ac331c77c1050e9359f481)
    - [git-ref] refactor (9a30f87292aff1d4a2f043ba160df6b09bce16c8)
    - [git-ref] the first succeeding test (cebfdb463ac2d86f56bb3a2d57c0487a8b233fd8)
    - [git-ref] the first failing test (7e802a0576230dfc666c253d484ea255f265f92f)
    - [git-ref] sketch ref creation (c5241b835b93af497cda80ce0dceb8f49800df1c)
    - [git-ref] A sketch of how it looks like with Store backref (1a08f1c0365afe7d5e6fbc80bdd382d193d4b881)
    - [git-ref] more scaffolding (8c6e8844627878e981e597de0c29408cf51582a4)
    - [git-ref] clear it out and move existing functionality to git-object (fa548ce94db3dd3969add494756fcc34e48985a3)
    - (cargo-release) version 0.5.0 (b6b58560b7c3bc88e2b8b780be5ceb4cb508a346)
    - [pack-gen] refactor (61554e2effcbafef9cff0b407351c2fae0d2916c)
</details>

## v0.6.0 (2021-08-27)

### BREAKING

- rename `file::Store::packed()` to `file::Store::packed_buffer()`
### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 7 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 252 commits contributed to the release over the course of 8 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#163**
    - Adjust collaboration guidelines to allow and even require PRs (998ae6bf214d576cbf3f5b53f8d75e908ec63474)
 * **Uncategorized**
    - Release git-ref v0.6.0 (b191a88512d4841385d2d4806abf243e193f25b6)
    - [ref #190] refactor (e34be7e24ee49a539b6ee8dc5737fdb23f416922)
    - Release git-protocol v0.10.0 (b60ddaeda7040b71d85c9ad85b28775be9cdeecc)
    - [ref #190] more Target conversions… (1fe1b42ac2b04f8145fc7312ea03cb47f791aec5)
    - Release git-transport v0.11.0 (cac343cf279bf822e9ddd32bfa656d279631ef50)
    - [repository #190] refactor (7a111b126cfb318acb2d09d119315150a38b7cd3)
    - Release git-packetline v0.10.0 (08993382b55106cf34e6e142e84591b37e21b784)
    - [repository #190] shortcut to create references (28afd8e7cf09a17410c4a6ad57cddda608371364)
    - Release git-odb v0.21.0 (d4a63410e396d0078fd240935835548d1095326d)
    - [ref #190] add forward log iter and localize iter types… (c3e240da47021226311681f3bcd48983f354243f)
    - Release git-pack v0.9.0 (355d6c495d9fcf10f5a17572847acde3cbdd8094)
    - [repository #190] refactor (e751688a5378552b73cfddd07f38a0d0bb491b83)
    - Release git-traverse v0.8.0 (40c8506f289d5b8247dd7081b27614527a784757)
    - [ref #190] refactor (49fe1dc37c040206839c9d4399001ff12dc91174)
    - Release git-features v0.16.3 (342475f7c8ec0382432a411f15fc5dd7eadb1abb)
    - thanks clippy (023dedc41aa859cd49d208392a586deaf77bd1bd)
    - Release git-diff v0.9.0 (021318f8c176f4028b76acdcdfea8d544abe727e)
    - [ref #190] reverse reflog ergonomics (2de86f904f6ee63e292f9c701cc3524e8bfe87e4)
    - Release git-object v0.13.0 (bfaaf521d69fe2dbb419e56c9abd677fbd8a1424)
    - [repository #190] ref log for HEAD specifically (946bbf19ed3f793b0eb1c5c90a655140e12d7e21)
    - Release git-actor v0.5.0 (f74e89b024465a8649c30742e3306235d4fce359)
    - [ref #190] check for zero sized buffers in reverse log iterators… (998c7c65abb2c3eb5fc248b11ba816d09f1bedea)
    - [smart-release #174] prepare changelog (0d9a2b802d5a544a08ba1c88f9fd8fe62e8e3dc6)
    - [repository #190] reflog tests (641edde5608ff22bf18cea845ba1925b84a7b9f2)
    - Bump git-repository v0.8.0 (cdb45ffa0810e9fcc9fd25bff7b696c2d27eeef5)
    - [ref #190] First working sketch of reverse log iter access (4a36dedc17ce3124802d1b72330abc524fd98c6f)
    - [repository #174] adjust various changelogs (081faf5c3a21b34b7068b44d8206fb5770c392f5)
    - [ref #190] move remaining file store functions to extension trait (60fc215ccac529b4a14cb9d8260ab9ddec86758a)
    - Bump git-protocol v0.10.0 (82d5a0bb38903a8389e43cd5416e02e5496e661a)
    - [ref #190] Move file-log-specific functionality into own extension trait. (0b635e9778a98235cc9b47b12e58a175d1ca02b7)
    - Bump git-transport v0.11.0 (1149f1b716624f8f4fdaed20c803530aebc45599)
    - thanks clippy (376c045cf589e51b639cf6c3633c4a8fcae7b6aa)
    - [transport #174] prepare for release (f8bc51763e96d8d0a97d5f367c943441a98c8e95)
    - [repository #190] refactor (15d4ac8f4b08716f6b06938f01396fb8ba8e7086)
    - [odb #180] fix docs (bd50752dd9188acd92b8503db53cc2ce8112c61f)
    - [repository #190] a major step forward with `head()` access (43ac4f5acbe3ace5d43ed3ed1bc394d721f0e273)
    - [odb #180] refactor (eff21dae1083042412f45cd2f7a0faaf7d6400e6)
    - [ref #190] cache peeled objects properly (2cb511efe5833f860f3c17b8e5f5b4cd643baddb)
    - Bump git-odb v0.21.0 (7b9854fb35e86958a5ca827ec9a55b1168f38395)
    - [ref #190] fix docs (3e64ec102146e348b8d870377f180f8dadf5e876)
    - [odb #180] add changelog (acf1193e6b72433d4b74ec9fd39de148529224c5)
    - Bump git-ref v0.7.0 (ac4413ce4e45703d5fe722e7220d039217f0bdef)
    - [pack #179] refactor (76e66d1b9d24bb25a9f681d9612e52c8ccd60e2c)
    - [repository #190] experiment with 'HEAD' API… (c55ce4d8453c1ab4a107f5c6fb01521b422ee5c4)
    - [pack #179] move Tree traversal cache private (34e45d745cb8756831c56dc441695a25cd0069a9)
    - [ref #190] fix remaining tests (df21f25baaf867015fc9fc46a2cf4e778b0e80ee)
    - [pack #179] refactor (5a3677dd3f3dcab26a3d9270b6184fd0fe18c54e)
    - thanks clippy (14dff63fbc0d318bbc8a2618e0d72aaa98948acf)
    - [pack #179] refactor bundle (420dca29bccca6e7d759880d8342f23b33eead0d)
    - [ref #190] Use Raw Reference everywhere for great simplification… (7aeea9c36d4da04a806e68968356f8cc0dc11475)
    - [pack #179] fix docs (7ad7a4428d0e38f2ff776f7efab6996505d2bba2)
    - [ref #190] raw reference peeling (9473a71e5533e1474181241f8d3e1aebd9dea8d8)
    - [pack #179] refactor (ab6554b0cd5838f1ea4e82f6b5019798288076fa)
    - [repository #190] refactor (d6bef3afe7168659a75e26fb3ae2aa722fecf853)
    - [pack #179] refactor (620d8a54db5cd8367ec85c8b837cab710c509e3e)
    - [ref #190] introduce Raw reference type that simplifies everything… (86343416dec8026f32c57d164dec4bf9b75b6536)
    - [pack #179] add changelog (210256932a338038adb55c5475d8f90560aa4c12)
    - [ref #190] more tests (980e16a10806edba4553716d9533716a727f0c9e)
    - [packetline #178] fix compile warnings (c8d2e72d272243da7d853f78463552bfc58ed9d6)
    - [ref #190] deletions also use PreviousValue now (74f85b1fd8d9c34eca34a5ae516c4768f96b092f)
    - Bump git-packetline v0.10.0 (b09f3912e0addd7b4b0ef22bc3a24869d5011646)
    - [ref #190] refactor (0e65559e6d5a4b06c552e99e9c463559737f4b4d)
    - [packetline #178] fix docs (878d8e8d9f88a31dd9db30e381e65c1031919474)
    - [ref #190] be explicit about what the previous reflog oid is for… (c04c8b98a074d277067cee73ddef0609419a7bb8)
    - [packetline #178] refactor (0c7c5990fc71c0ee192e5ed42a6b8d268ea764fd)
    - [ref #190] don't claim there was a previous oid unnecessarily… (68f7fc2f2f57c32412ee2e46befc9cd2fdd7e973)
    - [packetline #178] fix docs (b3fd65d4130010d48afabe70b76880abcd6c8fb8)
    - [ref #190] refactor (07126d65946e981b339b6535986597cb328a1c9e)
    - [packetline #178] refactor (23438fd4a807376c1d4699732ea6c83c0bde574f)
    - [ref #190] Allow for explicit expected previous values (1a4786fb3bdb3d3a86b026dbf04e6baef6d3c695)
    - [packetline #178] rename PacketLine to PacketLineRef… (d4c16a93946244177606b58cc702b81a16424ad4)
    - [ref #190] prepare massive refactoring to get additional constraint (9741987e2f82b5ae202804882c728c1642d8e3a4)
    - [packetline #178] add changelog in preparation for breaking changes (ffd96f9fd747a99f0250444cf4b6f5a161646129)
    - [repository #190] show that unconditional creation of references doesn't is lacking… (06b9270e67823e9e911a9fa9d6eeeedcd93e62cb)
    - Bump git-traverse v0.8.0 (54f3541f1448a8afa044d3958fa1be5b074e4445)
    - allow incremental builds… (e4abcf39cba32803e650c60b9df6724ab9ae7378)
    - Bump git-diff v0.9.0 (2e2e7983178b3af7e5684995de68ed5d020927ec)
    - [repository #190] another commit() test… (4ec631c92349bbffa69c786838d2127b0c51970e)
    - [smart-release] Adjust commit message depending on whether we are skipping the publish… (c190c6b963dbaaf80a70a51135e591ee2cb9c157)
    - [repository #190] produce nice reflog messages (e7a8b62eb24f840f639aa436b4e79a4a567d3d05)
    - [object #177] cleanup CommitRefIter imports and git_object::Error (058f68a9e1cd79fd5a2a1235da42358bc92ed255)
    - [repository #190] commit::summary() (43f7568bd11fc310bac8350991ff3d4183dcd17b)
    - [object #177] dissolve 'immutable' module (70e11c21b0637cd250f54381d5490e9976880ad9)
    - [repository #190] thanks clippy (0763ac260450b53b42f3c139deae5736fef056ce)
    - [object #177] fix docs (2fd23ed9ad556b8e46cf650e23f0c6726e304708)
    - [repository #190] first version of 'commit(…)' without reflog message handling (bfcf8f17c7a89027e5bbcb5f85e3d0ba4036e8a0)
    - [object #177] resolve 'mutable' module (b201b3260e3eec98ed71716c1aab1ba4a06ab829)
    - [refs #190] refactor; handle value-checks in dereffed symlinks correctly (63bedc7647bb584353289e19972adf351765a526)
    - [object #177] refactor (216dd0f10add7a11ebdf96732ed7649d74815d64)
    - [object #190] consistent method naming (c5de433e569c2cc8e78f3f84e368a11fe95f522a)
    - [object #177] refactor (472e13b27e97a196c644d716cad1801bd62fac71)
    - [features #190] be more explicit about why sha1-asm is disabled (507d710d837c3911a9329c1c132eee912a37e1a8)
    - [object #177] Commit::write_to migration (60b936553bef3c9126d46ece9779f08b5eef9a95)
    - [ref #190] refactor (3f36a01976a149d518021f19d83e56dec43cfb98)
    - [object #177]  commit::RefIter -> CommitRefIter (e603306e81f392af97aa5afd232653de56bf3ce9)
    - [object #190] More conversion methods for Object (78bacf97d669f3adfebdb093054c162cfd5214fa)
    - [object #177] migrate immutable::commit into crate::commit (45d393438eac2c7ecd47670922437dd0de4cd69b)
    - [repository #190] put git-lock into ST1… (26a6637222081997ad7c08f4dc8d8facfb9cf94e)
    - [object #177] refactor tag write_to (7f1955916ae9d7e17be971170c853487e3755169)
    - [repository #190] refactor (1e029b4beb6266853d5035c52b3d85bf98469556)
    - [object #177] tag::RefIter -> TagRefIter (28587c691eb74e5cb097afb2b63f9d9e2561c45d)
    - [repository #190] A way to write objects and the empty tree specifically (7c559d6e1b68bc89220bca426257f383bce586ae)
    - [object #177] into_mutable() -> into_owned() (7e701ce49efe5d40327770a988aae88692d88219)
    - [various #190] rename 'local-offset' to 'local-time-support' (3a7d3793a235ac872437f3bfedb9dd8fde9b31b1)
    - [object #177] fix docs (25d8e7b1862bd05489359b162a32c6ad45ecdf9a)
    - [repository #190] Make local-offset available on demand only… (1927be7764f6af04ecc715dd52c631a3c8e16577)
    - [object #177] move mutable objects to crate::* (c551c0236c64f3237cb9be7f35159f753d4b871f)
    - [actor #190] methods to get an actor signature at the current time (6d0beddb20092a80b113a39c862d6b680d79deb6)
    - [object #177] migrate immutable::tree to crate::tree (fa5cd0648d5c855060ab2b75ee933851987c2dcf)
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments (461dc53ba3bc07d55fdb4aad7570ba9176a8b360)
    - [object #177] rename immutable::* to immutable::*Ref (6deb01291fb382b7fb9206682e319afa81bacc05)
    - Release git-object v0.13.0 (708fc5abd8af4dd7459f388c7092bf35915c6662)
    - Merge branch 'git-ref-refactor' (5dbf753ce2035ffd07e4bce7ceb3bcd4e309c16e)
    - [pack #172] A note about empty packs in Bundle writer (09a777f1da5e792c5eb4c8ff9e83504ad8d19c5c)
    - [ref #175] follow (try_)find(_what) naming convention (679895cf866d643e768e353af614a55aeed2ba5c)
    - Merge pull request #172 from mellowagain/main (61aebbfff02eb87e0e8c49438a093a21b1134baf)
    - [ref #175] fix docs (dd1edc34f88231fa95cf6f88beead700c6289ba1)
    - Fix formatting of performance-tasks.md (917967e2d464a79a119fb217f687e751394bc5b9)
    - Merge branch 'Byron:main' into main (dc58eca510e5a067acdeaad4b595a34b4598a0cd)
    - [ref #175] refactor log line (7ac948a8f8610b87aa2773ba2841cbfa43eecae4)
    - Release git-actor v0.4.0 (16358c9bf03604857d51bfa4dbfd2fc8c5210da7)
    - Allow creation of empty indices (d122fc79cc9b9a52a2817bdd46d3215c10e61129)
    - Release git-testtools v0.5.0 (574ede9d7874c6b6016bee9ab0ccc7ce18ec353b)
    - [ref #175] refactor (1243459e917b394d007bd7c157143670dc8dd51f)
    - [actor #173] fix docs (2d7956a22511d73b767e443dac21b60e93f286dd)
    - A note about the project board to help with transparency (d8500043ab6b66335e9e09ba1706564a28421bbe)
    - Release git-testtools v0.5.0 (86e0a92c7dc3b69a766aeac1b675b148d61a7ec5)
    - [ref #175] make 'mutable' module private (a80dbcf083bfcf2e291013f7b13bba9e787c5cb4)
    - [actor #173] refactor (08a18498d62f1d5bdabbb4712b08f3d17d63e16c)
    - Upgrade to nom-7 (f0aa3e1b5b407b2afd187c9cb622676fcddaf706)
    - Release git-actor v0.5.0 (a684b0ff96ebfc5e4b3ce78452dc21ce856a6869)
    - [actor #173] rename immutable::Signature to SignatureRef! (96461ace776d6b351b313d4f2697f2d95b9e196e)
    - some helpful remarks; be more specific about fixing breakage (778396568d701faf542e5b5722e6b2c4343244d0)
    - [actor #175] refactor (ec88c5905194150cc94db4d4c20e9f4e2f6595c3)
    - [stability #171] Another question to ask before stabilizing a crate… (c2bc1a6d2b2a1b0ab4963d7edf1b8ab62ba97e4e)
    - Update COLLABORATING.md (e1a04cf8b305c9346d91ff1d4e14693c08283083)
    - [ref #175] refactor (292e567eaa04a121fb4d7262bb316d37dd8ad11f)
    - Release git-lock v1.0.0 (f38f72c73f69775358d8b047de2e354364fcafc2)
    - First draft of collaboration guide (ec3f0ecf72d47edc1b3b9b5db39c6857dd8a42e0)
    - Release git-tempfile v1.0.0 (123853539dc30ddea2d822ab177ee09b191bdf1b)
    - Adjust contribution recommendation (3aae2e2edbbaa7825d9ec5c12a77c317619b6408)
    - [smart-release #171] it's about time we get some tests (48a489b4247ed6feff222924bdcdb53ce45c6ce6)
    - [pack #170] there can only be one (dce4f97a84aa6a73e31e7397501cfce27241c5b8)
    - [stability #171] prepare git-lock and git-tempfile release (3a1cf4d441b53c880b5c887916302a493ad28b41)
    - [pack #170] clru allows for free lists, reducing allocation pressure... (4d820d2f94dc3afc062bbd25e969c87410212c3a)
    - [stability #171] Prime git-tempfile and git-lock for release (01278fe4e28bf97ce6a2b8947198683646e361ee)
    - [pack #170] basic progress for resolution (ada0b96e3707c06d7d6f7e4002907e12b45f7419)
    - [stability #171] mark git-hash and git-actor as ST1 as well (32caae1c32aae38bde59756e52848bef1cef049b)
    - [pack #170] Basic entry resolution without progress (7461f31f03d67ecc9fdf398adf3cb6d4eb365412)
    - [stability #171] does this fix the issue with cargo doc? (04755323abb52f6091790d7caf2f5c1d7c1a685b)
    - [pack #170] first step towards resolving in multi-threaded mode… (f3c21f99594ab4080b8aa1ffed9ea8a33e18fabd)
    - [stability #171] git-ref is now ST1 and available through git-repository (50154cd02fdd90930a1d7c5a4406d53c8067cb4b)
    - [pack #170] Don't double-lookup trees during traversal… (7b068296fe5ca10af212d8fe2662940188b7359c)
    - [stability #171] fix schematic (999e81335034f3505552740befbced4680870297)
    - Revert "[pack #67] Don't pre-fetch packed objects during counting" (811bb54991636f7e517087b62cf0c8c8cc2ad9e6)
    - [stability #171] Simply commit on git-ref/git-config stability tier 1… (f6560ffe8b9280c7e9c32afe0294ea3ee169dcf5)
    - [pack #67] Don't pre-fetch packed objects during counting (d08b6739d8e9294b795aba75e9c7f9f20645af2b)
    - [stability #171] Add the concept of Foundation Crates… (8819bdeb96b5c23baf3b47df990aa33b04dcd174)
    - Release git-pack v0.9.0 (7fbc9617da97d4ba4bb3784f41d4163c0839c03c)
    - [smart-release #171] Try to avoid unstable git-repository features… (c8f325bed5d644eded035109702098f9fed3fba3)
    - [pack #67] refactor (14717f6132672a5d271832a68de0b323b73abb2a)
    - [stability #171] Don't suggest pinning of pre-release crates… (9301bbf0b227448b983847a0c2689bd76f8154ae)
    - [pack #67] Optimize caches based on cache debugging (1271c01d2635ab49474add61a9feb78b98bd6180)
    - Merge branch 'main' into stability (11bae437e473fef6ed09c178d54ad11eee001b1d)
    - [pack #67] Add cache debugging capabilities to git-features (8776c9834ac4622b3057f5db464a9817ed9acdb0)
    - cleanup imports (e6693032f1391416fd704c21617051ddfb862a3a)
    - [pack #167] Use custom uluru version to avoid a lot of allocations… (7bd3671ad949d62f84147ef7ff3fde59937fee54)
    - update dependencies (e9a98bc0078189f58b7c6e47bf46949cbe0730ee)
    - [pack #164] fix docs (08ee674c55cef6ab76520de2f836b246c907888c)
    - [stability #171] Don't provide access to less stable crates in `Respository` (e4c5b58ad935c907dfbd0d61049453dcb64a7e19)
    - Merge branch 'main' into 162-repo-design-sketch (e63b63412c02db469fbdb17da82cd1e9fda1ef0f)
    - [stability #171] update README with stability information… (f330daa06577eabbd61c66526710371a14228274)
    - [repository #164] top-level easy docs (6b71c51f703aa3b6a7d5a110d04294dd7ea4e8b0)
    - Revert "[pack #167] Use custom uluru version to avoid a lot of allocations…" (4c2ea212bbffb0ba3c21ba388dfc79cc7a1c4734)
    - [stability #171] How to handle the MSRV (9be1fcedf94e65b84f9769f74410a7c4f374f6ba)
    - [repository #165] see if `git-config` can already be placed… (d287a4aec70e5dd33976a25d9a849c44d62d77c9)
    - [pack #167] Use custom uluru version to avoid a lot of allocations… (8d499762b74c08437d901bb98806e0a1fc6f93bb)
    - [stability #171] Don't leak unstable plumbing crates in git-repository… (71eb30f1caa41c1f9fe5d2785b71c9d77922c2af)
    - [repository #165] fix docs (b4fdfd7a21057f89f4b6263c0c291003241e2833)
    - [pack #167] a single-threaded special case for counting… (65e29de45a92c82cebd832634ab194db19a1b590)
    - [stability #171] about transitioning from pre-release to release (bdbdb653d172b988a7cd91810bacdc6cd2ba1626)
    - [repository #165] add limitations along with possible workarouds (7578f1e2e578010eee087a9176d53a5862ec8862)
    - [pack #167] generalize over immutable insertions… (169f000087aab18f0257fb0c61dc3b3901e97505)
    - [stability #171] finish tier description… (4fe125973304b765f0171deb1c26bca64bbff5d7)
    - [repository #165] assure packed-refs are always uptodate (a5605df9b83a25f1726b181b78d751987d71a32b)
    - [pack #167] refactor (6bf0f7e86312b2a4d262c80979c61c94519bd4b0)
    - [stability #171] Rough descriptions of ST 3 and 2 (340935c7c2ba34785313e529e2ed93c84abc2cfb)
    - [repository #165] Allow cloning packed-refs and try to see how it differs… (7ec32b7662995b5a60aba1bd932830e68ab1dbdc)
    - [pack #167] progress is handled by reducer… (a22f8e171e705bc42fcf290789e8e05423bd72d1)
    - [stability #164] First sketch of stability MD… (a7353cd1d9999be4744a1c70a37f3c0ffaad706a)
    - Release git-ref v0.6.0 (0bb4c133da96f6a96d9f1767848ada792a27c2be)
    - [pack #167] Error handling for object input (0aac40c88a5c26f7c295db8433b510b168f15ca3)
    - [ref #165] refactor (66624c3ef1faf7048ee86ed73cf5f622802c061e)
    - thanks clippy (d689599d1b819c18a3be60075170dbe00462e216)
    - Revert "[repository #165] PROOF: GATs will work as expected!" (853f0723d3d202b1cc2e653109ae92aa14d4d437)
    - [pack #167] remove iterator based count objects impl… (7ec2f2b40e83aaa218360a8b5989792cd67de2ed)
    - [repository #165] PROOF: GATs will work as expected! (7f56dbd82db2abc18b8e6d228c8a5f54b3dbf32a)
    - [features] refactor (0958fc8dbaa72dda0c1e2d40a88d74b4e18bfe39)
    - [repository #165] refactor (1547d0b062e35bad2229dac532e6f30bf105db73)
    - [pack] A non-iterator version of parallel object counting… (04fe855a37577d3da5bbd619807b44e449947893)
    - [repository #165] refactor; fine grained allow(missing_docs)… (aa0511f80f11de8e83fc333e78db369ceb9b2794)
    - [features] refactor (d4605cde6d825c0bfaf4282c4cbd85d9f07dc43f)
    - [repository #165] prepare for writing light docs for Easy (f8834c9c8d2ab2ce87857c6773c6204f60df240e)
    - thanks clippy (41d7a443aa63b6ee997fd38ceee05b9b1be3e577)
    - [repository #165] refactor (3a0160ed1c5bc33d330ad4e9189c4937d194e98d)
    - [repository #162] cleanup imports (983d11a1f46c1ad21dbf2d57b63ecf979fab48b9)
    - [repository #165] fmt (a02d5aa8ef0e4a1118a9d8523c3f34b836461952)
    - [smart-release #162] use TreeRef capabilities to lookup path (51d19433e6704fabb6547a0ba1b5c32afce43d8b)
    - [repository #165] Don't panic on repo borrow error… (b2f644a73c2b1945ab71c5f5719c9b2b32c01b07)
    - [repository #162] what could be a correct implementation of a tree path lookup (1f638eee0aa5f6e1cc34c5bc59a18b5f22af4cbc)
    - thanks clippy (b496d9952924afdb67e9ba8ea0b9b61c8c8fb1f2)
    - [repository #162] detachable ObjectRefs and a few conversions (ec123bb615035684e52f2d786dfb41d0449823d2)
    - [repository #165] Write about the GAT plan to make this better one day (d793ecd00f55b5bf7c6dcaee8772975e97bd5e30)
    - [repository #162] finally let smart-release use the correct abstraction for peeling (ba243a35ff6f059e5581c6f7ff80e1253ceca6f8)
    - [repository #165] quick test to see if Access2 can become Access… (45acc7a9d6a89977563872c2eac389a2b78b9e27)
    - [repository #162] Add id field to ObjectRef… (f5ba98ebd0e1d7d0491871be58476cb6882b8436)
    - [repository #165] Generalizing over mutable Repos is possible too… (0f7efe3f2e2608213ad5c75b52db876dd4214908)
    - [repository #162] Make clear that Objects are actually references… (d1e68435d0b7d9dcc9e0099be3c0c5723dc08e93)
    - [repository #165] show that Access2 works for all Easy* types… (b8ceefed275953aa36d823d51b466cd100729905)
    - [repository #162] another attempt to find a decent peeling abstraction… (716d623fb189eb3002d2137827dbfeb143f6ed12)
    - [repository #165] First success with creating a shared borrow to the repo (f2a38b20aee484e0354d3e2e3db9cc880ae95310)
    - [repository #162] attach the Object to 'Access' (9a125640da19d5633e51df40dee5332eb9600462)
    - Revert "[repository #165] FAIL Look into `owned_ref` crate" (a1443e4982fa4d1a1615554a37294d56fd9026eb)
    - [repository #162] refactor (a32d361fd5cb0eb1a4112d834b53c1625372a7bc)
    - [repository #165] FAIL Look into `owned_ref` crate (09aa714f2db5ad220b0e76a65e01e394663f08b4)
    - [repository #162] trying new names (b3f453b33f8cda04526110a82f0e0a46a3bb2e34)
    - [repository #165] FAIL AsRef works for basic refs but… (02979b61e6bc4e1de3b3badc784a950477b31cad)
    - [repository #162] put impl for finding object data into the extension trait (91b9446fc7035047ebefaa7907e6a8224b56cf27)
    - [repository #165] FAIL try to generalize with Borrow… (295ba95a341775b566c18e897a2d58a94e6d98f9)
    - [repository #162] experiment with finding objects… (312a69256a67a0f9d3f3f5c5f9eaf51b50971c5e)
    - [repository #165] FAIL See if EasyExclusive can work… (016debbfce7a29502742408da304c80405063230)
    - thanks clippy (f2fb0266ba64d002a9913699bcf5843647843beb)
    - [repository #165] introduce EasyShared (a119ad94096a3464b98f6a6bc26c92ba6efa9474)
    - [repository #162] Cannot ever store a RefCell Ref in an object… (5c171995383fa9a3698b6aaf3fbd9537110c0299)
    - [repository #165] First thoughts about stale caches (7f8b63e23ef3561117249668d14507cec1508ad3)
    - [repository #162] experiemnt with optionally keeping data in Object (b8a8e08e1d972e5069b136c30407c079825b7e1d)
    - [repository #165] hide all easy::State fields behind result-enforcing methods (000c537ab766a50679764118af50731b3bab39e5)
    - [smart-release #162] Fix short flags (08f3418a0b763b7860d95536446fe615cf361adf)
    - [repository #165] pack cache access only with errors (2353e5092599228f147ef58c0f0cd45c63c126e2)
    - [smart-release #162] Object can be used like a git_hash::ObjectId (c7bc730836f05fe9d967320a6858443a649a59ce)
    - [repository #165] assure packed-refs is only used non-panicking (a355d943b986307216161bad38e5bb89f8608b49)
    - [smart-release #162] format everything (8ff83e5c511ae29979348789bd6e7a2f72b16f1c)
    - [repository #165] refactor (16fce637561af29727a8fa025f6ddece853fcc20)
    - Update crate status of git-index to reflect recent advancements (a258d2d6675f6266147c621d291578e2c2121fdf)
    - [repository #165] a sample of a simpler way to create a tag (fb8f58412cdd32991a182a41cbc0d463127a4e0e)
    - [smart-release #162] don't throw away work… (b43b780c0382683edc859e3fbd27739716a47141)
    - [smart-release #165] Use generic edit-reference functionality (be3e57f6221dc87505ba1aad1166e28c328c3b54)
    - [smart-release #162] a demo of attaching and detaching objects… (ff2927ce3fede654d491559fde1c7b07be6a6979)
    - [repository #165] sketch generic ref file editing (3a026aea2a98648a6b624bca9661555f5a147494)
    - [smart-release #162] an actual Data type… (7fd996f5f631f83665e81c0f89c34cc47f270d2b)
    - [repository #165] refactor (00ec15dcfdb839095e508139d238df384ea418eb)
</details>

