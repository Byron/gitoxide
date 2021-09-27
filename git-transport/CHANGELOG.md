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

 - 208 commits contributed to the release over the course of 12 calendar days.
 - 47 commits where understood as [conventional](https://www.conventionalcommits.org).
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
    - Make use of fixed git-conventional (b7b92b6c72051d462ab01c7645ea09d7d21cb918)
    - prepare test for basic merging… (0a14cedbd68058ac296e34a84ab1fe1083a0bf5e)
    - update git-conventional dependency (2d369e863b15269ba8714b025fe596f69e5b1217)
    - nicer 'thanks clippy' message (43442162aa22f561a33cab78936514d05d8214a0)
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
    - thanks clippy (c55f90977756c794939454072e4cc648f1e4348f)
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

### v0.11.1 (2021-08-29)

- instruct docs.rs which features to use for more useful documentation



#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 38 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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
    - Release git-ref v0.6.0 (b191a88512d4841385d2d4806abf243e193f25b6)
    - [ref #190] refactor (e34be7e24ee49a539b6ee8dc5737fdb23f416922)
    - Release git-protocol v0.10.0 (b60ddaeda7040b71d85c9ad85b28775be9cdeecc)
    - [ref #190] more Target conversions… (1fe1b42ac2b04f8145fc7312ea03cb47f791aec5)
</details>

### v0.10.1 (2021-08-17)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-transport v0.10.1 (dc74d1946b89fb42fa8644db31b1fe1a52a56f05)
    - Apply nightly rustfmt rules. (5e0edbadb39673d4de640f112fa306349fb11814)
</details>

### v0.10.0 (2021-08-13)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-transport v0.10.0 (b94427835bf922aa9388cdd78200c79a3c31da43)
    - Release git-packetline v0.9.0 (7ffbd602c08605026b0bb97ab85216907badaf09)
    - remove dev-dependency cycles by removing their version (c40faca41632cd2a226daf4ddf5293b65d1fdc82)
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 (f123f69c7a4f9fd1c98bd2f60ebc953a6739fe04)
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 (c67291ff9bcdff9a747d87241f6a71015607af05)
    - bump transport version to 0.10 (f26a3d3a2745f3eb69d76e0cfd718a90cf74f003)
    - (cargo-release) version 0.8.0 (ad6d7f9c2b4f8879d466e758fc9b51ece6879e96)
    - (cargo-release) version 0.6.0 (d704bca7de0a6591f35345c842d6418b36ecd206)
    - (cargo-release) version 0.7.0 (2ef3106eb84981e2dabd84f81362b4e44f938ea6)
    - (cargo-release) version 0.5.0 (c2f94a51bce287be301090450cb00cde57e92f76)
    - (cargo-release) version 0.4.0 (d69d0ac21989243fdafa514fa41579fd51bc2558)
    - [transport] A much better name for 'is_stateful()` (f15f1e85fda76eef72c3754d625cf51e3c454eea)
</details>

### v0.9.0 (2021-08-10)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 4 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 147 commits contributed to the release over the course of 90 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Revert "[ref] break dev-dependency cycle" (436e89b18cb157b3d30bd24b8d1acef25631ec2a)
    - (cargo-release) version 0.3.0 (0e9c73abd17e0dd21952275077ae53ad7e7aa1af)
    - (cargo-release) version 0.16.0 (1231dbd16dacefb39adec8e067c312d313a82e3c)
    - clippy on tests and thanks clippy (a77a71cf02d328a2a964388928d6b2a235a0aa85)
    - thanks clippy (e1964e43979b3e32a5d4bfbe377a842d2c0b10ea)
    - Bump async-trait from 0.1.50 to 0.1.51 (ce0b81e8f5c652d389ff876844bc42bcfa687921)
    - [transport] more convenient check for available capabilities (e9ed952d35fa9ffa142f941d75c385abec3997ef)
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
    - Bump thiserror from 1.0.25 to 1.0.26 (9682590095dc3a502b0c84ccd206ca4797635092)
    - [transport] remove Transport::close()… (4268a9bcf733413f7326be7af487a8fcdec1f71c)
    - [transport] implement Transport for &mut T: Transport as well (372fb8183aff19bd0f2d17ea74409b2ca3a08511)
    - [transport] tests for extra parameters (fffd926a3d5c6abfa732aa2305a4a05fdd06254d)
    - [protocol] extra_parameters are forwarded from delegate to handshake (03e3db3809bd031d7d0c151ada2542214d7e32c0)
    - [transport] allow setting a custom URL in git::Connection (f7437e041b2f3a8d51012972bd443d3c4b0a9252)
    - [transport] async transports support extra params (a0d67569b5c947d8158177e308f0919df2f182a3)
    - [transport] extra_headers for http (6026dcc07674ee9ea79503aab07491dd395e51a4)
    - [transport] extra-parameters for the http protocol (d30bcf18afda19d89b8cb020e193405bfd2d3787)
    - [transport] git::Connection handles extra-parameters (961b6a40aaa003497abbd17fd53485c6cb2b2857)
    - [transport]  File implementation doesn't need to inherit git::Connection's… (951b1e26a1dca054aa5af6a565fde3c733b43ffd)
    - [transport] unsupported protocol versions now abort the fetch operation (812aa3bc02a823cb9277847db905e76a50ee7413)
    - [transport] flexible version of version support check doesn't actually work :D (2b220f0758cb7a96a66b256552f13a020cdee3fc)
    - [transport] improve docs for `is_stateful()` (22f7e6719d2e3931f8cd4bb4e94c23c1f9f84189)
    - Merge branch 'pubcap' (292f8ff2851dff846eda1b80943de718f08e65be)
    - (cargo-release) version 0.1.1 (e9cdc958e7ce2290e2d7958cdb5aa9323ef35d37)
    - Add missing docs (a6cbbdeecbfe459556579a2d991bd546452c04c3)
    - [actor] fix dependencies (3ff918efa0b94dd20f781a3d038a0449cd9c7a59)
    - [git-transport]: make capabilities parsing public (2f3725efcaa439db4e10ade1b9fbeb1258fd93c1)
    - thanks clippy (6200ed9ac5609c74de4254ab663c19cfe3591402)
    - [async-client] unblock the async delegate in the cheapest possible way… (a3b5d75d387dc5d6c44f695f63df8803613637a2)
    - Revert "[async-client] Try to bring 'Send' back but…" (52eb953fcc44cce19604b1df6a600237b8c81392)
    - [async-client] Try to bring 'Send' back but… (3a06adb41f6b2946f78044e4ab1385e6441fc40f)
    - refactor (2a406d62c02db24c39980f9ae636b87e2d707faf)
    - [async-client] frame for async connect (9ada0805fc5896f8ef1a31dc821b789b7f0438a6)
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
    - refactor (2eefe1712131a69298be02e94df8b6ba844afcd9)
    - refactor (14c909341d243ca3dcc42d343aeee65d28045b65)
    - [git-protocol] async capabilities and arguments abstractions (aa3eacbd53665d6b76bd9706d801d1189a970261)
    - [git-transport] see how things can be moved to a different thread… (c271d323086486a9d1dbe004b33fdb7d9eec45ed)
    - [git-transport] partial transfer to thread doesn't work in test… (4a6dfd40e465226d0f9c7eb02cfb721b55bbff41)
    - [git-transport] allow fetch processing to be offloading to another thread (a1302e0ff549e96362c441d8eecec56f1ef4ca43)
    - Revert "[git-transport] async-executor (Local) hangs…" (ec8bcd0f36b46ff319ad6d6da9ffcb80dd5b0429)
    - [git-transport] async-executor (Local) hangs… (68ac51b384cae11f5dca103160bf3d519305364e)
    - Revert "[git-transport] attempt to mix 'blocking' but realize that now things need to be static." (e3677537d7858a113008849ac8ace136f5d5c4d2)
    - [git-transport] attempt to mix 'blocking' but realize that now things need to be static. (3d296fae08ce0d4c2625008ab1fdcd7ede8dac54)
    - [git-transport] V2 transport tests work on async (e04a1c98a1ee0ba58fa326c9a68bd36230e229da)
    - [git-transport] first V2 test (f9da975ca777d2345e8c2842771b16a17af79cd3)
    - [git-transport] adapt extension trait in blocking code to match async version (95eee30191aed9421f69dcd6e1587c0b5a1f2dd2)
    - [git-transport] extension trait working (28fbd284d108d5db3f13c8cede5772e065e5f8fb)
    - [git-transport] a first step towards getting the extension trait to compile (b6929792c3d2ef9a73eb049eb88b06c8c763d899)
    - [git-transport] no warnings when building without any choice of client (3dc568a3a29dfea6ff311cf965ecce7f7eddbf63)
    - [git-transport] upgrade to futures-lite 1.12 with BufRead support (ee01c79887a892e001787bbefa93f75d9c4f1cfc)
    - [git-transport] Show how to use blocking git-pack code in non-blocking transports (de2ba3c4919d454894911c54fd4bb0e0a4665723)
    - [git-transport] handshakeV1 tests run in async! (d1c0e35817d183982a5b1eb7e545bfe83edb141e)
    - [git-transport] And a chance to have V1 working in async (2bf93fc72b3f9dcb63f8b24c77c95d518072431f)
    - [git-transport] refactor (64bb8b3937fcf7f14034ccfb6a72a24bf05f0320)
    - [git-transport] improve error handling considerably… (7b7d314851b8db230228c28fb38a5a6541ec865c)
    - [git-transport] Add remaninig git connection method… (73fcf38f9c334813572a6aeb7691758a524cac07)
    - [git-transport] refactor (db83600179b3f27770b989f5f8ae1dd459749354)
    - [git-transport] the first part of async transport for git connections (d94fbf83d3e57c54dead3fb849e63b1d37343cb2)
    - [git-transport] Split git connection into shared and blocking parts (0bfe69385932698b99871717d823fe645e4eabb8)
    - [git-transport] refactor (2342e8a6a4ceca12603cb5e2c350edc2d4e71580)
    - [git-transport] refactor (957403e11e0f7b3c97aa1996b2e936bbdb7ee12c)
    - [git-transport] refactor (e58035452be10328c3e5e1991bafbab3f71d3353)
    - [git-transport] re-enable `request()` method of main trait… (3adbade31afa163a499fc2946f5af5ef3f367387)
    - [git-transport] RequestWriter complete (a05fff3b2d987a5750e11945306bfa3731ed5ca3)
    - [git-transport] refactor (03a3aedf17a91465279800d8028cc7435326534a)
    - [git-transport] ARGH: PIN!!! (71379ac25c44bc744ed0e93d2b126d4959bc4469)
    - [git-transport] naive attempt to make Request async… (b819546b0096a4abfbe5ada25a1ac661a084cfc9)
    - [git-transport] ExtendedBufRead for Async… (d4e56c8efd586b571445e0085ce518c5efb8f5e6)
    - [git-transport] First stab at ExtendedBufRead, but… (13f73d2f9b65d5ea829185af669532c3797cf90b)
    - [git-transport] put request writer into general spot… (af07ebf44fbb3386cd5176441fd707cc820b71d0)
    - [git-transport] refactor (5f98ac140f9f3260d3d5a784d1aa1e1ac8c37114)
    - [git-transport] fix docs (fbfc8271431f7c19adbed5e095d7c2ee10dda5e5)
    - [git-transport] refactor (011ece04827d75aa6d93e9fcae449aaba4167f80)
    - [git-transport] the first async trait (2abac2a2df8033c6d2578e5afb88bb34aab86988)
    - [git-transport] refactor (73df12987a9255efc4724e1761f335a072d3bcaf)
    - [git-transport] the first async-only type (88109a54ad594df6d18cf6b66a9c89a76fc0cdf5)
    - [git-transport] all non-IO types are now shared (209c780efff32d63ce7edc8b1f92fac0cd1a396d)
    - [git-transport] feature toggle for async-client; prepare for test (95e6801b121a0744908552090b855cb3dbe99e64)
    - [git-transport] refactor (592d9ac03b0d26848435a43480c526a4a0e0efb8)
    - [git-transport] remove maybe_async from dependencies, add async-client feature (e57aad3a19ec89fd0aa4d8670430434f0dc4c826)
    - (cargo-release) version 0.15.0 (d69d9fb0931f8257cef96ef14a89da9340ad9738)
    - [git-packetline] Use io::(Result|Error) everywhere (374f129e0d1473db9a2107c408f655da032efe89)
    - [git-packetline] refactor (f038ca1e1c6d99bfcedb0387abc4151b188750c6)
    - [git-packetline] document feature toggle (8b8a1aafb04b2e305cf2674c15530b430dad4969)
    - [git-packetline] refactor (1328c5b4001f380936beff73e1f822f14e41e98b)
    - (cargo-release) version 0.6.0 (ec5a54e9f3543afddc9f972f16135edc6ef6ff5b)
    - [git-packetline] refactor (e5769d1e7668ae54c667d2593c0c22e7723710c0)
    - [git-packetline] refactor (fef3c9f0aed3f6a509a71e8ff20050c6ea660f56)
    - (cargo-release) version 0.9.0 (18f6d011043203523f1d0dacf657704ed3f9cf89)
    - [git-transport] simplify parsing capabilities from lines (401af0974742f10c8b9b3c9752e9d30205e96c16)
    - refactor (8ce28e74545ded2909417df9091da866fb343710)
    - [git-transport] test capabilities in blocking and async mode (66eb2a5c803d3365c8d9522f24843a8b73dff76d)
    - refactor (558b208f5055ab0562d3704e4fb62693eaab94fe)
    - [git-transport] first round of getting capabilities into 'dual' mode… (3af353b8b3ed0ee608cbc96d1cd45a3165907a12)
    - [git-transport] remove default features to force being explicit everywhere (d1b39f8093c032a172237a584c9208479611a866)
    - [git-transport] A first async test, right now there is nothing to test though (9741ae1ee0fcf65c144d87cd17d8fe547b288b12)
    - Tests follow crate structure closely (again) (8d6e46a84c41d7f04f2dbbb1a4602159b1a96c8b)
    - Make the blocking client the default… (9d62ca338927139708246ce0934f1cb317f14784)
    - Revert "Remove maybe-async for now" (ebd57017f80d0c12f6fe9a8d236843323c638311)
    - refactor (84d150952d9cd72a05d83419d4fc013c75d7b2dc)
    - refactor (141228219d33e8056489514f91221d803888edd8)
    - refactor (f16d057054ad6fabc664bbcb00f75e5974f05db9)
    - refactor (976da51efc5720300dfd3093e377284d1c4ccf3c)
    - refactor (7ac6a05442b0d7e97c886f8f57b8695a14761027)
    - refactor (cd027498944abde8339c421b5527abb9c3495de3)
    - Remove maybe-async for now (97e96f49ce9e4ac325faebe112cec9c11cdc715c)
    - refactor (6e6f4acf4b3c704989928347f10f1725e6a866bd)
    - refactor git-transport test in preparation for async testing (42d5bf7f5e8f4d88d3f849febec0c6d2678e0d06)
</details>

### v0.8.0 (2021-05-09)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 21 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [track publish] git-packetline/0.5.0 (422932a920bd9d1607e13a4bd63140da9d3d241e)
    - (cargo-release) version 0.8.0 (411a05ead1546c76fe51f359fbcb961a1140535e)
    - (cargo-release) version 0.5.0 (8c4cc3fb5922d1a761463bbbad65e59f91cce4cb)
    - [async-transport] Cargo.toml and traits to be more 'realistic' (9a617a5e68f032dbaba9a902b558666992683701)
    - [async-transport] The very first step (b9e5559afa776dc1a7eecc90cc219da5ff911d79)
    - (cargo-release) version 0.14.0 (a760f8c013e13ba82daa1acf1a4a57e0818a008d)
    - (cargo-release) version 0.13.0 (ac2eddb06eb3d8a9a3dcdcd796eb54a7e45ab935)
</details>

### v0.7.0 (2021-04-08)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.7.0 (334b7e1b838b5201f2484be42dee3c4d2fd789d7)
    - (cargo-release) version 0.12.0 (3b71e7e8416e550b47e5aed2259c1181497ac9e8)
</details>

### v0.6.0 (2021-03-26)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 21 commits contributed to the release over the course of 70 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 (50fb6f25e9afa900ac1c3cfb88d7ca0d5a9a95f7)
    - (cargo-release) version 0.3.0 (d5c6643a41d295eaf7aabb84eab435e42a11dd42)
    - thanks clippy (749ceba246fb8a4cb8d48fa86184619fef500108)
    - Merge pull request #50 from Byron/edward-shen/odb-zlib-ng (acb90d755fb02c37f8a5a431778abcbe143fb5e5)
    - Clear out non-gitoxide tasks from tasks.md (fb52a24ab40d575649857c28c478c551c560756f)
    - Conform imports (fd737317379af80f8e0ba9a9a8fc505fb60fd177)
    - [git-config] Fix must_use lints (71aff75d02329caf78c61d3c1dd8ab3c33b8597d)
    - Fix error type argument order and spell fields out (819568e9c5be14cec1e9e1cdc915b4c286c2ed00)
    - Update tasks list with possible features for `dua`, `treediff` and google apis (f05037c52148509dfb5a59413eecd81db5814bf4)
    - [git-odb] Return error on invalid packs (88de64d433b44996d5f8be733b50e1949c71e42d)
    - dependency update (80d5cb6fe978a6b49a82eed7fada76e38e5d5352)
    - [git-odb] Fix Inflate::once (36f6bbd451a5474cb6dac0259904e4aed7fd11d9)
    - Update git-config information in README with planned features (1f34be9b49e74027cea32711a65282d431ecc13f)
    - [git-odb] Remove unnecessary tests (ebe41cadc4acb38326e59d193fd3b1e501146943)
    - [git-config] Update README.md (cb94dd7bb3b6288431384c304b1cc568850a3227)
    - [gix] Use flate2 by default (f1158a1a4bc8e13913461db4d4851e32d57816ff)
    - Slim down git-config with cargo-diet (1c555e04d395eadb6b22639afd41c0892d48fa0d)
    - [gix] Add optional zlib feature (f1f96658a6cd6165ba9c9d7acb809fcaf2c46f9c)
    - (cargo-release) version 0.11.0 (1aa1f5e84a07427d5d7f3231735fe9c1923f506f)
    - (cargo-release) version 0.2.0 (0c39373de5aba0acc4aaa330bf51b6abd4f50474)
    - support for radicle urls (2c5b955b07073c5ef0e7bbe3ab20f0047770440b)
</details>

### v0.5.1 (2021-01-05)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 7 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.1 (0cf1d06f5e289b541a842af03b59229fec833ca7)
    - silence so far unknown clippy lints (b5f2a4b079665daa8b9e0228acc59d1eddd603b2)
    - thanks clippy (343ab9adb62da1dde495fc209c179137bbe59a10)
    - complete git-transport docs (fa2dc9d65100f0f3f97358746a37dc722bae12c3)
    - documentation for capabilities in git-transport (5ec79faaa2568cee9333b4bb0c96e8f0ee5a2433)
    - more docs for git-transport (3a867e945edad05dd65b75111628b99fa955c03f)
    - more git-transport docs (6cd69b9a6b79ba8f9297cf2bb6e36dd8f63845a2)
</details>

### v0.5.0 (2020-12-16)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (28df5e9131aec3efb2b68db204662b92b232b33c)
    - use git-hash in git-features (5b307e076f6f5975592c8b177c122c91c1d809c6)
</details>

### v0.4.0 (2020-12-15)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 (32aefc051c7ad9d1a160f77db070df7fa4843dbc)
    - (cargo-release) version 0.4.0 (72eaeceed135e4cc5c943685f4c902d03597c4d2)
    - (cargo-release) version 0.9.0 (a89fdb98f64bb0ca070fa79a1f58f1232bb14090)
</details>

### v0.3.0 (2020-12-15)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 49 commits contributed to the release over the course of 84 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 (d19ee35cc6683c63e0eabd717e4758075faeaa71)
    - (cargo-release) version 0.3.0 (eade7d101e071153055b07d9c6ae3c1452493a21)
    - thanks clippy (ba9b3c2345887353e02fc081be80733f1c5e22d9)
    - uograde everything else (0cd79d00bce3f042b5cc849cf48739e29f95fcb0)
    - (cargo-release) version 0.8.0 (47c00c2228cf25c79e1fa3eb4229c7ab24de91e5)
    - refactor (b3a8bb5f7f0c6e80259922546928c2739c24f7b5)
    - refactor (f9e8d2932c02c22bf57acd39fb0a9e6d521070bd)
    - cargo clippy Rust 1.48 (475a68ce33b895de911939c51afa159df534f7b8)
    - (cargo-release) version 0.7.0 (7fa7baeb3e7d008a25e4d714eff908e2516c828b)
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
    - refactor (7c3c80acf487296014ae9f2f9b88865c6aa6d98e)
    - (cargo-release) version 0.4.3 (5b47a1a051243ec2aa407297a19d41b30447bfab)
    - (cargo-release) version 0.4.0 (0d7b60e856325009431172e1df742a1cd2165575)
    - refactor (8930610c3ad73d2c1294880c3081f0662525f339)
    - Enforce using the correct version of clap (fd6457f3a006873506543846d9400b4a66833e48)
</details>

### v0.2.1 (2020-09-14)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 14 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 217 commits contributed to the release over the course of 31 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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
    - [clone] reassure ourselves that ERR lines are handled, always (925370b3f1d701d3376bdc80a4876e407b54c400)
    - [clone] Response parsing up to (optional) pack (24064c77f2969380fb92ea66109df86e84060324)
    - [clone] properly handle V2 response parsing (0d7d768278234824e03c5e74dacaafca3ee65713)
    - [ref-ls] A way to abort on multiple delimiters; first tests work (8d44912e7215b85c6931b7b829bd73ac38584424)
    - refactor (feec5be335a99a4c47ba98f93803863044575838)
    - [ref-ls] Allow multiple delimiters at the same time (cfae63a5f7d2d99560dd857f7220980d70c4c4d8)
    - [ref-ls] It would be practical to simply have access to the line provider… (5fba78796d3bcc16f812dc3202d521ee057e86f9)
    - thanks clippy (27f30df9a8046fe4e872837e36dd497096660282)
    - [ref-ls] don't leak the PacketLine error type in Transport interface (58ddd2955a407efb6a46249810b916c6035eb5ff)
    - [ref-ls] support for line peeking in packet line readers (0c0c57522972f2a49ed5261474114da062e6ab15)
    - [ref-ls] don't enforce V1 for local interactions (7b333369de1221f9bfbbe03a3a13e9a09bc1c907)
    - [ref-ls] don't do anything on drop (9f18d9b9062d61d6da6e2bb7564fe5edbb1528c4)
    - [ref-ls] Transport layer knows whether it's stateful or not (22c3640b70bb6925d72794eeaeda48b0687f2047)
    - [ref-ls] Always send a flush before closing the connection (918f19f0c2dc202ed2014e30b7247e63a0f6a51e)
    - [ref-ls] git protocol now supports user expansion… (d88e9da15068aede3a587cbc857702fcfbdc6c6e)
    - refactor (e07fbd63db297cd9f70f8b86b1f1f56b15e270a8)
    - refactor (7b5ce695a05630c20ab461c63ff1f9b5fc662958)
    - [ref-ls] allow ssh to work with tildes in paths (301ae81d1062fb002b080e8cdbb0bec134dd4de6)
    - [ref-ls] first stab at leaving path resolution to upload pack (51dad09221c118884db7e52d1337eb2ab476e744)
    - [ref-ls] verify also ssh works (1ef39aed71a684157363e27d0ae092a616782d41)
    - [ref-ls] tune request to actually work in all cases, particularly for github (6bab2f343347dca45288a9f17f1b05fc62611080)
    - [ref-ls] Make credentials helper truly work (7f3c3a71db7eeba1d37481ba1b522d5ded654237)
    - [ref-ls] Finally fix http content encoding (and fixtures to go with it) (49b7ad97075474ced5232345c7afac9d657c72b4)
    - [ref-ls] This actually makes things work in real-life (24ebc59d669dd22c68efa76eb3bcd66b6b59a3dd)
    - [ref-ls] provide blanket impl at least to be less specific (0223e7f4bf3eb5b3d3f3f430d82ce3386a6a566e)
    - [ref-ls] Make things compile (b6506a46ef59d8e25b245fa8caac5b4de4fdaa3d)
    - refactor (b38290e4a8fcabd758f26a15407710ab2abcdc07)
    - refactor (202383add69e7667fb2043d55e17f8064bc658c9)
    - thanks clippy (b060f42d097e52b5891e3b774ebd8ea8b076d011)
    - [clone] support automatic downgrade to protocol version 1 (4cf36436f11eb95d420c1147a1ec8adb618ea5fb)
    - [clone] transport provides desired protocol version (c39b64598a4119f0cf3c8aaf30e32996632fb51c)
    - [clone] update README, improve delegate docs (dc7908f1546239ade71f4147a389a001769311f5)
    - [clone] features for V1 fetch (5b24a559dfb03c99ee360e9997650c443fd30077)
    - [clone] Support explicitly closing (v2) connections (41e4cb22de9e06bfc5aa93246f931f483335fa69)
    - refactor (dda62fc1a170793768ef1791db85a0c3cca0fad1)
    - [clone] Prevent accidental leakage by transforming back to the 'right' type (2d469c66ec47be2e1bc3e0b1f3d17dfea5050970)
    - refactor (88ecda11dc1d97a7460a449350945dcac2f13752)
    - [clone] support for git-credentials helper (a6546dab8d6d0dc4453052b77278cf5bb96aaade)
    - [clone] make URL available in transport layer (67784478b96f8afd142e52982e2161a1f05d2ec9)
    - [clone] http basic auth support for all kinds of calls (572fb54b1445d25d55713ca3d68e19bede2b3cff)
    - [clone] first sketch of basic authentication support for http (c5b2d046f58a8cd3b66097c48ea9399ac34246d7)
    - [clone] sketch for identity handling (b23f47029fba50c7bba23a6ebe135e129ee9392a)
    - refactor (22cb37d26f18653b81f52e23f58b5797c763f883)
    - [clone] Add (hardcoded) timeout when connecting via TCP (02c195ba312b174ada5733321c08a8294f360cdd)
    - thanks clippy (712527f2adfad18c2527ee7bf9bb8841897db4e0)
    - [clone] Finish implementing ssh access, which might even work (8b843f2f08c3b070db427a3a8f2c08f4d778914c)
    - [clone] Add support for SSH prefixes in otherwise local service invocations (a1db2172dc691765dec907a226e0790c36358c1f)
    - [clone] once again, for SSH we need to delay calling the actual service (2c70275e45b487a966d1772cf1e7a90e96cbbaad)
    - [clone] Support for the probably very unnkown VIRTUAL_HOST env var (36fe20cb3a82fe6fa78cc18f7d71d25b9022397c)
    - [clone] Allow connecting via the protocol (eb7be2bd5f75ac3a04fb1b6afdc9377478f7818e)
    - [clone] be sure to wait on the spawned child on drop to prevent resource depletion (768d7f2b704f569e117a63a690c3b3769a2b1442)
    - thanks clippy (2528c82f4708a0258985d3ebfde6cba10a72c9c6)
    - [clone] implement file based protocol using git-<service> processes (be254a9316c08e17702eeb4c65b4dde8e6bb2e6e)
    - [clone] add 'Process' mode for git connection (e38c7bf1a2a9409762e891b7bb50f509d9b0d03d)
    - refactor (2ecb9759f6a916c0a887800df625480c123aa5f6)
    - [clone] first steps towards launching git-upload-pack while… (41f05f13a1fac078b694e6f4a9c8f52eeaff4191)
    - [clone] Http fetch test for V2 (81618ae8f4e60fbfbb424de6e42bf796dabb47f8)
    - [clone] http test for ls-refs V2 (3ef1e47d7e3781bdc52daac0d266dcbaa3dfb07a)
    - [clone] finish test for git-based V2 command invocation (9384f327e6c9bf5d6fc6f17d5d0ed573213fc5d8)
    - [clone] support for V2 arguments (8d56e7961c7de15197d1127617194fde028cc2aa)
    - refactor (f46c89d087e387702d2f887946ff0da1fdb19117)
    - refactor (9ed859a5ebd8f4a1654107c9909f99739c73435d)
    - [clone] Using a normal writer, we can't produce delimiter packets (1877b5f09d65fac6963b75afdf22afa938c7aac8)
    - [clone] first sketch of extension trait to invoke V2 commands (90eed9d5a8a672fe6c899c07b98b51e1e783b656)
    - [clone] Finally, HTTP requests are properly handled, it all works out! (a6121d93eec50406cbd0b1b8a8f1fbbbabec0f53)
    - [clone] Helper now integrates with Http trait, neat (b462bc7abbb8468063d85d08d656b820fdac903e)
    - [clone] first sketch of 'HeaderAndBody' helper (226f0967b557f1186ce5a1ca6f6e80e6926d3210)
    - refactor (f2ff90d65edd91c4f6dc6baaf1242df31ef0ef2e)
    - [clone] a way to change progress handling on the fly (c1bcc0adf04a32e9332fae047fba066d4cff6538)
    - [clone] first steps towards more flexible sideband switching (3d959e68f1b4906ac143e26eb14d65bdf03ef62a)
    - [clone] Issue: shoehorning header handling into the body reader (4c304f1520fdc06c03aceb389049154cc53edea9)
    - thanks clippy (bdcaf3680a13fde84ff7f9cbe3f49b09c8e55d8f)
    - [clone] Now we get to the point where uploads start, but… (8bd618262c963b23f84ad5214b25efe474abb851)
    - [clone] first steps towards testing posting via http… (b6a7e752053254a3547c9afcb6254201fd9f69a8)
    - refactor (a810f9f354a5e532b819d542ed2a2fdf7a42eb17)
    - refactor (5c2bd5f244bacc776be6f5f45dfd79f03b3a3093)
    - [clone] make on-drop messages do the right thing (5a39d70c00dbe04598137f479dca54c1ec2dd98e)
    - [clone] first test for request - ideally we manage to add a lifetime to the closure box… (db1a5b83338b12a14c3cc53886f5362feef1370f)
    - thanks clippy (913e55de74d3cfecba78345945f1b965b47d407d)
    - refactor (de22323c075de5bcb2957418410876101a05c9da)
    - refactor (bad836163c101f74e1eea862698edcbf730a05d5)
    - refactor (466557c6c0e4c7b5afe94bee4a4ab07a714089b2)
    - [clone] on-demand line writer, it's cheap (8ddd0fa9f74c9ac44e305d7fb0e53fb816bea0b6)
    - [clone] it shows that the packetline writer is better to be owned (f2c6e9fa763d10f7c72ad4ff29f273f8a6c6872f)
    - refactor (aceaaed45be5d523c9b4c1f98444b84619cbc13f)
    - refactor (2cdda7af8ae884b5efde8861f13d85b07d643b94)
    - refactor (521516f8c5713070ca393bd3b54994cb162d7523)
    - refactor (3738897a0347694458717e0abbb052ed79c3546d)
    - refactor (2e68315a7a1e47d2659eb3c047ba6015475bd9bf)
    - [clone] first sketch of http request (8b4befb9ef589d02f9232d72229731b646614fcf)
    - refactor (23af7e1e7c19d6e0db43540016ef9d851d0a048a)
    - [clone] support writing multiple messages on drop for the 'clone' case (92664425cb70d3d646dab97a714efc7f7b99b96d)
    - thanks clippy (2ed10de5f6213ca5ed68f072b9984bff32a5d67c)
    - [clone] Sketch 'request()' implementation for git protocol (fd0e0e9e49f5481c14e17a462f9e507663fd5e6a)
    - [clone] Allow progress callback to know if it's an error line (0c41844dbad5182ad3ea8d15dcfd0af92263936c)
    - [clone] sketch for generic request/response pattern suitable for clone/fetch (e0fd5a60960e0768a68878f38c034be9c44c6039)
    - thanks clippy (what would I do without you <3) (631af04c87f0b6b22c3ac1ef0d300a02bbdcd217)
    - [clone] Capabilities now can have multiple values (per command) for V2 (44dcea6ed33549e97cfbdd006f9f8fb3ce2e8597)
    - [clone] First step towards http V2 handshake shows capabilities are… (f58a78595658e30cac216c0ddade891cda745eae)
    - [clone] remaining handshake V2 assertions (1a58955f70a945e94d9846424c35c341a8661549)
    - [clone] first sketch of git handshake, V2 (bf1f05bbb229f98cfa636c2b974b687042168d20)
    - [clone] git protocol sends in packet line format, which is now enforced (4ce591646c2c6958bc3cf67a7dc6f792d507c30b)
    - refactor (44b06a7d4b11494271e59ef3069f0fe326c9eadf)
    - thanks clippy (ee5abfcf85889a56495844e4403bc40ebaa01a29)
    - [clone] Configure http timeouts, just so that it is done (070855a5bda314c5843ea9091351e8f8540d7d05)
    - refactor (8b1dc4846c54be78b2df0f3d02c4efa53c7f79a6)
    - [clone] Allow differentiating HTTP permission errors (4c9c413c1cdd24dea59274931b335be3daf3653d)
    - [clone] abort early on HTTP status errors (e829c0ab9923ff38c36196cbe196d158cb3a1ea7)
    - refactor (791c05e08d9c6148fa0f89894c9b2abdadf3f503)
    - [clone] more http test validations (e697b8cbc236783938fca70c0a4b46ccdf10c084)
    - Revert "[clone] FAIL: try to communicate error codes after request" (350de7cd517f8b94e263312f5dda1515ae6297a9)
    - [clone] FAIL: try to communicate error codes after request (2501ddd9f377cc869817fb32be213d943a9666a0)
    - [clone] Check for 'smart' protcols (2960645ffb160745cac82b2e7267dcff10286420)
    - [clone] validate expected http service announcement (a224a2c8190b1d45a61eef4cdef620e7f3bea659)
    - [clone] Keep line reader around in http transport (feb259645651309b31df91b18ab247d6955f9a7f)
    - thanks clippy (I really tried) (e8880fb6e69c15bd44c21edb1ff6de4d8738e036)
    - [clone] unbelievable, but it now seems to work as expected (88dbbf5294446bfad1534d1c0635e9a5876ef769)
    - [clone] quick hack to finish http set service, but something is seriously wrong… (dd93504624f57e16b71119a8aadbe3aa8b971a01)
    - [clone] non-deterministic behaviour when parsing HTML, despite ignoring the encoding (bab3ec324ddbee50cfddfef76fd7715286fc9caf)
    - [clone] It definitely doesn't want to read the data to the end with 'chunked' (49f1acac1817c02b76014965f0d26cdfaa4f062e)
    - [clone] for good measure: configure posts (more) correctly (e491e588eb2d3785ac7cc5a226733836e6552309)
    - [clone] Disabling transfer decoding makes it better, but… (3a1b8bcfad37f8f5a0587d7f1a40a499f0c32131)
    - [clone] It looks like curl is stumbling over the 'chunked' header (279a3868d1d8ac56bfb7db57a8fb9853bf992f7b)
    - [clone] Fix deadlock - classic, and obvious (72a165ea23b4ba119eb97bbe30912aa25b7502fe)
    - [clone] possibly correct impl of Handler; still hangs though :D (aefd8d459030ce3b88c7629894a36ace4ba0fb31)
    - [clone] Fair enough - it locks up somewhere, let's see :D (33a1a2217e27121c8061ef9547c2589fed39e015)
    - [clone] Improve usability of posts… (e1b944ef7d79294f56193431abade939e307c19a)
    - [clone] Actually use that abstraction (d0bdbe42ed8a62f23643904f9d2284e31fdbabea)
    - [clone] generalization of get and post (e62adc937d2c0256c15358e5ee6f76fc2cc5318f)
    - [clone] Curl can now use remote to perform operations (get only for now) (a82f0280f4240bb29801b1beabc96e6f7fa451d7)
    - [clone] try sending curl error even harder… (b450bfc3d917d13dde9f32427dd905fb35d51063)
    - [clone] first sketch of remote-curl, a way to transform curl into Read/Write (22b4b39a3e009d25bcfb18c33be1ca4dfcd76f2d)
    - [clone] Send headers with BufReaders (6a95aaab582941c6d1697dde6982c0aa8896c73d)
    - refactor (d4276719ac5e54bb70ee5a88c534acbf94e4a817)
    - [clone] Fixed shortcomings of http error handling, with thiserror (424e1598a2b37f7b67250ad5b7ec62abdfe75f58)
    - [clone] Allow to use specific HttpErrors, at the expense of source (b16a8c5c153e7368f17c0a7110157f5a545a35ba)
    - [clone] Fix 'small' compile (without http) (29ca5e8df83beede03f966de2aeae5e30638f6bc)
    - [clone] First step towards 'remote http executor' (f1e48d7c178b15f0a0b9ba41fb4f94e4c6f0c74a)
    - [clone] things get more complicated…take a step back maybe? (f77863726a5b7001e47f8f6b48eb0709a1cd0856)
    - [clone] Right before actually performing the http call… (5bf9e6a21c1bdab3010e362604e324bcebbb45b0)
    - [clone] better user agent header (4396587a2d1a0ca50119947048252709cafa277d)
    - [clone] in small steps to getting the http 'interface' right (43f2a92c6f51a097f6f707953fe215c28c95fa04)
    - [clone] A utility to respond with mock replies on a socket (1bf7ef739dc4c94b4be8a0748333c4486b6555b7)
    - [clone] improvements to Http trait; prep for curl tests (9f69d6ab3964b2cd566b5a6c4b739804d7cf3f7f)
    - [clone] a piped iterator (5148c85efc70c0ec06be3ebce267ce727c8ee4e1)
    - thanks clippy (c4f570fcae7e21745a37a4265b05d21e6149157b)
    - [clone] frame for implementing 'pipe' support (c55568127ff943cc6749dba5054d7b3e93c049eb)
    - refactor (bfda633234fa5661a34a98f9f8071570f6cea10c)
    - [clone] sketch for http infrastructure to get going with curl (835129968b5414c980f24dc73aa89b43ac39bcaa)
    - [clone] an easy way to get a few HTTP replies for consumption by the client (8b082d068f1adc0224ab5e0c646a91742dffaa5f)
    - refactor (0bbd87e023da4e2cf0412e4c017816be11fc4174)
    - refactor (bbce340e89bbc8b53b6632897ba5bf6dbdeafe11)
    - thanks clippy (73a6868963993a3328e7d8fe94e5a6ac5078a944)
    - [clone] Make it optional to abort the packet line reader on 'ERR <e>' (abf9c3b3c9fe757a7418626cd985960f58718357)
    - [clone] Finally it all works exactly as desired… (c5bbb57ad7069c839757f72432d23c43de0b61da)
    - [clone] Most of the V1 handshake works, but… (318024bc14b0bc88e7728bdacaa0c32265618f4d)
    - [clone] YES! Boxes with dyn traits and lifetimes… (5e35d0acf9efcb787eec0591c6f02735ae417e60)
    - [clone] FAIL: Right, need a box after all (6e5792746d164696a1b3bcb64f100328380e19df)
    - [clone] FAIL: can't pass line reader as box (633341dd5f3fbd7b910c545e203e0bd734b5f989)
    - [clone] sketching how to possibly return Line readers while keeping it sane… (4ba123b8e543a2ef3ba07aaf467b208047db0e1d)
    - thanks clippy (81c0185bdb7f483021db1ab85064863c17f33571)
    - refactor (f8ff1c753300c3af7d328440a3591343cbe5f040)
    - [clone] capability parsing (5b019afe49fe1b35f72b91e4919f7a31e87f4f94)
    - refactor (2b40961a2c653d1f5be7d31337e4eed8f08f900a)
    - [clone] a little closer to handling the client handshake (1a4f84dd698d89b83d251f81797cb4cf8c3ceb34)
    - [clone] first frame for testing transport layer interactions (e1100c817bdf49d960aff7b7bee99302583d599c)
    - refactor (f3c5c059169e9cc998ec0c80baf637142eb200ef)
    - bump git-features to 0.4 to allow publishes after breaking changes (9d6b8790e2edd7fa01b3239adff86a7cd2393f10)
    - [clone] move packet-line code into own crate (879af671fcde405d3d08ddbc07ea70d0bee23ef1)
    - [clone] http protocol is now optional (06c0816bd0ed0ae0f125c1fa93ffe1b89e7e7eb1)
    - [clone] (very) First stab at http protocol connection (218a5ebbd5c7b466406fd488ade6c60bde3f78a6)
    - [clone] Better error handling for generalized `connect(…)` (713808cd8bd326b632c2b8f0cfbe7f147b1fa0aa)
    - [clone] fix git-transport crate size (720f4442b06702b9efc1c257d2b54d4a22d3649d)
    - [clone] enable git-transport tests (8e07be44ae38830f4894ed80ea9faab567593256)
    - refactor (104b7fef720751ba7306cee4010e90a20bd6955d)
    - thanks clippy (c62bfa277b854ee21c6774ce88f086a2926dd858)
    - [clone] expand-path should be server-side (8a38856a811078d1d453db9c0e0ad7b6baaaed3c)
    - [clone] the return of actually parsing remote progress (c465fdecd4840627a3c6943af014999f5c9cc3e1)
    - [clone] move packet-lint into transport layer (c0dd8315089243164d82c444499a459756a0337b)
    - [clone] sample on how SSH connection fits in (a56205935ead562a8388e86565919081261dea2a)
    - [clone] first sketch of transport layer's connection logic (f10cee5638a220fff629af274baebbcc0f4f0f61)
    - Allow dual-licensing with Apache 2.0 (ea353eb02fd4f75508600cc5676107bc7e627f1e)
</details>

### v0.2.0 (2020-09-12)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 55 commits contributed to the release.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - update dependency chain in release script (9af47995922fb7bb61729138cbc8c75e7111bdc3)
    - refactor (e4bcfe6406b14feffa63598c7cdcc8ecc73222bd)
    - remove quickerror dependency from git-odb (7e2749521b6c873766a2f6f96e6c91a0c6a9dbf3)
    - (cargo-release) version 0.2.0 (779e9d0ad67c20fa9cec14359e87774ca2d74ee4)
    - refactor (6a84f137754cddfdcb9b1fec3e353762ebb3ce2b)
    - refactor (7874c35bccb74ae7670335e633efa7eaebc72630)
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
</details>

### v0.0.0 (2020-07-12)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 17 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Cargo-diet for the top-level crate (19e7fec7deb5a6419f36a2732c90006377414181)
    - add missing license description (2b80181ad428a9bf267a9660886f347a850fc76f)
    - Make crates publishable (5688a3427ff3673e1422d43106f4d685fa837aed)
    - \#[forbid(unsafe)] for all crates (afda8039259b7a30cfed5dbcdd9caf4773b4c234)
    - cleanup - don't build and run tests while there is nothing to test (4a153da0d60a30615fc402cfecb977f0d771594a)
    - prepare git-transport just so that we don't forget to take the name (2c3ad7d916ca513cc9dff26ff2150bae0dcb93e1)
</details>

### v0.11.0 (2021-08-27)

- upgrade to the latest git-packetline release
#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 104 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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
</details>

