# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 11 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 171 commits contributed to the release over the course of 12 calendar days.
 - 43 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 7 unique issues were worked on

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
    - loose reference iteration with non-dir prefixes… (293bfc0278c5983c0beaec93253fb51f00d81156)
    - Add 'references().all().peeled().'… (650241251a420602f74037babfc24c9f64df78d8)
    - smart-release: filter refs correctly, but… (2b4a61589a7cba3f7600710e21304e731ae3b36a)
    - smart-release: find tag references by name… (72e175209441b12f3d4630e5118e21a3156146df)
    - commit traversal along the first parent… (7bce49c1d27cb279b61ff51de0038e01fcf3561e)
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

## v0.9.2 (2021-09-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-diff v0.9.2 (17c411f7679f4386eb3225c56dac80084787ed2b)
    - Bump git-object v0.14.0 (d4fc81f6390443f8c8561d91ac27ea4a6318fb62)
</details>

## v0.9.1 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 74 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-diff v0.9.1 (cedae8d61f44a2de46edbac8afe19b7d8fa15cbf)
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
</details>

## v0.9.0 (2021-08-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 59 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-diff v0.9.0 (021318f8c176f4028b76acdcdfea8d544abe727e)
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
    - Release git-object v0.13.0 (708fc5abd8af4dd7459f388c7092bf35915c6662)
</details>

## v0.8.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-diff v0.8.2 (3ad082939c52cfd6d679ebefcbaea4b16b12cfdb)
    - Apply nightly rustfmt rules. (5e0edbadb39673d4de640f112fa306349fb11814)
</details>

## v0.8.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-diff v0.8.1 (41b218f456ceea448d3b6a524e05970c478bdf6b)
    - remove dev-dependency cycles by removing their version (c40faca41632cd2a226daf4ddf5293b65d1fdc82)
</details>

## v0.8.0 (2021-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 (f123f69c7a4f9fd1c98bd2f60ebc953a6739fe04)
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 (c67291ff9bcdff9a747d87241f6a71015607af05)
    - Release git-object v0.12.0 (7006150ac314d19814608723f69f6e70a72f9262)
    - (cargo-release) version 0.18.0 (b327590d02fec5536c380b2d39dd7be089ca7c40)
</details>

## v0.6.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 (4b71e15c3ba4a17ff2da5a5ef79986a2832fa3f2)
    - (cargo-release) version 0.5.0 (e21142ba1a113b2afc4725d4d4225dff519c513a)
    - (cargo-release) version 0.17.0 (c52a49176bd294bb36db74b4293cdb684a2ab7f6)
</details>

## v0.5.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (1687e599be98d97925fbab594f31cf5558e9d2b1)
    - (cargo-release) version 0.4.0 (28e58f6b43a44e010da749a5618df02441f0d2e8)
    - (cargo-release) version 0.11.0 (a5be31c4cf7c0b538a1ed4a52ff5c3a992c6feff)
    - Revert "break more dev-depedency cycles up to git-odb" (22337ce23995eee474e7dfb2e37fb56814522942)
</details>

## v0.4.1 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.1 (9790c1590ec7180b76241b9f5ad7711d13abc7cc)
    - break more dev-depedency cycles up to git-odb (7ee278bf5b04adc5e4ab82cb83a3519f93587176)
</details>

## v0.4.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 19 commits contributed to the release over the course of 83 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - fix release order to match actual dependencies (65ff8c1c106182820dc6e4a308f71708e657f07f)
    - (cargo-release) version 0.5.0 (ae02dabae961089a92a21e6a60a7006de4b56dad)
    - clippy on tests and thanks clippy (a77a71cf02d328a2a964388928d6b2a235a0aa85)
    - refactor (a92f1e68beb0f946d0e117934b244d5aa1b6b5fc)
    - (cargo-release) version 0.4.0 (866f86f59e66652968dcafc1a57912f9849cb21d)
    - [git-ref] the first failing test (7e802a0576230dfc666c253d484ea255f265f92f)
    - [git-odb] refactor (2958145a0ae1ef582bbf88352f5567d5c2b5eaf0)
    - (cargo-release) version 0.16.0 (769c649c00c009bf5a3f7c0611a7b999618f2938)
    - [git-odb] refactor (721303db232f87857aae58e12b342e5fb0139306)
    - [git-odb] refactor (ea224e9ee5f7efcbf4942a2a6fc7e4d790b2be50)
    - [git-odb] refactor (6a1b16ae98edc9a694b945a12a7866eb17fc6be3)
    - (cargo-release) version 0.10.0 (5d7ee6a105abbb6efeed8624bade936bb59dbc55)
    - [git-traverse] fix potential lifetime issue (fcf2e8fb5356e5d4fb541347a9ca37306362815a)
    - [git-diff] refactor (fa8b4e8549c5992b8e622979aba3d11a6197bcc3)
    - [git-diff] refactor (9373cd6281b679d556255893ab0252e33bb86e77)
    - [git-diff] refactor (087e85367c27bb3684c6ad543c7eae46762e5e44)
    - (cargo-release) version 0.4.0 (c85d59a9a63d3cb503d906dcbeff2e585e4397e4)
    - [git-diff] enforce greater restraint when using path-ids (ad893203912d60f382dab66bcd38e2fc312b7246)
    - (cargo-release) version 0.3.0 (684de4b376ecd4cc5330f7ac8643352ea9580ed3)
</details>

## v0.3.0 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 6 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [track publish] git-traverse/0.2.0 (56b1a555e265c037c253c4ac3ccea153b27081e3)
    - (cargo-release) version 0.15.0 (d91b2412381e3c8c1f24c38469e821c3c3960e34)
    - (cargo-release) version 0.3.0 (3f2f8de01088f8bf09ff04443534db513c522f6c)
    - (cargo-release) version 0.2.0 (3fb8377ff36422fe7607fb9172edf8bd5a4db995)
    - (cargo-release) version 0.9.0 (84897fd8e6e1b0269da0303d6a0de8f9e0eb58e5)
    - refactor (082f8d0a4219246050d4594ba8cf769c8f5cdc90)
    - [traverse-tree] one test to pin implementation down a little (f0aeee1ca3d9c0fd1290c1912226c7dae396e10b)
    - refactor (cceff1cf5297a6e507f8b44672181ba2600c748c)
</details>

## v0.2.0 (2021-05-02)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.14.0 (d9514eec64579ef77c9f2ac5dfe87cd302180eb9)
    - (cargo-release) version 0.2.0 (ca48e06b19076db961d81f8759ae564d5a5b7f6c)
    - And it's a wrap for git-diff docs for now (9e09dd560a23d52d0469ce4fc13de01f7efce227)
    - refactor (6e6453d9e044499c9ee0a85d79dd75906adb9fb8)
    - [traversal] Add remaining missing docs (2f573f39c47879f7f318be9efa357e10a9e14ed2)
    - refactor (c0318cfa13dc32cf6c01879feae60158bc46d708)
    - git-diff docs (76af15b708842fd0adaef6f685fd40101e8f7d72)
    - rename 'Locate' to 'Find' - shorter and just as good (60f72f573a7696323e09bf4add80d5fbce22c99d)
    - (cargo-release) version 0.13.0 (5c791af217fac6a171d174ad9f4ee5f4d5282892)
    - [traversal] experiment uses git-traverse (360935640cbae5b691dcd976422bf00f9768e1c0)
    - [changes] more flexible handle of state (11db16b585e7551fa0d85644ee085b95a9dc2c1e)
    - a new crate: git-traverse (1a9af50f1fca0e7e939f339b885c66dcb95e44e5)
</details>

## v0.1.0 (2021-04-30)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 65 commits contributed to the release over the course of 4 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - git-diff - fix include directive (c684382f5cac8c667a0a19b9b2cc95bd32d025d5)
    - prepare test utilities for release… (d35e654747f96cec93bdecd1314ce325129cbc44)
    - (cargo-release) version 0.8.0 (a1ce210003ff07bf11291018bb182cbc7913647b)
    - (cargo-release) version 0.3.0 (e9665c784ae7e5cdaf662151395ee2355e9b57b6)
    - (cargo-release) version 0.1.0 (cb7b667255eafb6e378569892f47574533a698dc)
    - [traversal] run libgit2 parallel first to have a chance to get data more quickly (0a3564d5e949e328ee2923ee1b96a5d369102f9b)
    - [traversal] add CommitIter::tree_id() convenience method (6affd9d90d56d89774fcd4843638309a198815bf)
    - [tree-diff] another test, but no new outcome except that it seems to work (e295b539df0bb3e4ae7093f09d6dcda8326029c5)
    - [tree-diff] And another test that showed something was indeed wrong (362680ff77f00dd305939090cb903003ff7be679)
    - refactor (85c5781def8b45b01d4e46af97bbf24e1aa6da88)
    - refactor (109c4e0bf2ecb307da882d42584f769da19db02d)
    - refactor (e7a7ee81b0b40336671b28b7eecbac6ce40c4c23)
    - [tree-diff] Beginning of more nested test-suite… (b8a90e7c9347b0eefdbef6f4c724cc0561cd79c9)
    - [tree-diff] the last todo, gone by test (d7418f3342319a31b3f591ecfe1d5d9b1b198e9c)
    - [tree-diff] consider that windows does do symlinks differently (b1b6e0014dd02b66db538a262c4a0f7f891870e5)
    - [tree-diff] another green test (2627df0bbb9da9eb8a3d1bdbe725fe35bf24071e)
    - [tree-diff] be independent on commit hashes (05e8e4a060d8e47e6d98e188d8a93b01947f8035)
    - [tree-diff] another green test (1bfa9daa95bf5a5643f3b70fdb8031e757ae1506)
    - [tree-diff] another green test (9ca57fa9bc7a52170d109b323b1b1a74172604c1)
    - [tree-diff] a new failing test (c6eb6773f6768f3b24a4267ba2e0d3e6ce0aaa14)
    - tree-diff] another test (1eb961c8f22e8dc4a1988da09ce6521ca26fbfb4)
    - [tree-diff] less todos (that break tests if present) (03f87fe4bfa002aec57a074c64835ceab120fee9)
    - [tree-diff] another test (b23012ebb943a0382b5cc3c2757a763f9183dda8)
    - [tree-diff] looks like windows now does line ending conversions for us (ff32a8f98d96a7fa28c7e0f4021d4a7ed7e30787)
    - [tree-diff] another green test (ec681da870e1677efc6c97dba35c1ccf21ea4724)
    - refactor (d5509369f509feddb1c3c10bae8b65c5dd3da35f)
    - [tree-diff] one more test green + refactor (bc5549db2ad16222761219d8652caf64867a889f)
    - [tree-diff] ManuallyDrop turns of drop behaviour, and I think it's Ok… (b885805e9d9cf5a02635b86cd5f86db5bbf57a4e)
    - [tree-diff] [FAIL] try to use peekable()… (0dcdc0efd59dda8a14db38c8a064d7caca9d1e0d)
    - [tree-diff] a step towards catching up with rhs (bbe7beb606071610f1506ab1f29456eb79f56f8b)
    - [tree-diff] more tests (none of which hits new code paths) (791c4291926fd3aa2ab413d1058b2257976e8d87)
    - [tree-diff] deletion of directory and replacing it with a file (28e3fdd54036dcd4a227062e9db01017196c20e0)
    - [tree-diff] test modification within a directory (ff82a82c1bd1b884afddea5baffb7448437561d1)
    - thanks clippy (c223e31074d989024e22e8331eeb4280fb01cfab)
    - [tree-diff] The first example of recursion works (f86566c646d8c9a1bb0304508faecc0e2eb163d8)
    - step towards zero-alloc traversal (f554c77b8371deb987e2365381b85dd6d4325b74)
    - refactor (ca1359414c6dc0ca3f9052299c7f088d83b38777)
    - refactor (aa1897d870df3fb76193f7e4f33e135760732288)
    - refactor (a717dbaaafcbb0869bd189f1b625e5ff84a9ae72)
    - refactor (8087ca3e856f2c5a9c409a94ff8b54fcf295c894)
    - refactor (46583c1fff415f742466b93c0821b21e7c9e7e1c)
    - refactor (fdc8c7975a67b332eff995ca8046cafdb3bbeae2)
    - [tree-diff] refactor into iterator based model (29b527aaea101c9b4e885db1c6d3533ef2310c54)
    - refactor (9ce98322bc578832495082e8a9c147d12542262b)
    - [tree-diff] A step closer to handling additions in a directory (a11f210bec2c6c55bcf8cebe00e116e835306360)
    - [tree-diff] actually windows might have a point, let's see (0020a7cc368ffc5b62d6618f94a4cdec36c6d512)
    - [tree-diff] detect modifications (b87f2b46783152964c24d6e7566a1787be60a932)
    - [tree-diff] See if this works on windows (95db1de95a585ac1fa8a185b201300e86e5f34da)
    - [tree-diff] the first succeeding test - additions (619d4f05516ca0e54016b7ee8ab0433d6839ef7f)
    - refactor (a4d5f99c8dc99bf814790928a3bf9649cd99486b)
    - refactor (11018e1453ba6b130403f6d9f699881a93955c06)
    - [tree-diff] The first proper test with an API I like (ae6944eaf874a7d52f1f061e5d0d0a4d642c20b5)
    - refactor (633cba7c1ff1f63c32613bedf963d1bd89afaee1)
    - refactor (3c10d0613ec00606a678c65e05ab1fda0ef742f7)
    - delegate-based tree diff traversal for maximum flexibility and performance (cbacca0be8bc8cb968b26438fc2caf48a447c542)
    - Maybe avoid even more allocations? At the expense of usability. (230ef0447a56e9acd28efc6b71c5406e1b43653c)
    - probably a good idea to just use a graph for now to avoid a huge trap (6b43cdca4749840fd179492bf9b7d7b9fb595814)
    - Sketch of how changes could actually be returned. (a48db50049657f8299423c8eaacc1d44da0a5b34)
    - refactor (03ee510a5f9c24b6acddaec1d30ea3ad39174603)
    - Second sketch of 'fluid' diff API that hopefullly makes clear how it works (ef6d469dfe22b8cdc816960b1be717483e3cdf8f)
    - First sketch of diff API (fc3f2b7066538e31f8d4bb1053d70dcabd5fbab1)
    - Better ergonomics for accessing decoded objects (ae3eab6d6e4b96e207372fa8cb82f5ac9833e5e4)
    - Make sure releases of 'git-diff' don't get too big (378dde703978812c6ffa39b51a4a7edd19a903ba)
    - Frame for testing tree(&tree) diffing (28c78f558e625f1d61bfa455f43bf6701e71703b)
    - More explicit expectations towards entries in mutable Trees (d94f84ceac637d2b6495be01dfc8eeb2494922f2)
</details>

## v0.0.0 (2021-04-26)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Fix includes to assure generated fixtures aren't can't be added (9bf4fc4616b1d04f2ea7215b4c835025e53feb32)
    - Add git-diff crate (42fdd8d94b6fb65c1900cfef4f44dad619f7f09d)
</details>

