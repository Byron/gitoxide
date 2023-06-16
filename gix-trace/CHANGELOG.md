# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.0 (2023-06-16)

### New Features

 - <csr-id-093efafa7c39aa03bfef4894779cca6e3716f471/> add `tracing` feature toggle to provide minimal tracing API
   This API is based on `tracing-core`, not on tracing, and provides a limited
   API that is always available, while being a no-op if `tracing` isn't enabled.
   
   That way, plumbing crates can add instrumentation at will.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add changelog prior to release of `gix-trace` ([`e1305c3`](https://github.com/Byron/gitoxide/commit/e1305c34092cdb067fbe597050bba4927cff491c))
    - Change MSRV to 1.65 ([`4f635fc`](https://github.com/Byron/gitoxide/commit/4f635fc4429350bae2582d25de86429969d28f30))
    - Add `tracing` feature toggle to provide minimal tracing API ([`093efaf`](https://github.com/Byron/gitoxide/commit/093efafa7c39aa03bfef4894779cca6e3716f471))
</details>

