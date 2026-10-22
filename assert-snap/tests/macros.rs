#![allow(dead_code)]

use std::collections::BTreeMap;

use assert_snap::*;

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

// ===== Macro hygiene tests =====

#[test]
fn test_macro_hygiene_variable_named_redaction_rules_in_replacement() {
    let redaction_rules = "****";
    assert_snap!("secret", "****", "secret" => redaction_rules);
}

#[test]
fn test_macro_hygiene_variable_named_redaction_rules_in_pattern() {
    let redaction_rules = "secret";
    assert_snap!("secret", "****", redaction_rules => "****");
}

#[test]
fn test_macro_hygiene_variable_named_redaction_rules_in_limit() {
    let redaction_rules = 1;
    assert_snap!("secret secret", "**** secret", [redaction_rules] "secret" => "****");
}

#[test]
fn test_macro_hygiene_assert_snap_expected_named_actual() {
    let actual = "MATCHING_VALUE";
    assert_snap!("MATCHING_VALUE", actual);
}

#[test]
#[should_panic]
fn test_macro_hygiene_assert_snap_silent_false_positive_prevention() {
    let actual = "WRONG_VALUE";
    assert_snap!("EXPECTED_VALUE", actual);
}

#[test]
#[should_panic]
fn test_macro_hygiene_assert_debug_snap_silent_false_positive_prevention() {
    let actual = "WRONG_VALUE";
    assert_debug_snap!("EXPECTED_VALUE", actual);
}

#[test]
fn test_macro_hygiene_no_scope_pollution() {
    let actual = 42;
    let expected = 99;
    assert_snap!("hello", "hello");
    assert_debug_snap!(100, 100);
    assert_eq!(actual, 42);
    assert_eq!(expected, 99);
}

// ===== Trailing comma tests =====

#[test]
fn test_assert_snap_trailing_comma_basic() {
    assert_snap!("hello", "hello",);
}

#[test]
fn test_assert_snap_trailing_comma_with_rule() {
    assert_snap!(
        "secret",
        "****",
        "secret" => "****",
    );
}

#[test]
fn test_assert_snap_trailing_comma_with_limit_rule() {
    assert_snap!(
        "secret secret",
        "**** secret",
        [1] "secret" => "****",
    );
}

#[test]
fn test_assert_snap_trailing_comma_multiple_rules() {
    assert_snap!(
        "secret 123",
        "**** [NUM]",
        "secret" => "****",
        r"\d+" => "[NUM]",
    );
}

#[test]
fn test_assert_debug_snap_trailing_comma_basic() {
    assert_debug_snap!(Some(42), Some(42),);
}

#[test]
fn test_assert_debug_snap_trailing_comma_with_rule() {
    assert_debug_snap!(
        Some("secret"),
        Some("****"),
        "secret" => "****",
    );
}

// ===== Non-printable / invisible character tests =====

#[test]
fn test_assert_snap_matching_with_invisible_characters() {
    let response = "status for auth-api\0\0\0: 200 OK";
    assert_snap!(response, "status for auth-api\0\0\0: 200 OK");
}

#[test]
#[should_panic]
fn test_assert_snap_failing_with_invisible_null_bytes() {
    let response = "status for auth-api\0\0\0: 200 OK";
    assert_snap!(response, "status for auth-api: 200 OK");
}
