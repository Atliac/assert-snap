# Cargo Features

`assert-snap` provides optional features to tailor functionality and minimize dependencies according to your project's needs.

## Feature Overview

| Feature | Default | Description |
| :--- | :--- | :--- |
| `diff` | **Enabled** | Prints a unified diff using the `similar` crate when an assertion fails. |

## Disabling Default Features

To disable default features (for instance, to disable `diff` and minimize external dependencies):

```toml
[dependencies]
assert-snap = { version = "0.0.1", default-features = false }
```
