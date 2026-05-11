pub fn _remove_char(s: &str) -> String {
    s[1..s.len() - 1].to_string()
}

#[cfg(test)]
mod tests {
    use super::_remove_char;

    #[test]
    fn sample_cases() {
        assert_eq!(_remove_char("eloquent"), "loquen");
        assert_eq!(_remove_char("country"), "ountr");
        assert_eq!(_remove_char("person"), "erso");
        assert_eq!(_remove_char("place"), "lac");
        assert_eq!(_remove_char("ok"), "");
        assert_eq!(_remove_char("ooopsss"), "oopss");
    }
}