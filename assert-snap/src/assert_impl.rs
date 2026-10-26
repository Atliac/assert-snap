//! Implementation module containing core assertion comparison and unified diff display logic.

use std::borrow::Cow;

use crate::redaction::{RedactionRule, apply_redactions};
use crate::theme::Theme;

/// Asserts that an actual value matches an expected snapshot value after applying redaction rules.
///
/// This function compares `actual` against `expected` after applying the provided
/// `redaction_rules` to the actual value. If they don't match, it prints diagnostic information
/// and panics.
///
/// # Arguments
///
/// * `method` - The assertion method name displayed in diagnostic output.
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
pub fn assert_snap(method: &str, actual: &str, expected: &str, redaction_rules: &[RedactionRule]) {
    let theme = Theme::current();
    let caller = std::panic::Location::caller();
    println!(
        "{brand}📸 {method}:\n{brand:#}📄 {loc}{}:{}{loc:#}\n",
        caller.file(),
        caller.line(),
        brand = theme.brand,
        loc = theme.location,
    );
    println!(
        "{style}🔍 ACTUAL VALUE{style:#}",
        style = theme.actual_header
    );
    println!("{}\n", actual);
    let actual = apply_redactions(actual, redaction_rules);
    if matches!(actual, Cow::Owned(_)) {
        println!(
            "{style}🛡️ REDACTED VALUE{style:#}",
            style = theme.redacted_header
        );
        println!("{}\n", &actual);
    }
    if expected != actual {
        println!(
            "{style}🎯 EXPECTED VALUE{style:#}",
            style = theme.expected_header
        );
        println!("{expected}\n");
        #[cfg(feature = "diff")]
        show_diff(&actual, expected, &theme);
        panic!("{style}❌ ASSERTION FAILED{style:#}\n", style = theme.fail);
    }
    println!("{style}✅ ASSERTION PASSED{style:#}\n", style = theme.pass);
}

/// Displays a unified diff between the expected snapshot string and the actual/redacted output.
///
/// Enabled when the `diff` feature flag is active.
#[cfg(feature = "diff")]
#[track_caller]
fn show_diff(actual: &str, expected: &str, theme: &Theme) {
    use similar::{ChangeTag, TextDiff};
    let actual = format!("{actual}\n");
    let expected = format!("{expected}\n");
    println!("{style}📊 DIFF{style:#}", style = theme.diff_header);
    let diff = TextDiff::from_lines(expected, actual);
    println!(
        "{del}--- EXPECTED VALUE{del:#}\n{ins}+++ ACTUAL/REDACTED VALUE{ins:#}",
        del = theme.diff_del,
        ins = theme.diff_ins,
    );

    for hunk in diff.unified_diff().missing_newline_hint(false).iter_hunks() {
        println!(
            "{hunk_style}{}{hunk_style:#}",
            hunk.header(),
            hunk_style = theme.diff_hunk
        );
        for change in hunk.iter_changes() {
            let (sign, style) = match change.tag() {
                ChangeTag::Delete => ("-", theme.diff_del),
                ChangeTag::Insert => ("+", theme.diff_ins),
                ChangeTag::Equal => (" ", theme.diff_equal),
            };
            let change = change.as_str().unwrap_or_default();
            let mut change_escaped = String::with_capacity(change.len());
            for c in change.chars() {
                if '\\' == c || '\n' == c || '"' == c {
                    change_escaped.push(c);
                } else {
                    for ce in c.escape_debug() {
                        change_escaped.push(ce);
                    }
                }
            }
            print!("{style}{sign}{change_escaped}{style:#}");
        }
    }
}
