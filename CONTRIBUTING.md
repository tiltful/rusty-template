# Contributing

Thanks for contributing!

## Development setup

Install stable Rust with [rustup](https://rustup.rs/), then install the project tools as needed:

- cargo-make: `cargo install cargo-make`
- pre-commit: follow the [pre-commit installation guide](https://pre-commit.com/#installation)
- cargo-audit: `cargo install cargo-audit`
- cargo-deny: `cargo install cargo-deny`
- git-cliff: `cargo install git-cliff`
- cargo-release: `cargo install cargo-release`
- cargo-nextest: `cargo install cargo-nextest --locked`

Install the Git hooks:

```sh
pre-commit install --hook-type pre-commit --hook-type commit-msg
```

Run `cargo make quality` before opening a pull request. Run `cargo make ci` when all optional tooling (cargo-audit, cargo-deny and git-cliff) is installed.\n\nThe commit-message validator is covered by `python3 -m unittest discover -s tests -p 'test_*.py'`.

## Commit messages

Use **Conventional Emoji Commits**. Put the Gitmoji first, then the Conventional Commit type and optional scope:

- `✨ feat(api): add endpoint`
- `🩹 fix(config): handle missing value`
- `📚 docs: update setup guide`
- `♻️ refactor(parser): simplify parser`

A longer body is optional and begins after one blank line. Footers are optional and begin after another blank line. Use trailer syntax:

```text
✨ feat(api): add endpoint

Expose the new endpoint and document its response.

Reviewed-by: Tilt
Refs: #123
```

For a breaking change, add `❗` before the colon or use the footer
`🚨 BREAKING CHANGE: description`.

Keep commits focused and write the subject in the imperative mood.
