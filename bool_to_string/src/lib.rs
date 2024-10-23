fn boolean_to_string(b: bool) -> String {
    match b {
        // String::from and to_string allocate both on the heap
        true => String::from("true"),
        false => "false".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean_to_string_validate() {
        assert_eq!(boolean_to_string(true), "true");
        assert_eq!(boolean_to_string(false), "false");
    }
}
