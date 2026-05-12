fn _xo(s: &str) -> bool {
    s.chars().filter(|c| c.eq_ignore_ascii_case(&'x')).count()
        == s.chars().filter(|c| c.eq_ignore_ascii_case(&'o')).count()
}

#[cfg(test)]
mod tests {
    use super::_xo;

    fn do_test(s: &str, expected: bool) {
        let actual = _xo(s);
        assert_eq!(
            expected, actual,
            "Test failed.\n\nInput:    {s:?}\nExpected: {expected}\nActual:   {actual}\n"
        )
    }

    #[test]
    fn sample_tests() {
        do_test("xo", true);
        do_test("Xo", true);
        do_test("xxOo", true);
        do_test("xxxm", false);
        do_test("Oo", false);
        do_test("ooom", false);
    }
}
