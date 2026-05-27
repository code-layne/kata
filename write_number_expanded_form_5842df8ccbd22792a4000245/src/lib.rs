fn _expanded_form(n: u64) -> String {
    n.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(_expanded_form(12), "10 + 2");
        assert_eq!(_expanded_form(42), "40 + 2");
        assert_eq!(_expanded_form(70304), "70000 + 300 + 4");
    }
}