mod assert_impl;
mod redaction;

// #[macro_export]
// macro_rules! assert_snap {
//     ($real_value:expr,$($t:tt)*) => {
//         assert_snap_impl!("assert_snap", &format!("{real_value}"), $($t)*);
//     };
//     ($real_value:expr) => {
//         assert_snap_impl!("assert_snap", &format!("{real_value}"),);
//     };
// }

// #[macro_export]
// macro_rules! assert_debug_snap {
//     ($real_value:expr,$($t:tt)*) => {
//         assert_snap_impl!("assert_snap", &format!("{real_value:#?}"), $($t)*);
//     };
//     ($real_value:expr) => {
//         assert_snap_impl!("assert_snap", &format!("{real_value:#?}"),);
//     };
// }

macro_rules! assert_snap_impl {
    // Internal muncher: Base case (end of rules)
    (@munch $vec:ident;) => {};

    // Internal muncher: Rule WITH limit (followed by a comma and more rules)
    (@munch $vec:ident; [$limit:expr] $pattern:expr => $replacement:expr , $($rest:tt)*) => {
        $vec.push(RedactionRule {
            pattern: $pattern,
            limit: $limit,
            replacement: $replacement,
        });
        assert_snap_impl!(@munch $vec; $($rest)*);
    };

    // Internal muncher: Rule WITH limit (last rule)
    (@munch $vec:ident; [$limit:expr] $pattern:expr => $replacement:expr) => {
        $vec.push(RedactionRule {
            pattern: $pattern,
            limit: $limit,
            replacement: $replacement,
        });
    };

    // Internal muncher: Rule WITHOUT limit (followed by a comma and more rules)
    (@munch $vec:ident; $pattern:expr => $replacement:expr , $($rest:tt)*) => {
        $vec.push(RedactionRule {
            pattern: $pattern,
            limit: 0,
            replacement: $replacement,
        });
        assert_snap_impl!(@munch $vec; $($rest)*);
    };

    // Internal muncher: Rule WITHOUT limit (last rule)
    (@munch $vec:ident; $pattern:expr => $replacement:expr) => {
        $vec.push(RedactionRule {
            pattern: $pattern,
            limit: 0,
            replacement: $replacement,
        });
    };

    // Main arm with rules
    (
        // &str
        $assertion_id:expr,
        // &str
        $source_file_path:expr,
        // &str
        $real_value:expr,
        // Rules token stream
        $($rules:tt)+
    ) => {
        use redaction::*;

        let mut redaction_rules = Vec::new();
        assert_snap_impl!(@munch redaction_rules ; $($rules)+);

        assert_impl::assert_snap(
            $assertion_id,
            $source_file_path,
            $real_value,
            &redaction_rules,
        );
    };

    // Main arm without rules
    (
        // &str
        $assertion_id:expr,
        // &str
        $source_file_path:expr,
        // &str
        $real_value:expr
    ) => {
        assert_impl::assert_snap(
            $assertion_id,
            $source_file_path,
            $real_value,
            Default::default(),
        );
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_snap_impl() {
        // With limits
        assert_snap_impl!("1", file!(), "Hello World!", [1]"o"=>"#", "xx"=>"**");
    }
}

fn get_assertion_id(
    assertion_method: &str,
    source_file_path: &str,
    real_value: &str,
    rules: &str,
) -> String {
    blake3::hash(format!("{assertion_method},{source_file_path},{real_value},{rules}").as_bytes())
        .to_string()
}
