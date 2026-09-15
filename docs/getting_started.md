# Getting Started

## Installation

Add `assert-snap` to your `Cargo.toml` dependencies (or `[dev-dependencies]` if only used in tests):

```toml
[dev-dependencies]
assert-snap = "0.0.1"
```

## Basic String Assertion (`assert_snap!`)

Use `assert_snap!` for types that implement `Display`. Pass the actual value first, followed by the expected snapshot string:

```rust,ignore
use assert_snap::assert_snap;

let actual_output = "User connected successfully";
let expected_snapshot = "User connected successfully";

assert_snap!(actual_output, expected_snapshot);
```

If the actual value does not match the expected snapshot, `assert-snap` will panic and display a unified diff highlighting the differences.

## Debug Assertion (`assert_debug_snap!`)

Use `assert_debug_snap!` for types that implement `Debug`. The macro converts both the actual value and expected expression into debug representations:

```rust,ignore
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
