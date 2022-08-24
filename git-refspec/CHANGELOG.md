# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.0 (2022-08-24)

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 51 commits contributed to the release over the course of 18 calendar days.
 - 18 days passed between releases.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
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
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
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

