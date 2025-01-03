use crate::entities::token::Token;

/// This function demonstrates basic operations with the Token struct from the Uniswap SDK-Core.
#[test]
fn main() {
    // Create a new Token instance for DAI
    let dai_token = Token::new(
        1, // Assuming chain_id is 1 for Ethereum mainnet
        "0x6B175474E89094C44Da98b954EedeAC495271d0F"
            .parse()
            .unwrap(), // Assuming this is the correct address
        18, // Decimals for DAI
        Some("DAI".to_string()), // Symbol for DAI
        Some("Dai Stablecoin".to_string()), // Name for DAI
        None, // Assuming no buy fee
        None, // Assuming no sell fee
    );

    // Print the token's address and decimals
    println!("DAI Token Address: {}", dai_token.address);
    println!("DAI Token Decimals: {}", dai_token.decimals);

    // Convert a token chain id
    let token_id = dai_token.chain_id;
    println!("token_id {}", token_id);
}
