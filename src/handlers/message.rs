use axum::Json;
use axum::http::StatusCode;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use bs58;
use std::str::FromStr;

use solana_sdk::signature::{Keypair, Signer, Signature};
use solana_sdk::pubkey::Pubkey;

use crate::{
    errors::ApiError,
    models::message::{
        SignMessageRequest, SignMessageResponse, SignMessageData,
        VerifyMessageRequest, VerifyMessageResponse, VerifyMessageData,
    },
};

/// POST /message/sign
/// Signs an arbitrary message using the provided base58 secret key.
pub async fn sign(
    Json(payload): Json<SignMessageRequest>
) -> Result<Json<SignMessageResponse>, ApiError> {
    if payload.message.is_empty() || payload.secret.is_empty() {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "Missing required fields",
        ));
    }

    let secret_bytes = bs58::decode(&payload.secret)
        .into_vec()
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "Invalid base58 secret"))?;

    let keypair = Keypair::from_bytes(&secret_bytes)
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "Invalid secret key length"))?;

    let sig = keypair.sign_message(payload.message.as_bytes());

    let data = SignMessageData {
        signature: STANDARD.encode(sig.as_ref()),
        public_key: keypair.pubkey().to_string(),
        message: payload.message.clone(),
    };

    Ok(Json(SignMessageResponse { success: true, data }))
}

/// POST /message/verify
/// Verifies a signature against a message and public key.
pub async fn verify(
    Json(payload): Json<VerifyMessageRequest>
) -> Result<Json<VerifyMessageResponse>, ApiError> {
    let pubkey = Pubkey::from_str(&payload.pubkey)
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "Invalid base58 pubkey"))?;

    let sig_bytes = STANDARD
        .decode(&payload.signature)
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "Invalid base64 signature"))?;

    if sig_bytes.len() != 64 {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "Invalid signature length"));
    }

    let mut sig_array = [0u8; 64];
    sig_array.copy_from_slice(&sig_bytes);
    let signature = Signature::from(sig_array);

    let valid = signature.verify(&pubkey.to_bytes(), payload.message.as_bytes());

    let data = VerifyMessageData {
        valid,
        message: payload.message.clone(),
        pubkey: payload.pubkey.clone(),
    };

    Ok(Json(VerifyMessageResponse { success: true, data }))
}
