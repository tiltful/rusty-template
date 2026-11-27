<div align="center">

<h1>{{project-name}}</h1>

{{description}}

[![CI](https://github.com/{{github_username}}/{{project-name}}/actions/workflows/ci.yml/badge.svg)](https://github.com/{{github_username}}/{{project-name}}/actions/workflows/ci.yml)
[![License](https://img.shields.io/github/license/{{github_username}}/{{project-name}})](LICENSE)

</div>

## Features

- Rust 2024, stable toolchain and unsafe-code prohibition
- Selectable minimal, modular and DDD architecture profiles
- Cloneable application state, use-case boundaries, domain invariants and repository ports when an architecture profile is selected
- Selectable Tokio, tracing, Axum, SQLx, Redis, reqwest and config dependencies
- Structured JSON logs with environment filtering and tracing-error context
- Axum starter with gzip compression, HTTP request tracing and graceful shutdown
- Typed configuration with optional `config.toml`, `.env` loading, `APP_PORT` environment overrides and validated `u16` port values
- Optional SQLx/PostgreSQL, Redis and rustls-based reqwest client setup
- Binary and library projects receive only relevant tasks and assets
- cargo-chef multi-stage Docker build with minimal scratch runtime
- Formatting, Clippy, tests, security auditing, CycloneDX SBOM, changelog and cross-platform GitHub Releases

## Architecture profiles

Choose an architecture profile during generation:

- `minimal` keeps a single entrypoint and adds no architectural ceremony.
- `modular` separates application logic, presentation mapping and cloneable process state.
- `ddd` demonstrates a small domain model with validated value objects, a repository port, an application use case, an infrastructure adapter and shared application state.

These profiles use the standard library only. Axum can be enabled independently; when selected with `modular` or `ddd`, the generated route receives the typed `AppState` through Axum's `State` extractor. Treat the examples as a starting boundary, not a mandate to create a layer for every function.

## Configuration

When `config + dotenvy` is selected, the application loads optional `config.toml`, then `.env`, then process environment variables. Later sources take precedence, so environment variables override file values. The HTTP port is exposed as `APP_PORT` (default `3000`); the generated `.env.example` also includes `PORT` for projects that do not enable the config integration. The port is deserialized as `u16`, so malformed and out-of-range values fail at startup rather than being silently accepted. Copy `.env.example` to `.env` for local development. The Docker image includes the generated `config.toml` when this integration is selected; provide secrets through runtime environment variables, never by committing them.

Example `config.toml`:

```toml
port = 3000
```

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
cargo install cargo-machete
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
