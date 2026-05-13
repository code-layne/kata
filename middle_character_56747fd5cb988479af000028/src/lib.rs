fn _get_middle(s:&str) -> &str {
    todo!("get middle of string")
}

#[test]
fn example_tests() {
    assert_eq!(_get_middle("test"), "es");
    assert_eq!(_get_middle("testing"), "t");
    assert_eq!(_get_middle("middle"), "dd");
    assert_eq!(_get_middle("A"), "A");
    assert_eq!(_get_middle("of"), "of");
}