fn _string_to_array(s: &str) -> Vec<String> {
    s.split_whitespace()
        .map(str::to_string)
        .collect::<Vec<String>>()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::_string_to_array;

    fn dotest(s: &str, expected: &[&str]) {
        let actual = _string_to_array(s);
        assert_eq!(
            actual, expected,
            "Test failed with s = \"{s}\"\nExpected {expected:?} but got {actual:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest("Robin Singh", &["Robin", "Singh"]);
        dotest("CodeWars", &["CodeWars"]);
        dotest(
            "I love arrays they are my favorite",
            &["I", "love", "arrays", "they", "are", "my", "favorite"],
        );
        dotest("1 2 3", &["1", "2", "3"]);
    }
}
