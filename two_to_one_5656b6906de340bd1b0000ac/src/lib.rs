use std::collections::{BTreeSet, HashSet};

fn _longest(a1: &str, a2: &str) -> String {
    let added_str = a1.to_string() + &a2;
    let mut uniq_sorted: Vec<char> = added_str
        .chars()
        .collect::<HashSet<char>>()
        .into_iter()
        .collect::<Vec<char>>();
    uniq_sorted.sort();

    uniq_sorted.iter().collect::<String>()
}

fn _longest2(a1: &str, a2: &str) -> String {
    a1.chars()
        .chain(a2.chars())
        .collect::<BTreeSet<char>>()
        .iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        let longest = _longest2(s1, s2);
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest, exp);
        println!("{}", longest == exp);
        assert_eq!(&longest, exp)
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
        testing(
            "loopingisfunbutdangerous",
            "lessdangerousthancoding",
            "abcdefghilnoprstu",
        );
    }
}
