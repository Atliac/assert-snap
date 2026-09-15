# Introduction

`assert-snap` is a snapshot testing and assertion library for Rust that supports flexible, regex-based dynamic data redactions and detailed unified diff output.

## Overview

When writing unit tests or integration tests, asserting complex data structures or formatted strings can be tedious. `assert-snap` provides macros (`assert_snap!` and `assert_debug_snap!`) that allow you to compare actual runtime values against expected snapshot strings while redacting non-deterministic data (such as timestamps, UUIDs, memory addresses, or secret tokens).

## Core Features

- **`assert_snap!`**: Assert types implementing `Display`.
- **`assert_debug_snap!`**: Assert types implementing `Debug`.
- **Regex Redactions**: Inline rules to scrub volatile data before checking assertions.
- **Unified Diff**: Built-in diff printing powered by `similar` on assertion failures (enabled by default via the `diff` feature).
