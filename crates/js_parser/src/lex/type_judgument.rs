use regex::Regex;

pub fn is_number(s: &str) -> bool {
    Regex::new(r"^[0-9]+$").unwrap().is_match(s)
}

pub fn is_string(s: &str) -> bool {
    Regex::new(r#"^["'][^"']*["']$"#).unwrap().is_match(s)
}

pub fn is_blank(s: &str) -> bool {
    Regex::new(r"^[\s\t\n]+$").unwrap().is_match(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_is_number() {
        assert!(is_number("1"));
        assert!(!is_number("h"));
    }

    #[test]
    fn test_test() {
        println!("{}", env::current_dir().unwrap().display())
    }
}
