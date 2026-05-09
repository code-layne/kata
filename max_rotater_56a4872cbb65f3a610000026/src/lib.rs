use std::collections::VecDeque;
fn _max_rot(n: u64) -> u64 {
    let mut chars = n.to_string().chars().collect::<VecDeque<char>>();
    let mut rotated:Vec<u64> = Vec::with_capacity(chars.len());
    rotated.push(n);

    for rotation in 0..chars.len() {
        let swap_char = VecDeque::remove(&mut chars, rotation).unwrap();
        chars.push_back(swap_char);
        rotated.push(chars.iter().collect::<String>().parse::<u64>().unwrap());
    }

    rotated.iter().max().unwrap().clone()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(_max_rot(38458215), 85821534);
        assert_eq!(_max_rot(195881031), 988103115);
        assert_eq!(_max_rot(896219342), 962193428);
        assert_eq!(_max_rot(69418307), 94183076);
    }
}
