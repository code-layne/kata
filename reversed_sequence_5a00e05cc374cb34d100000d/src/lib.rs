fn _reverse_seq(n: u32) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::with_capacity(n as usize);
    for i in 0..n {
        vec.insert(i as usize, n - i);
    }
    vec
}

fn _reverse_seq2(n: u32) -> Vec<u32> {
    (1..=n).rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(_reverse_seq(5), [5, 4, 3, 2, 1].to_vec());
        assert_eq!(_reverse_seq2(5), [5, 4, 3, 2, 1].to_vec());
    }
}