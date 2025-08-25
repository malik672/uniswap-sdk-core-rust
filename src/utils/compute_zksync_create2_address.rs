use alloy_primitives::{Address, B256, Bytes, keccak256};

#[inline]
#[must_use]
pub fn compute_zksync_create2_address(
    sender: Address,
    bytecode_hash: B256,
    salt: B256,
    input: Option<Bytes>,
) -> Address {
    let prefix = keccak256("zksyncCreate2");
    let input_hash = keccak256(input.unwrap_or_default());
    let mut bytes = [0; 160];
    bytes[0..32].copy_from_slice(prefix.as_slice());
    bytes[44..64].copy_from_slice(sender.as_slice());
    bytes[64..96].copy_from_slice(salt.as_slice());
    bytes[96..128].copy_from_slice(bytecode_hash.as_slice());
    bytes[128..160].copy_from_slice(input_hash.as_slice());
    Address::from_word(keccak256(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{address, b256};

    #[test]
    fn test_compute_zksync_create2_address() {
        const USDCE: Address = address!("3355df6D4c9C3035724Fd0e3914dE96A5a83aaf4");
        const WETH: Address = address!("5AEa5775959fBC2557Cc8789bC1bf90A239D9a91");
        let mut bytes = [0; 96];
        bytes[12..32].copy_from_slice(USDCE.as_slice());
        bytes[44..64].copy_from_slice(WETH.as_slice());
        bytes[92..96].copy_from_slice(3000_u32.to_be_bytes().as_slice());
        let salt = keccak256(bytes);
        let result = compute_zksync_create2_address(
            address!("8FdA5a7a8dCA67BBcDd10F02Fa0649A937215422"),
            b256!("010013f177ea1fcbc4520f9a3ca7cd2d1d77959e05aa66484027cb38e712aeed"),
            salt,
            None,
        );
        assert_eq!(result, address!("ff577f0E828a878743Ecc5E2632cbf65ceCf17cF"));
    }
}
