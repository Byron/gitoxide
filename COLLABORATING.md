
### On how to work together

- **we do trunk-based development** with a _twist_
   - Long lasting feature branches are discouraged, and instead small commits to `main` are preferred.
   - **use preferably short-lived PRs when…**
     - …you potentially want feedback on changes
     - …changes would break _known_ downstream users, to allow those who are affected to influence the outcome towards greater value.
   - **use issues for analysis and discussion of features and to track PRs…**
     - …if collaboration on a feature and prior feedback is desired
   - **feel free to use the [project-board] to organize your issues, PRs or cards**
- **`main` must never be broken or show warnings**
   - An easy way to achieve this is to run `make tests check-size` before pushing or `make tests check-size && git push`.
   - If you're unsure about remembering to do this, we suggest using a pre-commit git hook.
- **if `main` breaks on CI** _which can happen nonetheless_…
    - …and you _do know_ the cause, please fix it immediately. If necessary by reverting the offending commit until a more durable fix is possible.
    - …and you _do not know_ the cause, please open a PR to invite collaborators for their input. This is to avoid multiple collaborators 
      trying to fix the issue independently, causing merge-conflicts and confusion. We use this PR as synchronization primitive.
- **for crates _you own_**
    - feel free to make any kind of changes to it, including major ones.
    - please use `cargo smart-release` for publishing to crates.io as it will handle dependencies properly.
- **for crates _you do not own_**
    - for major or architectural changes please open a [discussion], an issue or a PR to allow
      participation and don't merge until there is agreement.
    - for minor code changes, if they are minor, feel free to make any kind of change you need.

The workflow can be changed after public discussion - to get started, open a PR.


Please see the [development guide] for more detailed information on how code and cargo manifests are structured.

[development guide]: https://github.com/Byron/gitoxide/blob/main/DEVELOPMENT.md
[project-board]: https://github.com/Byron/gitoxide/projects
[discussions]: https://github.com/Byron/gitoxide/discussions

