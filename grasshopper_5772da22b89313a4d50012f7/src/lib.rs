fn _greet(name: &str, owner: &str) -> String {
    format!("Hello {}", if name.eq(owner) {"boss"} else {"guest"})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(_greet("Daniel", "Daniel"), "Hello boss");
        assert_eq!(_greet("Greg", "Daniel"), "Hello guest");
    }
}
