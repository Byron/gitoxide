# Contributing

We are happy to have you and help you get started. If you have questions, feel free to
[start a discussion][discussions].

## Where to start

- **Pull requests:** If you have a fix for a problem, open a PR directly. No prior issue is required.
- **Issues:** Report bugs or shortcomings only when you haven't found a solution, or the problem is too
  unclear to get started on a fix.
- **Discussions:** Start a [discussion][discussions] before implementing any feature, or work likely to
  lead to a large PR. This includes submissions expected to add or modify more than ~500 source lines of code (SLOC).

## Preparing a pull request

We recommend running `just test` during development to assure CI is green before pushing. See the
[development guide] for implementation practices.

## Prevent agent impersonation

AI agents communicating through a person's account must identify themselves, for example in issue or
PR descriptions and comments, and speak only for themselves. They must not generate statements on the
account holder's behalf, including claims that the account holder reviewed, tested, or approved the work.
The account holder must write any such statement themselves. AI assistance that does not replace the
person as the speaker, such as proofreading or wording polish, does not require identification.

Attributing AI assistance in commit metadata, for example with an `Assisted-by:` or `Co-authored-by:` trailer,
is welcome but not required.

[development guide]: https://github.com/GitoxideLabs/gitoxide/blob/main/DEVELOPMENT.md
[discussions]: https://github.com/GitoxideLabs/gitoxide/discussions
