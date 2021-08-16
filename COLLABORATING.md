
### Guide

- we do trunk based development
- `main` must never be broken or show warnings. An easy way to achieve this is to run `make tests check-size` before pushing.
- if `main` breaks which can happen nonetheless, please fix the issue right away.
- for crates **you own**
    - feel free to make any kind of changes to it, including major ones.
    - please use `cargo smart-release` for publishing to crates.io as it will handle dependencies properly.
- for crates **you do not own**
    - for major or architectural changes please open a disccussion, an issue or a PR to allow
      participation and don't merge until there is agreement.
    - for minor code changes, if they are minor, feel free to make any kind of change you need.

The workflow can be changed after public discussion, just open a PR for it to get started.


