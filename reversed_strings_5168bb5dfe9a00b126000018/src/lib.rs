fn _solution(phrase: &str) -> String {
    phrase.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_test() {
        assert_eq!(_solution("world"), "dlrow");
    }
}