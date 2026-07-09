//! # assert-snap
//!
//! A snapshot testing and assertion library for Rust.
//!
//! ## Example
//!
//! ```rust
//! use assert_snap::add;
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
/// use assert_snap::add;
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
