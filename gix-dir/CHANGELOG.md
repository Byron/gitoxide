# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.0 (2024-02-25)

### Bug Fixes

 - <csr-id-95d10ee9371196cbcb8e599d28d9d05fa8b68221/> pathspec prefixes still allows directory collapsing.
 - <csr-id-dc200bf6f2cb10b6f0e45dd83bf9f82173cbb04f/> proper submodule handling
   Previously it was possible for `.git` files in directories to
   not trigger repository detection.
 - <csr-id-c04954a89dfdd8c230050b6175e2a132c73bdbfa/> assure `Action::Cancel` doesn't run into unreachable code.

### New Features (BREAKING)

 - <csr-id-bd5f44925306aa342b2b1c547779799b72372212/> Represent `DotGit` as `ExtendedKind`
   This cleans up the model despite also making it harder to detect
   whether something is a DotGit.
 - <csr-id-b6ea37a4d20e008c0b447090992c6aade0191265/> simplify `walk()` signature to compute `root` with pathspec directory.
   This makes the overall handling more unified, while assuring it's always
   in the worktree.
   
   And as a pathspec directory isn't exactly the same as a user-specified root,
   it's also possible to override this automation.
 - <csr-id-4567dbb2abf3d05bebe2206afafc40002a376d26/> allow to emit all collapsed entries.
   This is useful for rename tracking as it allows to see all files
   that may take part in a rename (i.e. when a directory is renamed).

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 57 calendar days.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Prepare changelogs prior to release ([`f2e111f`](https://github.com/Byron/gitoxide/commit/f2e111f768fc1bc6182355261c20b63610cffec7))
    - Merge branch 'status' ([`d53504a`](https://github.com/Byron/gitoxide/commit/d53504a1fad41cec7b6ca2a4abb7f185d8941e3f))
    - Make it even harder to remove your own CWD ([`4d5767c`](https://github.com/Byron/gitoxide/commit/4d5767cd394d755104aa7f0c1ed5b8e01bf74b12))
    - Assure that we don't artificially make non-recursable directories visible ([`1a26732`](https://github.com/Byron/gitoxide/commit/1a26732fe897161f9bfa397efdb07aa57f3c7341))
    - Represent `DotGit` as `ExtendedKind` ([`bd5f449`](https://github.com/Byron/gitoxide/commit/bd5f44925306aa342b2b1c547779799b72372212))
    - Pathspec prefixes still allows directory collapsing. ([`95d10ee`](https://github.com/Byron/gitoxide/commit/95d10ee9371196cbcb8e599d28d9d05fa8b68221))
    - Merge branch 'status' ([`bb48c4c`](https://github.com/Byron/gitoxide/commit/bb48c4ce22650b8c76af3b147e252ebe7cedb205))
    - More natural top-level handling ([`44ccc67`](https://github.com/Byron/gitoxide/commit/44ccc67a5b4a481f769399c41f0d3fc956fd8ec8))
    - Simplify `walk()` signature to compute `root` with pathspec directory. ([`b6ea37a`](https://github.com/Byron/gitoxide/commit/b6ea37a4d20e008c0b447090992c6aade0191265))
    - Allow to emit all collapsed entries. ([`4567dbb`](https://github.com/Byron/gitoxide/commit/4567dbb2abf3d05bebe2206afafc40002a376d26))
    - Proper submodule handling ([`dc200bf`](https://github.com/Byron/gitoxide/commit/dc200bf6f2cb10b6f0e45dd83bf9f82173cbb04f))
    - Assure `Action::Cancel` doesn't run into unreachable code. ([`c04954a`](https://github.com/Byron/gitoxide/commit/c04954a89dfdd8c230050b6175e2a132c73bdbfa))
    - Merge branch 'status' ([`b8def77`](https://github.com/Byron/gitoxide/commit/b8def77e91ddc82a39ec342b89f558702a8f1d8c))
    - Make sure that `*foo*` prefixes don't end up matching any directory. ([`482d6f3`](https://github.com/Byron/gitoxide/commit/482d6f3f773fd74ddcea4be0b36ebea89017397a))
    - Merge branch 'dirwalk' ([`face359`](https://github.com/Byron/gitoxide/commit/face359443ba33e8985ec1525d5ec38b743ea7a9))
    - Implementation of the Git-style directory walk. ([`3252cfd`](https://github.com/Byron/gitoxide/commit/3252cfd570b0c0897c51939e1a8c45b35c861c53))
    - Merge branch 'gix-status' ([`c3983c6`](https://github.com/Byron/gitoxide/commit/c3983c6b8d63d85ec713ae8d661723f9cf0bd55b))
    - Initial version of the `gix-dir` crate ([`22acf0d`](https://github.com/Byron/gitoxide/commit/22acf0def5c62563300aa8eaef01cb94bcd15645))
</details>

