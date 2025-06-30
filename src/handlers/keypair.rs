use axum::Json;
use solana_sdk::signature::{Keypair, Signer};
use bs58;
use crate::errors::ApiError;
use crate::models::keypair::{KeypairResponse, KeypairData};

/// POST /keypair
/// Generate a new Solana keypair and return its base58-encoded pubkey + secret.
pub async fn generate() -> Result<Json<KeypairResponse>, ApiError> {
    // 1) generate
    let kp = Keypair::new();

    // 2) encode
    let pubkey_b58 = bs58::encode(kp.pubkey()).into_string();
    let secret_bytes = kp.to_bytes(); // [u8; 64]
    let secret_b58 = bs58::encode(secret_bytes).into_string();

    // 3) build response
    let resp = KeypairResponse {
        success: true,
        data: KeypairData {
            pubkey: pubkey_b58,
            secret: secret_b58,
        },
    };

    Ok(Json(resp))
}
