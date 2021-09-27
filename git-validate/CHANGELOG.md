# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 110 commits contributed to the release over the course of 4 calendar days.
 - 20 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 10 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#192**
    - smart-release: assure the current package version is actually breaking (fb750b65ca64c894ffb79cd0049f10a8db255ab6)
    - smart-release: better verbosity handling when comparing to crates-index (f6f2d1b2c1c50d36ee046ed58ffffed0444cd25a)
    - smart-release(feat): turn off safety bump with its own flag (a040f7d882eb5f6db0d54ba7e32437da3579a075)
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
    - Write new changelogs with bat if available (cca8e52fdd2ebd16b08247d428ed5387a1058cd5)
    - Use `cargo diet` to reduce package size (cc5709e812aea79e9d9a6f16637d09f22cb73f81)
    - Write markdown changelog to lock file (400046ec65100a15cd1757143c1abba05091f129)
    - refactor (b05ce15a31aba9b0084792b7f0e7155b73b46e2d)
    - Basic serialization of ChangeLog (205b5698072c6919036190cacac120a7dd5dbd73)
    - support for generated headers (bcc4323785c5aca698e5af2ee5cf32e171727ed3)
    - refactor (1ebb7365ce564d55bd4f16f7316375b9458b4659)
    - Use 'to_*' when converting `easy::Object` to specific object kind (1cb41f81cffe19c75aadf49a5cc7ec390ec6cae7)
    - transform history segments into changelog parts (348b05cbe6e93e871393a6db9d1ebfea59ec7fdb)
    - layout structure for ChangeLog generation from history items (40e9075238f7272c08497851f55d0b525f47f2db)
    - more general commit history (39522ec59d2eb7f439c75a5cc5dc0315db9497d5)
    - Invert meaning of changelog's --dependencies flag… (51eb8cba67edf431ebe3e32232022dbf8971e6ac)
    - rename --skip-dependencies to --no-dependencies… (77ed17c703e502e132cda9a94eb8c63db0b627ad)
    - Adjust changelog… (fb0dbfc60df2df51ed6a02ad60d04ef3557e950c)
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
    - thanks clippy (a89d08c4ce28f0c466f01758e9f4db09eeb02458)
    - thanks clippy (a554b9d356d4e44c9504f7b35aa2c4f9c660df9b)
    - Bump git-repository v0.10.0 (5a10dde1bcbc03157f3ba45104638a8b5b296cb9)
    - thanks clippy (d15fded08224c45dcbd34cf742398e2594f39964)
</details>

## v0.5.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-validate v0.5.2 (7bcbf9d24da16a87a23817a94ae6821f506eb1d5)
    - Apply nightly rustfmt rules. (5e0edbadb39673d4de640f112fa306349fb11814)
</details>

## v0.5.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-validate v0.5.1 (fdd844a80e49f884e30d8a244e50284a9fc7080d)
    - remove dev-dependency cycles by removing their version (c40faca41632cd2a226daf4ddf5293b65d1fdc82)
</details>

## v0.5.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (bf15c2a2f285046b094093760c1969007ee75e25)
    - (cargo-release) version 0.4.0 (70ef3442775b54ba9e4ee9ebfffb37af9804cc5b)
</details>

## v0.4.0 (2021-08-10)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 19 commits contributed to the release over the course of 76 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 (0d5c8b96dfdfb96e4fc82623f756f6c7f7046e90)
    - [ref #152] all tests and impl for refname expansion (9cef2f2f166514048fae52ceec5a86a2849be286)
    - [ref #152] refactor (431dd8655397b0ae88a5144d5c8553ba63e46c8f)
    - clippy on tests and thanks clippy (a77a71cf02d328a2a964388928d6b2a235a0aa85)
    - [validate] assure we can't accidentally write windows paths (02f127ba3a525c6f2f02dc6aaad2f16ec76a182a)
    - [ref] on the way towards realistic transactions… (c808cb17b2fea12e018fabb789862e9b7703e49b)
    - (cargo-release) version 0.3.0 (87db688f23475d7232731429d770848aea228492)
    - [validate] disallow missing docs, fill in the remaining ones. (a593e796cdba02700cb5747cf208c2f59f771f10)
    - [tempfile] crate frame (1b04c39030b436fb6850fbfa0c39a4fed7df727c)
    - (cargo-release) version 0.3.0 (6b33678f83e6d261ca15c4a7634ff5b4e66d81dd)
    - (cargo-release) version 0.2.0 (3286e42547b59df6365087cbae9ce1c9c959faad)
    - [git-ref] find_one_existing(…) for convenience (7a443ffc148ae8161ba93351ffd16631f79e095c)
    - [git-ref] the first green find_one test (30177e81451bd4fb51dd3297502fa3c63f67286e)
    - (cargo-release) version 0.2.0 (132789475400abe660b30ef6d2c5ff57821dd2c4)
    - [git-ref] refactor (0758867a8886be466bfb30aaffd628964b047a3d)
    - thanks clippy (474b73bbe8679d0c58a4a7567d06e623ac4c1788)
    - [git-ref] all validation tests green (53123100201cae11e2dab1536d30c12f123ea710)
    - [git-ref] more tests green (4f5a1d0c55f790cdf6d6bd4e5b1957f5c1e0b290)
    - [git-ref] more tests for invalid ref paths (db3f1b1b0326448514d566cde7ed9e9d210fa8ba)
</details>

## v0.1.0 (2021-05-26)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [git-ref] use git-validate crate (6b4f937f13ad62bc2c7e5b0fc14416feb9c313ba)
    - [git-ref] migrate tag::name validation to git-validate (1ec4a54ed67c89515cf5d2725eb394aac8395d7c)
    - [git-ref] setup git-validate crate for sharing of this kind of code (530d3920bb5ebbd1d0b0af05eda6bdbf5a5cb45e)
</details>

