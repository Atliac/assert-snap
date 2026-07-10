use std::borrow::Cow;

use regex::Regex;

/// A rule describing a single redaction to apply to a snapshot string.
///
/// - `pattern` is a regular expression to search for.
/// - `limit` is the maximum number of matches to replace (`0` replaces all).
/// - `replacement` is the text to substitute in place of each match.
pub(crate) struct RedactionRule<'a> {
    pub(crate) pattern: &'a str,
    pub(crate) limit: usize,
    pub(crate) replacement: &'a str,
}

/// Applies `redaction_rules` sequentially to `data`, returning the redacted
/// result. Borrows `data` when no rules modify it; otherwise owns the modified
/// string.
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
}
