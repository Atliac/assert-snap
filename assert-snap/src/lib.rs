pub mod assert_impl;
pub mod redaction;

#[macro_export]
macro_rules! assert_snap {
        // ($real_value:expr,$($t:tt)*) => {
    //     assert_snap!(&assertion_id,$real_value,$($t)*);
    // };
    ($actual:expr, $expected:expr) => {
        let actual = format!("{}", $actual);
        let expected = format!("{}", $expected);
        $crate::assert_snap!(@assert_str, &actual, &expected);
    };
    // Main arm with rules
    (
        @assert_str,
        // &str
        $actual:expr,
        // &str
        $expected:expr,
        // Rules token stream
        $($rules:tt)+
    ) => {
        use redaction::*;

        let mut redaction_rules = Vec::new();
        assert_snap!(@munch redaction_rules ; $($rules)+);

        $crate::assert_impl::assert_snap(
            $actual,
            $expected,
            &redaction_rules
        );
    };

    // Main arm without rules
    (
        @assert_str,
        // &str
        $actual:expr,
        // &str
        $expected:expr
    ) => {
        $crate::assert_impl::assert_snap(
            $actual,
            $expected,
            Default::default()
        );
    };

    (@munch $vec:ident;) => {};

    // Rule WITH limit (followed by a comma and more rules)
    (@munch $vec:ident; [$limit:expr] $pattern:expr => $replacement:expr , $($rest:tt)*) => {
        $vec.push(RedactionRule {
            pattern: $pattern,
            limit: $limit,
            replacement: $replacement,
        });
        assert_snap!(@munch $vec; $($rest)*);
    };

    // Rule WITH limit (last rule)
    (@munch $vec:ident; [$limit:expr] $pattern:expr => $replacement:expr) => {
        $vec.push(RedactionRule {
            pattern: $pattern,
            limit: $limit,
            replacement: $replacement,
        });
    };

    // Rule WITHOUT limit (followed by a comma and more rules)
    (@munch $vec:ident; $pattern:expr => $replacement:expr , $($rest:tt)*) => {
        $vec.push(RedactionRule {
            pattern: $pattern,
            limit: 0,
            replacement: $replacement,
        });
        assert_snap!(@munch $vec; $($rest)*);
    };

    // Rule WITHOUT limit (last rule)
    (@munch $vec:ident; $pattern:expr => $replacement:expr) => {
        $vec.push(RedactionRule {
            pattern: $pattern,
            limit: 0,
            replacement: $replacement,
        });
    };
}

// #[macro_export]
// macro_rules! assert_debug_snap {
//     ($real_value:expr,$($t:tt)*) => {
//         assert_snap!("assert_snap", &format!("{real_value:#?}"), $($t)*);
//     };
//     ($real_value:expr) => {
//         assert_snap!("assert_snap", &format!("{real_value:#?}"),);
//     };
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_snap_no_rules() {
        assert_snap!(@assert_str, "Hello World!", "Hello World!");
    }

    #[test]
    fn test_assert_snap() {
        // Test with redaction rules - replace "secret" with "****"
        assert_snap!(@assert_str,
            "User password is secret123",
            "User password is ****123",
            "secret" => "****"
        );
    }

    #[test]
    fn test_assert_snap_with_limit() {
        // Test with limit - only replace first occurrence
        assert_snap!(@assert_str,
            "secret and secret",
            "**** and secret",
            [1] "secret" => "****"
        );
    }

    #[test]
    fn test_assert_snap_multiple_rules() {
        // Test with multiple redaction rules
        assert_snap!(@assert_str,
            "api_key=abc123 password=xyz789",
            "api_key=**** password=****",
            r"api_key=.+\s" => "api_key=**** ",
            [1] r"password=.+" => "password=****"
        );
    }
}
