fn _longest_consec(strarr: Vec<&str>, k: usize) -> String {
    let mut longest = String::new();

    if k == 0 { return longest };

    for candidate in strarr.windows(k).map(|w| w.join("")) {
        if candidate.len() > longest.len() {
            longest = candidate;
        }
    }

    longest
}

fn _testing(strarr: Vec<&str>, k: usize, exp: &str) -> () {
    assert_eq!(&_longest_consec(strarr, k), exp)
}

#[test]
fn basics_longest_consec() {
    _testing(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2, "abigailtheta");
    _testing(vec!["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", "oocccffuucccjjjkkkjyyyeehh"], 1,
            "oocccffuucccjjjkkkjyyyeehh");
    _testing(vec![], 3, "");
    _testing(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 3, "ixoyx3452zzzzzzzzzzzz");
    _testing(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
    _testing(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
}