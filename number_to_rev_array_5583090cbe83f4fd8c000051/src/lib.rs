fn _digitize(n: u64) -> Vec<u8> {
    Vec::from(n.to_string().as_bytes())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(_digitize(348597), vec![7, 9, 5, 8, 4, 3]);
        assert_eq!(_digitize(35231), vec![1, 3, 2, 5, 3]);
        assert_eq!(_digitize(0), vec![0]);
    }
}