pub mod assert_impl;
pub mod redaction;

#[macro_export]
macro_rules! assert_snap {
    ($actual:expr, $expected:expr) => {
        let actual = format!("{}", $actual);
        let expected = format!("{}", $expected);
        $crate::assert_snap!(@assert_str, &actual, &expected);
    };

    ($actual:expr, $expected:expr,$($tail:tt)+) => {
        let actual = format!("{}", $actual);
        let expected = format!("{}", $expected);
        $crate::assert_snap!(@assert_str, &actual, &expected, $($tail)+);
    };

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
        #[allow(clippy::vec_init_then_push)]
        {
            assert_snap!(@munch redaction_rules ; $($rules)+);
        }

        $crate::assert_impl::assert_snap(
            $actual,
            $expected,
            &redaction_rules
        );
    };

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

#[macro_export]
macro_rules! assert_debug_snap {
    ($actual:expr, $expected:expr) => {
        let actual = format!("{:#?}", $actual);
        let expected = format!("{:#?}", $expected);
        $crate::assert_snap!(@assert_str, &actual, &expected);
    };

    ($actual:expr, $expected:expr,$($tail:tt)+) => {
        let actual = format!("{:#?}", $actual);
        let expected = format!("{:#?}", $expected);
        $crate::assert_snap!(@assert_str, &actual, &expected, $($tail)+);
    };
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    // ===== assert_snap tests =====

    #[test]
    fn test_assert_snap_no_redaction() {
        assert_snap!("Hello World!", "Hello World!");
    }

    #[test]
    fn test_assert_snap_basic_redaction() {
        assert_snap!(
            "User password is secret123",
            "User password is ****123",
            "secret" => "****"
        );
    }

    #[test]
    fn test_assert_snap_with_limit() {
        assert_snap!(
            "secret and secret",
            "**** and secret",
            [1] "secret" => "****"
        );
    }

    #[test]
    fn test_assert_snap_multiple_rules() {
        assert_snap!(
            "api_key=abc123 password=xyz789",
            "api_key=**** password=****",
            r"api_key=.+\s" => "api_key=**** ",
            [1] r"password=.+" => "password=****"
        );
    }

    #[test]
    fn test_assert_snap_regex_special_chars() {
        assert_snap!(
            "price=$100.50",
            "price=****",
            r"\$\d+\.\d+" => "****"
        );
    }

    #[test]
    fn test_assert_snap_no_match() {
        assert_snap!(
            "nothing to hide here",
            "nothing to hide here",
            "secret" => "****"
        );
    }

    #[test]
    fn test_assert_snap_limit_zero_means_unlimited() {
        assert_snap!(
            "secret secret secret",
            "**** **** ****",
            [0] "secret" => "****"
        );
    }

    #[test]
    fn test_assert_snap_format_args() {
        let name = "Alice";
        let age = 30;
        assert_snap!(
            format!("User: {}, Age: {}", name, age),
            "User: Alice, Age: 30"
        );
    }

    // ===== assert_debug_snap tests =====

    #[test]
    fn test_assert_debug_snap_basic() {
        assert_debug_snap!(42, 42);
    }

    #[test]
    fn test_assert_debug_snap_struct() {
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }
        assert_debug_snap!(Point { x: 1, y: 2 }, Point { x: 1, y: 2 });
    }

    #[test]
    fn test_assert_debug_snap_vec() {
        assert_debug_snap!(vec![1, 2, 3], vec![1, 2, 3]);
    }

    #[test]
    fn test_assert_debug_snap_option() {
        assert_debug_snap!(Some("value"), Some("value"));
        assert_debug_snap!(None::<String>, None::<String>);
    }

    #[test]
    fn test_assert_debug_snap_enum() {
        #[derive(Debug, PartialEq)]
        enum Color {
            Red,
            Green,
            Blue,
        }
        assert_debug_snap!(Color::Red, Color::Red);
        assert_debug_snap!(Color::Blue, Color::Blue);
    }

    #[test]
    fn test_assert_debug_snap_with_redaction() {
        assert_debug_snap!(
            "User { name: \"Alice\", password: \"secret123\" }",
            "User { name: \"Alice\", password: \"****123\" }",
            "secret" => "****"
        );
    }

    #[test]
    fn test_assert_debug_snap_with_multiple_redaction_rules() {
        #[derive(Debug)]
        struct Config {
            api_key: String,
            password: String,
            debug: bool,
        }
        let config = Config {
            api_key: "abc123".into(),
            password: "xyz789".into(),
            debug: true,
        };
        assert_debug_snap!(
            config,
            Config {
                api_key: "****".into(),
                password: "****".into(),
                debug: true,
            },
            "abc123" => "****",
            "xyz789" => "****"
        );
    }

    #[test]
    fn test_assert_debug_snap_with_limit() {
        assert_debug_snap!(
            vec!["secret", "secret", "secret"],
            vec!["****", "secret", "secret"],
            [1] "secret" => "****"
        );
    }

    #[test]
    fn test_assert_debug_snap_complex_struct_with_redaction() {
        #[derive(Debug)]
        struct User {
            id: u64,
            email: String,
            token: String,
        }
        let user = User {
            id: 1,
            email: "user@example.com".into(),
            token: "Bearer abc.def.ghi".into(),
        };
        assert_debug_snap!(
            user,
            User {
                id: 1,
                email: "user@example.com".into(),
                token: "Bearer ****".into(),
            },
            r"Bearer [A-Za-z0-9._-]+" => "Bearer ****"
        );
    }

    #[test]
    fn test_assert_debug_snap_map() {
        use std::collections::BTreeMap;
        let mut map = BTreeMap::new();
        map.insert("key1", "value1");
        map.insert("key2", "secret");
        let mut expected = BTreeMap::new();
        expected.insert("key1", "value1");
        expected.insert("key2", "****");
        assert_debug_snap!(
            map,
            expected,
            "secret" => "****"
        );
    }

    #[test]
    fn test_assert_debug_snap_format_args() {
        let name = "Bob";
        let score = 95;
        assert_debug_snap!(
            format!("Player: {}, Score: {}", name, score),
            "Player: Bob, Score: 95"
        );
    }
}
