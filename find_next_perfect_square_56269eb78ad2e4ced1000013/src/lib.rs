fn _find_next_square(sq: u64) -> Option<u64> {
    let root = (sq as f64).sqrt() as u64;
    if root * root == sq {
        Some((root + 1).pow(2))
    } else {
        None
    }
}
#[cfg(test)]
mod tests {
    use super::_find_next_square;

    fn do_test(n: u64, expected: Option<u64>) {
        let actual = _find_next_square(n);
        assert_eq!(actual, expected, "\nYour result (left), did not match the correct answer (right) for n = {n}");
    }

    #[test]
    fn sample_tests() {
        do_test(121, Some(144));
        do_test(625, Some(676));
        do_test(319_225, Some(320_356));
        do_test(15_241_383_936, Some(15_241_630_849));
        do_test(155, None);
        do_test(342_786_627, None);
    }
}