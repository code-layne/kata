fn solution(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut pair = String::new();
    for c in s.chars() {
        pair.push(c);
        if pair.len() == 2 {
            result.push(pair);
            pair = String::new();
        }
    }
    if !pair.is_empty() {
        pair.push('_');
        result.push(pair);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution(""), [] as [&str; 0]);
    }
}
