# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.7.3 (2023-02-20)

### Bug Fixes

 - <csr-id-135d317065aae87af302beb6c26bb6ca8e30b6aa/> compatibility with `bstr` v1.3, use `*.as_bytes()` instead of `.as_ref()`.
   `as_ref()` relies on a known target type which isn't always present. However, once
   there is only one implementation, that's no problem, but when that changes compilation
   fails due to ambiguity.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - compatibility with `bstr` v1.3, use `*.as_bytes()` instead of `.as_ref()`. ([`135d317`](https://github.com/Byron/gitoxide/commit/135d317065aae87af302beb6c26bb6ca8e30b6aa))
</details>

## 0.7.2 (2023-02-17)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### New Features (BREAKING)

 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `git-features` and `git-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

### Changed (BREAKING)

 - <csr-id-2a0a87a04e7b4d6ed3be3d8adc89917576727686/> remove lifetime of `match_group::Fix`, keeping `RefSpec` instances instead
   That lifetime unnecessarily complicated things and wasn't worth keeping
   due to being a premature optimization.
 - <csr-id-4c4f82170d08b910a7f64482431c99956b1a04c3/> reject all invalid negative refspec patterns.
   Git is more lenient, but will then fail to match against such patterns
   which seems like avoidable surprising behaviour.
 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

### New Features

 - <csr-id-21b21b6c25e1d8d1da9464b7bef06a795f679210/> add `RefSpecRef::expand_prefix()` method to avoid missing prefixes.
   The current implementation might cause refspecs to end up ignored as
   they don't have a prefix, and in protocol V2 it would then fail
   to add a ref-spec filter which causes them to be missed.
   
   With `expand_prefix()`, we assure that there are all possible prefixes
   that can contain partial names, similar to what git does.
 - <csr-id-d7f63a6c60a826dc862bd13adbef041e4ac6d8ab/> `RefSpec::allow_non_fast_forward()` to get information about 'force' quickly.
 - <csr-id-6df179b5cf831402444cc78429a57f835358376e/> `RefSpecRef::prefix()` to return the two-component prefix of a refspec's source. #(450)
 - <csr-id-abdf83f494e2a9fba4a8d9fcb776f2c84baebd3e/> Simple serialization for `Instruction` and `RefSpecRef` type.
   It's also a way to normalize input strings as there is only one way
   to serialize instructions, which themselves are already normalized
   towards what's possible.

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### Bug Fixes

 - <csr-id-d34292294a0c41991aebd5af46e7bd7e3ad7324d/> Fixes build for parsing fuzz harness
 - <csr-id-d53ddcde948cfbd7773eb830cbb636626b32debb/> `HEAD` may now return itself as prefix in `RefSpecRef::prefix()` and `expanded_prefix()`.
   Previously, the expanded prefix would be a list of possibilities, even
   though it's such a common case that we really want to avoid spamming the
   remote about it when asking for HEAD during clone for instance.
 - <csr-id-278ff7a6ee084ea864193a5ca25b6cd0f18e19a0/> `RefSpecRef` instruction uses the correct lifetime.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 234 commits contributed to the release over the course of 196 calendar days.
 - 14 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 5 unique issues were worked on: [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470), [#571](https://github.com/Byron/gitoxide/issues/571), [#691](https://github.com/Byron/gitoxide/issues/691), [#XXX](https://github.com/Byron/gitoxide/issues/XXX)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 10 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - improve docs for `Instruction` ([`911f68f`](https://github.com/Byron/gitoxide/commit/911f68fe099e3eac7dd4a0f15fcd682657564389))
    - `HEAD` may now return itself as prefix in `RefSpecRef::prefix()` and `expanded_prefix()`. ([`d53ddcd`](https://github.com/Byron/gitoxide/commit/d53ddcde948cfbd7773eb830cbb636626b32debb))
    - fix docs ([`9d0f31e`](https://github.com/Byron/gitoxide/commit/9d0f31e4bea4b47f1a91fa4dc38f592bdbf976a8))
    - add `RefSpecRef::expand_prefix()` method to avoid missing prefixes. ([`21b21b6`](https://github.com/Byron/gitoxide/commit/21b21b6c25e1d8d1da9464b7bef06a795f679210))
    - `RefSpec::allow_non_fast_forward()` to get information about 'force' quickly. ([`d7f63a6`](https://github.com/Byron/gitoxide/commit/d7f63a6c60a826dc862bd13adbef041e4ac6d8ab))
    - `RefSpecRef` instruction uses the correct lifetime. ([`278ff7a`](https://github.com/Byron/gitoxide/commit/278ff7a6ee084ea864193a5ca25b6cd0f18e19a0))
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
    - prepare git-refspec changelog prior to release ([`3383408`](https://github.com/Byron/gitoxide/commit/3383408ce22ca9c7502ad2d1fab51cf12dc5ee72))
    - empty `git-refspec` crate for name reservation prior to implementation ([`871a3c0`](https://github.com/Byron/gitoxide/commit/871a3c054d4fe6c1e92b6f2e260b19463404509f))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **[#571](https://github.com/Byron/gitoxide/issues/571)**
    - refactor ([`ef7467c`](https://github.com/Byron/gitoxide/commit/ef7467cd61762dc4206fa82e66adc287ba2f7f52))
    - assure sub-branches can be matched correctly ([`c9383c6`](https://github.com/Byron/gitoxide/commit/c9383c693e0b3c506c9b1c42f339f40c1145f998))
 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
 * **[#XXX](https://github.com/Byron/gitoxide/issues/XXX)**
    - prepare changelogs prior to release ([`8c0bca3`](https://github.com/Byron/gitoxide/commit/8c0bca37ff9fbaadbe55561fb2b0d649980c95b1))
 * **Uncategorized**
    - Release gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`7fc00f8`](https://github.com/Byron/gitoxide/commit/7fc00f87d74aedf631ce4032be1cdfe1804c7e7d))
    - Release gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`59e9fac`](https://github.com/Byron/gitoxide/commit/59e9fac67d1b353e124300435b55f6b5468d7deb))
    - Release gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`48f5bd2`](https://github.com/Byron/gitoxide/commit/48f5bd2014fa3dda6fbd60d091065c5537f69453))
    - Release gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`a5869e0`](https://github.com/Byron/gitoxide/commit/a5869e0b223406820bca836e3e3a7fae2bfd9b04))
    - Release gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`41d57b9`](https://github.com/Byron/gitoxide/commit/41d57b98964094fc1528adb09f69ca824229bf25))
    - Release gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`e313112`](https://github.com/Byron/gitoxide/commit/e31311257bd138b52042dea5fc40c3abab7f269b))
    - Release gix-features v0.26.4, gix-actor v0.17.1, gix-glob v0.5.3, gix-path v0.7.1, gix-quote v0.4.1, gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`6efd0d3`](https://github.com/Byron/gitoxide/commit/6efd0d31fbeca31ab7319aa2ac97bb31dc4ce055))
    - Release gix-date v0.4.2, gix-hash v0.10.2, gix-features v0.26.4, gix-actor v0.17.1, gix-glob v0.5.3, gix-path v0.7.1, gix-quote v0.4.1, gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`6ccc88a`](https://github.com/Byron/gitoxide/commit/6ccc88a8e4a56973b1a358cf72dc012ee3c75d56))
    - Merge branch 'rename-crates' into inform-about-gix-rename ([`c9275b9`](https://github.com/Byron/gitoxide/commit/c9275b99ea43949306d93775d9d78c98fb86cfb1))
    - rename `git-testtools` to `gix-testtools` ([`b65c33d`](https://github.com/Byron/gitoxide/commit/b65c33d256cfed65d11adeff41132e3e58754089))
    - adjust to renaming of `git-pack` to `gix-pack` ([`1ee81ad`](https://github.com/Byron/gitoxide/commit/1ee81ad310285ee4aa118118a2be3810dbace574))
    - adjust to renaming of `git-odb` to `gix-odb` ([`476e2ad`](https://github.com/Byron/gitoxide/commit/476e2ad1a64e9e3f0d7c8651d5bcbee36cd78241))
    - adjust to renaming of `git-index` to `gix-index` ([`86db5e0`](https://github.com/Byron/gitoxide/commit/86db5e09fc58ce66b252dc13b8d7e2c48e4d5062))
    - adjust to renaming of `git-diff` to `gix-diff` ([`49a163e`](https://github.com/Byron/gitoxide/commit/49a163ec8b18f0e5fcd05a315de16d5d8be7650e))
    - adjust to renaming of `git-commitgraph` to `gix-commitgraph` ([`f1dd0a3`](https://github.com/Byron/gitoxide/commit/f1dd0a3366e31259af029da73228e8af2f414244))
    - adjust to renaming of `git-mailmap` to `gix-mailmap` ([`2e28c56`](https://github.com/Byron/gitoxide/commit/2e28c56bb9f70de6f97439818118d3a25859698f))
    - adjust to renaming of `git-discover` to `gix-discover` ([`53adfe1`](https://github.com/Byron/gitoxide/commit/53adfe1c34e9ea3b27067a97b5e7ac80b351c441))
    - adjust to renaming of `git-chunk` to `gix-chunk` ([`59194e3`](https://github.com/Byron/gitoxide/commit/59194e3a07853eae0624ebc4907478d1de4f7599))
    - adjust to renaming of `git-bitmap` to `gix-bitmap` ([`75f2a07`](https://github.com/Byron/gitoxide/commit/75f2a079b17489f62bc43e1f1d932307375c4f9d))
    - adjust to renaming for `git-protocol` to `gix-protocol` ([`823795a`](https://github.com/Byron/gitoxide/commit/823795addea3810243cab7936cd8ec0137cbc224))
    - adjust to renaming of `git-refspec` to `gix-refspec` ([`c958802`](https://github.com/Byron/gitoxide/commit/c9588020561577736faa065e7e5b5bb486ca8fe1))
    - rename `git-refspec` to `gix-refspec` ([`3c56012`](https://github.com/Byron/gitoxide/commit/3c56012d7ba0e82ca566c0de9ad9dc2966f3a34f))
    - adjust to renaming of `git-revision` to `gix-revision` ([`ee0ee84`](https://github.com/Byron/gitoxide/commit/ee0ee84607c2ffe11ee75f27a31903db68afed02))
    - adjust to renaming of `git-transport` to `gix-transport` ([`b2ccf71`](https://github.com/Byron/gitoxide/commit/b2ccf716dc4425bb96651d4d58806a3cc2da219e))
    - adjust to renaming of `git-credentials` to `gix-credentials` ([`6b18abc`](https://github.com/Byron/gitoxide/commit/6b18abcf2856f02ab938d535a65e51ac282bf94a))
    - adjust to renaming of `git-prompt` to `gix-prompt` ([`6a4654e`](https://github.com/Byron/gitoxide/commit/6a4654e0d10ab773dd219cb4b731c0fc1471c36d))
    - adjust to renaming of `git-command` to `gix-command` ([`d26b8e0`](https://github.com/Byron/gitoxide/commit/d26b8e046496894ae06b0bbfdba77196976cd975))
    - adjust to renaming of `git-packetline` to `gix-packetline` ([`5cbd22c`](https://github.com/Byron/gitoxide/commit/5cbd22cf42efb760058561c6c3bbcd4dab8c8be1))
    - adjust to renaming of `git-worktree` to `gix-worktree` ([`73a1282`](https://github.com/Byron/gitoxide/commit/73a12821b3d9b66ec1714d07dd27eb7a73e3a544))
    - adjust to renamining of `git-hashtable` to `gix-hashtable` ([`26a0c98`](https://github.com/Byron/gitoxide/commit/26a0c98d0a389b03e3dc7bfc758b37155e285244))
    - adjust to renamining of `git-worktree` to `gix-worktree` ([`108bb1a`](https://github.com/Byron/gitoxide/commit/108bb1a634f4828853fb590e9fc125f79441dd38))
    - adjust to renaming of `git-url` to `gix-url` ([`b50817a`](https://github.com/Byron/gitoxide/commit/b50817aadb143e19f61f64e19b19ec1107d980c6))
    - adjust to renaming of `git-date` to `gix-date` ([`9a79ff2`](https://github.com/Byron/gitoxide/commit/9a79ff2d5cc74c1efad9f41e21095ae498cce00b))
    - adjust to renamining of `git-attributes` to `gix-attributes` ([`4a8b3b8`](https://github.com/Byron/gitoxide/commit/4a8b3b812ac26f2a2aee8ce8ca81591273383c84))
    - adjust to renaminig of `git-quote` to `gix-quote` ([`648025b`](https://github.com/Byron/gitoxide/commit/648025b7ca94411fdd0d90c53e5faede5fde6c8d))
    - adjust to renaming of `git-config` to `gix-config` ([`3a861c8`](https://github.com/Byron/gitoxide/commit/3a861c8f049f6502d3bcbdac752659aa1aeda46a))
    - adjust to renaming of `git-ref` to `gix-ref` ([`1f5f695`](https://github.com/Byron/gitoxide/commit/1f5f695407b034377d94b172465ff573562b3fc3))
    - adjust to renaming of `git-lock` to `gix-lock` ([`2028e78`](https://github.com/Byron/gitoxide/commit/2028e7884ae1821edeec81612f501e88e4722b17))
    - adjust to renaming of `git-tempfile` to `gix-tempfile` ([`b6cc3eb`](https://github.com/Byron/gitoxide/commit/b6cc3ebb5137084a6327af16a7d9364d8f092cc9))
    - adjust to renaming of `git-object` to `gix-object` ([`fc86a1e`](https://github.com/Byron/gitoxide/commit/fc86a1e710ad7bf076c25cc6f028ddcf1a5a4311))
    - adjust to renaming of `git-actor` to `gix-actor` ([`4dc9b44`](https://github.com/Byron/gitoxide/commit/4dc9b44dc52f2486ffa2040585c6897c1bf55df4))
    - adjust to renaming of `git-validate` to `gix-validate` ([`5e40ad0`](https://github.com/Byron/gitoxide/commit/5e40ad078af3d08cbc2ca81ce755c0ed8a065b4f))
    - adjust to renaming of `git-hash` to `gix-hash` ([`4a9d025`](https://github.com/Byron/gitoxide/commit/4a9d0257110c3efa61d08c8457c4545b200226d1))
    - adjust to renaming of `git-features` to `gix-features` ([`e2dd68a`](https://github.com/Byron/gitoxide/commit/e2dd68a417aad229e194ff20dbbfd77668096ec6))
    - adjust to renaming of `git-glob` to `gix-glob` ([`35b2a3a`](https://github.com/Byron/gitoxide/commit/35b2a3acbc8f2a03f151bc0a3863163844e0ca86))
    - adjust to renaming of `git-sec` to `gix-sec` ([`eabbb92`](https://github.com/Byron/gitoxide/commit/eabbb923bd5a32fc80fa80f96cfdc2ab7bb2ed17))
    - adapt to renaming of `git-path` to `gix-path` ([`d3bbcfc`](https://github.com/Byron/gitoxide/commit/d3bbcfccad80fc44ea8e7bf819f23adaca06ba2d))
    - adjust to rename of `git-config-value` to `gix-config-value` ([`622b3e1`](https://github.com/Byron/gitoxide/commit/622b3e1d0bffa0f8db73697960f9712024fac430))
    - Release git-refspec v0.7.2, git-repository v0.34.0 ([`1210c19`](https://github.com/Byron/gitoxide/commit/1210c1926851495df5d6fd3f6906602a7e423548))
    - Release git-date v0.4.2, git-hash v0.10.2, git-features v0.26.2, git-actor v0.17.1, git-glob v0.5.3, git-path v0.7.1, git-quote v0.4.1, git-attributes v0.8.2, git-config-value v0.10.1, git-tempfile v3.0.2, git-lock v3.0.2, git-validate v0.7.2, git-object v0.26.1, git-ref v0.24.0, git-sec v0.6.2, git-config v0.16.0, git-command v0.2.3, git-prompt v0.3.2, git-url v0.13.2, git-credentials v0.9.1, git-diff v0.26.1, git-discover v0.13.0, git-hashtable v0.1.1, git-bitmap v0.2.1, git-traverse v0.22.1, git-index v0.12.3, git-mailmap v0.9.2, git-chunk v0.4.1, git-pack v0.30.2, git-odb v0.40.2, git-packetline v0.14.2, git-transport v0.25.4, git-protocol v0.26.3, git-revision v0.10.2, git-refspec v0.7.2, git-worktree v0.12.2, git-repository v0.34.0, safety bump 3 crates ([`c196d20`](https://github.com/Byron/gitoxide/commit/c196d206d57a310b1ce974a1cf0e7e6d6db5c4d6))
    - prepare changelogs prior to release ([`7c846d2`](https://github.com/Byron/gitoxide/commit/7c846d2102dc767366771925212712ef8cc9bf07))
    - Merge branch 'Lioness100/main' ([`1e544e8`](https://github.com/Byron/gitoxide/commit/1e544e82455bf9ecb5e3c2146280eaf7ecd81f16))
    - fix typos ([`39ed9ed`](https://github.com/Byron/gitoxide/commit/39ed9eda62b7718d5109135e5ad406fb1fe2978c))
    - thanks clippy ([`bac57dd`](https://github.com/Byron/gitoxide/commit/bac57dd05ea2d5a4ee45ef9350fa3f2e19474bc0))
    - Merge remote-tracking branch 'origin/main' ([`f5fbcfb`](https://github.com/Byron/gitoxide/commit/f5fbcfbcf281429d56c46f815ca86da848410083))
    - Fixes build for parsing fuzz harness ([`d342922`](https://github.com/Byron/gitoxide/commit/d34292294a0c41991aebd5af46e7bd7e3ad7324d))
    - Release git-date v0.4.1, git-features v0.26.1, git-glob v0.5.2, git-attributes v0.8.1, git-tempfile v3.0.1, git-ref v0.23.1, git-sec v0.6.1, git-config v0.15.1, git-prompt v0.3.1, git-url v0.13.1, git-discover v0.12.1, git-index v0.12.2, git-mailmap v0.9.1, git-pack v0.30.1, git-odb v0.40.1, git-transport v0.25.3, git-protocol v0.26.2, git-revision v0.10.1, git-refspec v0.7.1, git-worktree v0.12.1, git-repository v0.33.0 ([`5b5b380`](https://github.com/Byron/gitoxide/commit/5b5b3809faa71c658db38b40dfc410224d08a367))
    - prepare changelogs prior to release ([`93bef97`](https://github.com/Byron/gitoxide/commit/93bef97b3c0c75d4bf7119fdd787516e1efc77bf))
    - Merge branch 'patch-1' ([`b93f0c4`](https://github.com/Byron/gitoxide/commit/b93f0c49fc677b6c19aea332cbfc1445ce475375))
    - thanks clippy ([`9e04685`](https://github.com/Byron/gitoxide/commit/9e04685dd3f109bfb27663f9dc7c04102e660bf2))
    - Release git-ref v0.23.0, git-config v0.15.0, git-command v0.2.2, git-diff v0.26.0, git-discover v0.12.0, git-mailmap v0.9.0, git-pack v0.30.0, git-odb v0.40.0, git-transport v0.25.2, git-protocol v0.26.1, git-revision v0.10.0, git-refspec v0.7.0, git-worktree v0.12.0, git-repository v0.32.0 ([`ffb5b6a`](https://github.com/Byron/gitoxide/commit/ffb5b6a21cb415315db6fd5294940c7c6deb4538))
    - prepare changelogs prior to release ([`4381a03`](https://github.com/Byron/gitoxide/commit/4381a03a34c305f31713cce234c2afbf8ac60f01))
    - Release git-date v0.4.0, git-actor v0.17.0, git-object v0.26.0, git-traverse v0.22.0, git-index v0.12.0, safety bump 15 crates ([`0e3d0a5`](https://github.com/Byron/gitoxide/commit/0e3d0a56d7e6a60c6578138f2690b4fa54a2072d))
    - Release git-features v0.26.0, git-actor v0.16.0, git-attributes v0.8.0, git-object v0.25.0, git-ref v0.22.0, git-config v0.14.0, git-command v0.2.1, git-url v0.13.0, git-credentials v0.9.0, git-diff v0.25.0, git-discover v0.11.0, git-traverse v0.21.0, git-index v0.11.0, git-mailmap v0.8.0, git-pack v0.29.0, git-odb v0.39.0, git-transport v0.25.0, git-protocol v0.26.0, git-revision v0.9.0, git-refspec v0.6.0, git-worktree v0.11.0, git-repository v0.31.0, safety bump 24 crates ([`5ac9fbe`](https://github.com/Byron/gitoxide/commit/5ac9fbe265a5b61c533a2a6b3abfed2bdf7f89ad))
    - prepare changelogs prior to release ([`30d8ca1`](https://github.com/Byron/gitoxide/commit/30d8ca19284049dcfbb0de2698cafae1d1a16b0c))
    - Release git-date v0.3.1, git-features v0.25.0, git-actor v0.15.0, git-glob v0.5.1, git-path v0.7.0, git-attributes v0.7.0, git-config-value v0.10.0, git-lock v3.0.1, git-validate v0.7.1, git-object v0.24.0, git-ref v0.21.0, git-sec v0.6.0, git-config v0.13.0, git-prompt v0.3.0, git-url v0.12.0, git-credentials v0.8.0, git-diff v0.24.0, git-discover v0.10.0, git-traverse v0.20.0, git-index v0.10.0, git-mailmap v0.7.0, git-pack v0.28.0, git-odb v0.38.0, git-packetline v0.14.1, git-transport v0.24.0, git-protocol v0.25.0, git-revision v0.8.0, git-refspec v0.5.0, git-worktree v0.10.0, git-repository v0.30.0, safety bump 26 crates ([`e6b9906`](https://github.com/Byron/gitoxide/commit/e6b9906c486b11057936da16ed6e0ec450a0fb83))
    - prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Merge branch 'main' into read-split-index ([`c57bdde`](https://github.com/Byron/gitoxide/commit/c57bdde6de37eca9672ea715962bbd02aa3eb055))
    - Merge branch 'adjustments-for-cargo' ([`083909b`](https://github.com/Byron/gitoxide/commit/083909bc7eb902eeee2002034fdb6ed88280dc5c))
    - adjust to changes in `git-testtools` ([`4eb842c`](https://github.com/Byron/gitoxide/commit/4eb842c7150b980e1c2637217e1f9657a671cea7))
    - Release git-hash v0.10.1, git-hashtable v0.1.0 ([`7717170`](https://github.com/Byron/gitoxide/commit/771717095d9a67b0625021eb0928828ab686e772))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - prepare changelogs prior to release ([`423af90`](https://github.com/Byron/gitoxide/commit/423af90c8202d62dc1ea4a76a0df6421d1f0aa06))
    - Merge branch 'main' into write-sparse-index ([`c4e6849`](https://github.com/Byron/gitoxide/commit/c4e68496c368611ebe17c6693d06c8147c28c717))
    - Merge branch 'gix-clone' ([`def53b3`](https://github.com/Byron/gitoxide/commit/def53b36c3dec26fa78939ab0584fe4ff930909c))
    - Merge branch 'main' into gix-clone ([`91bf67a`](https://github.com/Byron/gitoxide/commit/91bf67af9751d1e6beb78fb77b40f05352b98215))
    - Merge branch 'fix-571' ([`2514334`](https://github.com/Byron/gitoxide/commit/2514334c17f543e3e18ac43261990ad412b7c7ae))
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - Merge branch 'clone' ([`507dc7e`](https://github.com/Byron/gitoxide/commit/507dc7e706cb3c9d89d048b3aff5df239a9b6788))
    - remove lifetime of `match_group::Fix`, keeping `RefSpec` instances instead ([`2a0a87a`](https://github.com/Byron/gitoxide/commit/2a0a87a04e7b4d6ed3be3d8adc89917576727686))
    - Another test to validate components must not be empty ([`b2c9af1`](https://github.com/Byron/gitoxide/commit/b2c9af1cf7eedfb618c47d0598cfcef636e793ff))
    - Merge branch 'main' into new-http-impl ([`702a161`](https://github.com/Byron/gitoxide/commit/702a161ef11fc959611bf44b70e9ffe04561c7ad))
    - make fmt ([`53acf25`](https://github.com/Byron/gitoxide/commit/53acf2565743eff7cead7a42011107b2fc8d7e0e))
    - Merge branch 'fetch-pack' ([`3c49400`](https://github.com/Byron/gitoxide/commit/3c49400809c7c2120f4ce704c19a0421545b5acd))
    - `RefSpecRef::prefix()` to return the two-component prefix of a refspec's source. #(450) ([`6df179b`](https://github.com/Byron/gitoxide/commit/6df179b5cf831402444cc78429a57f835358376e))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
    - Release git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0 ([`f5c36d8`](https://github.com/Byron/gitoxide/commit/f5c36d85755d1f0f503b77d9a565fad6aecf6728))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - make fmt ([`429cccc`](https://github.com/Byron/gitoxide/commit/429cccc5831c25a7205a12dc7a0443ac48616e2c))
    - Merge branch 'filter-refs' ([`3773b92`](https://github.com/Byron/gitoxide/commit/3773b92b8372c9a40a74d281149ca65b057a7da9))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - thanks clippy ([`74a5f22`](https://github.com/Byron/gitoxide/commit/74a5f2262154c5cb5434c1ef2854c4ec3d839f89))
    - thanks clippy ([`016cd1f`](https://github.com/Byron/gitoxide/commit/016cd1f70a536ac95eaa8b80958110caa096d875))
    - thanks clippy ([`b8ac13e`](https://github.com/Byron/gitoxide/commit/b8ac13e5074fa08111fcef1092432ed3a2326c6e))
    - thanks clippy ([`73b405f`](https://github.com/Byron/gitoxide/commit/73b405fe70cf7d53e5e011cf69ea654f4bd96dd2))
    - make fmt ([`535e967`](https://github.com/Byron/gitoxide/commit/535e967666c6da657ff1b7eff7c64ab27cafb182))
    - Merge branch 'main' into filter-refs-by-spec ([`1f6e5ab`](https://github.com/Byron/gitoxide/commit/1f6e5ab15f5fd8d23719b13e6aea59cd231ac0fe))
    - Merge branch 'fix-522' ([`5869e9f`](https://github.com/Byron/gitoxide/commit/5869e9ff2508d5a93c07635277af8764fcb57713))
    - Release git-hash v0.9.9 ([`da0716f`](https://github.com/Byron/gitoxide/commit/da0716f8c27b4f29cfff0e5ce7fcb3d7240f4aeb))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into filter-refs-by-spec ([`cef0b51`](https://github.com/Byron/gitoxide/commit/cef0b51ade2a3301fa09ede7a425aa1fe3527e78))
    - Release git-object v0.20.3, git-ref v0.15.4, git-config v0.7.1, git-diff v0.18.0, git-traverse v0.16.3, git-pack v0.22.0, git-odb v0.32.0, git-url v0.7.3, git-transport v0.19.3, git-protocol v0.19.1, git-refspec v0.1.1, git-repository v0.23.0, safety bump 6 crates ([`85a3bed`](https://github.com/Byron/gitoxide/commit/85a3bedd68d2e5f36592a2f691c977dc55298279))
    - Release git-features v0.22.3, git-revision v0.4.4 ([`c2660e2`](https://github.com/Byron/gitoxide/commit/c2660e2503323531ba02519eaa51124ee22fec51))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge branch 'remote-ls-refs' ([`39d585d`](https://github.com/Byron/gitoxide/commit/39d585d9f9ac6f3ecf51359c8e37f0a50e21ed45))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge branch 'main' into remote-ls-refs ([`c82bbfa`](https://github.com/Byron/gitoxide/commit/c82bbfaddc45bf9b5b55f056613046d977d9ef09))
    - prepare for release of git-repository ([`8aa5389`](https://github.com/Byron/gitoxide/commit/8aa5389d5a1bdd3a07f1caa1c2f55c8af4f9844a))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge branch 'main' into write-index-v2 ([`a938986`](https://github.com/Byron/gitoxide/commit/a938986877302c197d1aed087594c5605416fe5f))
    - Merge branch 'main' into remote-ls-refs ([`de61c4d`](https://github.com/Byron/gitoxide/commit/de61c4db7855d6925d66961f62ae3d12cc4acf78))
    - thanks clippy ([`4bd747c`](https://github.com/Byron/gitoxide/commit/4bd747cb3e126fe5b1d540270cfbd731cffd42ef))
    - thanks clippy ([`b9e1cdb`](https://github.com/Byron/gitoxide/commit/b9e1cdbf19045816387d922abd9c886419ff6bf2))
    - Merge branch 'parse-refspec' ([`2ba338e`](https://github.com/Byron/gitoxide/commit/2ba338e28eb45d4d3215dd6ff9882611880d4cd9))
    - thanks clippy ([`6c963b0`](https://github.com/Byron/gitoxide/commit/6c963b022e48854001353f8909c3e8314c2e5861))
    - thanks clippy ([`b62ee56`](https://github.com/Byron/gitoxide/commit/b62ee56b5bf468b9673b01034ec284faf1a7c7c2))
    - Release git-refspec v0.0.0 ([`d406689`](https://github.com/Byron/gitoxide/commit/d406689f01c8fa7cc81b52d6500a44303e719ec2))
</details>

## 0.7.1 (2023-01-10)

A maintenance release without user-facing changes.

## 0.7.0 (2023-01-09)

A maintenance release without user-facing changes.

## 0.6.0 (2022-12-30)

A maintenance release without user-facing changes.

## 0.5.0 (2022-12-19)

A maintenance release without user-facing changes.

## 0.4.0 (2022-11-21)

### New Features (BREAKING)

 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `gix-features` and `gix-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

## 0.3.1 (2022-11-06)

### New Features

 - <csr-id-21b21b6c25e1d8d1da9464b7bef06a795f679210/> add `RefSpecRef::expand_prefix()` method to avoid missing prefixes.
   The current implementation might cause refspecs to end up ignored as
   they don't have a prefix, and in protocol V2 it would then fail
   to add a ref-spec filter which causes them to be missed.
   
   With `expand_prefix()`, we assure that there are all possible prefixes
   that can contain partial names, similar to what git does.

### Bug Fixes

 - <csr-id-d53ddcde948cfbd7773eb830cbb636626b32debb/> `HEAD` may now return itself as prefix in `RefSpecRef::prefix()` and `expanded_prefix()`.
   Previously, the expanded prefix would be a list of possibilities, even
   though it's such a common case that we really want to avoid spamming the
   remote about it when asking for HEAD during clone for instance.

## 0.3.0 (2022-10-10)

### New Features

 - <csr-id-d7f63a6c60a826dc862bd13adbef041e4ac6d8ab/> `RefSpec::allow_non_fast_forward()` to get information about 'force' quickly.
 - <csr-id-6df179b5cf831402444cc78429a57f835358376e/> `RefSpecRef::prefix()` to return the two-component prefix of a refspec's source. #(450)

### Bug Fixes

 - <csr-id-278ff7a6ee084ea864193a5ca25b6cd0f18e19a0/> `RefSpecRef` instruction uses the correct lifetime.

### Changed (BREAKING)

 - <csr-id-2a0a87a04e7b4d6ed3be3d8adc89917576727686/> remove lifetime of `match_group::Fix`, keeping `RefSpec` instances instead
   That lifetime unnecessarily complicated things and wasn't worth keeping
   due to being a premature optimization.

## 0.2.0 (2022-09-20)

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

## 0.1.1 (2022-08-28)

Maintenance release without user-facing changes.

## 0.1.0 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

## 0.0.0 (2022-08-05)

Initial release for name reservation.

