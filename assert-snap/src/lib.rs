mod assert_impl;
mod redaction;

#[macro_export]
macro_rules! assert_snap {
    () => {};
}

macro_rules! assert_snap_impl {
    (
        // assertion method (&str)
        $method:expr_2021,
        // assertion id (&str)
        $id:expr_2021,
        // Cow<'a, str>
        $real_value:expr_2021) => {
        todo!()
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_assert_snap_impl_no_redaction() {
        let id = "1";
        let path = assert_impl::get_snap_file_path(id, file!());
        println!("{path:?}");
    }
}
