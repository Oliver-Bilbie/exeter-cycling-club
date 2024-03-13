use regex::Regex;

pub fn validate_email(email: &String) -> bool {
    match Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$") {
        Ok(re) => re.is_match(&email),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_email() {
        let email = "hello@world.com".to_string();
        assert_eq!(validate_email(&email), true);
    }

    #[test]
    fn no_at_symbol() {
        let email = "helloworld.com".to_string();
        assert_eq!(validate_email(&email), false);
    }

    #[test]
    fn no_dot() {
        let email = "hello@worldcom".to_string();
        assert_eq!(validate_email(&email), false);
    }

    #[test]
    fn no_domain() {
        let email = "hello@world.".to_string();
        assert_eq!(validate_email(&email), false);
    }

    #[test]
    fn no_special_chars() {
        let email = "helloworldcom".to_string();
        assert_eq!(validate_email(&email), false);
    }
}
