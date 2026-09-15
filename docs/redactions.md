# Redacting Dynamic Data

Dynamic values like generated IDs, secret keys, memory addresses, or timestamps can cause snapshot test instability. `assert-snap` allows appending redaction rules directly inside macro invocations.

## Basic Redaction Syntax

The basic redaction syntax uses the pattern:

```text
"regex_pattern" => "replacement"
```

This redacts all matches of the regular expression in the actual output before comparing against the expected snapshot:

```rust,ignore
use assert_snap::assert_snap;

let actual = "Response time: 142ms, status: 200";
let expected = "Response time: [DURATION], status: 200";

assert_snap!(
    actual,
    expected,
    r"\d+ms" => "[DURATION]"
);
```

## Redactions with Limits

You can restrict the maximum number of redactions by prefixing the rule with `[limit]`:

```text
[limit] "regex_pattern" => "replacement"
```

By default (or with `[0]`), all matches are redacted. Specifying a limit (e.g. `[1]`) restricts redaction to at most that number of matches:

```rust,ignore
use assert_snap::assert_snap;

let actual = "token: secret_abc, session: secret_xyz";
let expected = "token: ****, session: secret_xyz";

assert_snap!(
    actual,
    expected,
    [1] "secret_[a-z]+" => "****"
);
```

## Multiple Redaction Rules

You can combine multiple redaction rules separated by commas:

```rust,ignore
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
