# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

### Unreleased

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 16 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 214 commits contributed to the release over the course of 12 calendar days.
 - 48 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 8 unique issues were worked on

#### Commit Details

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
    - Implement --write actually (69d36ffbeea68259add2d8e15a9eb74137b14156)
    - Parse more user generated section content, adapt existing changelogs to work correctly (2f43a54298e7ecfff2334627df149fe0882b5d1d)
    - Sketch merging logic… (1932e2ca848db57f3907b93e804553524dfa27ac)
    - Make use of fixed git-conventional (b7b92b6c72051d462ab01c7645ea09d7d21cb918)
    - prepare test for basic merging… (0a14cedbd68058ac296e34a84ab1fe1083a0bf5e)
    - update git-conventional dependency (2d369e863b15269ba8714b025fe596f69e5b1217)
    - nicer 'thanks clippy' message (43442162aa22f561a33cab78936514d05d8214a0)
    - first test and sketch for stripping of additional title values (55b7fe8c9391e3a9562e084ae7524bb9f83ec36c)
    - Basic message parsing, either conventional or not, without additions (b3b6a2dc07c2eff38556ee66b9290b0c66b463ed)
    - Show with simple example how the round-tripping works, neat (9510d9bd2c3b2d5cffe32485d7bc3fff374343ee)
    - collect unknown text so things don't get lost entirely… (60040c9301e6468c72a0c52095c0b86f8b3041f5)
    - parse back what we write out, perfectly… (5cab315b0f28d9b9f6f6b4e037d053fb501fdfaa)
    - fix journey test (3006e5975e023c9ad56e62ce3163dd65964c0c57)
    - feat: `CommitRef::message_trailers()` as shortcut… (5324391f581c5ad2c09604f0beeac7df852bfb33)
    - more tests for trailers iterator (c3b0161eb76aaf806a7d82232ec7ac1a304052a3)
    - Write new changelogs with bat if available (cca8e52fdd2ebd16b08247d428ed5387a1058cd5)
    - feat: `BodyRef::trailers()` allows iterating trailer tokens and values (175e1cbdfebfc6f01784fffdaf0859cd6c23377e)
    - Use `cargo diet` to reduce package size (cc5709e812aea79e9d9a6f16637d09f22cb73f81)
    - Some tests and sketch for BodyRef parsing (3953c245461941c636ce5d755e6a469f7fa3eabe)
    - Write markdown changelog to lock file (400046ec65100a15cd1757143c1abba05091f129)
    - refactor (b05ce15a31aba9b0084792b7f0e7155b73b46e2d)
    - feat: CommitRef::summary() and `MessageRef::body()` methods (1714d05df812aa373da485492b342e58e9e7c17d)
    - refactor (7055dc81e9db448da89ab2ee0ba2ffe07cd00cc2)
    - Basic serialization of ChangeLog (205b5698072c6919036190cacac120a7dd5dbd73)
    - Another test for footer separation, simple version (b4391862b67a09330476a82d520bfc3a698a4fbe)
    - support for generated headers (bcc4323785c5aca698e5af2ee5cf32e171727ed3)
    - Return to safety (35313b9f7c92edd1afdbe22d1f592baabc0abc9c)
    - refactor (1ebb7365ce564d55bd4f16f7316375b9458b4659)
    - omg nom parsing works… (cd11704dd0d469cd23d7ee6905d6219b26ba4563)
    - Use 'to_*' when converting `easy::Object` to specific object kind (1cb41f81cffe19c75aadf49a5cc7ec390ec6cae7)
    - FAIL: not really successful to continue down the 'fold' road (d9afc22f161fb60195571be09d2d816d67050575)
    - transform history segments into changelog parts (348b05cbe6e93e871393a6db9d1ebfea59ec7fdb)
    - three tests failing with nom (13646e8dfe97d8503d0cef935c4c303f82271aa4)
    - layout structure for ChangeLog generation from history items (40e9075238f7272c08497851f55d0b525f47f2db)
    - Revert " FAIL: try to use nom-way of the previous body parsing…" (d1e6f621c2898ad9f03b2ee712019e6a10b44035)
    - more general commit history (39522ec59d2eb7f439c75a5cc5dc0315db9497d5)
    - FAIL: try to use nom-way of the previous body parsing… (909f6682ac1de6be0eb8b66015b3f250daca17cd)
    - Invert meaning of changelog's --dependencies flag… (51eb8cba67edf431ebe3e32232022dbf8971e6ac)
    - sketch nom version of the message parser… (1ec47ded5793cac1f2633262d59bfbae4a0e14be)
    - rename --skip-dependencies to --no-dependencies… (77ed17c703e502e132cda9a94eb8c63db0b627ad)
    - Research commit message trailers just to learn that I want to skip them (c332b8fb335f6c4081add894c3fcdcab298fc828)
    - Adjust changelog… (fb0dbfc60df2df51ed6a02ad60d04ef3557e950c)
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
    - split data::output::count::objects into files (8fe461281842b58aa11437445637c6e587bedd63)
    - use new git_pack::cache::Object trait (b209da29f361512ba757febf56bc1aca039f2a41)
 * **Uncategorized**
    - Merge branch 'changelog-generation' (bf0106ea21734d4e59d190b424c22743c22da966)
    - thanks clippy (b856da409e6a8fdc81ea32ebb4a534b0e70baebc)
    - Merge branch 'main' into changelog-generation (c956f3351d766c748faf0460780e32ac8dfe8165)
    - thanks clippy (c55f90977756c794939454072e4cc648f1e4348f)
    - don't claim to change manifest version if it's the same one (11eebdcc572a72b2e66a9db3cae0a01f12a81619)
    - thanks clippy (b200ee8d7522f0c83e0e01f0d793784cba7028aa)
    - thanks clippy (4b3407d0baf32b6eeb04cee07faa4bb9c1270e4e)
    - Bump git-traverse v0.9.0, safety bump 8 crates (d39fabb8757369aa19452a457f610fe21dc13a14)
    - thanks clippy (1dece2b8dd18d0266210152c749c39595d70db5a)
    - thanks clippy (2b5542761ab160cd9460b133928efd6f0cb55e75)
    - thanks clippy (4ea11264296063278977c5539e2d68367475464a)
    - thanks clippy (a89d08c4ce28f0c466f01758e9f4db09eeb02458)
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

### v0.10.1 (2021-09-07)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 64 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-packetline v0.10.1 (4f9da02ae0f0ce8e62b20852319f46ab26b88d89)
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
</details>

### v0.9.1 (2021-08-17)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-packetline v0.9.1 (2276e2aefb8a4e51024644826249b3f97da2ccdb)
    - Apply nightly rustfmt rules. (5e0edbadb39673d4de640f112fa306349fb11814)
</details>

### v0.9.0 (2021-08-13)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-packetline v0.9.0 (7ffbd602c08605026b0bb97ab85216907badaf09)
    - remove dev-dependency cycles by removing their version (c40faca41632cd2a226daf4ddf5293b65d1fdc82)
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 (f123f69c7a4f9fd1c98bd2f60ebc953a6739fe04)
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 (c67291ff9bcdff9a747d87241f6a71015607af05)
</details>

### v0.8.0 (2021-08-11)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.8.0 (ad6d7f9c2b4f8879d466e758fc9b51ece6879e96)
    - (cargo-release) version 0.18.0 (b327590d02fec5536c380b2d39dd7be089ca7c40)
</details>

### v0.7.0 (2021-08-11)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.7.0 (2ef3106eb84981e2dabd84f81362b4e44f938ea6)
    - (cargo-release) version 0.17.0 (c52a49176bd294bb36db74b4293cdb684a2ab7f6)
</details>

### v0.6.0 (2021-08-10)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 6 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 136 commits contributed to the release over the course of 89 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#77**
    - [git-packetline] refactor (aa61993066b0bcb29e53edbb6eb1525781827426)
 * **Uncategorized**
    - Revert "[ref] break dev-dependency cycle" (436e89b18cb157b3d30bd24b8d1acef25631ec2a)
    - clippy on tests and thanks clippy (a77a71cf02d328a2a964388928d6b2a235a0aa85)
    - thanks clippy (e1964e43979b3e32a5d4bfbe377a842d2c0b10ea)
    - [ref] refactor (bd94ea55c1b598e507b5717ee5a5d6f14830c3bb)
    - [pack] fix docs (e7b9d9613874cd1ebaf740dc08db467c461a4751)
    - Bump futures-io from 0.3.15 to 0.3.16 (3c23820d3f0d3567f44215cdb0ad13ab675a201f)
    - [ref] basic lookup rule impl; needs more test cases (3226f775129231b4bc4735baf9e14a187665ace3)
    - Remove unnecessary unsafe code (83e207a44aece0ff4870e57990bd5aaf43f38e22)
    - [ref] fix compile warning on windows (c32877415aba8df6d5a37cfd799b218e3a29b18a)
    - Merge branch 'parser-into-iter-without-alloc' (a799ca8d6c2e51303512160ddef7477e176ab01b)
    - [ref] a test case specifically for lookup rules (ab3a34f481ebe335578e3a7dbff325087b4ba647)
    - Implement Parser::into_iter without extra allocation (aa79924b36c0d717cc65d7471fedd27eb41e83a5)
    - dependency update (059fa3318e3e76c407e456d28a28cb834d532719)
    - [ref] improve parse failure handling in packed-ref lookup (ba62aab4308d44092d151d11d9be44ba6bfddb02)
    - Remove unnecessary pub(crate) exports (3d2456e11709f0461b37c6df55ecc3861ca4cab5)
    - fix docs (2698daec29ac68f928a06f2bc9f4df44fcc8222c)
    - fix build (22bda81712b1379869abf764d47c05e03f697a50)
    - thanks clippy (3f7e27b91e2c7d66959f5f4c1a667f3315111cd6)
    - thanks clippy (6200ed9ac5609c74de4254ab663c19cfe3591402)
    - [async-client] unblock the async delegate in the cheapest possible way… (a3b5d75d387dc5d6c44f695f63df8803613637a2)
    - Revert "[async-client] Try to bring 'Send' back but…" (52eb953fcc44cce19604b1df6a600237b8c81392)
    - [async-client] Try to bring 'Send' back but… (3a06adb41f6b2946f78044e4ab1385e6441fc40f)
    - Prevent selecting mutually exclusive features (7f5da18c39b84af788ea1366ccca2c8b9d09f755)
    - (cargo-release) version 0.2.0 (3286e42547b59df6365087cbae9ce1c9c959faad)
    - Manually fix crc in tooling (48fa9bc80876a0186f43add6c6d3477385241f5e)
    - [git-protocol] update cargo-features (1fdb5ac0663eed28d389729bf7ca4cc5d2c876a3)
    - Bump crc from 1.8.1 to 2.0.0 (07f08ac1ea04ec278993ad1a5fc1d4f243bf8eb7)
    - [git-protocol] remove compile warnings if no client type is specified… (478a98056afd2504050391262dabc921b59425c5)
    - tryout dependabot (872eb12825b4222bd86d0aeb092d82f505d0c08a)
    - thanks clippy (57106e21089ae3c3a529295bceb8c0a515e2c2b6)
    - fix docs (bca7594713d623e0f0a4b82b658c26ee9a041eaa)
    - [git-transport] Fix http build (3469e99afeaf354db4020d5a363b05a031894096)
    - [git-protocol] fix build (4cce6487d6d514541afee1a9aa92043f186136d3)
    - [git-protocol] builds without features work (a1945ff22f3412be1fbfac76236d487896ec4685)
    - [git-protocol] async Delegate (1aa678172f0eb75af76017addd3dff4d7e62ff41)
    - [git-protocol] async fetch tests work (fe434a58d321b3ac12644827e65eb4db11cfe5fb)
    - thanks clippy (0759ade3e8e97927f452eabd11e249bb93aa54e2)
    - [git-protocol] fetch tests nearly compile in async (97fb186df5661fb297c2c9485186dbfe0ed1d504)
    - [git-transport] refactor (d09153f34135609224929f77175f3e0ac04ea12e)
    - [git-protocol] fetch in sync and async… (47760399bffd030c848e0ef6df52a4765d8fb566)
    - [git-transport] Properly implement Transport for Boxed types (47b10c9b4ccba5a0928819c92eda472bbfda0c50)
    - [git-protocol] refactor (80379fd32aae02f2975d8637326188655f85b474)
    - [git-transport] refactor (3b0baee6b856b510ea839a1e294e9a99aafaa3ac)
    - [git-protocol] build should fail if mutually exclusiive features are set (72cf9401dda6e1bb465cce8d65ce66a7cc6a03fd)
    - [git-protocol] refactor (94d7be4a16f2c2e68a9dacf120eef7a417a8a6b9)
    - dependency update (6d2278b009a39564a13b2f9ed21b71039b7f1271)
    - [git-protocol] refactor (990099b01bfd54b926f0f4e7ecf727c423a23b8e)
    - Bump crossbeam-utils from 0.8.4 to 0.8.5 (fce4d107c7abc778bbdfcd37349c3075e54fd756)
    - [git-protocol] refactor (d623cf7db4488815ad5a2afd2d1bcbbbda275d2c)
    - Bump maybe-async from 0.2.4 to 0.2.6 (d99a1a815809d22c7384c6ecb1275e39fb911d91)
    - [git-protocol] async response (c49855738bc164f65130cb307ba612b71c3fa83e)
    - Bump cargo_toml from 0.9.1 to 0.9.2 (28687b169a30766a3aab3e485c44bc2d3b419b90)
    - refactor (14c909341d243ca3dcc42d343aeee65d28045b65)
    - [git-transport] upgrade to futures-lite 1.12 with BufRead support (ee01c79887a892e001787bbefa93f75d9c4f1cfc)
    - [git-transport] ExtendedBufRead for Async… (d4e56c8efd586b571445e0085ce518c5efb8f5e6)
    - (cargo-release) version 0.16.0 (769c649c00c009bf5a3f7c0611a7b999618f2938)
    - [git-packetline] refactor (7e513f1fa3ba143bb1ae5f9052c195043a53943c)
    - [git-packetline] Switch back to pin-project-lite (63cb0fcb6248e5b9489156d602235d0300858cbc)
    - [git-packetline] all tests green (fed6c69fd8b2877a66fe9d87916f3d54a3fc342b)
    - [git-packetline] Nearly there - one failing test and its known why it does that (51c63c081df4bd26adef7b8336034aee74237a86)
    - [git-packetline] another green test (e67d77d545530ddce18846b0a5e3d732f071a11b)
    - [git-packetline] Custom implementation of read_line future to avoid extra work… (91c28954babfd863340a165721d3dab186b668a1)
    - [git-packetline] read_line test green, but… (8007c653d9e2065db913f683a1aa39bd2e016ee5)
    - [git-packetline] fix compile errors if no features are specified (a2b44c81a993b08d7786ca8139796f586229c90b)
    - [git-packetline] YES, finally, the first green test (f16b0124e778b5b8d2272228cf1644f9706df85c)
    - Revert "Revert "[git-packetline] It compiles with parent as option, even with state machine"" (e300f9fbbf1dda914b3d53bfac584eaa59ffe03f)
    - Revert "[git-packetline] An Option really does the trick" (8eb78f51f753680d1ad7123ed07c9d4fc2562632)
    - [git-packetline] An Option really does the trick (c05bd795156d7c3ca72ab39a01b57684c87d32c0)
    - Revert "[git-packetline] It compiles with parent as option, even with state machine" (890cc5018b8816ce369e09e3fbe8041f7421d602)
    - [git-packetline] It compiles with parent as option, even with state machine (a97bbfd6a4fafaf672186af72a53ed75fd817948)
    - [git-packetline] Even without pin projection lifetimes don't add up (7e834f584da1be7d00a0671df33d52171f79595f)
    - [git-packetline] [FAIL] For some reason the is a lifetime mismatch again… (b4ff4e7fae38dda4d281f41fb20abbd57c02993f)
    - [git-packetline] first step towards state based impl (22740c5bd2cc0805cc795038b997ca189e1df6ec)
    - [git-packetline] Use what's learned previously to make it compile without added buffer (88511f7f68f19db2e60ea4801e26243f39ad654e)
    - Revert "[git-packetline] get it to compile by resorting to another buffer" (38665173722ec57d72a3eb43f619e586ece81138)
    - [git-packetline] get it to compile by resorting to another buffer (01e15c8b6e4e582d75069f6e38f22ce37e5fb29c)
    - [git-packetline] [HACKY-SUCCESS] It's possible to do it, but how to do it without unsafe? (96d0ecf535753068c440b8c9909f7e72bba6b5b9)
    - [git-packetline] [FAIL] No, cannot poll a dynamically created future (194c991d64fdf8fb6cffe12d5a8b6a2ba761e36e)
    - [git-packetline] [FAIL] try to brute-force keeping futures for polling… (42a7d00252434e6f0b200fbb4a0155415e2e8537)
    - [git-packetline] [FAIL] try to impl fill_buf - can't return parent buffer (1e8b006d3f8bed554ff247613b05a851849b574e)
    - [git-packetline] Upgrade to pin_project as drop impl is needed (3d5340424020a95b39e8c7ee747bdfdae934bdd0)
    - [git-packetline] A step towards implementing poll_fill_buf (3c487de86b9b7a7647372d7caf940617c571b9a1)
    - [git-packetline] Frame for async sideband (adc365e019b2fead79e1a4ad5657a9d6b49305fd)
    - [git-packetline] Use underlying StreamPeekIter buffer instead of copying into own (88b8bc33eda0c41af24575998a65232e5ce57e23)
    - [git-packetline] [FAIL] try to get rid of second buffer in sideband reader (4d8f4b5ba5ffb7044b0525d4f63778688f72d12e)
    - [git-packetline] streaming peek iter with async support (60164fdaad02b538f1238232852bb231ec894269)
    - [git-packetline] fix docs (4a47c9ea79bc908bbba81d1ffa021c53a9246101)
    - [git-packetline] refactor (e8b2dd118859222d87eacaa194a118225d450c00)
    - [git-packetline] Async IO for packetline serialization. (3bb9cf15a4703a88fab98223923f1acf50e57a46)
    - [git-packetline] refactor (2a84b787df693e8ce95bcde01663f6cdef8494cd)
    - [git-packetline] encode module now available as async edition (119fcc328aa1778f64d6b7342d1e439a8ac081a4)
    - [git-packetline] Use io::(Result|Error) everywhere (374f129e0d1473db9a2107c408f655da032efe89)
    - [git-packetline] Deduplicate 'encode' module tests (34f48c310643d5246799ad7d2ac968c36289893e)
    - [git-packetline] refactor (f038ca1e1c6d99bfcedb0387abc4151b188750c6)
    - [git-packetline] remove now unnecessary duplicate tests (c8178d7fe03e3dc6b24edc68f29a32dbf43b6d3c)
    - [git-packetline] Use maybe_async to deduplicate tests - neat (439a7b76c3d306a979890aedd0d857527830c1dc)
    - [git-packetline] refactor (d698d7bc4cfd49c6f752dab17f669bce27aa437a)
    - [git-packetline] All tests for high-level writer pass (eef8c9f0b320cea89e900cfd7b5eed54d3bc7a8f)
    - [git-packetline] OMG it's green! (fbffd898eedc3a16369aeb65a496f6460fd5238e)
    - [git-packetline] An owning inplementation of the LineWriter (70ce3c96f189e51a0d4d8b5f1f572372f64bcb0a)
    - [git-packetline] An owning LineWriter (445fac6b079a8728a5b17f1a5cb70178fafe2c8a)
    - Revert "[git-packetline] Use no pin projections" - let's own the writer (6c5750a810fd8a13c67e947b72ec4dcdb717552b)
    - [git-packetline] Use no pin projections (dc4e0e5946dd24e92b52c592863e28736fcb636e)
    - [git-packetline] Allow different lifetimes for writer and buffers (3b3c53dc85d70cce7a58aa5eb21e3b97249f6e45)
    - [git-packetline] A complete LineWriter implementation by hand, OMG (32995484a83756fd522f4b7ba45150254809ebfe)
    - [git-packetline] write prefix properly (432b2145e3618a0989ed0a99eb80b1827afe79c8)
    - [git-packetline] write hex_len properly (acdcfb7b8b26adb4c77e5e1e6d550ab2cfe9b7dd)
    - [git-packetline] it compiles, but write_all needs to be implemented by hand (2c44350d6906d5a01e985e6b5d1e690fd1ee35af)
    - [git-packetline] First draft of LineWriter - and it shows some teeth (13127ee2dc93a993b952fb4e94d0736836496067)
    - [git-packetline] Make failing test pass officially for now (cbd6291a75565a8a15f38f7ffd6bc4918aa46a3a)
    - [git-packetline] it turns out that a simple write trait isn't simple (793369807fed9f4ddab5db012d84b2b83c2d9613)
    - [git-packetline] Calling auto-generated futures isn't easy :D (836123890d2604e9398589a98cd11feeb9810c7a)
    - [git-packetline] All encode capabilities that Write needs (88a971d01f80bedeb180198585d0d6ba2f63bfc0)
    - [git-packetline] the first green encode test (ebc4703a26fc2d8a6d88a336489c1b8400d6c387)
    - [git-packetline] Now maybe_async would be useful (ab4b30e4cebe52b5b3a6c9c19ce1f1d51f570cc4)
    - [git-packetline] refactor (7d792887d743cc649ae20010a3686a14f65cd3ad)
    - [git-packetline] fix tests (b26c43bf5bd50e7dd0aaa9587e2e45c035ddcad8)
    - [git-packetline] prepare 'packetline' and 'encode' for async (1a986fb45e5286ddebf974e3498509876ff0ee08)
    - [git-packetline] One tiny step closer, and it's obvious there is more IO :D (0bef59cc930187f2ac9b760d127fcb38c4fcc341)
    - [git-packetline] the first green test (916c862f218bb0ae936e701500df7158fbdc6815)
    - [git-packetline] the first very failing test… (0220bca6515f0cc46e649a696400ff458407a681)
    - [git-packetline] add async-io feature toggle (727ad9700803d105f1a72c7cd7c7e8fe1a383c52)
    - refactor (c8ba842ca30a41eedc900526e9081a9e79b7a344)
    - [git-packetline] 'blocking-io' feature toggle and tests'blocking-io' feature toggle and tests (380e8b21bb34da5974ac661de0537a762bfceeb2)
    - [git-packetline] fix doc links (cf50f28f9237ef246d523e6ed7e574948da1df7b)
    - [git-packetline] refactor (1328c5b4001f380936beff73e1f822f14e41e98b)
    - thanks clippy (334e129e956a62400fc240effc7f527f10abc3d5)
    - [git-packetline] Fix performance regression (513e7ad2c1a38c27fd9715f37e33e6cdec79f1fa)
    - [git-packetline] Deduplicate read-line logic as well, with perf regression (1c13706c812f5a14559fcf0b983cdf4420bb1ef5)
    - [git-packetline] refactor (17ab380e552c5da56b06a8addd0d43c1b7f310fa)
    - [git-packetline] Step one towards less code duplication (d863de0085ae73248f96fb8fcc4fce0a7941a7b4)
    - [git-packetline] more docs (4591e4601c4fee3cb7cc37dafd02bef83441e69a)
    - (cargo-release) version 0.6.0 (ec5a54e9f3543afddc9f972f16135edc6ef6ff5b)
    - [git-packetline] refactor (e5769d1e7668ae54c667d2593c0c22e7723710c0)
    - [git-packetline] refactor (fef3c9f0aed3f6a509a71e8ff20050c6ea660f56)
</details>

### v0.5.0 (2021-05-09)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 133 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [track publish] git-odb/0.15.0 (7617998c0c36830836697680f1e95cc0c6a4c176)
    - (cargo-release) version 0.5.0 (8c4cc3fb5922d1a761463bbbad65e59f91cce4cb)
    - (cargo-release) version 0.15.0 (d91b2412381e3c8c1f24c38469e821c3c3960e34)
    - (cargo-release) version 0.14.0 (d9514eec64579ef77c9f2ac5dfe87cd302180eb9)
    - (cargo-release) version 0.13.0 (5c791af217fac6a171d174ad9f4ee5f4d5282892)
    - refactor (77764f3b9c3e8202119bb9327e150089c3ecbb9b)
    - refactor (edf7d382148aa139485c8279c2a50dc6c86d481d)
    - refactor (ca98221d5a512dabf683cc1da56d40a17285f2fb)
    - bump git-odb minor version (5c833ce64babd00b7ced3e3a1c9ed3dbd260c9f4)
    - (cargo-release) version 0.11.0 (fd698e334e44d5c478c162f98d09afd9ce7a6895)
    - (cargo-release) version 0.10.0 (316177729e42f8d000a40ab01b9b97621e7179e8)
    - (cargo-release) version 0.9.0 (efc898381d830e44487c62e35a665d3ccd0a2d39)
    - (cargo-release) version 0.8.0 (1ccfdcdb96b59c6415e7fbc800371d594b2ef7a1)
    - thanks clippy (343ab9adb62da1dde495fc209c179137bbe59a10)
    - deny missing docs for git-packetline (3a78840481c60dd122dedda090f1a235c9a21088)
</details>

### v0.4.1 (2020-12-26)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 9 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.1 (7c623dec0f62f123cdf46ae8c36d7b18cb55b00b)
    - Finish git-packetline docs (7ae3e7391042dddb6ac33c541a020f23eee294a1)
    - last remaining docs prior to refactoring (da966fcdbca656c87e34a16dcbd6e69d9488e93b)
    - docs for encode (213924de746871bf3152c5b8612c6b3515da1dbb)
    - docs for ReadWithSidebands (e277cce4d72c4d44122019a26e45c67c682d25b5)
    - Finish `Provider` docs (832f7f3d09d7cd2e7a7e7ac2526690d2d05acdc4)
    - more docs for git-packetline (3c7e727c4d7881deb1afa0f5596935993e477ec1)
    - Some more docs for git-packetline (77edb623610cc4c03b75e6f5da3af63b2604829d)
    - All crates use git-hash::Kind and its types, sometimes through git-object (124c171aaf546d8977e9913ff84e65383a80ee98)
</details>

### v0.4.0 (2020-12-15)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 (72eaeceed135e4cc5c943685f4c902d03597c4d2)
    - (cargo-release) version 0.6.0 (27f5955e047f35e21a86789eb46bfd89e1c99b44)
</details>

### v0.3.0 (2020-12-15)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 46 commits contributed to the release over the course of 84 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 (eade7d101e071153055b07d9c6ae3c1452493a21)
    - (cargo-release) version 0.5.0 (c767e07ccfc58a28e3e8ec22b590afdf0d92b9f2)
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
    - (cargo-release) version 0.4.0 (2272fa4bcacdaf1898e4cd8b791232fc1321227f)
    - (cargo-release) version 0.4.3 (5b47a1a051243ec2aa407297a19d41b30447bfab)
    - (cargo-release) version 0.4.0 (0d7b60e856325009431172e1df742a1cd2165575)
    - Enforce using the correct version of clap (fd6457f3a006873506543846d9400b4a66833e48)
    - update dependency chain in release script (9af47995922fb7bb61729138cbc8c75e7111bdc3)
    - refactor (e4bcfe6406b14feffa63598c7cdcc8ecc73222bd)
    - remove quickerror dependency from git-odb (7e2749521b6c873766a2f6f96e6c91a0c6a9dbf3)
    - (cargo-release) version 0.2.0 (779e9d0ad67c20fa9cec14359e87774ca2d74ee4)
    - refactor (6a84f137754cddfdcb9b1fec3e353762ebb3ce2b)
    - refactor (7874c35bccb74ae7670335e633efa7eaebc72630)
</details>

### v0.2.1 (2020-09-14)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 69 commits contributed to the release over the course of 26 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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
    - thanks clippy (6aeb68c587916610352644e0e7c4fe812957debd)
    - [clone] support for stopped_at() in provider and reader (6bd8c8723617e70c3e9daaddf284884aacefc483)
    - [ref-ls] A way to abort on multiple delimiters; first tests work (8d44912e7215b85c6931b7b829bd73ac38584424)
    - refactor (feec5be335a99a4c47ba98f93803863044575838)
    - [ref-ls] Allow multiple delimiters at the same time (cfae63a5f7d2d99560dd857f7220980d70c4c4d8)
    - [ref-ls] It would be practical to simply have access to the line provider… (5fba78796d3bcc16f812dc3202d521ee057e86f9)
    - [ref-ls] support for line peeking in packet line readers (0c0c57522972f2a49ed5261474114da062e6ab15)
    - [ref-ls] don't do anything on drop (9f18d9b9062d61d6da6e2bb7564fe5edbb1528c4)
    - fix packet-line tests (0939e6c7cf19395a8cfe09c76630dcb3614fa9d9)
    - [clone] Don't expose hex-error in public interfaces anymore (92dab3033890fe26fe2b901d87abe16abd065cce)
    - refactor (c138059434885536984996cd8fec002aba3d5fe1)
    - refactor (f2ff90d65edd91c4f6dc6baaf1242df31ef0ef2e)
    - [clone] a way to change progress handling on the fly (c1bcc0adf04a32e9332fae047fba066d4cff6538)
    - refactor (aceaaed45be5d523c9b4c1f98444b84619cbc13f)
    - refactor (2cdda7af8ae884b5efde8861f13d85b07d643b94)
    - [clone] Sketch 'request()' implementation for git protocol (fd0e0e9e49f5481c14e17a462f9e507663fd5e6a)
    - [clone] the problem actually was rooted in trying to read binary data (b7af002a445143e5437fe497a2d9fb1653adadae)
    - [clone] first impl of custom read-line (still fails) (7f2bdfa6276692557768ec7a9e969277d7f7db43)
    - [clone] Add test which probably indicates the need for a custom read_line(…) (2360a7003c07baf88ad3cd46d75bc31a06341301)
    - refactor (359765a89042f52d41281a31a4ad854215e99c33)
    - [clone] more tests for progress line handling (66c2958769797610ba415d39a050e0ffd0fb7c75)
    - [clone] decouple packet line from git-features and progress (13bf25edb64b8fd3ec77e24cce8911c020e91b11)
    - refactor (fb7dd267f12bb23ce5c2ba275e487b90f5117208)
    - thanks clippy (what would I do without you <3) (631af04c87f0b6b22c3ac1ef0d300a02bbdcd217)
    - refactor (94f0d8ab911625218728d9ba582eeed776f060ed)
    - [clone] Keep line reader around in http transport (feb259645651309b31df91b18ab247d6955f9a7f)
    - [clone] packet line readers now reset the parent automatically… (8250448e5c441cd14dfe77bfbbdb21b5f87ebf8c)
    - [clone] Make it optional to abort the packet line reader on 'ERR <e>' (abf9c3b3c9fe757a7418626cd985960f58718357)
    - [clone] Finally it all works exactly as desired… (c5bbb57ad7069c839757f72432d23c43de0b61da)
    - [clone] FAIL: can't pass line reader as box (633341dd5f3fbd7b910c545e203e0bd734b5f989)
    - [clone] sketching how to possibly return Line readers while keeping it sane… (4ba123b8e543a2ef3ba07aaf467b208047db0e1d)
    - [clone] Add Peek support for packet line reader (10f1ef7b9c59ec549a7c1e72cfce3dc42617b620)
    - [clone] a simpler peek version that will soon work (c35051bbafe3278d6cc17e9b29cd42092fcdf03f)
    - [clone] FAIL: try to have peek_line() borrowcheck (dea5672c374f95d13cf9b9629da09c51d4ff0375)
    - refactor (f3c5c059169e9cc998ec0c80baf637142eb200ef)
    - packet line writer deals with long lines and definitely isn't smart (549e6e69e58d93efb685efa4036c8999270b8182)
    - First rough implementation of packet line writer (721c215ec57ca55a22ddbbfa1e4e63a7f44c6cfd)
    - Don't try to find 'ERR ' in every packet line we parse… (922fcb6d718622bdd6e157edfb47d60cf2a5d4f5)
    - thanks clippy (25cdbecb791993ffe8a3fdf59ae826fa6c63039a)
    - no panics in packet line to let caller handle invariants; read… (a89a44388a353e7324bbed145ac4996bd677a15b)
    - [clone] as_read() support for packet lines (e214df5c3a63c26e046cf24cfe8ec5147946b042)
    - [clone] first stab at making packet liner reader more 'practical' (7178543804575040a3685a31dde5515f634d21a9)
    - [clone] prepare for making progress in packet line reader optional (ffe84c046129a12c384678c56e72f3fdfb04f550)
</details>

### v0.2.0 (2020-09-12)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 58 commits contributed to the release.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 (da830defc9cfa81ce159f6d908da828227760845)
    - refactor (4e89c3bc0f14cf9581348ae2c1620ade9dc1db8f)
    - refactor (3ec99dc7360c01b4f3c4593ff5049361e7043254)
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
</details>

### v0.1.0 (2020-08-18)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump git-features to 0.4 to allow publishes after breaking changes (9d6b8790e2edd7fa01b3239adff86a7cd2393f10)
    - [clone] move packet-line code into own crate (879af671fcde405d3d08ddbc07ea70d0bee23ef1)
</details>

### v0.10.0 (2021-08-27)

#### Breaking

* **renames / moves**
    - `immutable::PacketLine` -> `PacketLineRef`
    - `immutable::Error` -> `ErrorRef`
    - `immutable::Text` -> `TextRef`
    - `immutable::Band` -> `BandRef`
    - `immutable::DecodeBandError` -> `decode::band::Error`
    - `pub immutable::` -> `line::`
    - `pub write::` -> `write::`

* **removals**
   - `write::Writer` (is now only `Writer`)
   - `read::StreamingPeekableIter` (is now only `StreamingPeekableIter`)
#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 82 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-packetline v0.10.0 (08993382b55106cf34e6e142e84591b37e21b784)
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
</details>

