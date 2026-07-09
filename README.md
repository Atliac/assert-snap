# 📦 Crate Template

[![Crates.io](https://img.shields.io/crates/v/atliac-crate-template)](https://crates.io/crates/atliac-crate-template)
[![Documentation](https://docs.rs/atliac-crate-template/badge.svg)](https://docs.rs/atliac-crate-template)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
![Development: Active](https://img.shields.io/badge/Development-Active-blue)

A minimal, production-ready Rust template designed to jumpstart library (crate) development. It establishes a workspace-friendly structure and comes pre-configured with automated release pipelines, versioning, and cross-platform distribution.

## 🚀 How to Use

### Developing

1. Add a new package: `cargo new --lib <package-name>`
2. Work on the library: `cargo build` / `cargo test`

### CI Configuration

The project includes a GitHub Actions workflow for continuous integration. To modify the platforms used for testing (Clippy, tests, and documentation), edit `.github/workflows/ci.yml` and update the `supported_os` list:

```yaml
matrix:
  os: &supported_os [ubuntu-latest, macos-latest, windows-latest]
```

### Publishing

This project uses `release-plz` to automate updating [CHANGELOG.md](CHANGELOG.md), bumping versions, tagging Git commits, and publishing to [crates.io](https://crates.io).

Steps to enable `release-plz`:

1. **Allow PR Creation**: Navigate to `https://github.com/<user>/<repo>/settings/actions`. Under the **Workflow permissions** section, check the box for **"Allow GitHub Actions to create and approve pull requests"**.
2. **Configure PAT**: Navigate to `https://github.com/<user>/<repo>/settings/secrets/actions` and add a repository secret named `RELEASE_PLZ_TOKEN`.
    * This must be a GitHub Personal Access Token (PAT).
    * If using a **Fine-grained PAT**, grant **`Contents: Read and write`** (to push Git tags) and **`Pull Requests: Read and write`** (to open release PRs) permissions.
    * If using a **Classic PAT**, select the **`repo`** scope.
3. **Enable `release-plz` for Your Repository**: Update `if: github.repository == 'Atliac/crate-template'` to `if: github.repository == '<your-username>/<your-repo>'` in `.github/workflows/release-plz.yml`.
4. Publishing to `crates.io`**:
    1. **Enable in Config**: Edit [release-plz.toml](release-plz.toml) and remove the `publish = false` lines (or set them to `true`).
    2. **First Publish**: Run `cargo publish` manually from your local machine once. (Crates.io does not allow publishing a brand-new crate via automation).
    3. **Trusted Publishing**: Follow the [crates.io Trusted Publishing guide](https://crates.io/docs/trusted-publishing) to link your GitHub repository.

## 📜 License

This project is dual-licensed under:

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [MIT](https://opensource.org/licenses/MIT)

You may choose either license at your discretion.

## 🤝 Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions.
