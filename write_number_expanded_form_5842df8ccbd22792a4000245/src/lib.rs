fn _expanded_form(n: u64) -> String {
    let end_exponent = n.ilog10() as usize + 1;
    let mut expanded: Vec<u64> = Vec::with_capacity(end_exponent);
    let mut dividend: u64 = n.clone();
    for exponent in 1..end_exponent + 1 {
        let remainder = dividend % 10u64.pow(exponent as u32);
        if remainder != 0 {
            expanded.push(remainder);
        }
        dividend = dividend - remainder;
    }
    expanded.reverse();
    expanded
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(" + ")
        .to_string()
}

fn _expanded_form2(n: u64) -> String {
    n.to_string()
        .chars()
        .rev()
        .enumerate()
        .filter_map(|(exponent, remainder)| match remainder {
            '0' => None,
            remainder => Some(format!("{}{}", remainder, "0".repeat(exponent))),
        })
        .collect::<Vec<String>>()
        .into_iter()
        .rev()
        .collect::<Vec<String>>()
        .join(" + ")
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let mut result = _expanded_form2(12);
        assert_eq!(result, "10 + 2");
        result = _expanded_form2(42);
        assert_eq!(result, "40 + 2");
        result = _expanded_form2(70304);
        assert_eq!(result, "70000 + 300 + 4");
    }
}
