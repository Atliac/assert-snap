use std::{
    borrow::Cow,
    env::var,
    fs::{File, exists, read_to_string, write},
    io::Read,
    path::{Path, PathBuf},
};

use similar::TextDiff;

use crate::redaction::{RedactionRule, apply_redactions};

/// The name of the folder where snapshots will be stored.
pub(crate) const SNAP_FOLDER_NAME: &str = "assert_snap_snapshots";
pub(crate) const SNAP_ENV_PREFIX: &str = "ASSERT_SNAP_PASS";

#[track_caller]
pub(crate) fn assert_snap<'a>(
    assertion_id: &str,
    source_file_path: &str,
    real_value: &str,
    redaction_rules: &[RedactionRule],
) {
    let snap_file_path = get_snap_file_path(assertion_id, source_file_path);

    // DIFF needs a newline at the end of the content
    let mut real_value: Cow<str> = Cow::from(real_value);
    if !real_value.ends_with("\n") {
        real_value.to_mut().push('\n');
    }

    println!("===== REAL VALUE =====");
    println!("{real_value}");
    let real_value = apply_redactions(real_value.as_ref(), redaction_rules);
    if matches!(real_value, Cow::Owned(_)) {
        println!("===== REDACTED VALUE =====");
        println!("{real_value}");
    }

    println!("Snapshot file: {}", snap_file_path.display());
    let env_key_id = format!("{}_{}", SNAP_ENV_PREFIX, assertion_id);
    let env_key_all = format!("{}_ALL", SNAP_ENV_PREFIX);
    if should_update_snapshot(&env_key_id) || should_update_snapshot(&env_key_all) {
        update_snapshot(&snap_file_path, &real_value);
        println!("Snapshot updated.");
        println!("===== ASSERTION PASSED =====");
        return;
    }

    if !exists(&snap_file_path).expect("Failed to check if snapshot file exists") {
        println!("Warning: snapshot file doesn't exist.");
        handle_assertion_failure(&env_key_id);
    }

    let expected_value = read_to_string(&snap_file_path).expect("Failed to read snapshot file");
    if expected_value == real_value {
        println!("===== ASSERTION PASSED =====");
        return;
    }

    println!("===== EXPECTED VALUE =====");
    println!("{expected_value}");
    show_diff(&real_value, &expected_value);
    handle_assertion_failure(&env_key_id);
}

#[track_caller]
fn show_diff(real_value: &str, expected_value: &str) {
    println!("===== DIFF =====");
    let diff = TextDiff::from_lines(expected_value, real_value);
    println!(
        "{}",
        diff.unified_diff()
            .header("EXPECTED VALUE", "REAL/REDACTED VALUE")
    );
}

#[track_caller]
fn should_update_snapshot(env_key: &str) -> bool {
    const TRUE_VALUES: &[&str] = &["yes", "true", "1", "y", "on"];
    if let Ok(v) = var(&env_key)
        && TRUE_VALUES.contains(&v.to_lowercase().as_str())
    {
        println!("\n{} is set.", env_key);
        true
    } else {
        false
    }
}

#[track_caller]
fn update_snapshot(snap_file_path: &Path, value: &str) {
    if let Some(parent) = snap_file_path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    write(snap_file_path, value).unwrap()
}

#[track_caller]
fn handle_assertion_failure(env_key_id: &str) {
    println!("===== ASSERTION FAILED =====");
    println!("To update the snapshot, set either of the following environment variables:");
    println!("- {}=true", env_key_id);
    println!("- {}=true", format!("{}_ALL", SNAP_ENV_PREFIX));
    panic!()
}

#[track_caller]
pub(crate) fn get_snap_file_path(assertion_id: &str, source_file_path: &str) -> PathBuf {
    let source_file_path = Path::new(source_file_path);
    let parent = source_file_path
        .parent()
        .expect("source_file_path must have a parent directory");
    let file_name = source_file_path
        .file_name()
        .expect("source_file_path must have a file name");

    let mut snap_file_path: PathBuf = parent.into();
    snap_file_path.push(SNAP_FOLDER_NAME);
    snap_file_path.push(file_name);
    snap_file_path.push(assertion_id);
    snap_file_path
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    #[test]
    #[should_panic]
    fn test_assert_impl() {
        let real_value = "Hello World!";
        let id = "1";
        assert_snap(id, file!(), real_value, Default::default());
    }

    #[test]
    fn test_get_snap_file_path() {
        let id = "1";
        let source_file_path = "assert-snap/src/lib.rs";
        let expected = format!("assert-snap/src/{}/lib.rs/{}", SNAP_FOLDER_NAME, id);
        assert_eq!(
            get_snap_file_path(id, source_file_path),
            PathBuf::from(expected)
        );
    }
}
