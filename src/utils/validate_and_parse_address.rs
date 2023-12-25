use regex::Regex;


pub fn check_valid_address(address: &str) -> Result<String, String> {
    if starts_with_0x_len_42_hex(address) {
        Ok(address.to_string())
    } else {
        Err(format!("{} is not a valid address.", address))
    }
}

// Checks a string starts with 0x, is 42 characters long and contains only hex characters after 0x
pub fn starts_with_0x_len_42_hex(address: &str) -> bool {
    lazy_static::lazy_static! {
        static ref STARTS_WITH_0X_LEN_42_HEX_REGEX: Regex = Regex::new(r"^0x[0-9a-fA-F]{40}$").unwrap();
    }
    STARTS_WITH_0X_LEN_42_HEX_REGEX.is_match(address)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_address() {
        let valid_address = "0x1234567890123456789012345678901234567890";
        assert!(starts_with_0x_len_42_hex(valid_address));
        assert_eq!(
            check_valid_address(valid_address),
            Ok(valid_address.to_string())
        );
    }

    #[test]
    fn test_invalid_address() {
        let invalid_address = "0xinvalidaddress";
        assert!(!starts_with_0x_len_42_hex(invalid_address));
        assert_eq!(
            check_valid_address(invalid_address),
            Err(format!("{} is not a valid address.", invalid_address))
        );
    }
}
