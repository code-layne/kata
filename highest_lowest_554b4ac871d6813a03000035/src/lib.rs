fn _high_and_low(numbers: &str) -> String {
    let nums: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    format!(
        "{} {}",
        nums.iter().max().unwrap(),
        nums.iter().min().unwrap()
    )
}

#[test]
fn example_test_1() {
    assert_eq!("42 -9", _high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}

#[test]
fn example_test_2() {
    assert_eq!("3 1", _high_and_low("1 2 3"));
}
