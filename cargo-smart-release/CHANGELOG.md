### v0.4.0 (2021-09-??)

- Conservative pre-release version handling along with a flag to turn it off. See [this issue][194] for details.
- Rename `--allow-auto-publish-of-stable-crates` to `--no-auto-publish-of-stable-crates`, inverting its meaning.
- Add `--no-multi-crate-release` flag to return to previous default behaviour. All crate manifest changes are put into one commit.
- automatically bump pre-release transient dependents of published crates to prevent breakage down the road unless 
  `--no-conservative-pre-release-version-handling` is set.

[194]: https://github.com/Byron/gitoxide/issues/194

### v0.3.1 (2021-09-07) - internal improvements

### v0.3.0 (2021-08-27)

- add `--skip-dependencies` flag
- add `--verbose` flag and be less verbose in dry-runs by default to provide only essential information
- improvements to notification clarity

#### Breaking

- Use short flag for `--no-bump-on-demand` in `--bump-dependencies`

### v0.2.4 (2021-08-15)

- Fix auto-push functionality

### v0.2.3 (2021-08-15)

- Less verbosity by default which is helpful on the first run to get an overview. Use `--verbose/-v` for all the details.
- Also push tags and HEAD by default, unless `--skip-push` is specified.

### v0.2.2 (2021-08-15)

- support for unsorted packed-refs files

### v0.1.0 (2021-08-13)

- initial release
