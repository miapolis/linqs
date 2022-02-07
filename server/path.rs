use regex::Regex;

lazy_static! {
    // Regex that ensures that the string contains only letters, numbers, dashes, and underscores
    pub static ref VALID_LINK_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9\-_]+$").unwrap();
}

#[cfg(test)]
mod test {
    #[test]
    fn values() {
        assert!(super::VALID_LINK_REGEX.is_match("abcdefghijklmnopqrstuvwxyz"));
        assert!(super::VALID_LINK_REGEX.is_match("ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
        assert!(super::VALID_LINK_REGEX.is_match("0123456789"));
        assert!(super::VALID_LINK_REGEX.is_match("-"));
        assert!(super::VALID_LINK_REGEX.is_match("_"));
        assert!(super::VALID_LINK_REGEX.is_match("abcdefghijklmnopqrstuvwxyz0123456789-_"));
    }

    #[test]
    fn valid() {
        assert!(super::VALID_LINK_REGEX.is_match("alllowercase"));
        assert!(super::VALID_LINK_REGEX.is_match("ALLUPPERCASE"));
        assert!(super::VALID_LINK_REGEX.is_match("MixedCase"));
        assert!(super::VALID_LINK_REGEX.is_match("separated_words"));
        assert!(super::VALID_LINK_REGEX.is_match("with-hyphens"));
        assert!(super::VALID_LINK_REGEX.is_match("All_-TheThings-123"));
        assert!(super::VALID_LINK_REGEX.is_match("_-123-AZ"));
    }

    #[test]
    fn invalid() {
        assert_eq!(super::VALID_LINK_REGEX.is_match("!@#$"), false);
        assert_eq!(super::VALID_LINK_REGEX.is_match(""), false);
        assert_eq!(super::VALID_LINK_REGEX.is_match(" "), false);
        assert_eq!(super::VALID_LINK_REGEX.is_match("with spaces"), false);
        assert_eq!(super::VALID_LINK_REGEX.is_match("with/slashes"), false);
        assert_eq!(super::VALID_LINK_REGEX.is_match("with\\backslashes"), false);
        assert_eq!(super::VALID_LINK_REGEX.is_match("with\nnewlines"), false);
        assert_eq!(super::VALID_LINK_REGEX.is_match("with\rreturns"), false);
        assert_eq!(super::VALID_LINK_REGEX.is_match("with\ttabs"), false);
        assert_eq!(super::VALID_LINK_REGEX.is_match("with\r\nnewlines"), false);
        assert_eq!(super::VALID_LINK_REGEX.is_match("_-trailing-space "), false);
    }
}
