mod assert_impl;
mod redaction;

#[macro_export]
macro_rules! assert_snap {
    ($real_value:expr,$($t:tt)*) => {
        let assertion_id=get_assertion_id!("assert_snap",$($t)*);
        assert_snap_impl!(&assertion_id,$real_value,$($t)*);
    };
    ($real_value:expr) => {
        //assert_snap_impl!("assert_snap", &format!("{real_value}"));
    };
}

macro_rules! get_assertion_id {
    ($($t:tt)+) => {{
        let mut source=String::new();
        source.push_str(file!());
        source.push('\n');
        source.push_str(module_path!());
        source.push('\n');
        $(
            source.push_str(stringify!($t));
        )+

        let assertion_id=blake3::hash(source.as_bytes())
        .to_string();

        if(dev_debug_enabled())
        {
            println!("Assertion id {} generated from:",assertion_id);
            println!("{source}\n");
        }
        assertion_id
    }};
}

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
        $real_value:expr,
        // Rules token stream
        $($rules:tt)+
    ) => {
        use redaction::*;

        let mut redaction_rules = Vec::new();
        assert_snap_impl!(@munch redaction_rules ; $($rules)+);

        assert_impl::assert_snap(
            $assertion_id,
            file!(),
            $real_value,
            &redaction_rules,
        );
    };

    // Main arm without rules
    (
        // &str
        $assertion_id:expr,
        // &str
        $real_value:expr
    ) => {
        assert_impl::assert_snap(
            $assertion_id,
            $file!(),
            $real_value,
            Default::default(),
        );
    };
}

pub(crate) fn dev_debug_enabled() -> bool {
    const TRUE_VALUES: &[&str] = &["yes", "true", "1", "y", "on"];
    if let Ok(v) = std::env::var("ASSERT_SNAP_DEV_DEBUG")
        && TRUE_VALUES.contains(&v.to_lowercase().as_str())
    {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_snap_impl() {
        assert_snap!("Hello World", [1]"o"=>"#", "xx"=>"**");
    }
}
