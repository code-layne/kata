use itertools::Itertools;

fn _unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug + Clone,
{
    let mut last: Option<T::Item> = None;
    sequence.into_iter().filter_map(|x| {
        if last.as_ref() == Some(&x) {
            None
        } else {
            last = Some(x.clone());
            Some(x)
        }
    }).collect()
}
fn _unique_in_order2<T>(sequence: T) -> Vec<T::Item>
where
    T: IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug + Clone,
{
    sequence.into_iter().dedup().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(_unique_in_order("AAAABBBCCDAABBB".chars()), vec!['A', 'B', 'C', 'D', 'A', 'B']);
    }
}