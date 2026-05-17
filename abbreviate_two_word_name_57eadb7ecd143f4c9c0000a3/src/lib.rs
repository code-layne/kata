fn _abbrev_name(name: &str) -> String {
   name.to_string()
}
// Rust test example:
#[test]
fn sample_tests() {
    assert_eq!(_abbrev_name("Sam Harris"), "S.H");
    assert_eq!(_abbrev_name("Patrick Feenan"), "P.F");
    assert_eq!(_abbrev_name("Evan Cole"), "E.C");
    assert_eq!(_abbrev_name("P Favuzzi"), "P.F");
    assert_eq!(_abbrev_name("David Mendieta"), "D.M");
}