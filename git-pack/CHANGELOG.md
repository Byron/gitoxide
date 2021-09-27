# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

### Unreleased

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 17 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 241 commits contributed to the release over the course of 12 calendar days.
 - 54 commits where understood as [conventional](https://www.conventionalcommits.org).
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
    - a test case showing that headlines are currently ignored, and links too (2a57b755c5513544987be74b3b4b65d35e7718c9)
    - don't try to run tests in binaries that have none… (d453fe5086f819e590af78bba1083659fcc44c01)
    - It's already getting there, even though a few parts are completely missing (ee4aa08fc0ed4bd06c7a987b2a9c86425400d68a)
    - only parse into 'unknown' catch all in special cases… (c0296c4d28016044f7d82afeba10971a526eca36)
    - first basic parsing of unknown parts as segments in sections (f265982a58600b68674f8552252e1ea156fe163d)
    - quick and dirty switch to getting access to a range of parsed input… (f5902f2fa9a6b876497278c9c62a91e58de1c31f)
    - setup test for old method of parsing unknown text… (996c39d002d1781fd7193dabe958af6045936411)
    - refactor tests: unit to integration level (43263226420c0bd9db5d4920f5ce2f76c5367aa8)
    - Don't add a date to unreleased versions (ba4d02404e0a00c1b0d1553032c8062806d09b84)
    - Remove strong-weak typing for conventional type (b71c5790fd8c14f10df00a96f3a344c121278418)
    - Actually integrated generated changelog with existing ones… (aa095e2447fff350492c38600c7303d38ae38824)
    - Fix panic related to incorrect handling of character boundaries (9e92cff33f4f53d3b2d6b55a722d577c2dd6a4f2)
    - inform about 'bat's  absence (c82c5bc682f6b4cc53b5965e3a124a826933718f)
    - Parse message fully (and own it) to allow markdown generation (b8107e5d33da70f91225e9fd37443e3ba2b20f7c)
    - rename --no-bat to --no-preview… (1087dd81ce869de9c886379766a962ec30c93e36)
    - tests for conventional and unconventional description parsing (faade3f95f861736ec0ccf7f0a811c1cf12831cd)
    - basic merging now works (6c6c20002cf7632e8fed11b83a1e2f69b669d907)
    - sketch for finding insertion points and merging sections (2a4903348f6179f6939e6b87d3477e5643acb7b7)
    - Sketch merging logic… (1932e2ca848db57f3907b93e804553524dfa27ac)
    - prepare test for basic merging… (0a14cedbd68058ac296e34a84ab1fe1083a0bf5e)
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
    - clear error message if upload-pack reports an error (4701c84abc4346eda46b062effd38ba8a29a57f0)
    - parse issue numbers from description and clean it up (95c0a510f875e8fd889b87caee356a4c1e099ea8)
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
    - cache::Object trait for caching and retrieving whole objects (50cf610e8939812c3d2268c48835e2dac67d0c31)
    - object cache size is configurable (5a8c2da6cb1e2accf7cfdccc16bc3a1d0b2a7dbc)
    - remove object cache impl which now lives in git-pack (741558dd8194590c5cc8566aa22f96e73df38edf)
    - dynamically sized full-object speeds up diff-based object counting… (d6c44e6ab8f436020d4fb235e423b018fd1e7a9f)
    - Count ref-deltas in thin packs as well (80c6994149d19917c25e36e1bdf0dc8c9678365e)
    - Assure pack-ids are actually unique, the simple way… (0509b4fb5a78a3e4bfcacbeb661d262f8592884a)
    - Use Easy in the one spot where it is possible… (6a97bfabcec6597efe9282e6d5c9f0ac3ada61dc)
    - try to create persistent Easy iterator, but can't make it Send… (54a64a588ff72515451a3d0343306ac4abe1cb35)
    - Add '--thin' flag to pack-create and pass it on (2664d73f531a4b1f4bc784c1fe3a991711c86475)
 * **Uncategorized**
    - Merge branch 'changelog-generation' (bf0106ea21734d4e59d190b424c22743c22da966)
    - thanks clippy (b856da409e6a8fdc81ea32ebb4a534b0e70baebc)
    - thanks clippy (31498bbee4b2bc766b42171dfd6529d885d3bc84)
    - let's not force folks to not use debug info… (bc458c81abddb8c3f96b4c46a4a1dd8cd3a16723)
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

### v0.11.0 (2021-09-08)

- manual bump for safety as its dependencies have breaking changes



#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-pack v0.11.0 (c815bbe910983e46b0f1b6d3394d975da756c72d)
    - Bump git-pack v0.11.0 (5ae6ff52cd2cd1ccd1e26bb987c154eb19603696)
    - Bump git-object v0.14.0 (d4fc81f6390443f8c8561d91ac27ea4a6318fb62)
    - [repository #164] generic write_object() (c569f83363489dde03c8b9cd01e75d35f5e04dbc)
</details>

### v0.10.0 (2021-09-07)

- **renames**
   - `data::Object::into_commit_iter()` -> `data::Object::try_into_commit_iter()`
   - `data::Object::into_tree_iter()` -> `data::Object::try_into_tree_iter()`
   - `data::Object::into_tag_iter()` -> `data::Object::try_into_tag_iter()`



#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 86 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-pack v0.10.0 (b995441b6871132c4c14b426adf1f2688078121b)
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
</details>

### v0.9.0 (2021-08-27)

- **renames / moves / visibility**
   - `find::Find`  and `find::FindExt` only in `Find` and `FindExt` (not in `find` anymore)
   - `data::output::count::Count` -> `data::output::Count`
   - `data::output::entry::Entry` -> `data::output::Entry`
   - `Find::find_existing_*` -> `Find::find_*`
   - `Find::find_existing_*` -> `Find::find_*`
   - `Find::find()-> `Find::try_find()`
   - `bundle::Bundle` -> `Bundle`
   - `bundle::Error` -> `bundle::init::Error`
   - `pub tree::` -> `pub(crate) cache::delta::`
   - `data::object::Object` -> `data::Object`
   - `data::entry::Entry` -> `data::Entry`

* **new methods**
   - `Find::find_tag_iter()`
#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 4 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 185 commits contributed to the release over the course of 5 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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
    - [object #177] fix docs (07be6611d1742633815566443f71eef8b85ad5c0)
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments (461dc53ba3bc07d55fdb4aad7570ba9176a8b360)
    - [object #177] rename immutable::* to immutable::*Ref (6deb01291fb382b7fb9206682e319afa81bacc05)
    - Release git-object v0.13.0 (708fc5abd8af4dd7459f388c7092bf35915c6662)
    - [pack #172] A note about empty packs in Bundle writer (09a777f1da5e792c5eb4c8ff9e83504ad8d19c5c)
    - [ref #175] follow (try_)find(_what) naming convention (679895cf866d643e768e353af614a55aeed2ba5c)
    - Merge pull request #172 from mellowagain/main (61aebbfff02eb87e0e8c49438a093a21b1134baf)
    - Fix formatting of performance-tasks.md (917967e2d464a79a119fb217f687e751394bc5b9)
    - Merge branch 'Byron:main' into main (dc58eca510e5a067acdeaad4b595a34b4598a0cd)
    - Release git-actor v0.4.0 (16358c9bf03604857d51bfa4dbfd2fc8c5210da7)
    - Allow creation of empty indices (d122fc79cc9b9a52a2817bdd46d3215c10e61129)
    - Release git-testtools v0.5.0 (574ede9d7874c6b6016bee9ab0ccc7ce18ec353b)
    - [actor #173] fix docs (2d7956a22511d73b767e443dac21b60e93f286dd)
    - Release git-testtools v0.5.0 (86e0a92c7dc3b69a766aeac1b675b148d61a7ec5)
    - [ref #175] make 'mutable' module private (a80dbcf083bfcf2e291013f7b13bba9e787c5cb4)
    - [actor #173] refactor (08a18498d62f1d5bdabbb4712b08f3d17d63e16c)
    - Upgrade to nom-7 (f0aa3e1b5b407b2afd187c9cb622676fcddaf706)
    - Release git-actor v0.5.0 (a684b0ff96ebfc5e4b3ce78452dc21ce856a6869)
    - [actor #173] rename immutable::Signature to SignatureRef! (96461ace776d6b351b313d4f2697f2d95b9e196e)
    - some helpful remarks; be more specific about fixing breakage (778396568d701faf542e5b5722e6b2c4343244d0)
    - [stability #171] Another question to ask before stabilizing a crate… (c2bc1a6d2b2a1b0ab4963d7edf1b8ab62ba97e4e)
    - Update COLLABORATING.md (e1a04cf8b305c9346d91ff1d4e14693c08283083)
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
    - [stability #171] document git-repository cargo features (8f21e3d14658a6e73407b9cf8d9e6898c6a4c683)
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
</details>

### v0.8.2 (2021-08-17)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-pack v0.8.2 (39a3f71ba5997ac26d9994cdc7c2145af3220f64)
    - Apply nightly rustfmt rules. (5e0edbadb39673d4de640f112fa306349fb11814)
</details>

### v0.8.1 (2021-08-13)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-pack v0.8.1 (045eb094691324a398120f6039bbfa34b4fda1af)
    - remove dev-dependency cycles by removing their version (c40faca41632cd2a226daf4ddf5293b65d1fdc82)
</details>

### v0.8.0 (2021-08-12)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 (f123f69c7a4f9fd1c98bd2f60ebc953a6739fe04)
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 (c67291ff9bcdff9a747d87241f6a71015607af05)
    - Release git-object v0.12.0 (7006150ac314d19814608723f69f6e70a72f9262)
    - (cargo-release) version 0.18.0 (b327590d02fec5536c380b2d39dd7be089ca7c40)
</details>

### v0.6.0 (2021-08-11)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 (d704bca7de0a6591f35345c842d6418b36ecd206)
    - (cargo-release) version 0.6.0 (4b71e15c3ba4a17ff2da5a5ef79986a2832fa3f2)
    - (cargo-release) version 0.5.0 (e21142ba1a113b2afc4725d4d4225dff519c513a)
    - (cargo-release) version 0.17.0 (c52a49176bd294bb36db74b4293cdb684a2ab7f6)
</details>

### v0.5.0 (2021-08-11)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (c2f94a51bce287be301090450cb00cde57e92f76)
    - (cargo-release) version 0.4.0 (d69d0ac21989243fdafa514fa41579fd51bc2558)
    - (cargo-release) version 0.6.0 (d58f37e3b5a000fbe069aa869bd84f66d5c3210b)
    - (cargo-release) version 0.5.0 (1687e599be98d97925fbab594f31cf5558e9d2b1)
    - (cargo-release) version 0.4.0 (28e58f6b43a44e010da749a5618df02441f0d2e8)
    - (cargo-release) version 0.11.0 (a5be31c4cf7c0b538a1ed4a52ff5c3a992c6feff)
    - (cargo-release) version 0.4.0 (70ef3442775b54ba9e4ee9ebfffb37af9804cc5b)
    - [utils #154] refactor: bool.then(||this) - neat (1dec1c49032c8acb449e463fde41f403cb640e45)
    - Revert "break more dev-depedency cycles up to git-odb" (22337ce23995eee474e7dfb2e37fb56814522942)
</details>

### v0.3.1 (2021-08-10)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.16.1 (e10e55c1bf1b40965da9b8b6c87953e6eafda09a)
    - (cargo-release) version 0.3.1 (8b241977b31720e7f08809bca0b277267b29102e)
    - break more dev-depedency cycles up to git-odb (7ee278bf5b04adc5e4ab82cb83a3519f93587176)
</details>

### v0.3.0 (2021-08-10)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 7 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 142 commits contributed to the release over the course of 76 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 (0e9c73abd17e0dd21952275077ae53ad7e7aa1af)
    - (cargo-release) version 0.5.0 (ae02dabae961089a92a21e6a60a7006de4b56dad)
    - (cargo-release) version 0.16.0 (1231dbd16dacefb39adec8e067c312d313a82e3c)
    - (cargo-release) version 0.5.0 (0e11e98f0562c7baa9c90e18db6240731d165217)
    - [pack #153] finish transitioning to git-tempfile (38173fcf62c04b485c4b309bdf7e6b7afacfcd58)
    - thanks clippy (e1964e43979b3e32a5d4bfbe377a842d2c0b10ea)
    - [ref #139] add missing docs (5422ec8923a5f3c284f7094894a952a392812e63)
    - [pack] refactor (581fb51a84567e341d315e6bacee8e681718f7a7)
    - [pack] refactor (b19f6b9b1fcd5ebbc5b1f2a4bef0543b1c693bd1)
    - [pack] fix docs (e7b9d9613874cd1ebaf740dc08db467c461a4751)
    - [pack] fix build (98dd557b963acfe1c4e717451d222c187c46a5da)
    - [pack] update CRC values when changing entries to satisfy all consistency checks (990ea4866be2d22ae2043da2dcd9577b748de255)
    - [pack] fix trailer of last entry to match expected recomputed pack hash… (8d0ec7d7c0afb6112e66518a2987907d2e4d29e3)
    - [pack] refactor (1852e3ea98a462958862ab05f110649e3b06e2b5)
    - [pack] all tests running for now, but… (aec8439683c639f7b6e344cb76bf1dd9fc769d17)
    - [pack] hacky proof of concept that this actually works… (6085a9201ecbd9285547c1d17c9834f09e22fef9)
    - [pack] on the way to 'quickly' get a proof of concept (cdc7582ab7e35ec1daac44401bf7cb62e0b592a2)
    - [pack] refactor (685cce612eec99ed9f15d86d5ce2a7e6c270ae0d)
    - [pack] refactor (f822ebb9e899bd52d5baec8179a843c47d073e44)
    - thanks clippy (96ef0b036c3c94a45f3ab882a8b32bfcc1250653)
    - [pack] a quickly made iterator that writes input::Entries (116bdc4ba879da9785877ebca56ab3c57b9cfd98)
    - [pack] prepare a custom writing iterator for input::Entries… (a4d27648b4021bcf65c95dc5bcfa2b3d11f538fd)
    - thanks clippy (bd517d6374f20670086eedce2776a8ecf7d0d22b)
    - [pack] prepare bundle writer for yet another iterator wrapper… (33be1a1ffba34a64eeb04b4479790fec2f50bcba)
    - [pack] refactor (50861e6266a6e1800607eb19288e040846325c06)
    - [pack] refactor (dc07225d7eea04e0cfe61c87b56009e06491726c)
    - [pack] another todo down, the last one (3fc8c8ff5ab1c49b55e3b9e1af3fa2f0aee68b94)
    - [pack] one more todo down, it should work now, right?… (69a9ff17b3fe16de782ffabb76b87510e8a5b74e)
    - [pack] fix thin pack support test… (4bdebddd3791ba71f3f6b4182229a1c48c5a4a95)
    - [pack] definitely not working yet (690d9b7fbc34b7d2393649d39290071f81cb8bb1)
    - [pack] a step closer, new cases show up (75eaba36072cf29e76a97fbbd425f0861eb657e2)
    - [pack] refactor (a8512f89a4e0dd7492fa208c1da41eed9d6a208f)
    - [pack] improved test to validate a fix (e3eeeb146a0ba3dbe701b2e4da560309ff181753)
    - [pack] attempt to get a more realistic test, but… (2890737c7e074d31f3bb55acb63664a2da93faaa)
    - [pack] refactor (cabc1e5858d52806542ee8d9266bac36e5d39c96)
    - [pack] first succeeding test (f5da439dce93cc203dacb4a5e9d0ae68a87b9be4)
    - [pack] first reasonably failing test showing that offset computation is indeed wrong (df1bc2f66ff9e7046898b6937c5ad239313a70dc)
    - [pack] the first test for the lookup ref deltas iter (b162f9eb37f09f49e363376dc3f0c6c126442bbf)
    - [pack] Make use of thin-pack resolver when writing bundles… (9f43bf029624f7c94346646465e366609b89e2e1)
    - [pack] handle the same ref-base correctly (2f948545a935d2cb7c5a252ec74764440a9ff595)
    - [pack] thin pack resolver which might actually work (54f055a53e888156459340e8ab160650a198ab13)
    - [pack] first sketch of resolver for thin pack entries (ee428e07bcc3df9bc795d06068a444beed71f2d0)
    - [pack] refactor (a8fd70fdbff871779ad5a9ba491162ae49605c9f)
    - [pack] thanks clippy (7c2fc89c70aa6de9cb0707799918e623267326a8)
    - [pack] actually, this is how it works, so this code should be unreachable (8f359e1fc8cb99fcf0003eaab1d97cdeaac20876)
    - [pack] first step towards fixing bad-objects properly (3c965070a7c799f0507f9e7faae2896346bc9e65)
    - [pack] discard bad-object tracking in favor of delayed handling (31ce008208cdd3bc4f093abab6fabf4c8074c130)
    - Revert "[pack] fix race to finally make pack-gen missing objects…" (ad0d2a8e4e92d11351225db0115de0ed1210f9e3)
    - [pack] fix race to finally make pack-gen missing objects… (73394db1b048d3dc87b8b4934737f27b6a8a0d3c)
    - [pack] it seems git is just skipping bad objects during pack-gen (0f29b82b48f45f509016eb16ea92af7f6dbf65a6)
    - Revert "[pack] FAIL: See if not looking up the pack location speeds up counting…" (d03fe9732b69c6ca3b7a6df96097233661e53a05)
    - [pack] FAIL: See if not looking up the pack location speeds up counting… (48c49300a55e6443d5e4d94632979b6d07f2bc5a)
    - Revert "[pack] FAIL: speedup with Mutex<HashSet>" (df98edf48c49717136a6e8e5d9b1f64aeda17db2)
    - [pack] FAIL: speedup with Mutex<HashSet> (f8aca03c2d126574541c136019df4e51b52a5b10)
    - [pack] In single-threaded mode, use a huge cache for some speedup (aec8a9b4b9deb102b06390a19727eab7660621f9)
    - [pack] fix offset index properly by using chunk-absolute offsets (461c1eefe9214b07cd80a37292b23744846383d3)
    - [pack] forcefully fix issue with incorrect partition point (290bd65f10f5a64de6735b09119b7bbffc44254b)
    - [pack] test for parital pack without thin pack allowance… (1f48d3b58a1151a1fefce9bf4af5649837309a37)
    - [pack] pack-create with immediate counting and traversing… (b74a98fc87a92a8ccbaec59aeea5284731e2fe49)
    - [pack] entry writer now supports deltas and it seems to work even (fcda6f096f95a6322122229ac364a2dd5ea0ce6b)
    - thanks clippy (cc61f82f597d9a0ab43efaaccc2cb568b9aa746f)
    - [pack] on-demand cache for pack-offset to id lookup (0bfdea843606673005ecab6a482a9fce89a4cb69)
    - [pack] refactor (4bb3ce4f2e89dd817c284ed8ae9e2559ed60f9a2)
    - [pack] thin pack offset to index lookup (121aca45ecb1acce3496b1b2ac003aa95851f247)
    - [pack] refactor (372b9cee78a6b49eb7ebb5cf452a324e07775d98)
    - [pack] a way to obtain whole bundles for offset-to-index lookup (15fcbe254b75e8f74652711cc339ae5ade74d24c)
    - [pack] refactor (64b1dcdb0fb53749ce73017d0dc1e053689d17d4)
    - [pack] refactor (1d713b482264ddb0aba6a98e3918f8236ce12c80)
    - [pack] refactor (cdf020a3b29bc59062d3ccf56672e9c18201c67c)
    - [pack] refactor (2ccefb2832b326966a24d0cbcfd79ca5309f91aa)
    - [pack] refactor; entry-iterator now produces delta-objects (5dc370ba01d25a6e8b7f4bfa03259c83e6b1d758)
    - [pack] rough version of obtaining object indices for deltas (a58e270ef96011ffd2434539e3099cbe27aed3f3)
    - [pack] refactor (8cfa414482a4318ed385f42582ec885fb73134e3)
    - [pack] pass all data to where it belongs to… (af5cb1f4b809ac268ca3d878896854c966dcea97)
    - [pack] add the notion of thin-packs to the pack generator (a289bbaa36546109d3371a8fcd7a6dc3c363861f)
    - [pack] build an index of pack ranges as well (4d6ab7b74c325820a3760361faace380f958572f)
    - [pack] bundle::Location with pack offset; order counts by that… (f92f285167c6b5bc4d86f255e360c4534e38bb29)
    - [pack] better identify the currently implemented pack generation mode. (f9e3b3ca3bbf063e8d71c62fe607b812c745a969)
    - [pack] refactor (f3dc3da492e1dda5dd9e43fddc57da6a118081b3)
    - [pack] refactor (9ee1e22fa5c5d97ff626f0dfc44706272433bfef)
    - [pack] refactor (78d46c13d0510ee3e2e2f33cd60d624d63e85900)
    - [pack] refactor (69af3526b0fcfa8a270238f3e2cf59d332bd187e)
    - change wording (6c82a16d340acb9b11c5cf56c917c9fe6f2cdf0e)
    - Bump uluru from 2.1.1 to 2.2.0 (52e274fe985948b6b742ff7066fcb9831e427ba3)
    - Don't use ASM on windows for Sha1 as it fails to build there. (ba1fb7ab5bc03f5a23ece32ff1e144544e1eaeae)
    - Merge branch 'remove-unnecessary-unsafe' (7a3c5c14dc56d8711548d1b219a969836693cbaa)
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
    - thanks clippy (6200ed9ac5609c74de4254ab663c19cfe3591402)
    - fix build (dbfa49acf58b2c0763c5e98e5276860b43dfb27b)
    - Fix everything up so that… (5930563601d6c2148cf39e109f69f8b7c7dfcb36)
    - A first attempt to make intrerupt tools work, but… (8fb8d374ecfeffa3ae1bd07bf9bc5014351730f5)
    - fix pack tests (7968467cc0d392e3d223811ed36ae777531a5a36)
    - The last occurrence of the global git-features::interrupt usage gone (6820724be83ebf48c7ccf6a65a3d6383f766c9de)
    - another one (0a8ed0ecc078d76dc3a5fe13518cf43bfbb121f0)
    - And another one down (abce75eefff44b9538c112b60ad5e0596482e89c)
    - refactor (7f9be36ea909ee67555591287bcb140fdc54c801)
    - And one less usage of the global interrupt handler… (5da57a3b0efef75ad82cb4d1cd496fc7fc0f1c23)
    - thanks clippy (3b2e7650d8afe2c0e246e005ab1c321a157cbd44)
    - Make most interrupts local to the method or function (458899306a3f3c8578f185d7ecbf1ade2a7142dd)
    - [features] sketch of iterator to auto-check for interruptions (61d3a15c66b4c1be1d98715b8a60705a3a314455)
    - [pack] refactor (25f04baa100bd1996f48fbeb4c87e40ff1b27d90)
    - [pack] refactor (18cabb8618ffc324412302bfda208948abffb61f)
    - [pack] also put counts in order for stable packs (f299160cafd00f0fea00a2402901570f5ddf27d5)
    - [pack] fix run of 'cargo test --all' (e7ecdc195d03fa9a29ad1e44464b42e3ca6fb6a4)
    - [pack] a working in-order iterator (5fea926803bcc7b2ef7d8f156e3d31a503831091)
    - [pack] tests for error handling of in-order iterator (44892cca9309c4cca0eaa30dbedc65422a2699d1)
    - [pack] ground work for ordering in produced chunks (96806494d32243bd1798a89c094e220dbe050d68)
    - [pack] also run multi-threaded tests as part of unit-tests (5d3006a5d075bce9011b20920a84404952624c45)
    - Bump uluru from 2.0.0 to 2.1.1 (b6ac506ba2df0f82eaae64eaf023cc0c0376ddff)
    - [pack] hopefully fix tests on CI; verify determinism of pack (51dec8b3c661ba9071306ab89796aa93d9a25b65)
    - [pack] deterministic single-threaded pack generation (ddb6442fd6681a2dd3890a8a415003ec770c7d64)
    - [pack] refactor (cfdf8021ea1448ac4844b1f3bf252fefde2572fa)
    - [pack] basic statistics for entries (37229a650ceb0155aa7ca87b499fe188ac4bb565)
    - thanks clippy (18b2113b1e3c372145bc9037ee6a9de7efe4e506)
    - [pack] write packs to a directory with the proper name (3fbca7dd62752a7dd752b83a39ec8dfd7b2f2ea8)
    - [pack] refactor (f10adea76d92eada3ca204fe69e7b5f81a06d8cc)
    - [pack] fix docs (6ba471d228c45a3821b4984905a4b4ecaff5b0b0)
    - [pack] fix build (81ee633c7f482746bc28a2a43d74ebbaded7af5f)
    - [pack] statistics for counting objects seemingly work… (4e3deb1364dd1bef0af79d6aa97086a95b4983bc)
    - [pack] actual counts statistics (3a9f6d8a53da3235bde4a3f32859381d4843cb7e)
    - [pack] aggregate the count outcome (c7ac0e60a5d69f3a948d47c3acc3060cddbafb98)
    - [pack] use statistics reducer (0974ab176777bfa02ac0ea32915f6d9c46e3ddeb)
    - [pack] count object reducer sketch (ea4569282e2f63042869dd47205874c161bfecfe)
    - [pack] refactor (fdf485afa66af20abca586b04f588a33c167310f)
    - [pack] refactor (0514f1df113c5f6bf1c934b15741ca8ea47316ae)
    - [pack] refactor (37922d12765c221e747fad4ca813597490525279)
    - (cargo-release) version 0.3.0 (6b33678f83e6d261ca15c4a7634ff5b4e66d81dd)
    - (cargo-release) version 0.2.0 (3286e42547b59df6365087cbae9ce1c9c959faad)
    - refactor (a25a774675e2e9db1c891351077d3af2fd5c72ed)
    - [git-transport] Show how to use blocking git-pack code in non-blocking transports (de2ba3c4919d454894911c54fd4bb0e0a4665723)
    - (cargo-release) version 0.4.0 (866f86f59e66652968dcafc1a57912f9849cb21d)
    - [git-repository] towards git-repository as one stop shop (aea6cc536f438050cc0e02223de7702cd7912e75)
    - [git-ref] the first failing test (7e802a0576230dfc666c253d484ea255f265f92f)
</details>

### v0.2.0 (2021-05-25)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 (b213628feeb8dfa87dab489c7d3155a60e6a236d)
    - [git-odb] prep release (4984ce3e19b60b89a4337f90ac4b9c44c42558a0)
    - [git-odb] refactor (2958145a0ae1ef582bbf88352f5567d5c2b5eaf0)
    - [git-pack] fix docs (efd20d4e1afbfbe573d620dea4761c06f948a296)
    - [git-pack] refactor (ea2b3deab78882943e11270e4166ca7c340b03e1)
    - [git-pack] refactor (bc4b7b18a04506a3d08d66d1222d706b82a2f6e7)
    - [git-pack] refactor (157b6ff7b55ba2b7f8f90f66864212906426f8d7)
    - [git-pack] refactor (49c1c3ea67379c5a122a8c3921d8ff713e14d371)
    - (cargo-release) version 0.16.0 (769c649c00c009bf5a3f7c0611a7b999618f2938)
    - [git-pack] refactor (be6ddaa98fc1dcaf77dc0fd9c9d67754e74927e4)
    - [git-pack] used by git-odb (5d6ee07a8dec64fe5f68c14c418d922077fad3df)
    - [git-pack] refactor (1b2a245aa494c0f9cacc2ad6b8ca02e9891fdb4c)
    - [git-pack] move hash-writer to git-features as it's quite general purpose (80e5640169363910b4189fda58bb495c6677eaaa)
</details>

### v0.1.0 (2021-05-24)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [git-pack] prepare first release (4f9eb7070f0d84c5343de10e7122b5acc31885a7)
    - [git-pack] the very first version… (8c06cdb14269e798b7ff771ea3864f85fa673ed7)
</details>

