use std::borrow::Cow;

use crate::redaction::{RedactionRule, apply_redactions};

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
pub fn assert_snap<'a>(actual: &str, expected: &str, redaction_rules: &[RedactionRule]) {
    println!("===== ACTUAL VALUE =====");
    println!("{actual}\n");
    let actual = apply_redactions(actual.as_ref(), redaction_rules);
    if matches!(actual, Cow::Owned(_)) {
        println!("===== REDACTED VALUE =====");
        println!("{actual}\n");
    }

    if expected != actual {
        println!("===== EXPECTED VALUE =====");
        println!("{expected}\n");
        #[cfg(feature = "diff")]
        show_diff(&actual, &expected);
        panic!("===== ASSERTION Failed =====");
    }
    println!("===== ASSERTION PASSED =====");
}

#[cfg(feature = "diff")]
#[track_caller]
fn show_diff(actual: &str, expected: &str) {
    use similar::TextDiff;

    println!("===== DIFF =====");
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
