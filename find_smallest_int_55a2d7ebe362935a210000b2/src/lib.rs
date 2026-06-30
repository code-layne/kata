fn _find_smallest_int(arr: &[i32]) -> i32 {
    arr.iter().sum()
}
// Rust test example:
#[test]
fn sample_tests() {
    assert_eq!(_find_smallest_int(&[34, 15, 88, 2]), 2);
    assert_eq!(_find_smallest_int(&[34, -345, -1, 100]), -345);
}