<div align="center">

<h1>{{project-name}}</h1>

{{description}}

[![CI](https://github.com/{{github_username}}/{{project-name}}/actions/workflows/ci.yml/badge.svg)](https://github.com/{{github_username}}/{{project-name}}/actions/workflows/ci.yml)
[![License](https://img.shields.io/github/license/{{github_username}}/{{project-name}})](LICENSE)

</div>

## Features

- Rust 2024, stable toolchain and unsafe-code prohibition
- Selectable Tokio, tracing, Axum, SQLx, Redis, reqwest and config dependencies
- Binary and library projects receive only relevant tasks and assets
- cargo-chef multi-stage Docker build with minimal scratch runtime
- Formatting, Clippy, tests, security auditing, CycloneDX SBOM, changelog and cross-platform GitHub Releases

## Getting started

Install Rust using rustup, then run:

```sh
cargo build
cargo test
cargo make quality
```

Install cargo-make with `cargo install cargo-make`.
Install optional security/release tools as needed: `cargo install --locked cargo-audit cargo-deny git-cliff cargo-release` and `cargo install cargo-cyclonedx`.

Install the commit hooks with `pre-commit install --hook-type pre-commit --hook-type commit-msg`.

For binary projects, push a version tag such as `v0.1.0` to build Linux, Windows and macOS binaries and publish a GitHub Release.

## Commit messages

Use Gitmoji + Conventional Commits:

```text
✨ feat(api): add endpoint

Add the endpoint and document its response format.

Closes #123
```

Format: `<gitmoji> <type>(<scope>): <description>`.
An optional body starts after one blank line. Optional footers start after
another blank line and use trailer syntax, for example `Closes: #123`.
Breaking changes use `❗` before the colon or a
`🚨 BREAKING CHANGE: ...` footer.

## License

Licensed under the {{license}} license. See [LICENSE](LICENSE).
