//! Implementation module containing core assertion comparison and unified diff display logic.

use std::borrow::Cow;

use crate::redaction::{RedactionRule, apply_redactions};

/// Escapes non-printable control characters (such as `\0`, `\r`, `\x1b`) using
/// their debug representations, while preserving structural newlines (`\n`) and tabs (`\t`).
///
pub(crate) fn escape_invisible(s: &str) -> Cow<'_, str> {
    if !s.chars().any(|c| c.is_control() && c != '\n' && c != '\t') {
        return Cow::Borrowed(s);
    }

    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        if c.is_control() && c != '\n' && c != '\t' {
            for escaped in c.escape_debug() {
                result.push(escaped);
            }
        } else {
            result.push(c);
        }
    }
    Cow::Owned(result)
}

/// Asserts that an actual value matches an expected snapshot value after applying redaction rules.
///
/// This function compares `actual` against `expected` after applying the provided
/// `redaction_rules` to the actual value. If they don't match, it prints diagnostic information
/// and panics.
///
/// # Arguments
///
/// * `actual` - The actual value produced by the code under test.
/// * `expected` - The expected snapshot value to compare against.
/// * `redaction_rules` - A slice of redaction rules to apply to `actual` before comparison.
///   These are typically used to mask dynamic content like timestamps, IDs, or memory addresses.
///
/// # Panics
///
/// Panics if the redacted `actual` does not equal `expected`. When the `diff` feature
/// is enabled, a unified diff is printed showing the differences.
///
#[track_caller]
pub fn assert_snap(actual: &str, expected: &str, redaction_rules: &[RedactionRule]) {
    let caller = std::panic::Location::caller();
    println!(
        "📸 assert-snap ...\n📄 {}:{}\n",
        caller.file(),
        caller.line()
    );
    println!("🔍 ACTUAL VALUE");
    println!("{}\n", escape_invisible(actual));
    let actual = apply_redactions(actual, redaction_rules);
    if matches!(actual, Cow::Owned(_)) {
        println!("🛡️ REDACTED VALUE");
        println!("{}\n", escape_invisible(&actual));
    }
    if expected != actual {
        let expected = &escape_invisible(expected);
        println!("🎯 EXPECTED VALUE");
        println!("{expected}\n");
        #[cfg(feature = "diff")]
        show_diff(&escape_invisible(&actual), expected);
        panic!("❌ ASSERTION FAILED\n");
    }
    println!("✅ ASSERTION PASSED\n");
}

/// Displays a unified diff between the expected snapshot string and the actual/redacted output.
///
/// Enabled when the `diff` feature flag is active.
#[cfg(feature = "diff")]
#[track_caller]
fn show_diff(actual: &str, expected: &str) {
    use similar::TextDiff;

    println!("📊 DIFF");
    let diff = TextDiff::from_lines(expected, actual);
    println!(
        "{}",
        diff.unified_diff()
            .header("EXPECTED VALUE", "ACTUAL/REDACTED VALUE")
            .missing_newline_hint(false)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_assert_impl() {
        let real = "User(id=123, name=Alice)";
        let expected = "User(id=REDACTED, name=Alice)";
        let rules = [RedactionRule::new(r"id=\d+", "id=REDACTED", 0)];
        assert_snap(real, expected, &rules);
    }
}
