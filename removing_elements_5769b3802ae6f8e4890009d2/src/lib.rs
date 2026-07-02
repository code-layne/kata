fn _remove_every_other(arr: &[u8]) -> Vec<u8> {
    arr.to_vec()
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::_remove_every_other;

    #[test]
    fn sample_test() {
        assert_eq!(_remove_every_other(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), &[1, 3, 5, 7, 9]);
    }
}
