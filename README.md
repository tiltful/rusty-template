<div align="center">

<h1>{{project-name}}</h1>

{{description}}

[![CI](https://github.com/{{github_username}}/{{project-name}}/actions/workflows/ci.yml/badge.svg)](https://github.com/{{github_username}}/{{project-name}}/actions/workflows/ci.yml)
[![License](https://img.shields.io/github/license/{{github_username}}/{{project-name}})](LICENSE)

</div>

## Features

- Rust 2024, stable toolchain and unsafe-code prohibition
- Selectable Tokio, tracing, Axum, SQLx, Redis, reqwest and config dependencies
- Structured JSON logs with environment filtering and tracing-error context
- Axum starter with gzip compression, HTTP request tracing and graceful shutdown
- Optional integration bootstrap examples for environment config, SQLx/PostgreSQL types, Redis clients and rustls-based reqwest
- Binary and library projects receive only relevant tasks and assets
- cargo-chef multi-stage Docker build with minimal scratch runtime
- Formatting, Clippy, tests, security auditing, CycloneDX SBOM, changelog and cross-platform GitHub Releases

## Getting started

Install Rust using [rustup](https://rustup.rs/) and cargo-make:

```sh
cargo install cargo-make
```

Generate a project with the interactive prompts:

```sh
cargo generate --git https://github.com/tiltful/rusty-template
```

Or use a local checkout while developing the template:

```sh
cargo generate --path .
```

Then build and test:

```sh
cargo make quality
```

For the full local CI suite, install the optional security and SBOM tools. The tasks create `Cargo.lock` before auditing dependencies, so a freshly generated project works without a committed lockfile:

```sh
cargo make ci
```

Install optional security and release tools individually when needed:

```sh
cargo install --locked cargo-audit
cargo install --locked cargo-deny
cargo install cargo-cyclonedx
cargo install git-cliff
cargo install cargo-release
```

Install the commit hooks with [pre-commit](https://pre-commit.com/). The template validates commit headers, optional bodies, trailer footers, and breaking-change markers:

```sh
pre-commit install --hook-type pre-commit --hook-type commit-msg
```

For binary projects, push a version tag such as `v0.1.0` to build Linux (GNU and musl), Windows and macOS binaries and publish a GitHub Release.

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
