# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 13 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 149 commits contributed to the release over the course of 10 calendar days.
 - 25 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 8 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#164**
    - remove old and unnecessary experiment (aba8e5603833c85302db0b610802286a03a084df)
    - path::is (1f4e45a26a3d2727f00c3f248452dd41fc8a95be)
    - rename path::is_git to path::is (ac3b9efb7b90958274ce55800959d930f8641115)
    - path::discover (1958e8aa65eb97f9755f065d713f0a48c5e41b1b)
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
    - Make use of fixed git-conventional (b7b92b6c72051d462ab01c7645ea09d7d21cb918)
    - prepare test for basic merging… (0a14cedbd68058ac296e34a84ab1fe1083a0bf5e)
    - update git-conventional dependency (2d369e863b15269ba8714b025fe596f69e5b1217)
    - nicer 'thanks clippy' message (43442162aa22f561a33cab78936514d05d8214a0)
    - first test and sketch for stripping of additional title values (55b7fe8c9391e3a9562e084ae7524bb9f83ec36c)
    - Basic message parsing, either conventional or not, without additions (b3b6a2dc07c2eff38556ee66b9290b0c66b463ed)
    - Show with simple example how the round-tripping works, neat (9510d9bd2c3b2d5cffe32485d7bc3fff374343ee)
    - Sketch Message fields from which change logs can be built (b167d39ecf0cd306dcf4d2c00413251cbfd02ed6)
    - collect unknown text so things don't get lost entirely… (60040c9301e6468c72a0c52095c0b86f8b3041f5)
    - feat: `BodyRef::without_trailer()` for more obvious access than `*body` or `body.as_ref()` (f0ea526775793c9104e4ae27dd5d92b5a1202c5f)
    - parse back what we write out, perfectly… (5cab315b0f28d9b9f6f6b4e037d053fb501fdfaa)
    - refactor (ef3fc6d92c1d751d0032e072834f41d37cbb9200)
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
    - thanks clippy (d78d3828c7f80963c0b8803cb64e0ae5e08d0ba3)
    - Bump git-traverse v0.9.0, safety bump 8 crates (d39fabb8757369aa19452a457f610fe21dc13a14)
    - thanks clippy (1dece2b8dd18d0266210152c749c39595d70db5a)
    - thanks clippy (2b5542761ab160cd9460b133928efd6f0cb55e75)
    - thanks clippy (4ea11264296063278977c5539e2d68367475464a)
    - thanks clippy (a89d08c4ce28f0c466f01758e9f4db09eeb02458)
    - thanks clippy (a554b9d356d4e44c9504f7b35aa2c4f9c660df9b)
    - Bump git-repository v0.10.0 (5a10dde1bcbc03157f3ba45104638a8b5b296cb9)
    - thanks clippy (d15fded08224c45dcbd34cf742398e2594f39964)
    - thanks clippy (e56af5a0846652f177a88771d495bff5973abde3)
</details>

## v0.14.0 (2021-09-08)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-object v0.14.0 (fd79b2ee7e48178f992298b9bf950a365dc2f320)
    - [object #164] refactor (883343bbbae431cfb8ffb16f0d39838b0d7636d7)
    - Bump git-object v0.14.0 (d4fc81f6390443f8c8561d91ac27ea4a6318fb62)
    - [repository #164] Prepare `commit()` for a possible less-allocating future (0fd01f7071c785c27c56d2c034aac8dcdf690677)
    - [repository #164] generic write_object() (c569f83363489dde03c8b9cd01e75d35f5e04dbc)
    - thanks clippy (33a8fb34708407fd6b9a9ddabeaab51409aa1b03)
    - [object #164] Allow referenced objects to be serialized as well (a98d2985dae2259d72bb91a01548906862fee9f7)
</details>

## v0.13.1 (2021-09-07)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 94 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-object v0.13.1 (2c55ea759caa1d317f008966ae388b3cf0ce0f6d)
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
    - Release git-pack v0.9.0 (355d6c495d9fcf10f5a17572847acde3cbdd8094)
    - [repository #190] refactor (e751688a5378552b73cfddd07f38a0d0bb491b83)
    - Release git-traverse v0.8.0 (40c8506f289d5b8247dd7081b27614527a784757)
    - [ref #190] refactor (49fe1dc37c040206839c9d4399001ff12dc91174)
    - Release git-features v0.16.3 (342475f7c8ec0382432a411f15fc5dd7eadb1abb)
    - thanks clippy (023dedc41aa859cd49d208392a586deaf77bd1bd)
    - Release git-diff v0.9.0 (021318f8c176f4028b76acdcdfea8d544abe727e)
    - [ref #190] reverse reflog ergonomics (2de86f904f6ee63e292f9c701cc3524e8bfe87e4)
</details>

## v0.13.0 (2021-08-27)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 223 commits contributed to the release over the course of 8 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#163**
    - Adjust collaboration guidelines to allow and even require PRs (998ae6bf214d576cbf3f5b53f8d75e908ec63474)
 * **Uncategorized**
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
    - [smart-release #162] unify 'ext' visibility (ca082a75ff29de2a471cec4331a80f84477cca56)
    - [repository #165] refactor (0f13104375216ccf099ebc2fcf0d180ed0de5237)
    - thanks clippy (1f2d4584f8b650f7e751c8d2df9a5d27725f4f2f)
    - [repository #165] An experiment on transforming panics into errors… (1f5222660970e24eb2d82fed3917f234dce7e0eb)
    - [smart-release #162] a sketch for accessing objects data… (ba27101e08b2bab5d33b53fedcc0c6aa13b8f35e)
    - [repository #165] offer panicking type conversions for objects (f802f8c8c382f8063fa615fda022857a740a974a)
    - [smart-release #162] refactor (7f2421bddf7510d1cd6a12fa1457e3e842b38879)
    - [repository #165] try a more common naming convention for fallbile things… (fc703937a078937840ea1c254f11e64aaf31de90)
    - [smart-release #162] peeling objects to a certain target kind… (57851361f3fc729b964fd0ca5dca9f084fe20f5e)
    - [repository #165] refactor (6207735f7d955e8a1676c8ad549ce6c1137da760)
    - [smart-release #162] a single import path for ReferenceExt (7060797031e5bdbb8d635cc2da3269996bdfc4cc)
    - [repository #162] update crate status to reflect now 'easy' mode (6d00139ff76a39a066819b4310f920ecff525142)
</details>

## v0.12.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-object v0.12.2 (6e58edde2d7c881a6b957b7efafb63e2f517c9b4)
    - [object] argh, remove these tests for now no time for this (13d627d19aae3bb9d44ed0e9304e373e90f51547)
    - [object] simply exclude the feature from testing for now… (adba3b982c4b21889615afafcfcaa7ae1f91661d)
    - [object] fix magically smaller object size expectation (bf4d2d7c0a33a3f35646f776edce4b829f086f66)
    - Apply nightly rustfmt rules. (5e0edbadb39673d4de640f112fa306349fb11814)
</details>

## v0.12.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-object v0.12.1 (086baa261d0874e541e374d51d427727aa37a8ee)
    - remove dev-dependency cycles by removing their version (c40faca41632cd2a226daf4ddf5293b65d1fdc82)
</details>

## v0.12.0 (2021-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-object v0.12.0 (9c7a99a5a118cca26f1a07c2c650420b6983d5d3)
    - Release git-object v0.12.0 (7006150ac314d19814608723f69f6e70a72f9262)
    - Release git-actor-0.3.1 (727087dca243da4bc40bc87611a2f66234565be7)
</details>

## v0.11.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.11.0 (a5be31c4cf7c0b538a1ed4a52ff5c3a992c6feff)
    - (cargo-release) version 0.5.0 (bf15c2a2f285046b094093760c1969007ee75e25)
    - (cargo-release) version 0.3.0 (64efc0534ddc372b6e668b23c1e9d276098679c9)
    - (cargo-release) version 0.4.0 (70ef3442775b54ba9e4ee9ebfffb37af9804cc5b)
</details>

## v0.10.0 (2021-08-10)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 49 commits contributed to the release over the course of 82 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - fix release order to match actual dependencies (65ff8c1c106182820dc6e4a308f71708e657f07f)
    - (cargo-release) version 0.5.0 (ae02dabae961089a92a21e6a60a7006de4b56dad)
    - (cargo-release) version 0.4.0 (0d5c8b96dfdfb96e4fc82623f756f6c7f7046e90)
    - (cargo-release) version 0.2.0 (8ff511583e6d859e43ffda0ef75e2fecce3ed03c)
    - clippy on tests and thanks clippy (a77a71cf02d328a2a964388928d6b2a235a0aa85)
    - thanks clippy (e1964e43979b3e32a5d4bfbe377a842d2c0b10ea)
    - [ref] fix build (bad find&replace) (467395f19ce13ff8cd62499573d3cd4cb2e7797f)
    - [ref] refactor (e26c72fb1bf9392932ffe42843f3dec52c7bbd7d)
    - [ref] basic lookup rule impl; needs more test cases (3226f775129231b4bc4735baf9e14a187665ace3)
    - Remove unnecessary unsafe code (83e207a44aece0ff4870e57990bd5aaf43f38e22)
    - [ref] fix compile warning on windows (c32877415aba8df6d5a37cfd799b218e3a29b18a)
    - Merge branch 'parser-into-iter-without-alloc' (a799ca8d6c2e51303512160ddef7477e176ab01b)
    - [ref] a test case specifically for lookup rules (ab3a34f481ebe335578e3a7dbff325087b4ba647)
    - Implement Parser::into_iter without extra allocation (aa79924b36c0d717cc65d7471fedd27eb41e83a5)
    - dependency update (059fa3318e3e76c407e456d28a28cb834d532719)
    - [ref] improve parse failure handling in packed-ref lookup (ba62aab4308d44092d151d11d9be44ba6bfddb02)
    - Remove unnecessary pub(crate) exports (3d2456e11709f0461b37c6df55ecc3861ca4cab5)
    - [ref] refactor (207a799c1fcf490425f2e5dcf8274da83125af6f)
    - [ref] flexible and simple support for different hash lengths (9c2edd537fb86d2d7db874ec976d0cb1b8ec7c2e)
    - thanks clippy (c43730496ac5e1f7e66ee226099a4e99e151e97d)
    - [object] Add feature toggle for verbose errors… (4b63d8a8709a2674d287879c4d6538a74cd7869b)
    - [object] support for verbose errors for object parsing (8156f1037b87424864db73888108be12dedb5169)
    - [object] refactor (6f639835362224bade27dd4f45c275544a39625d)
    - [object] Generalize nom error handling and use nom-errors instead of custom ones (47c8a97194c9e401ee311234a269f8b8f3650ba1)
    - [object] remove unused dependencies (2f01e46a9b30f1231adf1e79c5923843e63cad86)
    - [object] cleanup parsing error handling by removing NomDetail (e91cb405381d84bc1021c3ac30dfe6061788f9b1)
    - [object] refactor (1ddb5c07b75aa2b9a9536125fbba1fc862b7fe34)
    - [object] replace custom context impl with the one by nom (9a6692d034976dbcf011b587140c7f47fbc583e2)
    - [object] refactor (8205429b2ac160525a7449e50edf04aaf027f12c)
    - [actor] git-object uses git-actor (d01dd2f9e9e8e2b81cdb1131a436d32b5819b731)
    - [actor] make signature parsing public, exposing nom :/ (a627972ecc53d38210c826f851ea9c5fec17b9cb)
    - [refs] try to get structure in place for reflog parsing (727c66a2560c00cc8e01fbe47503ffbb67147c59)
    - thanks clippy (6200ed9ac5609c74de4254ab663c19cfe3591402)
    - (cargo-release) version 0.3.0 (87db688f23475d7232731429d770848aea228492)
    - (cargo-release) version 0.3.0 (6b33678f83e6d261ca15c4a7634ff5b4e66d81dd)
    - (cargo-release) version 0.2.0 (3286e42547b59df6365087cbae9ce1c9c959faad)
    - (cargo-release) version 0.4.0 (866f86f59e66652968dcafc1a57912f9849cb21d)
    - (cargo-release) version 0.2.0 (132789475400abe660b30ef6d2c5ff57821dd2c4)
    - [git-object] use git-validate crate (4ba98e824417d1c58998fabee88549700a714bcf)
    - [git-object] refactor (d64d3266167ee224b651cc24c4dbc8d88c9c9876)
    - [git-ref] the first failing test (7e802a0576230dfc666c253d484ea255f265f92f)
    - Switch to latest nom (859e57eae93c3490523b7ed98f7a606acbd87a2f)
    - [git-ref] clear it out and move existing functionality to git-object (fa548ce94db3dd3969add494756fcc34e48985a3)
    - (cargo-release) version 0.5.0 (b6b58560b7c3bc88e2b8b780be5ceb4cb508a346)
    - [pack-gen] refactor (61554e2effcbafef9cff0b407351c2fae0d2916c)
    - [pack-gen] tag support for tree traversal (28ed260a73554d261c9b00c8ae9a46e66f123e6f)
    - (cargo-release) version 0.10.0 (5d7ee6a105abbb6efeed8624bade936bb59dbc55)
    - [pack-gen] more tests for Tag iterator (b69d6d6590ea9d8de4a50e31ab9f331a2e21faab)
    - [pack-gen] the first green test for Tag iterators (df5ef8a53cb4007058137890b414af510025fccf)
</details>

## v0.9.0 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 8 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#79**
    - refactor; add test for empty tree iteration (634029682da374f912068f5c8d5ec79d4837f7ea)
 * **Uncategorized**
    - [release-automation] no confirm mode for publishes (447188806f6390f5dd005b02dbd3c6970fd2d99d)
    - (cargo-release) version 0.9.0 (84897fd8e6e1b0269da0303d6a0de8f9e0eb58e5)
    - Merge branch 'patch-1' (5edc0762524112bb6716b3afcf23b2a4a0f5efd3)
    - refactor (a9e4feb0a81894568be730603446e2d061dd558f)
    - Allow empty trees when parsing them at once, fixes #79 (d34fd19db5b3802ae9c677a6cf481f42e8a7e073)
    - Fix formatting (a341995e6014b6ed0e43ae94fa1152aed6fcfd89)
    - Remove almost all unsafe code from Tree. (42b6033f3c367ccce37c82356183d165d37ae881)
    - refactor (9870923ce02d20beb98be5e4bb76ff8081804054)
    - [hours-demo] computation seems to work better now (26ecca2133af287ddc9146fb3a3fc73dddc945e9)
    - refactor (2d00c4ed6be5baa1c3389a61102e25eb7d543465)
    - [hours-demo] Maybe the pinnacle of performance… (f70c61ab56b4153030d524da69a514a667c6abb7)
    - remove debug-assert which doesn't hold - it's OK to have empty commit messages (13abc2d70c9aa42fb76e71d44c9c711e2780a5ba)
    - And it's a wrap for git-diff docs for now (9e09dd560a23d52d0469ce4fc13de01f7efce227)
    - [traversal] first impl based on git-odb::traver (76a3017b60d41957f5fea56bf7b2b2bf41aae0d5)
    - a new crate: git-traverse (1a9af50f1fca0e7e939f339b885c66dcb95e44e5)
</details>

## v0.8.0 (2021-04-30)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 33 commits contributed to the release over the course of 11 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.8.0 (a1ce210003ff07bf11291018bb182cbc7913647b)
    - (cargo-release) version 0.3.0 (e9665c784ae7e5cdaf662151395ee2355e9b57b6)
    - [traversal] add CommitIter::tree_id() convenience method (6affd9d90d56d89774fcd4843638309a198815bf)
    - [traversal] trying to get things done with gitoxide shows some teeth… (3fee661af8d67e277e8657606383a670f17e7825)
    - refactor; better iter error handling tests (9fe139b85c350c8cbb78975a94f4548130764b1c)
    - [tree-diff] more tests for the tree iterator (91b5a029337200a2873a21696020dcda08e335cb)
    - test error handling of commit iteration (fcec4b43f7c1d72680431ec3522b0db94728507f)
    - thanks clippy (41418ede15be9d0d18e49c34e4482c5701851404)
    - fix serde support for commit iter token (3bfcb49814ed5f14dd66206a54a9b85f13edd9d9)
    - [tree-diff] all the tests for commit iter (7ebea87b91c7cae3378fa3a5780d6c58e319c006)
    - [tree-diff] more tests (4f81450b13bfc14cede1bec3234d33ec0844ac3d)
    - [tree-diff] And there is a working commit iterator, needs more tests (d99184782e9e79b517b7703ab41fefdc2424994e)
    - [tree-diff] A complete nearly working impl of a Commit iterator (4711821dd54193737cff76ce904b18d29b518ac2)
    - Frame for Commit iterator (796b74a09cf1b4c8c4694d877a76da94d425bdc0)
    - first failing test for commit iterator; store two parents without alloc (8337514378148d72dc7f6d7474d6d0b794759589)
    - [tree-diff] one more test green + refactor (bc5549db2ad16222761219d8652caf64867a889f)
    - [tree-diff] refactor into iterator based model (29b527aaea101c9b4e885db1c6d3533ef2310c54)
    - [tree-diff] The least intrusive way to allow dealing with tree iterators (d41dd3c38ee34b250a4f5de120d7ae3e04e3212d)
    - [tree-diff] prototype an immutable tree iterator to avoid entry allocs (f38e5cdcd072873707a21f0b73c491ef1f1c7a8f)
    - [tree-diff] A step closer to handling additions in a directory (a11f210bec2c6c55bcf8cebe00e116e835306360)
    - refactor (a4d5f99c8dc99bf814790928a3bf9649cd99486b)
    - refactor (633cba7c1ff1f63c32613bedf963d1bd89afaee1)
    - First sketch of diff API (fc3f2b7066538e31f8d4bb1053d70dcabd5fbab1)
    - Better ergonomics for accessing decoded objects (ae3eab6d6e4b96e207372fa8cb82f5ac9833e5e4)
    - thanks clippy (829554805520170f69cadc61e6be5e7255737cff)
    - refactor (9d03843eeae388738d70b4251166081542893749)
    - fix debug assert, thanks gitpython (fe954b9f6d26bd8629f24a01bd2a06f9800deed0)
    - More explicit expectations towards entries in mutable Trees (d94f84ceac637d2b6495be01dfc8eeb2494922f2)
    - refactor (f19ea33709f7c31873e46d896ed7b6d06607f1e7)
    - An even better name for decode errors (f270850ff92eab15258023b8e59346ec200303bd)
    - Make clear it's a decode error we are using there (f45cb4b62122559e5701923e0a23dd5791ee2ced)
    - rename git-object::(owned->mutable)|(borrowed|immutable) #(67) (91ee55893bf4b27a47d86d51bae6f99b59b69147)
    - The first basic traversal utility #(67) (ea6610b9157d8d0dabd2ddd07c45dc6651b9cf84)
</details>

## v0.7.0 (2021-04-08)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 112 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#63**
    - Use new `oid` where possible in git-odb (68a709e0337d4969138d30a5c25d60b7dbe51a73)
    - refactor; better errors for invalid hash sizes (be84b36129694a2e89d1b81d932f2eba23aedf54)
    - Make ObjectId/oid happen! (ca78d15373ec988d909be8f240baefe75555e077)
    - Remove all public exports of git-hash types in git-object (accf89d25560e5ded6f44a1c4a898ee65d14f8f6)
    - Remove re-export of git_object::borrowed::Id (a3f28169c1268c1129852f279631d5a7f7540cdf)
    - Move git-hash::owned::Id into git-hash::Id (fdbe704b6c9ace2b8f629f681a0580b24749a238)
    - Rename `git_hash::*::Digest` to `Id` (188d90ad463d342d715af701b03f0ed392c977fc)
 * **Uncategorized**
    - (cargo-release) version 0.7.0 (b900914a00292217ba7b9bcac260591800395287)
    - (cargo-release) version 0.2.0 (4ec09f4d2239ea1d44f7145027e64191bf2c158c)
    - thanks clippy (cefbf3e02edebd1875cd2762eb231e6c379b1ebb)
    - upgrade depdendencies (e4a77112ee4f5d0ab61d0678aab8ee090335740c)
    - improved high-level docs for git-object (60036f20328600f0faaaf21ca30f1b9d0275d15f)
    - Add missing '.' at end of doc comments (71368544f97369a4d371d43513607c4805bd0fd0)
</details>

## v0.6.0 (2020-12-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.1.1 (4224c5b5ceeb6bd1dbe4aac46018be5cc82b77df)
    - All crates use git-hash::Kind and its types, sometimes through git-object (124c171aaf546d8977e9913ff84e65383a80ee98)
    - first round of git-object doc proof reading (524ce51eb3e606b1225a23fce62df2199799d4f3)
</details>

## v0.5.0 (2020-12-15)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 60 commits contributed to the release over the course of 84 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (fc7d600ac2c438c8b6b91f67cb69b0ac5ec37675)
    - `deny(missing_docs)` for git-object (8525684c6c69677f3e1b40a3673a817e111e9bff)
    - more docs for owned git-object (b79101d714f59a42a30eb47776486a212ec0f738)
    - a few more comments in git-object (171d269e428f711b163f6644ebf0c44c1279d497)
    - thanks clippy (ba9b3c2345887353e02fc081be80733f1c5e22d9)
    - refactor (d5d7cf9d3f42d83652a7b81bc6e1ee6731396d6b)
    - more git-object docs (ba595f6d4864eafc64f31460f7192cb48abd408a)
    - more docs of git-object::owned (0620dce7a3ac368354c73e3927eb96a6e4990f0c)
    - docs for git-object::borrowed (68e524d079fe9042ebba1e33457f934a64018623)
    - docs for git-object::borrowed::commit (c5c1df031aa391e0e65d0540f8414cbd1d1aa39f)
    - Merge branch 'commit-graph' into main (9cb09b248796f0ff5c9d3f3e857de4731324cfd5)
    - the daily commit (single handedly) (b528c2e1bf0a3211491535427c4bd36212711a0f)
    - Note about why git_features::hash::bytes_of_file() is not yet used (ca48fc4f7c00215acf95370fe894a6e585c18c13)
    - dependency update (988f90595b4f50354c636328d0c8556cf9964601)
    - specify the hash to create with 'hash::bytes_of_file' (c000294423ae0759b978399db3b69ac07c20578d)
    - document `loose::Object` entirely (d5eef9cdd06910eeaf1f1c4114b97638a29c7327)
    - move 'git_odb::hash::bytes_of_file' into git_features::hash (c5f6b4587ee4042a080c0505613b0c72fdfe5273)
    - thanks clippy (b9e0a87996b8f3c4531a392607c353a1f0824ce6)
    - Add and use borrowed::Id::null_sha1() (c717492d0038f55a6f21b48937b56a756890d214)
    - Updated `expect` message (e8d8d9351168b5423c44626ed8ac81cf7b013a02)
    - Update error message for type name (92cbb1314657abaad560d068e7395a70769f0592)
    - Document borrowed odb objects (7626f7f3af885f1b95801f9dbc71bee0bc77400e)
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
    - refactor (ba1d88364424eb60a0874a5726b62740dc348592)
    - take not of a few more obscure features (8f9570c602503f8689240a790766712dc1c4ec71)
    - (cargo-release) version 0.4.0 (2272fa4bcacdaf1898e4cd8b791232fc1321227f)
    - refactor (7c3c80acf487296014ae9f2f9b88865c6aa6d98e)
    - (cargo-release) version 0.4.3 (5b47a1a051243ec2aa407297a19d41b30447bfab)
</details>

## v0.4.0 (2020-09-12)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 97 commits contributed to the release over the course of 29 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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
    - [clone] proper parsing of V1 refs (d26230727ef795a819852bc82d6c2e9956809d8c)
    - [clone] Don't expose hex-error in public interfaces anymore (92dab3033890fe26fe2b901d87abe16abd065cce)
    - Allow dual-licensing with Apache 2.0 (ea353eb02fd4f75508600cc5676107bc7e627f1e)
    - refactor (a0bebd17500bccc08ed5a1c16e2ffcde89c71052)
</details>

## v0.3.0 (2020-08-12)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 72 commits contributed to the release over the course of 31 calendar days.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - update release script to match dependency order (e8df6c1ffb7afa27aff9abbe11c7e4b80d19b61e)
    - bump minor version to 0.3 (4351e2871c9dcf342b8471fffa74cae338a53269)
    - update to quick-error 2.0 (4b1b7849b47a54092b49821c39e864c86adda979)
    - thanks clippy (62d2ff383c5f7fe884057c70868569a811a73e00)
    - organize object type comparisons by probability… (19a5d9465f7962cfcc39ea31a2c84be6235e40ed)
    - don't cause re-allocs of the compression buffer (2bb6fd26235825484a8f60a49455fee71f08236c)
    - Reduce memory consumption (6d1a7a1292e8065d0a777cb6acd34776b1e23696)
    - Also read the pack trailer during iteration (98a8e17e791b6bcd92149d7ff68cbc9d9ceee087)
    - refactor; better tests (12d14bfe2aa089723a395287c5100aad6e838935)
    - first step towards putting the index file into position (d994c74d7cd9c9c004bf27f0b2ac23558ce9c50d)
    - Improve looks of documentation (11a32ebc2209d1a05eb4c4ec5131e85dfb43d9f6)
    - Finish Sink implementation (84f7908b1883ed6c484ca4e522ac530c8cc161d5)
    - Introduce hash kind, as this should be specified when writing an object (f5d0acf61ac5dd815bc5ece4462eb9a43dd9c44a)
    - (cargo-release) version 0.2.0 (76fe0ab5f0b58504a5ea5adb74b349b9d588e51e)
    - (cargo-release) version 0.2.0 (d350a13784685ea82b84646b18736986aeb68146)
    - beautifully implement shared extra-header access (920d1accc92d67019f0e654f8c4ab5c95d798925)
    - roundtrip Rust repo in stress test; accept more diverse trees when parsing (0347cdbf473d80c016745ffbaf582832fe2eba2a)
    - Make sure we write out trailing newlines properly in multi-line headers! (7f044c36279aadfd7a2ebeecedd7f2c20b2b7b52)
    - Consume PGP signature in tags fully (ffd6c31aa3adecc2dea6357373d88a495d63ba0d)
    - Support for very special tree entry mode… (2be2c9d31563b147f0f2a5c1cd03279c79f1dd6b)
    - make tagger signature optional (3358f9ae539c7b7878d87a209d678d2f08f94b1b)
    - remove now unused pgp_signature field - it's in extra-headers (c8c937c505e455572544a1a9da1b991ef4662b97)
    - proper support for extra-headers (d0feb2b5b30f9719bf3b40ac5b74f8a5a8515bc9)
    - Abiility to read mergetags (for now only these) as extra-headers (bd3a2db1068ce7509612ef1be0a108b7bb45ed49)
    - Switch to latest quick-error (976085614ee13a19fc1347209259a3dcf36ef95b)
    - Fully implement --encode and --re-encode flags (a7cfac83ddd859d9c2c25e457c0d7043738792dc)
    - empty trees are allowed, and they are special, too (6bed200ec1a528574edabf5783e9ebfb00add56d)
    - refactor (56b66ac069f24635a8fa74b4b2231dfb0a82a1ef)
    - Basic top-level object round-tripping (e851cbe585525b3e50114eb8d2a0662149bf2019)
    - refactor (ec5e50f607d59302d6db3944f6ea7b667f820927)
    - implement blob (f30caf4ff69fee36e65a2e404910b88e06d539bc)
    - refactor (335e98ab3a2e9c05141f1cd218197079bb51cfa5)
    - tree roundtrip (8b26a0e16c838270290cb3ac02b029100afe6b46)
    - prepare for writing out owned trees (2b6eced325057a884d56ed9db755a8699cbf8d00)
    - manual deserialize implementation, for now (9f46efd625d45a9ad947e9f7bc6f31f4426f3cfc)
    - Use borrowed::Id in trees for full type safety (5d57c1f7e3b9a84f7b46a4378015572155f3104b)
    - refactor (f7b8826ba144f54f3a3fe6096a5daafd29e25002)
    - commit round-tripping works with multi-line signatures (b692b0aa3f38507697096e671c990700d25933c8)
    - first attempts to roundtrip signatures shows I parse it wrongly :D (1b48367d02fde977ed4acab63e295c0c5feec357)
    - Prepare for allowing an owned, processed version of multi-line headers (f966e7f26cbbe99e5508215adaacf073e108bf48)
    - first attempt to round-trip multi-line headers (645ef946653caf2eed571d83c61e8b7d7c1cf4b4)
    - single-line header support (478c09e54cc73dcc5cce7aea6bc0702882c5f882)
    - The first basic version of commit serialization (5319f64036e09bce97285db19f30f988a5039761)
    - make reusing round-trip code easier (3b9d66c932075feb08cdf2967f7698daef9fd3ff)
    - refactor (987787e3084bbfc948ed3ca464909a2912f7b653)
    - Fix tests on windows, by ignoring them (512ed6c915b3db2cd3353ea23b945652ad1bef50)
    - Use borrowed::Id everywhere (9f876f04feaa3fd3bba9729fff7667708dc0c4be)
    - move git_object::Id into git_object::owned::Id - much better already! (50c71368a69f57b0a43061df105685e992ed384a)
    - basic integration of borrowed Id; translate between owned and borrowed (84ff638a183567593ace8056de2a856304d29d1d)
    - prepare to allow Id be owned and borrwed; abstract over hash type (d883c31dd14f253a3af153616007c9231fdf265a)
    - introduce the notion of IdRef (700736197b903cb6fe9ed60718e49e4be44199a7)
    - Use statically known borrowed arrays for perfect type safety! (3ead048bb999e6266831df2ca6c2022013529ab2)
    - refactor (766f3e491dc6ebcca20753cda3487545268721eb)
    - tags can write signatures (a48275e65bee7f544c19bc81307660ed4f60b8fa)
    - tags can write a message properly (b590b779a6f168db377bf5b4b796a4813bd19ccb)
    - green tests as basic tags can now be serialied (62a02b490055d9b95a5aae3cbe1641f42ff69df8)
    - more tests for signature serialization (5000f30bd0085c0afacf2c32d8a31224ec7337d0)
    - time serialization (1eb1e36992f9973977b4d94d55348b7a3eecd3fb)
    - prepare writing of time as part of signature (f560bc50a2a2e4c9697c17b59ec5cf4122992fab)
    - add new 'git-ref' crate; place ref name validation code there (1a0e84e627b17be1b1fb53b4dc98ab78e9cfb9a7)
    - refactor (b4392e880ed67464af9e8cfa2e343d10f23a949f)
    - some more boilerplate to actually implement complete ref name checking (087857a56654537fdfb5bfa6c66745184027517f)
    - very basic first steps of validated serialization (d3fd5ffe10015e2a13200a1fef5bd903532f81af)
    - it's probably OK to consume the borrowed objects when converting them to owned (101ddd586d4250aa5b3c8c6f8076456ae997faec)
    - try basics of roundtrip without consuming the source object (581794efcf4577c21f2ff078ba7844a71b47c0aa)
    - refactor (bca1f16a6f3da497e3488e333d5ebc99e39ee689)
    - first sketch of owned Tag in preparation for round-tripping (fa2745a5d5f7b6c4e02177e4080db7df6603b9fc)
    - refactor (90ae25d39aa4540fc2785eb7cb189eee102895c0)
    - refactor (256581bad6692a458b331b712d16ce2d5143cb75)
    - 'data -> 'a as it's shorter and also more idiomatic (71821e938887f448f1458642eda2bac365f2aa85)
    - refactor (dedd4dc91c26dfef368307345bb9e8d49637207c)
    - apply cargo-diet (better late than never :D) (295fc81a2e0e31d6d83eba7e169dc4ed05038083)
</details>

## v0.1.0 (2020-07-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 82 commits contributed to the release over the course of 26 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Make crates publishable (5688a3427ff3673e1422d43106f4d685fa837aed)
    - Handle windows newlines in test suite for packs as well. (ebd517633f099582dc2633e71f7bb7890acd14d1)
    - Fixup text file tests on windows (22880881d5e442acdeb8dd0cfb5ecc4f62783951)
    - Add metadata to allow docs.rs build all featueres (10f9386a12decc1f13999aee72be484c8f6d48ce)
    - git-odb with serde support (0da930cf23f215cc1e2bda8f7340a5d69370735a)
    - cut back on onnecessary annnotations: serde(borrow) (759915c75e473f65d35ba926aaca8303e5a77f9a)
    - serde support for all git-object types, incl. test (1ae8f9c8b6b699c3f4928905358f42187bef07a7)
    - learn from the best: with-serde -> serde1 (d651c218bfb7af5fc67ca4b9763703fb29788017)
    - commit to using bstr whenever something is not data bytes; remove miniserde (3183d1b02c2d7bb3c750f8472c29bb161641ca7f)
    - Prepare centralization of bstr as optional component (aa857d9df32dfc75f151154ca430ddfee907deed)
    - add support for miniserde (f80664769e4fdbab3d1ffa56510ba87e570ae9b0)
    - first gentle test of adding serde support selectively. (78d9bc0f54504dc809651aeb0d24e7ac6a3f0bb3)
    - Allow for more screen space when formatting (67943002e7f4215b5383bd0538786ce2857f011e)
    - Pack offset by index (69e35b1d8f24f366d675484a1bddbebd37b72e22)
    - test V1 lookup (e9c71271fa51d5420fcb205d2d3deb6d012f0d41)
    - validate sha1 of pack objects, some work, some don't for some reason… (aa8799a01b92c3c3b7d4347f745921bbb685c454)
    - Capability to write loose object headers, fast (de0aeff518ebd218b73bf472558f278f6bcdc17c)
    - simplify folder names (36fde1f90e9034060b5ede8a923365474659085e)
    - fix clippy (a9c5da7132eeaa6806b8190985a7aa25f9ef89d8)
    - more convenient access to our four object types (ecda6d23561dc176f7d7ad2565da8105efac614f)
    - even better trait derives (e78f9f64c8d52402197b1f946bcd32f0d83e6c7d)
    - Better trait support for basic types (6617386e37b69f6e036ab27280c946e271c99540)
    - Memory size checks for objects (ab51616bb250a62b5367e861c25c1d90ec60f720)
    - Make single-field objects blob and tree more explicit (1aef68f7e979324eb94966d44c160ffe537ee4a8)
    - add Blob type to parsed objects (d3e8e4b24ecda84665b994ccad768774efdcdc90)
    - fix imports (10f29675442c76b38e0a8deb757930a13af3a3bb)
    - try pub use with rename. Not bad in the docs, but maybe a bit confusing (526f3f8d3ca9fe9672b0518f1bc3b921f695c0d8)
    - refactor (2ffd7fa6c4e5a88042b7ee1d56ec8d8515f0991f)
    - refacto (ffc0089fd7f4ffd3eeb0d0411b6857a28b388001)
    - refactor (b9a16473ed028abc59fc5126db9530f2107498d8)
    - test for parsing trees from loose dbs (4f4824971d62d165fd4c2bea869fd88986dc259f)
    - refactor (9f9ccad37fea96954a2df9e314b6c154466dc0ca)
    - Move git-object tests to top-level for separation and cleanness (df42a012bcc489b78320126e51b1f121abe7c018)
    - Prefer integration level tests, but use unit-tests where appropriate (ec3be19c8d007565b814b4757f17811ec0e9de2c)
    - run previously unused method of Tree (0d159c2b76f2a8fc3c391fd990d8e7a4eeffc913)
    - Actually use the Tree object (635e735419af906579de681dbc27b36fd826406d)
    - handle commits without newlines; make tag newlines optional (c0b54bef5a2bcfce9b6deb90cdd27c7e0cc85810)
    - Handle tags without newline; document fixture processing step (344a5622953047e6f2a543bfb0355fb060a4b1e8)
    - Don't assume newlines in trees anymore (45d7c365072a9bada3a6e0b77ced7669fe5533a3)
    - Found huge issue with newlines polluting fixtures. (f182d22caf1dd9c262cdca6a1834478556a74f31)
    - first tree implementation, which seems to work well (9694fcbeb7bea6ebf814119ba5757110ae33bc55)
    - boilerplate for tree parsing (48c4c07098d807b3d62e540e06459c66fef94355)
    - refactor (d48cafa7edc4c6d977c396df4a26d235a3bd662c)
    - Add conversion traits for Object<->Tag|Commit (7dcbd5dc764a07685a66594e3ae5514a9df83082)
    - Make Commit available in borrowed object (b2d1b5d684bdfda5f922b466cc13d4ce2d635cf8)
    - Use smallvec to save memory in the common case (single parent) (263835b7e14e94bfb641067e8188e23d81bc9cac)
    - more tests (56248fe9a351572478cecda8520c25ec25664bc3)
    - Now gpg-signature parsing works correctly - thanks to peek(…) (7078dac0fc27594c63cd9550c8b8b4ac7a52a627)
    - first somewhat working version of single/multi-line signature parsing (dab5c6581dc218ee9a7f049c5499975f762d81cf)
    - support single-line gpg signatures (71330b526614a78e20e739aa8b1cd31b5cf2ce0e)
    - support for commit encoding field (40bffe9b36f5afcb9b3f147d47b94b5e82acaae8)
    - more commit tests, next up: encoding (ca4d3aad8f91189890b8445680406fddb6544af4)
    - first successful parsing of commit (b44765ad08f53a7062def35ecb7fe7624827df85)
    - parse BStr versions of hex-shas directly (e3a2b7782fa48f474c2e1d51a6b8c2ea2c561549)
    - parse parents (696e0a3c48e72373cb540d16b640ddb6fc2a2dcf)
    - Use BStr instead of Id to avoid parsing into something we might not use/need (7c97471c34362c9d3d56ccada252d3058aea6697)
    - factor out hex sha parsing (d650dd26a168ab5a8d679dfb4b93a7f2863a20f0)
    - refactor (0104f4c8a8449c2549bfcfeacfeb20f14b2ddc2d)
    - first stab at factoring header parsing into sub-parser (6f6ee8f721df9f3caf4db54346e7653f341552e3)
    - first fixtures for commit parsing (551f2d1f8e32e7e64a0d19e9e7d3b3ea63e9b449)
    - avoid unnecessary allocation when creating SHA1 paths in loose ODB (09d8d3a12e161a7f6afb522dbe8900a9c09bce06)
    - document existing use of unsafe, deny everywhere else (41f4bce9d9a492f8e20a6eb5b3eaf5adc6d78329)
    - cleanup integer parsing in loose object database (ecdce1a05d8c732afd53c6da6067bf591f96fa6a)
    - Add remaining tag tests, along with some fixes (06e22fb8aea281676e53f786ddb808dd77d3b702)
    - use bstr were possible (01dd4e2a978a9f5bd773dae6da7aa4a5ac1cdbbc)
    - the defining property is actually that the object is borrowing data (e0125fdb0a41ed139364084f6d679932f08b7b4f)
    - refactor (683360a6932f7d5e216dc0fdafa5890c6708d1e8)
    - move all tests into the top-level for nicer names basically :D (598901a768fec768b2519e7925ac623cb66582d6)
    - refactor (0f01e9fff39fb7f1234f57c6689c0e114d9aece7)
    - refactor (87bbea48d247b7938e74672e1a5cb1b8b5cc6a9f)
    - refactor; add more signature parsing tests (ba9c7de7ca93ac42d3c57315d743b321f8f9e3b5)
    - cleanup; all tests work! (7c9660354484869681356a8c4ef8057313e864f2)
    - fix whitespace (ebaaa00d9508141746a7c20e5d25d877f38733e9)
    - first version of tag message parsing - it's actually changed now (74b2328fcbbcffab9981c23e903c4f4c5d085aff)
    - implement parse_signature with nom, starting to like it (ebdf205038b66108c0331aa590388431427493b7)
    - First part of parsing tagger signatures (5b432703cf1c44bbf29e8bf89b297ea29c959be6)
    - generalize with Boxed error cause (824cd2cfbfaef605e953f0af193a036ef74bcac7)
    - first seemingly awkward way of not discarding too much error information… (6f9a636da5c2f33a612395a25e8b92e557d06e83)
    - refactor (fb287af33fcb75c01ac25dd484f529cbb49f3e6f)
    - the first sketch of parsing a tag with Nom and half-decent errors (4498dff1cf63abe53ae17b59d3658ab52235630d)
    - Use git-object in git-odb (07f7c318d55603e3731f08cb04d3da8ac2fcfea6)
    - Move all object related code into own crate… (605ef20ec5ccf66e4f42df6d0140e4087aa13053)
</details>

