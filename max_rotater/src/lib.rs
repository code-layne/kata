
fn max_rot(n: u64) -> u64 {
    todo!()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(max_rot(38458215), 85821534);
        assert_eq!(max_rot(195881031), 988103115);
        assert_eq!(max_rot(896219342), 962193428);
        assert_eq!(max_rot(69418307), 94183076);
    }
}
