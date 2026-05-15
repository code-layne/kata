fn _solution(phrase: &str) -> String {
    phrase.chars().into_iter().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_test() {
        assert_eq!(_solution("world"), "dlrow");
    }
}