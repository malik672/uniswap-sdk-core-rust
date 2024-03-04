use regex::Regex;

/// Checks if the input string is a valid Ethereum address.
///
/// # Arguments
///
/// * `ethereum_address` - A string slice that holds the Ethereum address to be validated.
///
/// # Returns
///
/// * If the input string satisfies the condition of starting with `0x` and being 42 characters long
///   with only hexadecimal characters after `0x`, returns `Ok(ethereum_address.to_string())`.
/// * Otherwise, returns an error message in the form of `Err(format!("{} is not a valid Ethereum
///   address.", ethereum_address))`.
pub fn check_valid_ethereum_address(ethereum_address: &str) -> Result<&str, String> {
    let valid_address_regex = Regex::new(r"^0x[0-9a-fA-F]{40}$").unwrap();
    if valid_address_regex.is_match(ethereum_address) {
        Ok(ethereum_address)
    } else {
        Err(format!(
            "{} is not a valid Ethereum address.",
            ethereum_address
        ))
    }
}

/// Validates the input string as an Ethereum address and returns the checksummed address.
///
/// # Arguments
///
/// * `ethereum_address` - A string slice that holds the Ethereum address to be validated and
///   checksummed.
///
/// # Returns
///
/// * If the input string satisfies the condition of starting with `0x` and being 42 characters long
///   with only hexadecimal characters after `0x`, returns the checksummed address.
/// * Otherwise, returns an error message in the form of `Err(format!("{} is not a valid Ethereum
///   address.", ethereum_address))`.
pub fn validate_and_parse_address(ethereum_address: &str) -> Result<String, String> {
    let checksummed_address = eth_checksum::checksum(ethereum_address);
    check_valid_ethereum_address(&checksummed_address)?;
    Ok(checksummed_address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ethereum_address() {
        let valid_address = "0x1234567890123456789012345678901234567890";
        assert!(check_valid_ethereum_address(valid_address).is_ok());
    }

    #[test]
    fn test_invalid_ethereum_address() {
        let invalid_address = "0xinvalidaddress";
        assert!(check_valid_ethereum_address(invalid_address).is_err());
    }

    #[test]
    fn test_validate_and_parse_address() {
        let valid_address = "0x1234567890123456789012345678901234567890";

        assert_eq!(
            validate_and_parse_address(valid_address),
            Ok(valid_address.to_string())
        );

        let invalid_address = "0xInvAlIdAddrEsS";
        assert_eq!(
            validate_and_parse_address(invalid_address),
            Err(format!(
                "{} is not a valid Ethereum address.",
                invalid_address
            ))
        );
    }
}
