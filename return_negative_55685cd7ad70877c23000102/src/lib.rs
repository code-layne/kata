use std::cmp::Ordering;

fn _make_negative(n: i32) -> i32 {
    match n.cmp(&0) {
        Ordering::Greater => -n,
        _ => n,
    }
}

#[cfg(test)]
mod tests {
    use super::_make_negative;

    #[test]
    fn sample_tests() {
        assert_eq!(_make_negative(1), -1);
        assert_eq!(_make_negative(-5), -5);
        assert_eq!(_make_negative(0), 0);
    }
}