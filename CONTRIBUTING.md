# Contributing to crate-template

Thank you for considering contributing to crate-template!

## Issues
Before reporting an issue, please check existing or similar issues that are currently tracked.

Additionally, please ensure that if you are submitting a bug ticket (i.e., something doesn't work) that the bug is reproducible. If we cannot reproduce the bug, your ticket is likely to be marked either `wontfix` or closed (although we will likely take note of it in case there is a secondary occurrence).

## Pull Requests

Contributions are always encouraged and welcome. Before creating a pull request, create a new issue that tracks that pull request, describing the problem in more detail. Pull request descriptions should include information about its implementation, especially if it makes changes to the existing architecture.

PRs should be small and focused and should avoid interacting with multiple facets of the codebase. This may require a larger PR to be split into two or more smaller PRs.

Unless the PR is for something minor (i.e., a typo), please ensure that an issue has been opened for the feature or work you would like to contribute beforehand. By opening an issue, a discussion can be held beforehand on scoping the work effectively and ensuring that the work is in line with the vision for crate-template. Without any linked issues, your PR may be liable to be closed if we (the maintainers) do not feel that your PR is within scope for the project.

It is also highly suggested to comment on issues you are interested in working on. By doing so, it allows others to see that something is being worked on and therefore avoids frustrating situations, such as multiple contributors opening a PR for the same issue. In such a case, any duplicate PRs will be closed unless it is clear that the original contributor is unable to continue the work.

You can link your PR back to a given issue by writing the following in your PR message:
```md
Fix #999
```

This will then auto-link issue 999 (for example) and will automatically close the issue once the PR has been merged.


## Developing

### Setup

This should be similar to most Rust projects.

```bash
git clone https://github.com/Atliac/crate-template
cd crate-template
cargo test
```

### Clippy, Fmt, and Doc

We enforce `clippy`, `fmt`, and `doc` checks for all pull requests.

```bash
cargo clippy --workspace --all-targets --all-features
cargo doc --no-deps --workspace --all-features
cargo fmt --all
cargo test --workspace --all-features
```

## AI Agent

As a contributor, you are welcome to use AI assistance (such as Copilot, Claude Code, or Cursor) for coding. Using AI does not change the quality bar or transfer responsibility away from you.

This project assumes all contributions are authored and verified by humans. Please ensure that local AI-specific configurations or rules—such as `AGENTS.md`, `.cursorrules`, or `.copilot-instructions`—are not committed to Git. These have been added to the `.gitignore` to prevent cluttering the repository with personalized agent configurations.
