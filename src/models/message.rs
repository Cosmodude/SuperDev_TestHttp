use serde::{Deserialize, Serialize};

/// Request payload for POST /message/sign
#[derive(Deserialize)]
pub struct SignMessageRequest {
    /// The message to sign
    pub message: String,
    /// Base58-encoded 64-byte secret key
    pub secret: String,
}

/// Request payload for POST /message/verify
#[derive(Deserialize)]
pub struct VerifyMessageRequest {
    /// The message that was signed
    pub message: String,
    /// Base64-encoded signature
    pub signature: String,
    /// Base58-encoded public key used to sign
    pub pubkey: String,
}

/// Response data for a successful sign
#[derive(Serialize)]
pub struct SignMessageData {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

/// Full JSON response for POST /message/sign
#[derive(Serialize)]
pub struct SignMessageResponse {
    pub success: bool,
    pub data: SignMessageData,
}

/// Response data for a successful verify
#[derive(Serialize)]
pub struct VerifyMessageData {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

/// Full JSON response for POST /message/verify
#[derive(Serialize)]
pub struct VerifyMessageResponse {
    pub success: bool,
    pub data: VerifyMessageData,
}
