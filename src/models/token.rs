use serde::{Deserialize, Serialize};

/// Incoming JSON for POST /token/create
#[derive(Deserialize)]
pub struct CreateTokenRequest {
    /// base58 pubkey of the mint authority
    pub mint_authority: String,
    /// base58 pubkey of the new mint account
    pub mint: String,
    /// number of decimals
    pub decimals: u8,
}

/// Incoming JSON for POST /token/mint
#[derive(Deserialize)]
pub struct MintTokenRequest {
    /// base58 pubkey of the mint
    pub mint: String,
    /// base58 pubkey of the destination account
    pub destination: String,
    /// base58 pubkey of the mint authority
    pub authority: String,
    /// amount to mint
    pub amount: u64,
}

#[derive(Serialize)]
pub struct AccountMetaInfo {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize)]
pub struct CreateTokenData {
    pub program_id: String,
    pub accounts: Vec<AccountMetaInfo>,
    pub instruction_data: String, // base64-encoded
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    pub success: bool,
    pub data: CreateTokenData,
}

#[derive(Serialize)]
pub struct MintTokenData {
    pub program_id: String,
    pub accounts: Vec<AccountMetaInfo>,
    pub instruction_data: String, // base64
}

#[derive(Serialize)]
pub struct MintTokenResponse {
    pub success: bool,
    pub data: MintTokenData,
}
