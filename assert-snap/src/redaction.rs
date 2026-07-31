//! Redaction module for masking dynamic or sensitive text using regular expressions.
//!
//! This module defines [`RedactionRule`], which encapsulates regex pattern matching, replacement rules,
//! and match limits, as well as internal helpers for applying transformations sequentially.

use std::borrow::Cow;

use regex::Regex;

/// A rule specifying how to redact data matching a specific regular expression pattern.
///
/// Rules consist of a regex pattern, a replacement string (supporting regex capture groups such as `$1`),
/// and a maximum replacement limit.
///
/// # Examples
///
/// ```rust
/// use assert_snap::redaction::RedactionRule;
/// let rule = RedactionRule::new(r"token_\w+", "[REDACTED]", 1);
/// ```
pub struct RedactionRule<'a> {
    /// The regular expression pattern to match.
    pub pattern: &'a str,
    /// The maximum number of replacements to make. If `0`, all occurrences are replaced.
    pub limit: usize,
    /// The replacement template (supports regex capture groups like `$1`).
    pub replacement: &'a str,
}

impl<'a> RedactionRule<'a> {
    /// Creates a new redaction rule.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The regular expression pattern to match against the input data.
    /// * `replacement` - The replacement template. Supports capture group references like `$1`, `$2`, etc.
    /// * `limit` - The maximum number of replacements to make. If `0`, all occurrences are replaced.
    ///
    /// # Returns
    ///
    /// A new [`RedactionRule`] instance.
    pub fn new(pattern: &'a str, replacement: &'a str, limit: usize) -> Self {
        Self {
            pattern,
            limit,
            replacement,
        }
    }
}

/// Applies a series of redaction rules to the input string sequentially.
///
/// Each rule defines a regular expression pattern, a replacement template (which supports
/// capture groups like `$1`), and a limit on the number of matches to replace. If the limit
/// is `0`, all occurrences matching the pattern are replaced.
///
/// # Panics
///
/// Panics if any of the regular expression patterns in the rules are invalid regex.
#[track_caller]
pub(crate) fn apply_redactions<'a>(
    data: &'a str,
    redaction_rules: &[RedactionRule],
) -> Cow<'a, str> {
    let mut result = Cow::from(data);
    for RedactionRule {
        pattern,
        limit,
        replacement,
    } in redaction_rules
    {
        let regex = Regex::new(pattern).unwrap();
        let redacted = regex.replacen(&result, *limit, *replacement);
        if let Cow::Owned(modified) = redacted {
            result = Cow::from(modified);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_redactions_empty_rules() {
        let data = "hello world";
        let rules = [];
        let result = apply_redactions(data, &rules);
        assert_eq!(result, "hello world");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn test_apply_redactions_no_match() {
        let data = "hello world";
        let rules = [RedactionRule {
            pattern: "foo",
            limit: 0,
            replacement: "bar",
        }];
        let result = apply_redactions(data, &rules);
        assert_eq!(result, "hello world");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn test_apply_redactions_single_replacement() {
        let data = "hello world hello";
        let rules = [RedactionRule {
            pattern: "hello",
            limit: 1,
            replacement: "hi",
        }];
        let result = apply_redactions(data, &rules);
        assert_eq!(result, "hi world hello");
        assert!(matches!(result, Cow::Owned(_)));
    }

    #[test]
    fn test_apply_redactions_limit_zero_replaces_all() {
        let data = "hello world hello";
        let rules = [RedactionRule {
            pattern: "hello",
            limit: 0,
            replacement: "hi",
        }];
        let result = apply_redactions(data, &rules);
        assert_eq!(result, "hi world hi");
    }

    #[test]
    fn test_apply_redactions_multiple_rules() {
        let data = "hello world";
        let rules = [
            RedactionRule {
                pattern: "hello",
                limit: 1,
                replacement: "hi",
            },
            RedactionRule {
                pattern: "world",
                limit: 1,
                replacement: "earth",
            },
        ];
        let result = apply_redactions(data, &rules);
        assert_eq!(result, "hi earth");
    }

    #[test]
    fn test_apply_redactions_capture_groups() {
        let data = "hello world";
        let rules = [RedactionRule {
            pattern: r"hello (\w+)",
            limit: 0,
            replacement: "hi $1",
        }];
        let result = apply_redactions(data, &rules);
        assert_eq!(result, "hi world");
    }

    #[test]
    #[should_panic(expected = "regex parse error")]
    fn test_apply_redactions_invalid_regex() {
        let data = "hello world";
        let rules = [RedactionRule {
            pattern: "(invalid",
            limit: 0,
            replacement: "hi",
        }];
        let _result = apply_redactions(data, &rules);
    }
}
