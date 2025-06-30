use serde::Serialize;

#[derive(Serialize)]
pub struct KeypairData {
    /// base58-encoded public key
    pub pubkey: String,
    /// base58-encoded secret key (64 bytes)
    pub secret: String,
}

#[derive(Serialize)]
pub struct KeypairResponse {
    /// always true on a 200
    pub success: bool,
    pub data: KeypairData,
}
