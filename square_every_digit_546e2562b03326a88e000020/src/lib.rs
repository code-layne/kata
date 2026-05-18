fn _square_digits(num: u64) -> u64 {
    num
}

#[cfg(test)]
mod tests {
    use super::_square_digits;

    #[test]
    fn test_square_digits() {
        assert_eq!(_square_digits(9119), 811181, "\nFailed with num 9119");
    }
}
