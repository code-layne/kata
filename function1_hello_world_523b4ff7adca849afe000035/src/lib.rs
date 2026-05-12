// Write a function greet() that returns "hello world!"
// Remember to use the correct return type
fn _greet() -> String {
    "hello world!".to_string()
}
fn _greet2() -> &'static str {
    "hello world!"
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greets_the_world() {
        assert_eq!(_greet(), "hello world!", "should return the correct message");
        assert_eq!(_greet2(), "hello world!", "should return the correct message");
    }
}