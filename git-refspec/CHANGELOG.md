# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### New Features

 - <csr-id-abdf83f494e2a9fba4a8d9fcb776f2c84baebd3e/> Simple serialization for `Instruction` and `RefSpecRef` type.
   It's also a way to normalize input strings as there is only one way
   to serialize instructions, which themselves are already normalized
   towards what's possible.

### Changed (BREAKING)

 - <csr-id-4c4f82170d08b910a7f64482431c99956b1a04c3/> reject all invalid negative refspec patterns.
   Git is more lenient, but will then fail to match against such patterns
   which seems like avoidable surprising behaviour.
 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 70 commits contributed to the release over the course of 18 calendar days.
 - 22 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 4 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - A more efficient representation for `validate::Fix` ([`e819fc6`](https://github.com/Byron/gitoxide/commit/e819fc68531e2d2de3d7df782f63f84941eeef57))
    - Make `specs` in `MatchGroup` public to reduce API surface. ([`2a7df32`](https://github.com/Byron/gitoxide/commit/2a7df323b15678aee3e61a41908aceb644873b11))
    - Allow `match_group::Fix` to be cloned. ([`85c49ec`](https://github.com/Byron/gitoxide/commit/85c49ec16ac7eeb2175fa43545c72b81da693ab1))
    - fix `match_group::Item` to make it uniform with how we typically name refs ([`21420da`](https://github.com/Byron/gitoxide/commit/21420dacb485795e80baabf2a300ff900036ba7b))
    - remote todo with note about our current understanding ([`9dc7a3f`](https://github.com/Byron/gitoxide/commit/9dc7a3f40dbf3d4802ed095fc21dfb3da67acfea))
    - Actually assure we don't try to write into the HEAD ref, which git avoids as well ([`1335618`](https://github.com/Byron/gitoxide/commit/13356184c735d72edb891d64b3de1bb5c981a6ad))
    - Allow 'HEAD' based refspecs to match correctly ([`7432a2b`](https://github.com/Byron/gitoxide/commit/7432a2bb5d11e9991ed5f9d1b29ecf79b10c676a))
    - tests to show that empty remotes actually work ([`2fdec73`](https://github.com/Byron/gitoxide/commit/2fdec7315a65117095f909b4d7d57a91ba666a43))
    - another test which doesn't manage to trigger a certain message from git. ([`4f48095`](https://github.com/Byron/gitoxide/commit/4f48095566fc1e2b440d542ef2c5118c3e37fddd))
    - fully drop 'funny' names ([`f137d60`](https://github.com/Byron/gitoxide/commit/f137d6010610d98de32edb4501053d7786181217))
    - A first version of the 'funny name' sanitization ([`c81e418`](https://github.com/Byron/gitoxide/commit/c81e418ae7f90b674ad005e4b42816c35332a417))
    - frame for testing of fixes ([`9148102`](https://github.com/Byron/gitoxide/commit/91481020c87cfa0cae9dd497fb87e7fb9dd33c8a))
    - refactor ([`d37fd04`](https://github.com/Byron/gitoxide/commit/d37fd044df2cc5355735121e63fcc3c54b8ea4cb))
    - all baseline specs are tested and pass ([`afc0a3d`](https://github.com/Byron/gitoxide/commit/afc0a3da864362ec7a0ab243f72daba4713db569))
    - the first test to validate conflict reporting ([`aef0a46`](https://github.com/Byron/gitoxide/commit/aef0a464811ce98e81d44d1417098c9adef035f5))
    - sketch of validation API along with test suite integration ([`70a765e`](https://github.com/Byron/gitoxide/commit/70a765e295295f87f8550453452d2ffe95b177be))
    - refactor ([`547129e`](https://github.com/Byron/gitoxide/commit/547129e98dfcac32ebc83e743f9aee05d038629b))
    - sketch `Outcome` type which can be used for later sanitization and validation. ([`53e17c1`](https://github.com/Byron/gitoxide/commit/53e17c10f663bc3c389a13cdfec3716da34dd311))
    - prepare first test for conflicts and validation ([`508a33a`](https://github.com/Byron/gitoxide/commit/508a33a5f279c9a6f29e98f560fcd54cea1ed77d))
    - just-in-time deduplication of mappings ([`8ed5d01`](https://github.com/Byron/gitoxide/commit/8ed5d01a75ceb03083b2bddc58b1e9dc26a66cd0))
    - adjust expectations to make first exclusion tests work ([`6e1b19b`](https://github.com/Byron/gitoxide/commit/6e1b19b7f07050c3fcb70187a4d6a4e4210d3343))
    - reject all invalid negative refspec patterns. ([`4c4f821`](https://github.com/Byron/gitoxide/commit/4c4f82170d08b910a7f64482431c99956b1a04c3))
    - basic negation implementation along with first failure. ([`e4931d0`](https://github.com/Byron/gitoxide/commit/e4931d0205c9b8e8e859e8ea940b67483e62a07e))
    - first tests for multiple refspecs ([`77db112`](https://github.com/Byron/gitoxide/commit/77db1127a8ccd71c75670b5d803cabcf93cbcedc))
    - refactor ([`4c73a19`](https://github.com/Byron/gitoxide/commit/4c73a19ae4b044df816e95a4fc19dc6481222a4c))
    - refactor ([`00401be`](https://github.com/Byron/gitoxide/commit/00401bef4279d4b8152ea4c149a00ddf50f518e3))
    - improved glob matching ([`eaf36e7`](https://github.com/Byron/gitoxide/commit/eaf36e7d0336be8398d0b1d9414d3ad73afbb393))
    - basic glob matching. ([`a93628c`](https://github.com/Byron/gitoxide/commit/a93628cb404987c498779b35994db0a05b3dbc0a))
    - type-system supprots  glob matching ([`4b73d11`](https://github.com/Byron/gitoxide/commit/4b73d11a4f0bef8db374cde567547a9ba7097719))
    - more tests for simple 1:1 fetch and update specs ([`74de83c`](https://github.com/Byron/gitoxide/commit/74de83cbea30b84136bfa191f471e137ae7af5c3))
    - Make it easy to obtain the local and remote sides of RefSpecs ([`67506b1`](https://github.com/Byron/gitoxide/commit/67506b1b1997c2b5951f0e1320b0459eac1366e2))
    - Don't reject object-id like heads on the receiving side. ([`6668c3f`](https://github.com/Byron/gitoxide/commit/6668c3f418663ed6f2ed56efd3d7e78d27124296))
    - make object-ids in the source position type-safe ([`413051d`](https://github.com/Byron/gitoxide/commit/413051d03c843c9c99dbc67f4a5f48d6f2b1aeb2))
    - prepare for dual-sided ref mapping to realize that it needs a special case. ([`7368fe4`](https://github.com/Byron/gitoxide/commit/7368fe4ee38bbd34bd811310afa8eeb78c475fda))
    - refactor ([`579e891`](https://github.com/Byron/gitoxide/commit/579e89188679942508f9da107d856ab782a512a1))
    - support testing source-only object names ([`bb61c49`](https://github.com/Byron/gitoxide/commit/bb61c49a9b6a3a109d7af3ddde43fc98bb712ec7))
    - preliminary matching of refs by name ([`426107f`](https://github.com/Byron/gitoxide/commit/426107fea911a2f75d3b624a1c7279cac4edc12e))
    - handle partial names as well ([`dc7f162`](https://github.com/Byron/gitoxide/commit/dc7f1620cb6d00af60cf78e02b4c2949a3e260e4))
    - generalize baseline assertion to support multiple input specs ([`b752e48`](https://github.com/Byron/gitoxide/commit/b752e48b4201c1f26401af39de0a7312b158607b))
    - first successful test ([`3625d5a`](https://github.com/Byron/gitoxide/commit/3625d5a0abb109270b046e2dc206d6f870164306))
    - top-level match-group loop without negation ([`c915a5f`](https://github.com/Byron/gitoxide/commit/c915a5f5f0d771e704b108b9442a605d62f0945e))
    - refactor to use a match-group instead. ([`4ba31c5`](https://github.com/Byron/gitoxide/commit/4ba31c55b57f644361e18e5d31d5df514cddd58a))
    - not using a matchgroup right away seems like the wrong approach ([`7f3bc30`](https://github.com/Byron/gitoxide/commit/7f3bc300dfb980d6e6aa72f8c22edd58fa9351fb))
    - actual expectation for first simple test ([`cec6905`](https://github.com/Byron/gitoxide/commit/cec69057585796ec7bc69f5a6295b97cddb8cb4f))
    - Get to the point where the matcher is invoked ([`cbbdf59`](https://github.com/Byron/gitoxide/commit/cbbdf59290d6c3fb4936b31e3b7836becb126ce4))
    - Simple serialization for `Instruction` and `RefSpecRef` type. ([`abdf83f`](https://github.com/Byron/gitoxide/commit/abdf83f494e2a9fba4a8d9fcb776f2c84baebd3e))
    - ground work for matcher tests ([`509764c`](https://github.com/Byron/gitoxide/commit/509764c95978115da129b8bb9baeb304634fa10c))
    - tag specific tests ([`4f35485`](https://github.com/Byron/gitoxide/commit/4f354852e15b469260bd3553e4f615f9612fabcc))
    - more tests to investigate conflict handling ([`192d4f7`](https://github.com/Byron/gitoxide/commit/192d4f78ba611f090dafda7ef5014efb900d2115))
    - a more realistic sketch for `Matcher`, which will need a surrounding `MatchGroup` ([`dd1d824`](https://github.com/Byron/gitoxide/commit/dd1d8244c8708bbc3583cc0f3f42ad967d5ad524))
    - more robust baseline tests on windows ([`54ca267`](https://github.com/Byron/gitoxide/commit/54ca267138a5116aa2215109b4abe00a64518feb))
    - pares FETCH_HEAD (as specs without local sides); sketch `Match` type ([`44228a0`](https://github.com/Byron/gitoxide/commit/44228a0b9c057bcc915bc0ade43b4ccb3cb916f2))
    - restore full ref names for baseline ([`f6124db`](https://github.com/Byron/gitoxide/commit/f6124db39dc0e828801a59310265d95a755ea46a))
    - parse basline mapping ([`3000a14`](https://github.com/Byron/gitoxide/commit/3000a14c1eed4a543fdef2fd8bcbacba2742aece))
    - parse baseline reflist which serves as input to the matcher ([`fce877f`](https://github.com/Byron/gitoxide/commit/fce877f8d2112fafdb71208784104a66b2313a40))
    - frame for baseline for fetch-matching ([`2569da5`](https://github.com/Byron/gitoxide/commit/2569da5988a055372a1b85660f93185603900dbe))
    - upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
    - prefer to represent instructions with Matchers ([`0887e2e`](https://github.com/Byron/gitoxide/commit/0887e2e0b7ebdcad30606a2633794ac8ff586091))
    - more examples using fully spelled out object names as fetch destination ([`095a099`](https://github.com/Byron/gitoxide/commit/095a09918dc080ba7794c6ff13db0ef0ead20d0d))
    - get more clarity about `git ls-remote` and `git fetch` ([`1b15fe8`](https://github.com/Byron/gitoxide/commit/1b15fe80817d600f39090848c7d144ff94ac398c))
    - a tiny sketch of a possible matching API ([`39d5ff3`](https://github.com/Byron/gitoxide/commit/39d5ff39ac58ec2abf2b55ee69df9905a4f303c2))
 * **Uncategorized**
    - make fmt ([`429cccc`](https://github.com/Byron/gitoxide/commit/429cccc5831c25a7205a12dc7a0443ac48616e2c))
    - Merge branch 'index-from-tree' ([`172f73c`](https://github.com/Byron/gitoxide/commit/172f73cf26878d153d51790fa01853fa4ba6beb7))
    - thanks clippy ([`74a5f22`](https://github.com/Byron/gitoxide/commit/74a5f2262154c5cb5434c1ef2854c4ec3d839f89))
    - thanks clippy ([`016cd1f`](https://github.com/Byron/gitoxide/commit/016cd1f70a536ac95eaa8b80958110caa096d875))
    - thanks clippy ([`b8ac13e`](https://github.com/Byron/gitoxide/commit/b8ac13e5074fa08111fcef1092432ed3a2326c6e))
    - thanks clippy ([`73b405f`](https://github.com/Byron/gitoxide/commit/73b405fe70cf7d53e5e011cf69ea654f4bd96dd2))
    - make fmt ([`535e967`](https://github.com/Byron/gitoxide/commit/535e967666c6da657ff1b7eff7c64ab27cafb182))
    - Merge branch 'git_date_parse' ([`75591fb`](https://github.com/Byron/gitoxide/commit/75591fb108ce440ba2f920bebf99158b407e3046))
    - Release git-hash v0.9.9 ([`da0716f`](https://github.com/Byron/gitoxide/commit/da0716f8c27b4f29cfff0e5ce7fcb3d7240f4aeb))
</details>

## 0.1.1 (2022-08-28)

Maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 4 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#XXX](https://github.com/Byron/gitoxide/issues/XXX)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#XXX](https://github.com/Byron/gitoxide/issues/XXX)**
    - prepare changelogs prior to release ([`8c0bca3`](https://github.com/Byron/gitoxide/commit/8c0bca37ff9fbaadbe55561fb2b0d649980c95b1))
 * **Uncategorized**
    - Release git-object v0.20.3, git-ref v0.15.4, git-config v0.7.1, git-diff v0.18.0, git-traverse v0.16.3, git-pack v0.22.0, git-odb v0.32.0, git-url v0.7.3, git-transport v0.19.3, git-protocol v0.19.1, git-refspec v0.1.1, git-repository v0.23.0, safety bump 6 crates ([`85a3bed`](https://github.com/Byron/gitoxide/commit/85a3bedd68d2e5f36592a2f691c977dc55298279))
    - Release git-features v0.22.3, git-revision v0.4.4 ([`c2660e2`](https://github.com/Byron/gitoxide/commit/c2660e2503323531ba02519eaa51124ee22fec51))
</details>

## 0.1.0 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 52 commits contributed to the release over the course of 18 calendar days.
 - 18 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 4 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - assure ref-specs handle equality, ordering and hashing according to their instruction ([`b4bf7d0`](https://github.com/Byron/gitoxide/commit/b4bf7d015a5d0d48bf7d0509d2fd930a1cb6f398))
    - cleanup crate structure ([`f0163c9`](https://github.com/Byron/gitoxide/commit/f0163c99bccfb7d6719217a8fa773667cabe95fd))
    - don't expose mode() as it's kind of messy and should be left as implementation detail ([`6278966`](https://github.com/Byron/gitoxide/commit/627896655ce04e1a306c5bf3970ebd6e73bb1a5d))
    - improve docs ([`c695a7e`](https://github.com/Byron/gitoxide/commit/c695a7e9716262d885c3ccfde68eb2077650b8ce))
    - add fuzz target and basic docs on how to run it ([`febf070`](https://github.com/Byron/gitoxide/commit/febf0706b83b36a71efbe669ee760c2d4ef14b72))
    - Add fuzz target ([`62d721a`](https://github.com/Byron/gitoxide/commit/62d721a5a7260adb408415810899bcf11d524d0c))
    - more push-spec restrictions ([`bb992ac`](https://github.com/Byron/gitoxide/commit/bb992acc13fc2a63ec5098e9fa8954909ef486ca))
    - more detailed tests of what's allowed and where ([`57a6e69`](https://github.com/Byron/gitoxide/commit/57a6e695697744459149dd6400c54b9c4c88a365))
    - disallow excludes in push mode ([`9c280b2`](https://github.com/Byron/gitoxide/commit/9c280b2de59773c6d13134e3257cf1da5731e35d))
    - don't allow object hashes in excludes ([`b889953`](https://github.com/Byron/gitoxide/commit/b8899532b461ebcdc0ecd33e54a8721e69136c22))
    - negative must not be empty ([`79e0eaf`](https://github.com/Byron/gitoxide/commit/79e0eaf4c754da47731f8c5ed3635339586b3d00))
    - support for `@` shortcut. ([`32d98e9`](https://github.com/Byron/gitoxide/commit/32d98e9c5db402bbfc04394218c0de30bfa64808))
    - and the entire test-suite passes ([`3fa52f8`](https://github.com/Byron/gitoxide/commit/3fa52f8ffd04721e1367706318542cf6d4e71f3b))
    - handle ref-name validation mostly correctly ([`d7c2789`](https://github.com/Byron/gitoxide/commit/d7c27899c76092bdc8e86f2784aaf67666f117dd))
    - refactor ([`e8c072e`](https://github.com/Byron/gitoxide/commit/e8c072e99e845ed1b4a0cc0a0ec7146c53561dcd))
    - tests causing all instrucitons ([`c23a21d`](https://github.com/Byron/gitoxide/commit/c23a21d3e50e62d29bef4e638049b0398d3fb20e))
    - tests for handling exclusions ([`c4499ce`](https://github.com/Byron/gitoxide/commit/c4499ce13aa9e71c7b0024ad8658bdbcbccf5c14))
    - Better handling of special cases ([`c99f575`](https://github.com/Byron/gitoxide/commit/c99f5750967a835afd9a99211b3520b441ae1881))
    - basic validation and detection of patterns ([`e4227d6`](https://github.com/Byron/gitoxide/commit/e4227d6ddd4cd021245bf6f352a0798457c37aae))
    - handle colon and empty on the right side ([`7afebb7`](https://github.com/Byron/gitoxide/commit/7afebb778b93611d924843c95acfd6b36f284fb2))
    - support for deletion ([`966a9e9`](https://github.com/Byron/gitoxide/commit/966a9e93b3afdcdc15af95c9fa3037d71af6e0ee))
    - add include directive ([`701d46f`](https://github.com/Byron/gitoxide/commit/701d46f020db5c5f86a0184ff345f30d077be8ed))
    - first successful test for returning a refspec. ([`6e5bd5c`](https://github.com/Byron/gitoxide/commit/6e5bd5c152403d76ba1cf3da2b984689cb6fe8c5))
    - sort out how expectations can be expressed in test suite ([`3f264af`](https://github.com/Byron/gitoxide/commit/3f264afda02235dbcdf712d957e37c71ce749f01))
    - refactor ([`6713793`](https://github.com/Byron/gitoxide/commit/6713793dca4054a7f8717e70c8e9e4b7e625e9b4))
    - sketch all possible instructions of fetch and push refspecs ([`0ba1b73`](https://github.com/Byron/gitoxide/commit/0ba1b73bf988357f4b27753b87432618edec697a))
    - run the baseline test and gather some information ([`5e4ee9b`](https://github.com/Byron/gitoxide/commit/5e4ee9ba422cac9eef2b558746f3a3aa4b67a5e4))
    - first few bits of error handling in parser ([`9c5fed2`](https://github.com/Byron/gitoxide/commit/9c5fed2e2a6ea388acde73be32f8b7f8687c415b))
    - frame for basic parsing ([`b9a4bdc`](https://github.com/Byron/gitoxide/commit/b9a4bdca41c074364b7bc26523784c35ac3196ce))
    - sketch data structure that should do the trick ([`5c823dc`](https://github.com/Byron/gitoxide/commit/5c823dcbfd3aca0a8846300629e01aed8d7b7e66))
    - all baseline test cases from git's test-suite ([`362bd46`](https://github.com/Byron/gitoxide/commit/362bd4651751960b3062fd1c65d58b986b46cc97))
 * **Uncategorized**
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge branch 'example-write-blob' ([`afedd7f`](https://github.com/Byron/gitoxide/commit/afedd7f86ec8ea18a8165f71698ecc886f5cf643))
    - Merge pull request #494 from ultrasaurus/patch-1 ([`86fe22c`](https://github.com/Byron/gitoxide/commit/86fe22cb1aad5944a229bc2a5252b36ef1fd59ef))
    - Merge branch 'main' into remote-ls-refs ([`95f2f4f`](https://github.com/Byron/gitoxide/commit/95f2f4f17f7eae174a64c7d9f6a1513d73b21bbb))
    - Merge branch 'example-new-repo' ([`946dd3a`](https://github.com/Byron/gitoxide/commit/946dd3a80522ef437e09528a93aa1433f01b0ee8))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge branch 'main' into remote-ls-refs ([`c82bbfa`](https://github.com/Byron/gitoxide/commit/c82bbfaddc45bf9b5b55f056613046d977d9ef09))
    - prepare for release of git-repository ([`8aa5389`](https://github.com/Byron/gitoxide/commit/8aa5389d5a1bdd3a07f1caa1c2f55c8af4f9844a))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge branch 'main' into remote-ls-refs ([`c4bf958`](https://github.com/Byron/gitoxide/commit/c4bf9585d815bc342e5fb383336cc654280dd34f))
    - Merge branch 'format_git_date_time' ([`99e12be`](https://github.com/Byron/gitoxide/commit/99e12bee16ab3f344c71818bfd1c95cf50e1721b))
    - Merge branch 'main' into remote-ls-refs ([`de61c4d`](https://github.com/Byron/gitoxide/commit/de61c4db7855d6925d66961f62ae3d12cc4acf78))
    - thanks clippy ([`4bd747c`](https://github.com/Byron/gitoxide/commit/4bd747cb3e126fe5b1d540270cfbd731cffd42ef))
    - Merge branch 'main' into remote-ls-refs ([`e8fc89d`](https://github.com/Byron/gitoxide/commit/e8fc89d36ab17a66e799bdec3ed71388b9730266))
    - thanks clippy ([`b9e1cdb`](https://github.com/Byron/gitoxide/commit/b9e1cdbf19045816387d922abd9c886419ff6bf2))
    - thanks clippy ([`6c963b0`](https://github.com/Byron/gitoxide/commit/6c963b022e48854001353f8909c3e8314c2e5861))
    - thanks clippy ([`b62ee56`](https://github.com/Byron/gitoxide/commit/b62ee56b5bf468b9673b01034ec284faf1a7c7c2))
</details>

## 0.0.0 (2022-08-05)

Initial release for name reservation.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - prepare git-refspec changelog prior to release ([`3383408`](https://github.com/Byron/gitoxide/commit/3383408ce22ca9c7502ad2d1fab51cf12dc5ee72))
    - empty `git-refspec` crate for name reservation prior to implementation ([`871a3c0`](https://github.com/Byron/gitoxide/commit/871a3c054d4fe6c1e92b6f2e260b19463404509f))
 * **Uncategorized**
    - Release git-refspec v0.0.0 ([`d406689`](https://github.com/Byron/gitoxide/commit/d406689f01c8fa7cc81b52d6500a44303e719ec2))
</details>

