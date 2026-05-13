fn _open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    const OPEN: &str = "Open";
    const SENIOR: &str = "Senior";
    const SENIOR_AGE_GE: i32 = 55;
    const SENIOR_HANDICAP_GT: i32 = 7;
    data.into_iter()
        .map(|(age, handicap)| {
            if age >= SENIOR_AGE_GE && handicap > SENIOR_HANDICAP_GT {
                SENIOR.to_string()
            } else {
                OPEN.to_string()
            }
        })
        .collect()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(
            _open_or_senior(vec![(45, 12), (55, 21), (19, -2), (104, 20)]),
            vec!["Open", "Senior", "Open", "Senior"]
        );
        assert_eq!(
            _open_or_senior(vec![(3, 12), (55, 1), (91, -2), (54, 23)]),
            vec!["Open", "Open", "Open", "Open"]
        );
    }
}
