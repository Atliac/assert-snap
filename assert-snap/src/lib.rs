//! # `assert-snap`
//!
//! A snapshot testing and assertion library for Rust that supports flexible, regex-based dynamic data redactions and detailed unified diff output.
//!
//! ## Main Macros
//!
//! - [`assert_snap!`]: Asserts any types implementing [`Display`](std::fmt::Display).
//! - [`assert_debug_snap!`]: Asserts any types implementing [`Debug`](std::fmt::Debug).
//!
//! ## Quick Example
//!
//! ```rust
//! use assert_snap::{assert_snap, assert_debug_snap};
//!
//! // Simple string assertion with regex redaction
//! assert_snap!(
//!     "User id: 12345, status: active",
//!     "User id: [ID], status: active",
//!     r"\d+" => "[ID]"
//! );
//!
//! // Debug format assertion
//! assert_debug_snap!(Some("secret"), Some("secret"));
//! ```

#[doc(hidden)]
pub mod assert_impl;
#[doc(hidden)]
pub mod redaction;
#[doc(hidden)]
pub mod theme;

/// Asserts any types implementing [`Display`](std::fmt::Display).
///
/// # Syntax
///
/// ```rust,ignore
/// assert_snap!(actual, expected);
/// assert_snap!(actual, expected, "pattern" => "replacement");
/// assert_snap!(actual, expected, [limit] "pattern" => "replacement");
/// assert_snap!(actual, expected, rule1, rule2, ...);
/// ```
///
/// # Parameters
///
/// - `$actual`: An expression implementing [`Display`](std::fmt::Display) or convertible via `format!("{actual}")`.
/// - `$expected`: An expression implementing [`Display`](std::fmt::Display) or convertible via `format!("{expected}")`.
/// - `[limit] pattern => replacement`: Optional redaction rules to scrub dynamic data (e.g. timestamps, UUIDs) before comparison.
///   - `limit` (optional): `[n]` limits replacement to the first `n` matches. If omitted or `[0]`, all matches are replaced.
///   - `pattern`: A regex pattern string.
///   - `replacement`: Replacement string or regex group reference (e.g. `"$1"`).
///
/// # Examples
///
/// Basic comparison:
/// ```rust
/// use assert_snap::assert_snap;
///
/// assert_snap!("hello world", "hello world");
/// ```
///
/// Comparison with redaction rules:
/// ```rust
/// use assert_snap::assert_snap;
///
/// assert_snap!(
///     "User id is 12345",
///     "User id is [ID]",
///     r"\d+" => "[ID]"
/// );
/// ```
///
/// Comparison with redaction rules with match limits:
/// ```rust
/// use assert_snap::assert_snap;
///
/// assert_snap!(
///     "secret and secret",
///     "**** and secret",
///     [1] "secret" => "****"
/// );
/// ```
#[macro_export]
macro_rules! assert_snap {
    ($actual:expr, $expected:expr $(,)?) => {
        match (&($actual), &($expected)) {
            (actual_val, expected_val) => {
                let actual_str = format!("{actual_val}");
                let expected_str = format!("{expected_val}");
                $crate::assert_snap!(@assert_str, stringify!(assert_snap($actual)), &actual_str, &expected_str);
            }
        }
    };

    ($actual:expr, $expected:expr, $($tail:tt)+) => {
        match (&($actual), &($expected)) {
            (actual_val, expected_val) => {
                let actual_str = format!("{actual_val}");
                let expected_str = format!("{expected_val}");
                $crate::assert_snap!(@assert_str, stringify!(assert_snap($actual)), &actual_str, &expected_str, $($tail)+);
            }
        }
    };

    (
        @assert_str,
        $method:expr,
        // &str
        $actual:expr,
        // &str
        $expected:expr,
        // Rules token stream
        $($rules:tt)+
    ) => {
        let mut redaction_rules = Vec::new();
        #[allow(clippy::vec_init_then_push)]
        {
            $crate::assert_snap!(@munch redaction_rules ; $($rules)+);
        }

        $crate::assert_impl::assert_snap(
            $method,
            $actual,
            $expected,
            &redaction_rules
        );
    };

    (
        @assert_str,
        $method:expr,
        // &str
        $actual:expr,
        // &str
        $expected:expr
    ) => {
        $crate::assert_impl::assert_snap(
            $method,
            $actual,
            $expected,
            ::std::default::Default::default()
        );
    };

    (@munch $vec:ident;) => {};

    // Rule WITH limit (followed by a comma and more rules)
    (@munch $vec:ident; [$limit:expr] $pattern:expr => $replacement:expr , $($rest:tt)+) => {
        $vec.push($crate::redaction::RedactionRule {
            pattern: $pattern,
            limit: $limit,
            replacement: $replacement,
        });
        $crate::assert_snap!(@munch $vec; $($rest)+);
    };

    // Rule WITH limit (last rule)
    (@munch $vec:ident; [$limit:expr] $pattern:expr => $replacement:expr $(,)?) => {
        $vec.push($crate::redaction::RedactionRule {
            pattern: $pattern,
            limit: $limit,
            replacement: $replacement,
        });
    };

    // Rule WITHOUT limit (followed by a comma and more rules)
    (@munch $vec:ident; $pattern:expr => $replacement:expr , $($rest:tt)+) => {
        $vec.push($crate::redaction::RedactionRule {
            pattern: $pattern,
            limit: 0,
            replacement: $replacement,
        });
        $crate::assert_snap!(@munch $vec; $($rest)+);
    };

    // Rule WITHOUT limit (last rule)
    (@munch $vec:ident; $pattern:expr => $replacement:expr $(,)?) => {
        $vec.push($crate::redaction::RedactionRule {
            pattern: $pattern,
            limit: 0,
            replacement: $replacement,
        });
    };
}

/// Asserts any types implementing [`Debug`](std::fmt::Debug).
///
/// The macro formats `actual` and `expected` using `{:#?}` pretty-printing prior to comparison.
///
/// # Syntax
///
/// ```rust,ignore
/// assert_debug_snap!(actual, expected);
/// assert_debug_snap!(actual, expected, "pattern" => "replacement");
/// assert_debug_snap!(actual, expected, [limit] "pattern" => "replacement");
/// assert_debug_snap!(actual, expected, rule1, rule2, ...);
/// ```
///
/// # Examples
///
/// Basic debug assertion:
/// ```rust
/// use assert_snap::assert_debug_snap;
///
/// assert_debug_snap!(Some(42), Some(42));
/// ```
///
/// Debug assertion with redaction rules:
/// ```rust
/// use assert_snap::assert_debug_snap;
///
/// #[derive(Debug)]
/// struct User {
///     name: String,
///     token: String,
/// }
///
/// let user = User { name: "Alice".into(), token: "secret_123".into() };
/// assert_debug_snap!(
///     user,
///     User {
///         name: "Alice".to_string(),
///         token: "****".to_string(),
///     },
///     "secret_123" => "****"
/// );
/// ```
#[macro_export]
macro_rules! assert_debug_snap {
    ($actual:expr, $expected:expr $(,)?) => {
        match (&($actual), &($expected)) {
            (actual_val, expected_val) => {
                let actual_str = format!("{actual_val:#?}");
                let expected_str = format!("{expected_val:#?}");
                $crate::assert_snap!(@assert_str, stringify!(assert_debug_snap($actual)), &actual_str, &expected_str);
            }
        }
    };

    ($actual:expr, $expected:expr, $($tail:tt)+) => {
        match (&($actual), &($expected)) {
            (actual_val, expected_val) => {
                let actual_str = format!("{actual_val:#?}");
                let expected_str = format!("{expected_val:#?}");
                $crate::assert_snap!(@assert_str, stringify!(assert_debug_snap($actual)), &actual_str, &expected_str, $($tail)+);
            }
        }
    };
}
