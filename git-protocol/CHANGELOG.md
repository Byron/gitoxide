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

 - 220 commits contributed to the release over the course of 12 calendar days.
 - 53 commits where understood as [conventional](https://www.conventionalcommits.org).
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
    - fix docs (90056c8b3442ba11646ddcf6790c506576fb2394)
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
</details>

### v0.10.4 (2021-09-10)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 24 commits contributed to the release.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#198**
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

### v0.10.3 (2021-09-07)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 40 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-protocol v0.10.3 (aa90f98eb45e93b629462b629660e38b1824c405)
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
</details>

### v0.10.2 (2021-08-29)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 24 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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

### v0.10.1 (2021-08-27)

- instruct docs.rs which features to use for more useful documentation



#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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
</details>

### v0.10.0 (2021-08-27)

- Various minor updates of pre-release dependencies

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 168 commits contributed to the release over the course of 3 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#163**
    - Adjust collaboration guidelines to allow and even require PRs (998ae6bf214d576cbf3f5b53f8d75e908ec63474)
 * **Uncategorized**
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
    - Fix formatting of performance-tasks.md (917967e2d464a79a119fb217f687e751394bc5b9)
    - Merge branch 'Byron:main' into main (dc58eca510e5a067acdeaad4b595a34b4598a0cd)
    - Release git-actor v0.4.0 (16358c9bf03604857d51bfa4dbfd2fc8c5210da7)
    - Allow creation of empty indices (d122fc79cc9b9a52a2817bdd46d3215c10e61129)
    - Release git-testtools v0.5.0 (574ede9d7874c6b6016bee9ab0ccc7ce18ec353b)
    - [actor #173] fix docs (2d7956a22511d73b767e443dac21b60e93f286dd)
    - A note about the project board to help with transparency (d8500043ab6b66335e9e09ba1706564a28421bbe)
    - Release git-testtools v0.5.0 (86e0a92c7dc3b69a766aeac1b675b148d61a7ec5)
    - [actor #173] refactor (08a18498d62f1d5bdabbb4712b08f3d17d63e16c)
    - Upgrade to nom-7 (f0aa3e1b5b407b2afd187c9cb622676fcddaf706)
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
    - cleanup imports (e6693032f1391416fd704c21617051ddfb862a3a)
    - [pack #167] Use custom uluru version to avoid a lot of allocations… (7bd3671ad949d62f84147ef7ff3fde59937fee54)
    - update dependencies (e9a98bc0078189f58b7c6e47bf46949cbe0730ee)
    - [pack #164] fix docs (08ee674c55cef6ab76520de2f836b246c907888c)
    - [stability #171] Don't provide access to less stable crates in `Respository` (e4c5b58ad935c907dfbd0d61049453dcb64a7e19)
    - Merge branch 'main' into 162-repo-design-sketch (e63b63412c02db469fbdb17da82cd1e9fda1ef0f)
    - [stability #171] update README with stability information… (f330daa06577eabbd61c66526710371a14228274)
    - Revert "[pack #167] Use custom uluru version to avoid a lot of allocations…" (4c2ea212bbffb0ba3c21ba388dfc79cc7a1c4734)
    - [stability #171] How to handle the MSRV (9be1fcedf94e65b84f9769f74410a7c4f374f6ba)
    - [pack #167] Use custom uluru version to avoid a lot of allocations… (8d499762b74c08437d901bb98806e0a1fc6f93bb)
    - [stability #171] Don't leak unstable plumbing crates in git-repository… (71eb30f1caa41c1f9fe5d2785b71c9d77922c2af)
    - [pack #167] a single-threaded special case for counting… (65e29de45a92c82cebd832634ab194db19a1b590)
    - [stability #171] about transitioning from pre-release to release (bdbdb653d172b988a7cd91810bacdc6cd2ba1626)
    - [pack #167] generalize over immutable insertions… (169f000087aab18f0257fb0c61dc3b3901e97505)
    - [stability #171] finish tier description… (4fe125973304b765f0171deb1c26bca64bbff5d7)
    - [pack #167] refactor (6bf0f7e86312b2a4d262c80979c61c94519bd4b0)
    - [stability #171] Rough descriptions of ST 3 and 2 (340935c7c2ba34785313e529e2ed93c84abc2cfb)
    - [pack #167] progress is handled by reducer… (a22f8e171e705bc42fcf290789e8e05423bd72d1)
    - [stability #164] First sketch of stability MD… (a7353cd1d9999be4744a1c70a37f3c0ffaad706a)
</details>

### v0.9.0 (2021-08-17)

#### BREAKING

- Add fifth argument to `fetch(…)`


#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-protocol v0.9.0 (466d8ea4df3bd6a0db482e8e57155df7b4e5608f)
    - [protocol] prepare release to fix crates-io instalations (83d74239108df420bd464c340762c1dfcb6ae78a)
    - bump git-protocol to v0.9.0 as there are breaking changes (b4e33408b8eb12c9418704f663322385fd1dfb25)
    - Apply nightly rustfmt rules. (5e0edbadb39673d4de640f112fa306349fb11814)
</details>

### v0.8.1 (2021-08-13)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-protocol v0.8.1 (b57c3397706940354c493cadf1ab93916b79f917)
    - Release git-transport v0.10.0 (b94427835bf922aa9388cdd78200c79a3c31da43)
    - Release git-packetline v0.9.0 (7ffbd602c08605026b0bb97ab85216907badaf09)
    - remove dev-dependency cycles by removing their version (c40faca41632cd2a226daf4ddf5293b65d1fdc82)
    - bump transport version to 0.10 (f26a3d3a2745f3eb69d76e0cfd718a90cf74f003)
    - (cargo-release) version 0.8.0 (ad6d7f9c2b4f8879d466e758fc9b51ece6879e96)
    - (cargo-release) version 0.7.0 (2ef3106eb84981e2dabd84f81362b4e44f938ea6)
    - [transport] A much better name for 'is_stateful()` (f15f1e85fda76eef72c3754d625cf51e3c454eea)
    - [protocol] Make fetch-connection usage explicit (29696f9b8e3ba3a72af1b099dac1c0866194d5ce)
</details>

### v0.8.0 (2021-08-10)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 125 commits contributed to the release over the course of 90 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Revert "[ref] break dev-dependency cycle" (436e89b18cb157b3d30bd24b8d1acef25631ec2a)
    - (cargo-release) version 0.5.0 (ae02dabae961089a92a21e6a60a7006de4b56dad)
    - (cargo-release) version 0.16.0 (1231dbd16dacefb39adec8e067c312d313a82e3c)
    - [protocol RL-#741] Respect delegate configuration when running only ls-refs (65ce8e1812ce820e3a0c40e39170339bf73234e5)
    - [protocol #145] Unify the `previous` and `previous_result` parameters… (96f77c78a08e975d367ca25ac5d07eb2253cf4e5)
    - Merge pull request #145 from kim/ref-in-want-corrections (8dfc943382909b45959b49381f6c21431d343a63)
    - Turn incremental compilation in dev off for now (2449621805a027421fb6eef08ce62f2ac595724d)
    - [protocol] remove misleading documentation about ref-in-want (9a8f6b5480bf55f52315ddf86ac28771147a4664)
    - clippy on tests and thanks clippy (a77a71cf02d328a2a964388928d6b2a235a0aa85)
    - thanks clippy (e1964e43979b3e32a5d4bfbe377a842d2c0b10ea)
    - Bump async-trait from 0.1.50 to 0.1.51 (ce0b81e8f5c652d389ff876844bc42bcfa687921)
    - Bump futures-io from 0.3.15 to 0.3.16 (3c23820d3f0d3567f44215cdb0ad13ab675a201f)
    - [protocol] Delegate will indicate end-of-operation when fetch is done (928f75ad939e35a159d3d2751d5d0f9d00d796af)
    - [protocol] Let 'fetch()' only be used via `git_protocol::fetch` (4bae2f959bdf3f4a12b378f5734d9abdc25af36d)
    - thanks clippy (eccbecb938f0c84b63ad7e1ee17fb8113ce89c2e)
    - [protocol] fix build (38aca4076037a6f8288c2cf483f134ea16c328d5)
    - [protocol] Allow both preparation delegate methods to fail (d89393bbd5fce130a50855316ef364083c62eccd)
    - [protocol] start trying LsRefsAction::Abort(Box<dyn Error>)… (660b9dcc4e5249506a7656b038333f64b109261d)
    - Merge branch 'negotiate-fallible' (27c8abe1948bc10c779efa33d4bc0b92741f6444)
    - [object] Generalize nom error handling and use nom-errors instead of custom ones (47c8a97194c9e401ee311234a269f8b8f3650ba1)
    - [protocol] adjust description of fetch::Error to match io::Error sources (23dafc6e24377ad00b70c0235fd7a8ff107eee0a)
    - [actor] refactor (bccb738edfc2e6923643a2e73f93b6acfdd7cf5c)
    - [protocol] fallible negotiation (e269a2cde18f604a36b33efb7e53f31ea5c45e2d)
    - [actor] FAIL an attempt to remove btoi errors (3f99cf531caacb93a3ce81b16d61be18e5d8a017)
    - [ref] Try using BorrowMut to avoid blanket trait impls, but… (4bb9bbad5b4e0c2e64a48a8e4a70a1b3af1ca3e3)
    - [protocol] only send flush packets in stateful connections (0995c225c92a0dcccd2514b53abcf8400d9342e1)
    - [transport] remove Transport::close()… (4268a9bcf733413f7326be7af487a8fcdec1f71c)
    - [ref] rename Action::Close to Action::Cancel… (cac1f6c757709797d193c6bca30e99fe40466ddc)
    - [transport] impl Delegate for &mut T: Delegate; refactor fetch() signature (2ded7f9b2659ab8705ad6b896aaf6ca5afb12a6c)
    - [transport] implement Transport for &mut T: Transport as well (372fb8183aff19bd0f2d17ea74409b2ca3a08511)
    - Merge branch 'ref-in-want' (f248557186384501e705473e0adab03d3fa10519)
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
    - [transport] tests for extra parameters (fffd926a3d5c6abfa732aa2305a4a05fdd06254d)
    - [protocol] extra_parameters are forwarded from delegate to handshake (03e3db3809bd031d7d0c151ada2542214d7e32c0)
    - [transport] unsupported protocol versions now abort the fetch operation (812aa3bc02a823cb9277847db905e76a50ee7413)
    - [transport] flexible version of version support check doesn't actually work :D (2b220f0758cb7a96a66b256552f13a020cdee3fc)
    - [protocol] make refs parsing functionality public (d6da891419f66208a8820185dd165e62b7a01a6e)
    - [protocol] async-io path handles improved refs parsing (328ab9c4ce739fe79f12ae539ea37e50c541b786)
    - [protocol] first step towards keeping InternalRef internal in blocking-io (6c4ed2d4dd352b4218419a1a79269a49cc91a992)
    - refactor (24697bc66363f8e8b1ff14a59fdf303ffdab132d)
    - [async-client] cleanup Send bounds! (c7dee44267462d5ece491b8a45cf35afa904ce81)
    - [async-client] refactor (b252932ee3eb26bb26560a849a9b13aca11cf00f)
    - [async-client] unblock the async delegate in the cheapest possible way… (a3b5d75d387dc5d6c44f695f63df8803613637a2)
    - Revert "[async-client] a taste of what it means to unblock the delegate" (2ba452ff1c9659f7433328b12732d792e7871102)
    - [async-client] a taste of what it means to unblock the delegate (4d6c10a6956bb9a81144a61ebb6bcab3aedb840e)
    - [async-client] prepare for unblocking the protocol delegate (796c7d54a20ef32a581be572e1d681f9727482de)
    - [async-client] refactor (0d5b911ad5f47ab8f044d6bbe660a6d1dfeecb5f)
    - Revert "[async-client] Try to bring 'Send' back but…" (52eb953fcc44cce19604b1df6a600237b8c81392)
    - [async-client] Try to bring 'Send' back but… (3a06adb41f6b2946f78044e4ab1385e6441fc40f)
    - [git-protocol] fix test (e30ea363311aa82486828c59755a012cc76751b1)
    - [git-protocol] no warnings when building without client (2f3066659280f7b43ca39d285166f11192ac7fa9)
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
    - [git-protocol] refactor (a8dc078e00d8a5689ba0d8070732421d35df50c8)
    - refactor (2eefe1712131a69298be02e94df8b6ba844afcd9)
    - [git-protocol] prepare response module for async (08b891b089081a3ec3c44ed27b1aca316391d0de)
    - [git-protocol] fix tests without any feature toggles (1da0b1ab9e22040d5b273a5604954859990e0334)
    - thanks clippy (91fdfba7cabc7331598903106a1dd7cea3b49eeb)
    - [git-protocol] refs now available in async (3a5b2cfcc50a48e09a6495c4c15af69596f966df)
    - [git-protocol] refactor (abf0b9d41a2509d35102970602e77fb45e898d52)
    - [git-protocol] prepare to translate refs (bf79c91b30be61135dd33122bb93b3cf3a49f586)
    - [git-protocol] no warnings if there is no client feature set (335e83136efa7cb0913bc5e317bb49d616ee0290)
    - [git-protocol] fix tests in case there is no client feature set (1ee551878ef21d20925fab00b3eef044ada97065)
    - [git-protocol] refactor (0b4ff166175dd51cded5131bcebf1edd80335abe)
    - [git-protocol] refactor (e99a03b360e4bb757904a03834297f14df67838f)
    - [git-protocol] async capabilities and arguments abstractions (aa3eacbd53665d6b76bd9706d801d1189a970261)
    - [git-protocol] now just a dummy async transport impl and… (c7f0b80182c08430a3720474eda41519b6814f17)
    - [git-protocol] a big step towards getting 'Arguments' test into async (5d1c30f3ceae6fe26a0d9961d135b44f371d9cd7)
    - [git-protocol] move everything into `blocking_io` for later translation… (fa03374fd42e127f5be7fb4da2bac85ea38c8afa)
    - [git-protocol] all blocking fetch tests (0d39b5d23659d29a9f0e33428db401a3a887c007)
    - [git-protocol] re-introduce credentials helper code (6a5575fa7dbfa2a835fabf6746494097c3af23c2)
    - [git-protocol] separate test configuration for async mode (62a117c4e6bd205c4bb1d224db7d8e80ba46322f)
    - [git-transport] fix git-protocol (0cc9537036003c86584223aa61f9c207a2c5c2df)
    - [git-protocol] simplify test setup (189ed2c32636ef59975dd15ec0ef61e8a62b98c0)
    - refactor (2ba9f915035a518bef3eb8b0ed1c9972c4a47cfa)
    - (cargo-release) version 0.4.0 (866f86f59e66652968dcafc1a57912f9849cb21d)
    - Switch to latest nom (859e57eae93c3490523b7ed98f7a606acbd87a2f)
    - (cargo-release) version 0.15.0 (d69d9fb0931f8257cef96ef14a89da9340ad9738)
    - Put prodash behind a feature toggle, too (966058d611c548e90c050462de52e36f1925e775)
    - [git-packetline] refactor (1328c5b4001f380936beff73e1f822f14e41e98b)
    - (cargo-release) version 0.6.0 (ec5a54e9f3543afddc9f972f16135edc6ef6ff5b)
    - [git-packetline] refactor (e5769d1e7668ae54c667d2593c0c22e7723710c0)
    - (cargo-release) version 0.8.0 (ccea4b6bcdaba0ee6c6a6236d225ea1276d2547c)
    - (cargo-release) version 0.9.0 (18f6d011043203523f1d0dacf657704ed3f9cf89)
    - [git-transport] simplify parsing capabilities from lines (401af0974742f10c8b9b3c9752e9d30205e96c16)
    - [git-protocol] separate tests those who need feature toggles (4a49d6406c9c39d75ab5021b6e213fd2c9d63adb)
    - [git-transport] remove default features to force being explicit everywhere (d1b39f8093c032a172237a584c9208479611a866)
    - Fix git-protocol (284f8af0599bee4e3de0e385b69a389713cee9f7)
    - refactor (141228219d33e8056489514f91221d803888edd8)
</details>

### v0.7.0 (2021-05-09)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 21 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [track publish] git-transport/0.8.0 (76f7f1ca0b5725dbb04ecf6248f1665ac0e2e7ea)
    - (cargo-release) version 0.7.0 (069184e55057a1655d2754cb1fd68a4424beff34)
    - (cargo-release) version 0.8.0 (411a05ead1546c76fe51f359fbcb961a1140535e)
    - (cargo-release) version 0.5.0 (8c4cc3fb5922d1a761463bbbad65e59f91cce4cb)
    - thanks clippy (17258cc58767caa6e71227898decd160ad0cdf13)
    - (cargo-release) version 0.14.0 (a760f8c013e13ba82daa1acf1a4a57e0818a008d)
    - (cargo-release) version 0.3.0 (e9665c784ae7e5cdaf662151395ee2355e9b57b6)
    - (cargo-release) version 0.13.0 (ac2eddb06eb3d8a9a3dcdcd796eb54a7e45ab935)
</details>

### v0.6.0 (2021-04-08)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#63**
    - git-protocol uses `oid` type (3930a6ff508f5bb2249fb2c2f21e00b74fecda22)
    - refactor; better errors for invalid hash sizes (be84b36129694a2e89d1b81d932f2eba23aedf54)
    - Make ObjectId/oid happen! (ca78d15373ec988d909be8f240baefe75555e077)
    - Remove all public exports of git-hash types in git-object (accf89d25560e5ded6f44a1c4a898ee65d14f8f6)
    - Remove re-export of git_object::borrowed::Id (a3f28169c1268c1129852f279631d5a7f7540cdf)
    - Make git-hash Error usage explicit (it's for decoding only) (4805cfc8d837bb111424b5e32f46d0fb9b12365a)
 * **Uncategorized**
    - (cargo-release) version 0.6.0 (8513f0fafbf8ae61d86df2d8b0aefa52d3eb1680)
    - (cargo-release) version 0.7.0 (334b7e1b838b5201f2484be42dee3c4d2fd789d7)
    - (cargo-release) version 0.12.0 (3b71e7e8416e550b47e5aed2259c1181497ac9e8)
    - (cargo-release) version 0.2.0 (4ec09f4d2239ea1d44f7145027e64191bf2c158c)
</details>

### v0.5.0 (2021-03-26)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 60 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (3cc4a5799fa1f487452b5c346b57fea97e45b47e)
    - (cargo-release) version 0.6.0 (50fb6f25e9afa900ac1c3cfb88d7ca0d5a9a95f7)
    - thanks clippy (0fc239cf9b773f72928b7c42344b578c6ff5d19f)
    - thanks clippy (749ceba246fb8a4cb8d48fa86184619fef500108)
    - (cargo-release) version 0.11.0 (1aa1f5e84a07427d5d7f3231735fe9c1923f506f)
</details>

### v0.4.1 (2021-01-05)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 5 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.1 (6244fb4cfbc40d35f46f4d1942519414a04ac355)
    - finish docs for `git-protocol` crate (598f700ce2a273a6f430c8d2442dbd71e21a2704)
    - revise trait documentation of git-protocol (52711283d456eefcbdc37ac8f8da36149afc1322)
    - docs for response in git-protocol (487de1383d801fb442abc8101666a0d9a050af15)
    - more docs for git-protocol (bca0cbd98ab02b63ac24b1d15baea602b02e1623)
    - docs for fetch::refs (6a97a3e5883d9a6c0011a68b16966d1f8be589d7)
    - docs for git credentials helper utilities (eb6bb6ee2fe22ad0621f7e1743a7e56adbc54bd1)
    - first pieces of docs for git-protocol (12d8a83fbc1b70bd2612ad62aa1a69e87914fe39)
    - thanks clippy (343ab9adb62da1dde495fc209c179137bbe59a10)
</details>

### v0.4.0 (2020-12-16)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (28df5e9131aec3efb2b68db204662b92b232b33c)
    - All crates use git-hash::Kind and its types, sometimes through git-object (124c171aaf546d8977e9913ff84e65383a80ee98)
    - use git-hash in git-features (5b307e076f6f5975592c8b177c122c91c1d809c6)
</details>

### v0.3.0 (2020-12-15)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 (e60dbe6c21843eab44d6f05fe70927252453cb41)
    - (cargo-release) version 0.4.0 (32aefc051c7ad9d1a160f77db070df7fa4843dbc)
    - (cargo-release) version 0.4.0 (72eaeceed135e4cc5c943685f4c902d03597c4d2)
    - (cargo-release) version 0.9.0 (a89fdb98f64bb0ca070fa79a1f58f1232bb14090)
    - (cargo-release) version 0.5.0 (fc7d600ac2c438c8b6b91f67cb69b0ac5ec37675)
</details>

### v0.2.0 (2020-12-15)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 67 commits contributed to the release over the course of 82 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 (a476a46b7b933a3c2fa4aa8c285beec1777a3f2d)
    - (cargo-release) version 0.3.0 (d19ee35cc6683c63e0eabd717e4758075faeaa71)
    - (cargo-release) version 0.3.0 (eade7d101e071153055b07d9c6ae3c1452493a21)
    - (cargo-release) version 0.8.0 (47c00c2228cf25c79e1fa3eb4229c7ab24de91e5)
    - cargo clippy Rust 1.48 (475a68ce33b895de911939c51afa159df534f7b8)
    - (cargo-release) version 0.7.0 (7fa7baeb3e7d008a25e4d714eff908e2516c828b)
    - Add and use borrowed::Id::null_sha1() (c717492d0038f55a6f21b48937b56a756890d214)
    - docs for Sink (e7a09f0628b44ae0c6b564ef41f044e51866f2df)
    - refactor (e4935e03040e1f4ded652ed43a1e0177eefb44f4)
    - a path towards making config Files editable (bc008c32a16849a212eced783aa14727765004c3)
    - replace 'ImpossibleVariantError' with 'std::convert::Infallible'` (c53638ccd9e392af839b7eb03826fa6aab94faff)
    - additional setters for more fluid edits (5a54dae6470c5dcf48bf96c16c5bbe2a8951be6a)
    - refactor (8c658da05a4649814eef9f7ab57525aff0605afc)
    - sketch out editing lossless of Files (8f00063bc9b6a63ffe44e58945be55acca40a714)
    - Add lean-plumbing docs for path of commit-graph-verify (5c7b52d658d5b86bf4cf05c724202e824016c0e2)
    - dependency update (7579b4326ee58f8aaeab80b8597cdf2ead82b0e6)
    - [commitgraph] Clean up `{file,graph}::verify::Error` types. (fa22cab259338dc140dd660f4f4b9bbc9d6cc3d0)
    - docs for compound object databases (813df7115eb643742158f975975eb7469443cc07)
    - [commitgraph] Implement basic commit-graph file verification. (2571113fea516737acedac08d66632ead499b474)
    - Skip comments as well (32cc6849444c16a3d2917c6de62e47597c9979da)
    - [commitgraph] Loosen lifetime restrictions on return values. (701f33c06b80deaabe7625b01d36e2a1b1af3a78)
    - Stop entry iteration when next section is encountered (83a1b83a1f7a0ff22850efc7b5b460f0c1ed8230)
    - [commitgraph] Replace `T as U` with `U::from(T)` or `t.try_into()`. (28f94b4bccdf317c9f4ccb62e0e3f3314f3995c9)
    - sketch of iteration over sections and entries (acb894762b38f77d21e6d70936727cf0daeaff6f)
    - [commitgraph] Tweak `File::iter_base_graph_ids` implementation. (5b067808a793e3515c0c12cf95c11b57beaa8d09)
    - sketch out section and entries access (06679d9b69575183231ddb22edd89ab29357632d)
    - [commitgraph] Add `Graph::at` constructor. (a783052d0cc2d3c9fa1dda3ea77286a79690d2c1)
    - refactor (b5fa727403a78e5f9238dd36d8b071eec425d731)
    - [commitgraph] Validate trailer section when parsing files. (1b738ac0719ec20b24982d148a386d63ec4dc2d6)
    - Turn off 'unused' warnings for experimental git-config crate (0b52eb0e75a268c5c7b6475677fd20acace3435b)
    - [commitgraph] Use `thiserror` instead of `quick_error`. (c8b1f74328965708e38a689b865660ad36f22ecb)
    - Revert "remove git-config from workspace while it's so fresh…" (99214f4c1097fa8da8f14f1279caf00db78fa822)
    - [commitgraph] Stub out commit-graph-verify plumbing command. (aacf0f05a909e5b7d9ffd5623ef9833e0465be93)
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
</details>

### v0.1.1 (2020-09-14)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 10 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 201 commits contributed to the release over the course of 31 calendar days.
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
    - [clone] refactor (ded46fd5eafcb1fa1ef99dcbdd933ee8631ed7dc)
    - [clone] support for progress that can handle writing pack files (46e0055eab47e402807b15c63b6a4577f5c0b7bb)
    - [clone] leave aborting the negotiation loop in the hands of the delegate (ea83ce73b16b24409dec4009f09a0cbf203a89f7)
    - [clone] sideband-all support (ecc8e091fb97a5d44828cd56412358b7043e47ba)
    - [clone] Actually pass pack file to the delegate (94c5e62b274b0fc39f64ee5b04273db5ead4a470)
    - [clone] Response parsing up to (optional) pack (24064c77f2969380fb92ea66109df86e84060324)
    - [clone] FAIL: try to model pack reading using ownership… (4ee14e322d904cafa297ad989a0d653e7f8e5d2f)
    - [clone] properly handle V2 response parsing (0d7d768278234824e03c5e74dacaafca3ee65713)
    - refactor (f2c31ec4f245ce4e42e1371c4c9095fc4124cf16)
    - refactor (fab9f99a1f73378747b07f2f27f69492da899cba)
    - [clone] Provide a handle to the packfile, if it is present in the response (fcb4cc1b011edb2597686fcf24ad383819a52389)
    - [ref-ls] A way to abort on multiple delimiters; first tests work (8d44912e7215b85c6931b7b829bd73ac38584424)
    - refactor (feec5be335a99a4c47ba98f93803863044575838)
    - [ref-ls] Allow multiple delimiters at the same time (cfae63a5f7d2d99560dd857f7220980d70c4c4d8)
    - [ref-ls] basic V2 acknowledgement and packfile parsing, but… (549f404378535390195dea4d6c5b6485db34b81e)
    - thanks clippy (ac88eefd56095995841f60f0cfdca78295006584)
    - [ref-ls] parse all V1 acknowledgements, without duplication (f7c15809d74729b92e4c64a71543a4850765a8f8)
    - [ref-ls] first stab at V1 acknowledgement parsing (1d21cd4a59c28fe5c631a12a10f332f4cc8fd3f3)
    - [ref-ls] It would be practical to simply have access to the line provider… (5fba78796d3bcc16f812dc3202d521ee057e86f9)
    - thanks clippy (27f30df9a8046fe4e872837e36dd497096660282)
    - [ref-ls] support for line peeking in packet line readers (0c0c57522972f2a49ed5261474114da062e6ab15)
    - [ref-ls] Let's make Acks copy, because owned::Id is as well (1f9cc44275d226a7e80e24ed592f6d6bd98de31a)
    - refactor (935d5fea48b0d8710be822e2d64c77a7008143c4)
    - [ref-ls] first sketch of V1 tests for result parsing (ack + pack) (fd16a5f265764ae9f18b9b9fc0f713ccfaaf2944)
    - [ref-ls] tests for stateless V1/V2 (d34afc6fcbcdc175c09b12d6697b01611dcd02ed)
    - [ref-ls] first step towards parsing negotiation result (51ecf7e248724cd0b499e7a8662df4511f24d6ee)
    - refactor (61e98128ddd85cde1a352b70f83870fdea0c6bac)
    - thanks clippy (6b1294a7046af84a13e34c3c43f8ddd2b3b1cb97)
    - [ref-ls] Argument tests for fetches (50cd260866b7dbc44653d8c193e6517e770f44eb)
    - [ref-ls] first argument tests for clone (83490ef764c2625ac34e42c27de7364d5445cdd6)
    - [ref-ls] Also add 'haves' in V2; some more assertions (3e6bfb1d144f9de4d45502b8257ea0f278d49376)
    - [ref-ls] Do feature assertions to not have to support old servers (9980ff9e52b466a56418857ca15fbcdc0d17b6b8)
    - [ref-ls] don't do anything on drop (9f18d9b9062d61d6da6e2bb7564fe5edbb1528c4)
    - [ref-ls] A step towards getting the negotiation right, really need tests (abb56d855d49f232d25e0326fdef13732605df5b)
    - [ref-ls] Transport layer knows whether it's stateful or not (22c3640b70bb6925d72794eeaeda48b0687f2047)
    - [ref-ls] Also re-send V1 features in each request, independently of statefulness for now (f8669d60cb349b6217227eea0d76664e8da9a458)
    - [ref-ls] potentially fix 'is-done' logic (f9e338f244806aa9f0e24352912091cb7d8e0e80)
    - [ref-ls] Sketch of sending arguments in V1 & V2 (e1d27b6693adca053bfb42d841c03ef16a256d88)
    - [ref-ls] first step towards supporting negotiation (27b6d2d24a92c1ffc1579a116a044cece50d9d20)
    - [ref-ls] probably all it takes to handle all capabilities of fetch arguments (d956ecc7d66544157d9233c4803b27fdc3fee1c4)
    - [ref-ls] first sketch of argument utility to help creating wants/haves (b0b0166c8dcc1094d7294ddf63e20c0ced2c85e7)
    - [ref-ls] fix feature validation in V2 (eb387d24267d90e731b41897c7e4071131508ce2)
    - update tasks (079fc02608432fb6c5539759813e336c3c9f6c58)
    - [ref-ls] Always send a flush before closing the connection (918f19f0c2dc202ed2014e30b7247e63a0f6a51e)
    - [ref-ls] Make credentials helper truly work (7f3c3a71db7eeba1d37481ba1b522d5ded654237)
    - [ref-ls] And it even doesn't work if it is the very same transport (4ba50fe06f7423c31f4cd78079d51ef3ffd51920)
    - [clone] support automatic downgrade to protocol version 1 (4cf36436f11eb95d420c1147a1ec8adb618ea5fb)
    - [clone] basic progress for fetch in protocol (1925d020b1ab922465f9555515f691b06aaba46a)
    - refactor (aa7e8b1eeccaa1182cfdc668592f61d8b28867d7)
    - refactor (b97507ec5cd041d0433977d78006fc0d9a35e88e)
    - [clone] update README, improve delegate docs (dc7908f1546239ade71f4147a389a001769311f5)
    - [clone] test ls-remote V2 (09077710fb489b7a6dfa2bace4fda47609a97e78)
    - thanks clippy (baf0b2c253b005e64762226dcf628b401b1684d4)
    - [clone] more tests for fetch features and arguments (a9468614e1f40de4e7442b3915c6ce09d58f8c01)
    - [clone] features for V1 fetch (5b24a559dfb03c99ee360e9997650c443fd30077)
    - [clone] assert on ref-prefix for ls-refs command (70347a5406e66a77c490010cd695ceffd80fb7e2)
    - thanks clippy (d55cd56c6721ab591157f9add4ba44507373398c)
    - refactor (f02232d6698a217e7ff87164bda2869777c54e33)
    - [clone] Getting there with feature handling for ls-refs (27c5adca6c428343492de69bdf2e4bd3ac9c89f3)
    - [clone] Remove intermediary mutable Capabilities implementation (f59344a6e39dac579624fa2a9db64cb10afcdb75)
    - refactor (5ea42ba9eece2f7d9557456fe6adda5058d0ae1a)
    - [clone] first step towards better organizing features/capabilities/argument names (7d45f3abad100e8fe5691430ea3e3b95c7ae068a)
    - dependency update (dea002855ef949a58851b1a3f853a59c57e4d164)
    - [clone] first sign of somethign working: ls-remote (df58fa15bc01cb047115577da58fec867f118cf9)
    - refactor; thanks clippy (03c3d176fc4c534798df9a6faf80d0722dcf0b33)
    - refactor (25122f2acc95c363ee573fa875d8573ad0ee7586)
    - [clone] V2 ref parsing (455fa0f3a607cdbf24f0833e05a8a4e75ddca0c2)
    - [clone] A better way to set the agent in V2 invocations (325d3a26e45c78aa953400229d131f2119f06f75)
    - [clone] Make the actual ls-refs call (898cb8b0d672420536387926f8c6b26fba698b81)
    - [clone] sketch of delegating simple commands along with arg/feature verification (c2ebc4875587db0936648d59440e07cc941f9503)
    - refactor (a6bcdc42a82b63d544b6ca6fd32d123f5ea0f4ae)
    - ignore keep-alive packages in case of 'sideband-all' (2e77b862896c5070246184290c138a68cefbe313)
    - refactor (ad0b2e9df98ad8f5a687849af32cb4593be9ae53)
    - thanks clippy (8b1ea290f8f132e5a3b11828acfe4859c3d19bc1)
    - [clone] apply another mild workaround to be able to use 'transport.close()' (ea636aea6d4486edee79280c33770961a422e6bf)
    - [clone] remove workaround (55cf16744126137ee70b06513c2daba116645aa9)
    - [clone] more safety checks (6f5a9f370542fd1d79a318e57fba65263f05028b)
    - thanks clippy (423458e8013b69a901a127c954281b8cb323fb26)
    - refactor (f29ea65de4693a6096d979531add42d1e0f3d04f)
    - [clone] proper parsing of V1 refs (d26230727ef795a819852bc82d6c2e9956809d8c)
    - [clone] A little more ref V1 parsing (4bc78425aba304b4e4967fb7599460366322ef41)
    - [clone] preparation of test for proper ref parsing (V1) (85cd5806299a2fd92e786e242f946fe9e29853c1)
    - refactor (99247f46673ff6772796bf55662e920200ba0c38)
    - refactor (c9853702e4b63dc217e94a838de8c5ee5c877a4d)
    - [clone] symref parsing from capabilities (8c2ff640cce4f5f42a3424405efc15b18f4aa7f4)
    - [clone] A step closer to parsing symrefs correctly (250a34045c26ae0f5c2e06b1943479887edfe412)
    - [clone] attempt to make refs more accessible… (fa1112c69911b4cee8b2d768f907114b910832ac)
    - refactor (c138059434885536984996cd8fec002aba3d5fe1)
    - [clone] Prevent accidental leakage by transforming back to the 'right' type (2d469c66ec47be2e1bc3e0b1f3d17dfea5050970)
    - thanks clippy (9afa7f9c95635559426395f61f670dfcd6f6154d)
    - [clone] a better workaround for the 'drop scope' issue (3ccf32be15efea134bd72bbcc59c3f79252eeb3b)
    - [clone] First step of workarounding rusts drop rules (6b479239cd2a60ebfe7a4b11f9e2df0a8ea4a096)
    - [clone] update tracking ticket information (650c4520ffc12b3c3861d406a7b8ffa2df5b5c04)
    - [clone] add Rustc issue to see if this is just my bad (ccb9b53bfecd0e6adcccfd6dc155e8c3033cf16e)
    - thanks clippy (fd6f9e5c9c2ac8f68ab885d9bbf2d5f7a77a732a)
    - [clone] Workaround for the drop-issue (43c61597b8907eba572eecf39b90bdca438ef7c3)
    - [clone] first attempt at adding authentication logic, but… (a36d14a6b916f6aafc2c5757acda7c32415370c5)
    - [clone] first rough sketch of (mutable) capabailities in the protocol side (13f7ecbf493d4de633fd872f9b75292378449165)
    - refactor (a567b24cb9e040d92c49364e6c4e45ff77895629)
    - refactor (88ecda11dc1d97a7460a449350945dcac2f13752)
    - [clone] frame for first 'fetch' tests (2da70f688da95434e256ba1f355dbb809100604a)
    - refactor (89aabde074b26a3d36579227912eec0b74ca5a91)
    - refactor (51f6142913ce520329f9829976ee364e226a41a7)
    - [clone] support for git-credentials helper (a6546dab8d6d0dc4453052b77278cf5bb96aaade)
    - refactor (cf0e45a7f129e91d377d15558378724ac0c1aca8)
    - [clone] decoding of credential message replies (1c2f56d0fd10d3592d0a6de298360b136b34467a)
    - [clone] encode message for git credentials helper (143549e0757d4fa7a8347aa1b8b4734e9b62bf04)
    - [clone] sketch for identity handling (b23f47029fba50c7bba23a6ebe135e129ee9392a)
    - [clone] put remaining remote progress parsing code into protocol (e03e0e58191c71220ea1f8b9207bab96b3f9b303)
    - refactor - decouple protocol from packetline (dc98db28b77cc6a0bff2248167942224e58cdd2e)
    - [clone] move packet-line code into own crate (879af671fcde405d3d08ddbc07ea70d0bee23ef1)
    - [clone] move packet-lint into transport layer (c0dd8315089243164d82c444499a459756a0337b)
    - [clone] link up lean plumbing command with gitoxide-core: pack-receive (5ea49c8aa0d449bed98ce0147ad222ff25c27c32)
    - [url] basic frame and first failing test (60aacf0c279d277c4abf13e62697a51feeee26fd)
    - [protocol] properly implement remote progress reporting (a81954a6a37afacd51add6661a656b8fb663ca54)
    - refactor (66e9cd1fa1d17cfaac1235b573ba0230230e549c)
    - thanks clippy (7f6e29033ae05285afad846157f9c44b8c8710a5)
    - [protocol] prepare passing most of remote progress on to prodash… (b8a34e5cf26c469ff69f29fd5d02c61605887929)
    - refactor (df8ebdc443458fa95f9fc7fbb43ca2b6d874d972)
    - refactor (2ea3288e57ddd5204821fd6efee6cbb05231e311)
    - refactor (2102cabc9860900e2b5d9391cdfde6e59ad4a119)
    - [protocol] remote::Progress can now parse the usual progress (b0e5601ae2d96b96b267b36b68ff7426c75ee3a8)
    - [protocol] first steps towards parsing remote progress (c3d0e7a490cfa4d114bf8c13b5b3803eb6187290)
    - [protocol] even starting to parse remote progress by hand is painful… (d68db3ca8a187d6e9b7e341dae3058ea210197fd)
    - Less ambiguous name for 'index-from-pack': 'pack-index-from-data' (386673ccc99d18d023c7df3fcd40e86d71960b25)
    - [protocol] handle errors as well; transmit progress (first part) (c48439818dbde32007a4ec350bc0599c5cbb0cf2)
    - [protocol] first successful test with pack reading (ad1e8bf7668a935733b0ba6a0f1573de2250eced)
    - [protocol] first stab at decoding sidebands in Read (51fe5960a84e48e41544ee6d8523b7bb1e2c6a82)
    - [protocol] allow Reader delimiter to be configured (5a01596ba4c9fc50beaa99260ff2b263f64e99a0)
    - refactor (78f27d8bd0dada168bf2502937cc82ee9b6cfcfe)
    - Revert "[protocol] an alternative version with external buffer" (157d810e50f3cc8dd12586ccd128be1d7c8a331a)
    - Revert "[protocol] But external buffers also don't help at all" (579a697536ff7de9727f5a7e517b83a3feb75540)
    - [protocol] But external buffers also don't help at all (8e711df01b812aac9e4197a196582cad47ee6bbe)
    - [protocol] an alternative version with external buffer (a862d22aaadbd1f096400d4bcd06bc5c1ce17425)
    - [protocol] a struggle - putting buffers in Read adapters = bad idea (e257426f3583b079120ed75e0bda2f035e70d94b)
    - [protocol] FAIL: keep referenced PacketLine for minimal copy (7e4d1f304b6821118f38a6cdab599cc02e6e949c)
    - [protocol] sketch of Read impl for pack line iterator (fe3b050ca7218aa7b4adf99e702534f5a6eaa70c)
    - refactor (c81caa3d178671c447846f346d08b60f59b313c4)
    - Revert "[protocol] FAIL: attempt to add an actual Iterator impl for packet lines" (2989781250e85042a5e26632df4b3471abe8adee)
    - [protocol] FAIL: attempt to add an actual Iterator impl for packet lines (a6e4cb13be7a3157d08fb899a7b9137a4f81c5b7)
    - refactor (20b10c5a52ed408a4d45e1f361dfa6faeb952850)
    - [protocol] thanks clippy (10b9017f1ced471a612713ab364e7c702078e756)
    - [protocol] tests for the reader (86d1a40d735d88b4da4b654fa573e53c67c5f3c4)
    - [protocol] A chance for the reader to actually work (d6aebed49320fc52dd1f11a42ec6dc54b2de8824)
    - refactor (8ebdcbd7e6ae9ecb874dabf689c8a4f7a2bc4f67)
    - [protocol] FAIL: finally the reader compiles with the 'slice split technique'… (58543cb13d88201f27ba015786d4916ee854ce67)
    - [protocol] FAIL3: giving up - it's quite impossible to do that without 'bytes' (047d67c9ed4d329718494076f1b741da16343906)
    - [protocol] reader FAIL: wherever the loop moves, it will not borrowcheck (cb154f25d0ca6431ea3be278b573d80fa43fc66d)
    - [protocol] FAIL2: lifetime issues with loop (c2ff0700a2ea7088cdfd1c66d140bc393b7a85ce)
    - [protocol] decode-band can fail on malformed input (0f468f983efe082900689b900a10ae81ffab0157)
    - refactor (ed1f3649a89cdb224efa0ce62a63372fd973cc3b)
    - [protocol] better handling of text-lines (7ad1db0cc1efd486b4ce9ecfef6f6a763f8d6aac)
    - [protocol] attempt to implement a streaming pack line reader (FAIL :D) (cc45cec34c43e93348fed7149c4ad5abd81dd775)
    - [protocol] add cargo-diet assertions (831b7587828b819844341ff451baf54694e7641c)
    - refactor (73e24c9c1966206125bea0bfa627b50ef339ce11)
    - [protocol] side-band channel encoding and decoding (9b4fb3eeecc7c383c7c9b9d890e7adf771ddc80a)
    - [protocol] suppot for V2 special lines (4e467194d19c2804b49f5f1c445f62a5d2dc7c44)
    - Encode and decode errors (3f4fd90333f80fc4a6b395dfb476d4ae0be921c7)
    - decode ERR lines as actual errors (1f58568f670b8b3dfc996b6e7dbd2d5ef59f0f28)
    - more tests (c34d88b18a23ee499b0df8e499bd772d41a9b8e1)
    - the first succeeding tests for streaming decoding :D (7ea25c5e94967d4480dd81bb2f3e4ad18a9d226e)
    - first stab at implementing streaming decoding of packet line… (843c6fb51e001fe9384e0f1c2cde8ec906250ee5)
    - cargo fmt (60cd21b7a2df78dbf57efbb51ab6e7a507b4f187)
    - Allow dual-licensing with Apache 2.0 (ea353eb02fd4f75508600cc5676107bc7e627f1e)
    - refactor (7e3f67dbb8bd17cc2ee0888db08c716d7c81539a)
    - packet line encoding with flush support (e924a595a4d9c9bd8647a72fd728f1bcb3f0db1a)
</details>

### v0.1.0 (2020-09-12)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 60 commits contributed to the release.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 (2272fa4bcacdaf1898e4cd8b791232fc1321227f)
    - (cargo-release) version 0.4.3 (5b47a1a051243ec2aa407297a19d41b30447bfab)
    - (cargo-release) version 0.4.0 (0d7b60e856325009431172e1df742a1cd2165575)
    - refactor (8930610c3ad73d2c1294880c3081f0662525f339)
    - Enforce using the correct version of clap (fd6457f3a006873506543846d9400b4a66833e48)
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

### v0.0.0 (2020-08-13)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - some README updates (14615143dc170217ca4acc80191f4e6725dc460a)
    - first bunch of tasks I see after studying parts of the protocol docs (9bd97bafd299efefd063dde73cef53fde9d36670)
</details>

