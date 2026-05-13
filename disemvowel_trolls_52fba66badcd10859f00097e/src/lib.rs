fn _disemvowel(s: &str) -> String {
    s.chars()
        .filter(|c| !"aeiou".contains(c.to_ascii_lowercase()))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::_disemvowel;

    #[test]
    fn example_test() {
        assert_eq!(
            _disemvowel("This website is for losers LOL!"),
            "Ths wbst s fr lsrs LL!"
        );
    }
}
