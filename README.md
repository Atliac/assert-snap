# 📸 assert-snap

[![Crates.io](https://img.shields.io/crates/v/assert-snap)](https://crates.io/crates/assert-snap)
[![Documentation](https://docs.rs/assert-snap/badge.svg)](https://docs.rs/assert-snap)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
![Maintenance: Active](https://img.shields.io/badge/Maintenance-Active-blue)

A snapshot testing and assertion library for Rust that supports flexible, regex-based dynamic data redactions and detailed unified diff output.

---

## ✨ Features

- 🎯 **`assert_snap!`**: Assert any types implementing `Display`.
- 🔍 **`assert_debug_snap!`**: Assert any types implementing `Debug`.
- 🛡️ **Regex Redactions**: Inline rules to scrub volatile data (timestamps, UUIDs, secret tokens, memory addresses) before checking assertions.
- 📊 **Unified Diff**: Built-in diff printing powered by `similar` on assertion failures (enabled via the `diff` feature, default ON).

---

## 🚀 Quickstart

### Basic String Assertion (`assert_snap!`)

```rust
use assert_snap::assert_snap;

let actual_output = "User connected successfully";
let expected_snapshot = "User connected successfully";

assert_snap!(actual_output, expected_snapshot);
```

### Debug Assertion (`assert_debug_snap!`)

```rust
use assert_snap::assert_debug_snap;

#[derive(Debug)]
struct User {
    id: u64,
    name: String,
}

let user = User { id: 42, name: "Alice".into() };

assert_debug_snap!(
    user,
    User {
        id: 42,
        name: "Alice".to_string(),
    }
);
```

---

## 🛡️ Redacting Dynamic Data

Dynamic values like generated IDs, secret keys, or timestamps can cause snapshot test instability. `assert-snap` allows appending redaction rules directly inside the macro invocation.

### Basic Redaction Syntax

Format: `"regex_pattern" => "replacement"` (redacts all matches)

```rust
use assert_snap::assert_snap;

let actual = "Response time: 142ms, status: 200";
let expected = "Response time: [DURATION], status: 200";

assert_snap!(
    actual,
    expected,
    r"\d+ms" => "[DURATION]"
);
```

### Redactions with Limits

Format: `[limit] "regex_pattern" => "replacement"`

By default (or with `[0]`), all matches are redacted. Specifying a limit (e.g., `[1]`) restricts redaction to that maximum number of matches:

```rust
use assert_snap::assert_snap;

let actual = "token: secret_abc, session: secret_xyz";
let expected = "token: ****, session: secret_xyz";

assert_snap!(
    actual,
    expected,
    [1] "secret_[a-z]+" => "****"
);
```

### Multiple Redaction Rules

You can combine multiple redaction rules separated by commas:

```rust
use assert_snap::assert_debug_snap;

#[derive(Debug)]
struct Session {
    id: String,
    token: String,
    active: bool,
}

let session = Session {
    id: "sess-98765".into(),
    token: "Bearer secret-token-123".into(),
    active: true,
};

assert_debug_snap!(
    session,
    Session {
        id: "sess-****".to_string(),
        token: "Bearer ****".to_string(),
        active: true,
    },
    r"sess-\d+" => "sess-****",
    r"Bearer .+" => "Bearer ****"
);
```

---

## ⚙️ Cargo Features

| Feature | Default | Description |
| :--- | :--- | :--- |
| `diff` | **Enabled** | Prints a unified diff using the `similar` crate when an assertion fails. |

To disable default features (such as `diff` to minimize dependencies):

```toml
[dependencies]
assert-snap = { version = "0.0.0", default-features = false }
```

---

## 📜 License

This project is dual-licensed under:

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [MIT](https://opensource.org/licenses/MIT)

You may choose either license at your discretion.

## 🤝 Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions.
