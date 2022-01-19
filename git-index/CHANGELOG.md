# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.0 (2022-01-19)

The initial release which can read a complete index, version 2 to 4, with all extensions.
The reading can be performed with multiple threads as well, partially depending on whether
certain extensions are present.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 71 commits contributed to the release over the course of 490 calendar days.
 - 509 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#293](https://github.com/Byron/gitoxide/issues/293)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - prepare changelogs for git-index and dependencies ([`f54bf4b`](https://github.com/Byron/gitoxide/commit/f54bf4bde92b892b6d425987a6a37e10319c4635))
    - Test for extra-long paths ([`3d61afe`](https://github.com/Byron/gitoxide/commit/3d61afe615227e2af96525eba5b0e8e2f94207f3))
    - Test for extended flags ([`ae3b697`](https://github.com/Byron/gitoxide/commit/ae3b69710cf316cb8164120d4b98f051eef363bc))
    - Use bitflags for Flags (in-memory and at-rest) ([`ea86eb0`](https://github.com/Byron/gitoxide/commit/ea86eb0bebb0f067fc8710779c2c296632451c54))
    - Use bitflags for entry Mode ([`53df605`](https://github.com/Byron/gitoxide/commit/53df6057a8c50007716d89e08f1efe70435f8613))
    - FSMN V2 decoding ([`04279bf`](https://github.com/Byron/gitoxide/commit/04279bffc8bd43abe85f559634436be789782829))
    - Failing test for fs-monitor V1 ([`625b89a`](https://github.com/Byron/gitoxide/commit/625b89a7786fe9de29c9ad2ca41a734174f53128))
    - Validate UNTR with exclude-file oids ([`20ebb81`](https://github.com/Byron/gitoxide/commit/20ebb81c9ece6c2d693edd6243eaa730fa7cf44c))
    - read remaining pieces of UNTR ([`9d9cc95`](https://github.com/Byron/gitoxide/commit/9d9cc95a24d86cf5f66f1746c09ece032640a892))
    - Make stat parsing more general/reusable ([`c41b933`](https://github.com/Byron/gitoxide/commit/c41b933d14f2e4538928e4fbd682e1017702e69c))
    - refactor ([`a1dc8de`](https://github.com/Byron/gitoxide/commit/a1dc8dedc5d9e1712295131d2332c21f3df4ac45))
    - simplify UNTR directory indexing ([`7857d08`](https://github.com/Byron/gitoxide/commit/7857d08a25eac1c7d4a91f04eb80a83ec5677d1b))
    - flatten UNTR directory list for later access via bitmaps ([`2e39184`](https://github.com/Byron/gitoxide/commit/2e391841af52f88b7a0472179e5dda89cc6c9808))
    - read UNTR directory blocks and bitmaps ([`59f46fe`](https://github.com/Byron/gitoxide/commit/59f46fe134e8619f501c79da4290cadd5548751c))
    - First portion of reading the untracked cache ([`ed2fe5d`](https://github.com/Byron/gitoxide/commit/ed2fe5dbfbcf79ffdcdceed90f6321792cff076d))
    - failing test for UNTR extension ([`223f2cc`](https://github.com/Byron/gitoxide/commit/223f2cc1c88f76dc75ca6706f1f61514ab52e496))
    - Add UNTR extension fixture ([`3c7ba24`](https://github.com/Byron/gitoxide/commit/3c7ba247a3fdab114d9d549de50d6143c7fcce0a))
    - REUC reading works ([`29c1af9`](https://github.com/Byron/gitoxide/commit/29c1af9b2d7b9879a806fc84cfc89ed6c0d7f083))
    - frame and test for REUC exstension ([`229cabe`](https://github.com/Byron/gitoxide/commit/229cabe8de9e1bd244d56d24327b05e3d80dfb6e))
    - add git index with REUC exstension ([`8359fdb`](https://github.com/Byron/gitoxide/commit/8359fdb6c49b263bc7ac2f3105254b83eac47638))
    - Support for 'sdir' extension ([`a38c3b8`](https://github.com/Byron/gitoxide/commit/a38c3b889cfbf1447c87d489d3eb9902e757aa4b))
    - Turn git-bitmap Array into Vec, as it will be able to adjust its size ([`9e99e01`](https://github.com/Byron/gitoxide/commit/9e99e016c17f0d5bcd2ab645261dfac58cb48be5))
    - first stab at decoding ewah bitmaps ([`353a53c`](https://github.com/Byron/gitoxide/commit/353a53ccab5af990e7c384b74b38e5429417d449))
    - 'link' extension decoding to the point where bitmaps are needed ([`e18a2fd`](https://github.com/Byron/gitoxide/commit/e18a2fd68e1c7c06fe2ff9cd1634704313822b5c))
    - support for errors in extensions ([`8971991`](https://github.com/Byron/gitoxide/commit/8971991808b1497b9584b52270259d3c625fa970))
    - failing test for link decoding ([`e1daf18`](https://github.com/Byron/gitoxide/commit/e1daf18041862570fdfe53ff88d879d0c3cb7182))
    - don't forget to fail on unknown mandatory extension ([`f7e2bdd`](https://github.com/Byron/gitoxide/commit/f7e2bdd7dcb21600d2aca7d29d131eefe43f0f4f))
    - Aggregation for index entries loaded in parallel ([`995994a`](https://github.com/Byron/gitoxide/commit/995994a895a6faa4537ae1a6564edc005be96a1a))
    - parallel loading of entries right before reducing them ([`de84a3a`](https://github.com/Byron/gitoxide/commit/de84a3a03bcc9dc3ff71810e35c869f9b73dd38f))
    - Frame for using the new 'scoped threads' feature in git-features ([`6fea17d`](https://github.com/Byron/gitoxide/commit/6fea17d1306679d0454d01aa59adf12cd83c7973))
    - single and multi-threaded index tests ([`a22cb0f`](https://github.com/Byron/gitoxide/commit/a22cb0f1ead9a2f32e43eb2fb378281e592a4ed3))
    - prepare decode options for better control of threads ([`30de988`](https://github.com/Byron/gitoxide/commit/30de988f6a97177fcb32ffce37f4c80f46306a20))
    - cleanup ([`99d7224`](https://github.com/Byron/gitoxide/commit/99d7224baa04c199a7eb7aa2675b39657b0aef6a))
    - Basic IEOT parsing ([`35bdee4`](https://github.com/Byron/gitoxide/commit/35bdee4bf77787bcbe6c3dd715a677e2e46a8ad1))
    - refactor ([`6f04f8b`](https://github.com/Byron/gitoxide/commit/6f04f8b8276de9c6b649642fb7c95eb5ffad77e4))
    - parse V4 delta-paths ([`06640e3`](https://github.com/Byron/gitoxide/commit/06640e3f98f25e9502db7ac68e1967d9fd25e8b2))
    - more thorough tests for more complex repo with more entries ([`273853f`](https://github.com/Byron/gitoxide/commit/273853f1614a0106c60d3d73c3bf72fb57b405e8))
    - The first test to validate an entry ([`f865ef6`](https://github.com/Byron/gitoxide/commit/f865ef6c626c9db39a09416333b6465fdd12c734))
    - Now with counting of consumed bytes in extensions ([`77a062c`](https://github.com/Byron/gitoxide/commit/77a062cdaff1bdf80556301f1e1aa41002af9cef))
    - Use correct post-header slice when parsing entries ([`da556b0`](https://github.com/Byron/gitoxide/commit/da556b0a64ac9ca8eaee62cab163789b55903b3d))
    - All code needed to load extensions… ([`0a03f19`](https://github.com/Byron/gitoxide/commit/0a03f196b7ec4dc1e0e2377c729467781c9e6c2c))
    - a step towards pasing V2 paths ([`01036ad`](https://github.com/Byron/gitoxide/commit/01036ad1bafb6a830734a9dd4f4e2949b8981a30))
    - Most of the entry decoding, name is still missing ([`53e2d75`](https://github.com/Byron/gitoxide/commit/53e2d754262d9752d3b106f7991543986ad5426f))
    - Extensions are optional, and so is their iteration ([`620d2e6`](https://github.com/Byron/gitoxide/commit/620d2e6bd4ef6d3281c096aaf344669bcf49e723))
    - Prepare a more complex test for tree parsing, requires entry parsing ([`e7e0679`](https://github.com/Byron/gitoxide/commit/e7e067977ef440cf3edb8812c0d614b5d8213b58))
    - parse TREE chunk ([`a2ea498`](https://github.com/Byron/gitoxide/commit/a2ea49841a333c8af18fd258781a649214a0ae0b))
    - Get closer to implementing a simple TREE extension decoding ([`49fcb6f`](https://github.com/Byron/gitoxide/commit/49fcb6f6ae9d6ed47e7c0c3ea2aa644d4e8cd264))
    - refactor ([`07e8fb2`](https://github.com/Byron/gitoxide/commit/07e8fb2cb6b7819eb34676ede57808b845298674))
    - the first actual assetion ([`c17240d`](https://github.com/Byron/gitoxide/commit/c17240d0cbd6134a77a69359611789f4eebc727d))
    - refactor ([`d4b3a07`](https://github.com/Byron/gitoxide/commit/d4b3a07489703fb6d5e9b9fb9328741172826db9))
    - refactor ([`9fdd34b`](https://github.com/Byron/gitoxide/commit/9fdd34b634f4f15eb6cf5c2e7912bdc32dd61de6))
    - Fix counting issue, checksum matches now ([`cc33752`](https://github.com/Byron/gitoxide/commit/cc337526365a04a23571123531f1ae565d386bcf))
    - Another big step, even though EOIE checksum is still bugged ([`9ffd523`](https://github.com/Byron/gitoxide/commit/9ffd5231c582a3870c6d25ea870c005e77e32276))
    - right before implementing a traversal over extension chunks ([`79ca582`](https://github.com/Byron/gitoxide/commit/79ca582045dd03434737c779b84c991acf1b0823))
    - refactor ([`9b28b18`](https://github.com/Byron/gitoxide/commit/9b28b18262c763608d60fba65e91fcb9ca3ddb3e))
    - first step towards reading the EOIE extension ([`068c716`](https://github.com/Byron/gitoxide/commit/068c716b46699234d6ad1db70be34b894e61d76a))
    - parse index header ([`5c731f8`](https://github.com/Byron/gitoxide/commit/5c731f831d007a4fe099cadc4ecaab113ab7e08a))
    - first stab at basic index file parsing ([`826ca0c`](https://github.com/Byron/gitoxide/commit/826ca0c6a6801ec2a67ca73ac17092e5f85fe9ce))
    - refactor ([`494ed46`](https://github.com/Byron/gitoxide/commit/494ed46acc54bd342f891416918032a2c4848cf1))
    - git-index uses memmap2 ([`fbfea28`](https://github.com/Byron/gitoxide/commit/fbfea28d2c9ed92e270c6a5aa603d3c84769ae8f))
    - The realization that FileBuffer really shouldn't be used anymore ([`b481f13`](https://github.com/Byron/gitoxide/commit/b481f136c4084b8839ebecb604dea5aa30d3a44e))
    - base setup for index testing ([`aa60fdf`](https://github.com/Byron/gitoxide/commit/aa60fdf3d86e08877c88f9e4973f546642ed1370))
    - notes on how test indices have been created ([`3040857`](https://github.com/Byron/gitoxide/commit/3040857ec4d2e0557b4920eaa77ddc4292d9adae))
 * **Uncategorized**
    - thanks clippy ([`09df2bc`](https://github.com/Byron/gitoxide/commit/09df2bcb4b45f72742d139530907be8aa4dc36f8))
    - thanks clippy ([`93c3d23`](https://github.com/Byron/gitoxide/commit/93c3d23d255a02d65b5404c2f62f96d94e36f33d))
    - Fix index without extension test & thanks clippy ([`066464d`](https://github.com/Byron/gitoxide/commit/066464d2ad2833012fa196fe41e93a54ab05457f))
    - thanks clippy ([`f477032`](https://github.com/Byron/gitoxide/commit/f47703256fe6a5c68ed3af6705bcdf01262500d6))
    - thanks clippy ([`5526020`](https://github.com/Byron/gitoxide/commit/552602074a99dc536624f0c6295e807caf32f58b))
    - thanks clippy ([`591511a`](https://github.com/Byron/gitoxide/commit/591511a739f91c5e8ff4243059ac98052a44c914))
    - remove dash in all repository links ([`98c1360`](https://github.com/Byron/gitoxide/commit/98c1360ba4d2fb3443602b7da8775906224feb1d))
    - refactor ([`e4bcfe6`](https://github.com/Byron/gitoxide/commit/e4bcfe6406b14feffa63598c7cdcc8ecc73222bd))
</details>

## v0.0.0 (2020-08-28)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - add placeholder for git-index crate ([`52ff13c`](https://github.com/Byron/gitoxide/commit/52ff13cf260b53423faf59e5c666ff1565bde947))
</details>

