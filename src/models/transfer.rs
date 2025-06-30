use serde::{Deserialize, Serialize};

/// Request for POST /send/sol
#[derive(Deserialize)]
pub struct SendSolRequest {
    /// Base58 "from" pubkey (sender)
    pub from: String,
    /// Base58 "to" pubkey (recipient)
    pub to: String,
    /// Amount in lamports
    pub lamports: u64,
}

/// Response data for a successful SOL‐transfer instruction
#[derive(Serialize)]
pub struct SendSolData {
    /// System program ID
    pub program_id: String,
    /// Ordered list of account pubkeys involved
    pub accounts: Vec<String>,
    /// Base64‐encoded instruction data
    pub instruction_data: String,
}

/// Full JSON response for POST /send/sol
#[derive(Serialize)]
pub struct SendSolResponse {
    pub success: bool,
    pub data: SendSolData,
}

/// Request for POST /send/token
#[derive(Deserialize)]
pub struct SendTokenRequest {
    /// SPL mint address
    pub mint: String,
    /// Token account to receive the minted tokens
    pub destination: String,
    /// Owner of the source token account (must sign)
    pub owner: String,
    /// Amount in the mint's smallest units
    pub amount: u64,
}

/// Account entry for send-token response
#[derive(Serialize)]
pub struct AccountSignerInfo {
    pub pubkey: String,
    #[serde(rename = "isSigner")]
    pub is_signer: bool,
}

/// Response data for a successful token transfer instruction
#[derive(Serialize)]
pub struct SendTokenData {
    pub program_id: String,
    pub accounts: Vec<AccountSignerInfo>,
    pub instruction_data: String,
}

/// Full JSON response for POST /send/token
#[derive(Serialize)]
pub struct SendTokenResponse {
    pub success: bool,
    pub data: SendTokenData,
}