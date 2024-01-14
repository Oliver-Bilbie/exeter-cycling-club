use regex::Regex;

pub fn validate_email(email: String) -> bool {
    match Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$") {
        Ok(re) => re.is_match(&email),
        Err(_) => false,
    }
}
