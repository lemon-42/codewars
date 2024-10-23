fn greet(name: &str) -> String {
    format!("Hello, {name} how are you doing today?")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_greet() {
        assert_eq!(greet("Ryan"), "Hello, Ryan how are you doing today?");
    }
}
