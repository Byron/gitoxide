# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.2.0 (2022-04-13)

### Changed (BREAKING)

 - <csr-id-6ce3611891d4b60c86055bf749a1b4060ee2c3e1/> `parse()` returns a `Pattern`.
   This is much more ergonomic as this is the only things we are ever
   interested in for matching. If necessary, from there one can also
   use the parts individually or alter them.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 50 commits contributed to the release over the course of 6 calendar days.
 - 6 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#301](https://github.com/Byron/gitoxide/issues/301)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - `parse()` returns a `Pattern`. ([`6ce3611`](https://github.com/Byron/gitoxide/commit/6ce3611891d4b60c86055bf749a1b4060ee2c3e1))
    - docs for git-glob ([`8f4969f`](https://github.com/Byron/gitoxide/commit/8f4969fe7c2e3f3bb38275d5e4ccb08d0bde02bb))
    - all wildmatch tests succeed ([`d3a7349`](https://github.com/Byron/gitoxide/commit/d3a7349b707911670f17a92a0f82681544ebc769))
    - add all character classes sans some of the more obscure ones ([`538d41d`](https://github.com/Byron/gitoxide/commit/538d41d51d7cdc472b2a712823a5a69810f75015))
    - frame for character classes ([`6b8d0d2`](https://github.com/Byron/gitoxide/commit/6b8d0d20b449f6adffd403d0555596041a6c1903))
    - fix all remaining bracket tests… ([`3afe2d2`](https://github.com/Byron/gitoxide/commit/3afe2d2b862c9a22b90cbfbf75da6c84ca91ebf4))
    - more bracket-range tests succeed ([`c64f71c`](https://github.com/Byron/gitoxide/commit/c64f71c38ff404e9c9f150e3e6d3e02ca11e9235))
    - make bracket matching work better ([`97aa9ed`](https://github.com/Byron/gitoxide/commit/97aa9ed22ccb927147a1e456ee6e3510ecc9f90a))
    - refactor ([`fa0440f`](https://github.com/Byron/gitoxide/commit/fa0440fb3c80f8052e08526cf260e929722ccf02))
    - first steps towards bracket matching ([`54fe029`](https://github.com/Byron/gitoxide/commit/54fe0294e36e6ae9a025ef8661d5e21fd488dc87))
    - adjust wildmatch corpus expectations as it won't match our preprocessor ([`48990af`](https://github.com/Byron/gitoxide/commit/48990af81110a411ad07e199916005a8885db920))
    - fix another issue around double-star ([`d15c2fb`](https://github.com/Byron/gitoxide/commit/d15c2fb0119edc7635efc174a703101e100c0b4c))
    - fix another special case ([`09095df`](https://github.com/Byron/gitoxide/commit/09095dfb123f419a3df715d48e60e1f8ec62d060))
    - fix double-star matches ([`43371b6`](https://github.com/Byron/gitoxide/commit/43371b6fa0d6e62d9cde0399f1c9dd3e76b95d99))
    - fix single-level double-star ([`e5a7995`](https://github.com/Byron/gitoxide/commit/e5a79951dc32d336ae5b6c4230b3058ed80456d6))
    - fix backslash handling; improve star handling ([`7907cb4`](https://github.com/Byron/gitoxide/commit/7907cb4e12b56bdbea6abdc59f1022a508a83c87))
    - new wildcard tests to help fix star matching ([`d21c654`](https://github.com/Byron/gitoxide/commit/d21c6541959b0fe34a3882ffcb9e657d6c685734))
    - All our simple wildmatches are working, a good start ([`321c4d2`](https://github.com/Byron/gitoxide/commit/321c4d2011617f2b13e29109cafe4566e53bfde3))
    - maybe even working double-star handling ([`48c57ff`](https://github.com/Byron/gitoxide/commit/48c57ff3299928fd427bfae3e4eeadf5a9ca8109))
    - slowly move towards star/double-star ([`4efd215`](https://github.com/Byron/gitoxide/commit/4efd21560c754062f09870d253b6a2809cb0efb1))
    - question mark support ([`e83c8df`](https://github.com/Byron/gitoxide/commit/e83c8df03e801e00571f5934331e004af9774c7f))
    - very basic beginnings of wildmatch ([`334c624`](https://github.com/Byron/gitoxide/commit/334c62459dbb6763a46647a64129f89e27b5781b))
    - fix logic in wildmatch tests; validate feasibility of all test cases ([`1336bc9`](https://github.com/Byron/gitoxide/commit/1336bc938cc43e3a2f9e47af64f2c9933c9fc961))
    - test corpus for wildcard matches ([`bd8f95f`](https://github.com/Byron/gitoxide/commit/bd8f95f757e45b3cf8523d3e11503f4571461abf))
    - frame for wildmatch function and its tests ([`04ca834`](https://github.com/Byron/gitoxide/commit/04ca8349e326f7b7505a9ea49a401565259f21dc))
    - more tests for early exit in case no-wildcard prefix doesn't match ([`1ff348c`](https://github.com/Byron/gitoxide/commit/1ff348c4f09839569dcd8bb93699e7004fa59d4a))
    - more non-basename shortcuts, and only wildcard matches left ([`45c6259`](https://github.com/Byron/gitoxide/commit/45c62597b50c3c4bac34e20cd2040b08833584cc))
    - make much clearer how base-path works and put in safe-guards ([`5bf503a`](https://github.com/Byron/gitoxide/commit/5bf503af86ce0dd4d0a79c9b1a451cf89b494a6e))
    - test that bases are ignored for basenames ([`1b26848`](https://github.com/Byron/gitoxide/commit/1b2684892419f234e6006b0f3820341f162dc28b))
    - refactor ([`056b368`](https://github.com/Byron/gitoxide/commit/056b3683eb2d4d4c478ae2655e6ef067d4d0d1e7))
    - a way to set a globs base path ([`3d58db8`](https://github.com/Byron/gitoxide/commit/3d58db8a9abfb91600216b8fc6f4109f5289d776))
    - get to the point where globs probably should have a base ([`2632988`](https://github.com/Byron/gitoxide/commit/263298876d1b10b12011c2a221b67126d6d8202d))
    - refactor ([`f2f3f53`](https://github.com/Byron/gitoxide/commit/f2f3f53574b4c0b5ba85780b134825f9128fa64f))
    - prepare for handling absolute patterns ([`df9778b`](https://github.com/Byron/gitoxide/commit/df9778b924610f6a82d93cdf12cfddda60e61789))
    - Keep track of absolute patterns, those that have to start with it ([`3956480`](https://github.com/Byron/gitoxide/commit/3956480e6fb5f4766a67ebf2860cae2f48125594))
    - basename parsing with simple pattern skips ([`d18ef14`](https://github.com/Byron/gitoxide/commit/d18ef14e7cbf9c6d316086d6c88b5676c4b7516c))
    - git-baseline now acts like a massive regression test ([`fe3d0a7`](https://github.com/Byron/gitoxide/commit/fe3d0a778210a46d46a7db15cc8d213706e45fee))
    - adjust signatures to know enough to implement git-like matching ([`b947ff9`](https://github.com/Byron/gitoxide/commit/b947ff9d2c5ae8810547066096c91c745d1466fe))
    - refactor; roughly sort regex by simplicity ([`a7c3a63`](https://github.com/Byron/gitoxide/commit/a7c3a630cd5661f26220b494f01e50c9f2dbd2e2))
    - Also parse the position of the first wildcard ([`4178a63`](https://github.com/Byron/gitoxide/commit/4178a6356ad11013ae08b6233de2bfb366bf4278))
    - prepare for upcoming wildcard-length field in glob pattern ([`a11f5d4`](https://github.com/Byron/gitoxide/commit/a11f5d441a22b844caefd31b9cb7783dd6b048ad))
    - refactor ([`f285ca0`](https://github.com/Byron/gitoxide/commit/f285ca03094655590d7014770ffb6f6a77d02289))
    - basic infrastructure for running git-baseline against our implementation ([`027869d`](https://github.com/Byron/gitoxide/commit/027869d57bd7fcb7234e814d1a22197cb64c05cf))
    - baseline tests for matches and no-matches ([`621c2ca`](https://github.com/Byron/gitoxide/commit/621c2cac7eed822cc8226c7b9aa8becf3db6872c))
    - bring in all ~140 tests for git pattern matching, git-ignore styile ([`f9ab830`](https://github.com/Byron/gitoxide/commit/f9ab830df2920387c1cffec048be3a4089f4aa40))
    - refactor ([`dbe7305`](https://github.com/Byron/gitoxide/commit/dbe7305d371c7dad02d8888492b60b882b418a46))
    - refactor ([`8a54341`](https://github.com/Byron/gitoxide/commit/8a543410e10326ce506b8a7ba65e662641835849))
 * **Uncategorized**
    - thanks clippy ([`b1a6100`](https://github.com/Byron/gitoxide/commit/b1a610029e1b40600f90194ce986155238f58101))
    - thanks clippy ([`1393403`](https://github.com/Byron/gitoxide/commit/1393403b826cf4a2fbaf6ef58d505c5c62fd5e0a))
    - thanks clippy ([`683233e`](https://github.com/Byron/gitoxide/commit/683233e86dab36cc438bed0f8b0338eb767f57a0))
</details>

## 0.1.0 (2022-04-07)

Initial release with pattern parsing functionality.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#301](https://github.com/Byron/gitoxide/issues/301)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - prepare changelog prior to release ([`2794bb2`](https://github.com/Byron/gitoxide/commit/2794bb2f6bd80cccba508fa9f251609499167646))
    - Add git-glob crate with pattern matching parsing from git-attributes::ignore ([`b3efc94`](https://github.com/Byron/gitoxide/commit/b3efc94134a32018db1d6a2d7f8cc397c4371999))
 * **Uncategorized**
    - Release git-glob v0.1.0 ([`0f66c5d`](https://github.com/Byron/gitoxide/commit/0f66c5d56bd3f0febff881065911638f22e71158))
</details>

