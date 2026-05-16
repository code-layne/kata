fn _greet(name: &str) -> String {
    format!("Hello, {} how are you doing today?", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(_greet("Ryan"), "Hello, Ryan how are you doing today?");
        assert_eq!(
            _greet("Shingles"),
            "Hello, Shingles how are you doing today?"
        );
    }
}
