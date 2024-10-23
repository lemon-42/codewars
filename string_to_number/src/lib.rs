fn string_to_number(s: &str) -> i32 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_number_valid() {
        assert_eq!(string_to_number("123"), 123);
        assert_eq!(string_to_number("-123"), -123);
        assert_eq!(string_to_number("0"), 0);
    }

    #[test]
    fn test_invalid_string_to_number() {
        assert_eq!(string_to_number("abc"), 0);
        assert_eq!(string_to_number("12abc345"), 0);
        assert_eq!(string_to_number(""), 0);
    }
}
