use std::ops::Add;

fn _longest(a1: &str, a2: &str) -> String {
    a1.to_string().add(a2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", _longest(s1, s2), exp);
        println!("{}", _longest(s1, s2) == exp);
        assert_eq!(&_longest(s1, s2), exp)
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
        testing("loopingisfunbutdangerous", "lessdangerousthancoding", "abcdefghilnoprstu");

    }
}
