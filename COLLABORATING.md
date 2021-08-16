
### Guide

- we do trunk-based development.
   - Long lasting feature branches are discouraged, and instead small commits to `main` are preferred.
- `main` must never be broken or show warnings. 
   - An easy way to achieve this is to run `make tests check-size` before pushing or `make tests check-size && git push`.
   - If you're unsure about remembering to do this, we suggest using a pre-commit git hook.
- if `main` breaks on CI _which can happen nonetheless_…
    - …and you _do know_ the cause, please fix it immediately. If necessary by reverting the offending commit until a more durable fix is possible.
    - …and you _do not know_ the cause, please open a PR to invite collaborators for their input. This is to avoid multiple collaborators trying to fix the issue independently,
      causing merge-conflicts and confusion. We use this PR as synchronization primitive.
- for crates **you own**
    - feel free to make any kind of changes to it, including major ones.
    - please use `cargo smart-release` for publishing to crates.io as it will handle dependencies properly.
- for crates **you do not own**
    - for major or architectural changes please open a disccussion, an issue or a PR to allow
      participation and don't merge until there is agreement.
    - for minor code changes, if they are minor, feel free to make any kind of change you need.

The workflow can be changed after public discussion, just open a PR for it to get started.

