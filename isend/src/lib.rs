pub fn isend(word: &str, ending: &str) -> bool {
    if word.ends_with(ending) {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_isend() {
        assert_eq!(isend("abc", "c"), true);
    }
}
