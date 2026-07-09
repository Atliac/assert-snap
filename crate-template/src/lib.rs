//! # atliac-crate-template
//!
//! A minimal, production-ready Rust template designed to jumpstart library (crate) development.
//! It establishes a workspace-friendly structure and comes pre-configured with automated release
//! pipelines, versioning, and cross-platform distribution.
//!
//! ## Example
//!
//! ```rust
//! use atliac_crate_template::add;
//!
//! assert_eq!(add(2, 3), 5);
//! ```

/// Adds two numbers.
///
/// This is a placeholder function to demonstrate the library structure.
///
/// # Examples
///
/// ```rust
/// use atliac_crate_template::add;
///
/// let result = add(2, 2);
/// assert_eq!(result, 4);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
