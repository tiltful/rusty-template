<div align="center">

# {{project-name}}

<p>{{description}}</p>

[![CI](https://img.shields.io/github/actions/workflow/status/{{github_username}}/{{project-name}}/ci.yml?branch=main&style=flat-square&logo=github&label=CI)](https://github.com/{{github_username}}/{{project-name}}/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/codecov/c/github/{{github_username}}/{{project-name}}?style=flat-square&logo=codecov&logoColor=white)](https://codecov.io/gh/{{github_username}}/{{project-name}})
[![License](https://img.shields.io/github/license/{{github_username}}/{{project-name}}?style=flat-square)](LICENSE)
{% if crate_type == "bin" %}
[![Latest Release](https://img.shields.io/github/v/release/{{github_username}}/{{project-name}}?display_name=tag&sort=semver&style=flat-square)](https://github.com/{{github_username}}/{{project-name}}/releases/latest)
{% endif %}
[![Rust](https://img.shields.io/badge/rust-1.94%2B-orange?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Stars](https://img.shields.io/github/stars/{{github_username}}/{{project-name}}?style=flat-square&logo=github)](https://github.com/{{github_username}}/{{project-name}}/stargazers)
[![Forks](https://img.shields.io/github/forks/{{github_username}}/{{project-name}}?style=flat-square&logo=github)](https://github.com/{{github_username}}/{{project-name}}/network/members)
[![Contributors](https://img.shields.io/github/contributors/{{github_username}}/{{project-name}}?style=flat-square)](https://github.com/{{github_username}}/{{project-name}}/graphs/contributors)
[![Issues](https://img.shields.io/github/issues/{{github_username}}/{{project-name}}?style=flat-square&logo=github)](https://github.com/{{github_username}}/{{project-name}}/issues)
[![Last Commit](https://img.shields.io/github/last-commit/{{github_username}}/{{project-name}}?style=flat-square)](https://github.com/{{github_username}}/{{project-name}}/commits/main)

**Built with Rust · Designed for maintainability · Ready to grow**

</div>

---

## ✨ Overview

{{description}}

Generated from [rusty-template](https://github.com/tiltful/rusty-template), a batteries-included Rust starter. This project gives you a clean foundation without forcing unnecessary abstractions: choose the architecture and integrations you need, then make the codebase yours.

## 🚀 Features

- 🦀 **Modern Rust** — Rust 2024 edition, stable toolchain and unsafe-code prohibition
- 🧱 **Architecture profiles** — minimal, modular or domain-driven design (DDD)
- 🧩 **Optional integrations** — Tokio, Axum, tracing, SQLx, Redis, reqwest and typed configuration
- ⚙️ **Configuration** — optional TOML and .env files with environment-variable overrides
{% if crate_type == "bin" %}
- 📦 **Container-ready** — cargo-chef multi-stage build, static musl binary and minimal scratch runtime
- 🏷️ **Release automation** — cross-platform GitHub Releases with checksums and an SBOM
{% endif %}
- 🔍 **Quality gates** — formatting, strict Clippy, tests, documentation and dependency hygiene
- 🔐 **Security-minded** — dependency auditing, license policy and CycloneDX SBOM generation

{% if crate_type == "bin" %}
## 🏗️ Architecture

Choose the profile that fits the project:

| Profile | What you get | When to choose it |
| --- | --- | --- |
| **minimal** | A straightforward entrypoint | Small tools, prototypes and focused services |
| **modular** | Separate application, presentation and state modules | Applications expected to grow |
| **ddd** | Domain model, validated value objects, repository port, use case and infrastructure adapter | Domain-heavy applications with clear boundaries |

The examples are intentionally small. Treat them as useful boundaries to build on, not a requirement to create a layer for every function. Axum can be enabled independently; with modular and DDD profiles, routes receive typed application state.
{% else %}
## 🏗️ Library layout

The generated library starts with a small documented public API, a unit-test module and Rustdoc examples. Extend the public surface deliberately and keep implementation details private until they need to be exposed.
{% endif %}

## 🧰 Tech stack

Integrations are opt-in. Enable only what your project actually uses.

| Integration | Purpose |
| --- | --- |
| Tokio | Async runtime |
| Axum + tower-http | HTTP server, middleware and compression |
| tracing + tracing-subscriber + tracing-error | Structured, contextual application logs |
| SQLx | Async database access (PostgreSQL) |
| Redis | Cache, state and message-oriented workloads |
| reqwest | HTTP client with rustls |
| config + dotenvy | Layered application configuration |

## ⚡ Getting started

### Prerequisites

- [Rust](https://rustup.rs/) with the stable toolchain
- [cargo-generate](https://github.com/cargo-generate/cargo-generate) to create a project from this template
- [cargo-make](https://github.com/sagiegurari/cargo-make) for the task shortcuts

Install the template generator and task runner:

```sh
cargo install cargo-generate cargo-make
```

### Generate a project

Run the interactive generator:

```sh
cargo generate --git https://github.com/tiltful/rusty-template
```

Or generate from a local checkout while developing the template:

```sh
cargo generate --path .
```

The prompts let you choose the package type, architecture profile, license and optional integrations. Review the generated files and initialize your own Git repository before pushing.

### Build and test

```sh
cargo build
cargo test
```
{% if crate_type == "bin" %}
Run the application locally:

```sh
cargo run
```
{% endif %}

Run the standard local quality checks:

```sh
cargo make quality
```

For the full CI-equivalent suite, install the optional audit and SBOM tools listed below, then run:

```sh
cargo make ci
```

{% if use_config %}
## ⚙️ Configuration

Settings are loaded in this order:

1. config.toml (optional)
2. .env (optional)
3. Process environment variables (highest priority)

The HTTP port defaults to 3000 and can be overridden with APP_PORT. It is parsed as a u16, so invalid or out-of-range values fail at startup.

Copy the example environment file before adding local values:

```sh
cp .env.example .env
```

Example config.toml:

```toml
port = 3000
```

Keep secrets out of version control. Use environment variables or your deployment platform's secret manager.
{% endif %}

{% if crate_type == "bin" %}
## 🐳 Docker

The included Dockerfile uses cargo-chef for dependency planning, builds a static musl executable and copies it into a minimal scratch runtime. The container runs as a non-root user.

```sh
docker build -t {{project-name}} .
docker run --rm -p 3000:3000 {{project-name}}
```
{% endif %}

## 🧪 Quality & security

| Command | What it checks |
| --- | --- |
| cargo make fmt | Apply Rust formatting |
| cargo make fmt-check | Verify formatting without changing files |
| cargo make clippy | Clippy with warnings denied |
| cargo make test | Unit and integration tests |
| cargo make doc | Documentation build with warnings denied |
| cargo make audit | Known dependency vulnerabilities |
| cargo make deny | Dependency and license policy |
| cargo make sbom | CycloneDX software bill of materials |
| cargo make quality | Core local quality checks |
| cargo make ci | Full CI task set |

Install the optional tools used by the full CI task set:

```sh
cargo install --locked cargo-audit
cargo install --locked cargo-deny
cargo install cargo-cyclonedx
```

The GitHub Actions workflow also collects coverage with cargo-llvm-cov and uploads the LCOV report to Codecov. The coverage upload is informational and does not fail CI if Codecov is unavailable. Connect the generated repository to Codecov to activate the percentage badge.

For the optional changelog and release-management tasks, install:

```sh
cargo install git-cliff cargo-release
```

The pre-commit task additionally requires [pre-commit](https://pre-commit.com/) and Python 3.

## 📦 Releases

{% if crate_type == "bin" %}
Binary projects include an automated release workflow. Push a version tag to build and publish platform-specific artifacts:

```sh
git tag v0.1.0
git push origin v0.1.0
```

The workflow builds Linux (GNU and musl), Windows and macOS binaries, generates checksums and attaches release notes and the SBOM to the GitHub Release.
{% else %}
This project is configured as a Rust library. Publish it to [crates.io](https://crates.io/) when it is ready for external use, and consider adding a release workflow that matches your distribution policy.
{% endif %}

## 🤝 Contributing

Contributions, bug reports and thoughtful suggestions are welcome.

1. Check existing issues or open a discussion before starting a large change.
2. Create a focused branch for your change.
3. Run cargo make quality and the relevant tests.
4. Follow the commit format below.
5. Open a pull request with context, motivation and test results.

Install [pre-commit](https://pre-commit.com/) and Python 3, then install the repository hooks:

```sh
pre-commit install --hook-type pre-commit --hook-type commit-msg
```

### Commit messages

Use **Gitmoji + Conventional Commits**:

```text
✨ feat(api): add endpoint

Add the endpoint and document its response format.

Closes #123
```

Format: **<gitmoji> <type>(<scope>): <description>**. Keep the subject concise; use the optional body to explain why, and standard trailers for issue references. Breaking changes can be marked with ❗ or a 🚨 BREAKING CHANGE footer.

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full contribution workflow.

## 📄 License

Licensed under the {{license}} license. See [LICENSE](LICENSE).

---

<div align="center">

Made with ❤️ and Rust.

</div>
