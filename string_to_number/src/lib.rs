fn string_to_number(s: &str) -> i32 {
    s.parse::<i32>().expect("Failed to parse the string to i32")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_number_valid() {
        assert_eq!(string_to_number("123"), 123);
        assert_eq!(string_to_number("-123"), -123);
        assert_eq!(string_to_number("0"), 0);
    }
}
