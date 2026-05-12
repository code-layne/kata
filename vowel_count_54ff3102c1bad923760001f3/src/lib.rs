fn _get_count(s: &str) -> usize {
    s.chars().filter(|&c| "aeiou".contains(c)).count()
}
#[test]
fn my_tests() {
    assert_eq!(_get_count("abracadabra"), 5);
}