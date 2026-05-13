fn _get_middle(s:&str) -> &str {
    if s.len() % 2 == 1 {
        &s[(s.len() / 2)..(s.len() / 2+1)]
    } else {
        &s[(s.len() / 2 - 1)..(s.len() / 2 + 1)]
    }
}

fn _get_middle2(s:&str) -> &str {
    s
}
#[test]
fn example_tests() {
    assert_eq!(_get_middle("test"), "es");
    assert_eq!(_get_middle("testing"), "t");
    assert_eq!(_get_middle("middle"), "dd");
    assert_eq!(_get_middle("A"), "A");
    assert_eq!(_get_middle("of"), "of");
}