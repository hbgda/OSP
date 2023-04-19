use once_cell::sync::Lazy;
use regex::Regex;

const NAME_PATTERN: Lazy<Regex> = Lazy::new(
    || Regex::new(r"[a-zA-Z'\-]*").unwrap());

const EMAIL_LOCAL_PATTERN: Lazy<Regex> = Lazy::new(
    || Regex::new(r"(?:^[^\s.]?(?:[a-zA-Z0-9!#$%&'*+\-/=?^_`{|}~][\.]{0,1})*[^\s.]+)").unwrap()); 

const EMAIL_DOMAIN_PATTERN: Lazy<Regex> = Lazy::new(
    || Regex::new(r"[^\.\-@][a-zA-Z0-9\-]+[\.][a-zA-Z0-9\-]*[^\.\-]").unwrap());

const PASSWORD_MIN_LENGTH: usize = 7;
const PASSWORD_MAX_LENGTH: usize = 21;
const PASSWORD_CHARS_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[a-zA-Z0-9]").unwrap()
});

pub fn validate_email(email: String) -> bool {
    let slices: Vec<&str> = email.split("@").collect();
    if slices.len() != 2 {
        return false;
    }

    let local = slices[0];
    let domain = slices[1];

    EMAIL_LOCAL_PATTERN.is_match(local) &&
    EMAIL_DOMAIN_PATTERN.is_match(domain)
}

pub fn validate_password(password: String) -> bool {
    password.len() >= PASSWORD_MIN_LENGTH &&
    password.len() <= PASSWORD_MAX_LENGTH &&
    password.to_uppercase() != password   &&
    password.to_lowercase() != password   &&
    PASSWORD_CHARS_PATTERN.replace_all(&password, "") != ""
}

pub fn validate_name(name: String) -> bool {
    name.len() > 0  &&
    NAME_PATTERN.is_match(&name)
}