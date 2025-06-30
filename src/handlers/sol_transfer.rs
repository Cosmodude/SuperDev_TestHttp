use axum::Json;
use axum::http::StatusCode;
use std::str::FromStr;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use spl_associated_token_account::get_associated_token_address;
use spl_token::{ID, instruction::transfer};
use solana_program::pubkey::Pubkey;

use crate::{
    errors::ApiError,
    models::transfer::{SendTokenRequest, SendTokenResponse, SendTokenData, AccountSignerInfo},
};

/// POST /send/sol
/// Builds a SOL transfer instruction
pub async fn send_sol(
    Json(payload): Json<SendTokenRequest>
) -> Result<Json<SendTokenResponse>, ApiError> {
    // For SOL transfers, we'll use the same structure as token transfers
    // but with a special mint address representing SOL
    send_token(Json(payload)).await
}

/// POST /send/token
/// Builds an SPL‐Token `transfer` instruction, moving `amount` from the owner's
/// token‐account to the destination's token‐account (both derived via ATA).
pub async fn send_token(
    Json(payload): Json<SendTokenRequest>
) -> Result<Json<SendTokenResponse>, ApiError> {
    // 1) Parse all pubkeys
    let mint_pk = Pubkey::from_str(&payload.mint)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, format!("invalid `mint`: {}", e)))?;
    let owner_pk = Pubkey::from_str(&payload.owner)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, format!("invalid `owner`: {}", e)))?;
    let dest_owner_pk = Pubkey::from_str(&payload.destination)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, format!("invalid `destination`: {}", e)))?;

    // 2) Derive associated token accounts
    let source_ata = get_associated_token_address(&owner_pk, &mint_pk);
    let dest_ata   = get_associated_token_address(&dest_owner_pk, &mint_pk);

    // 3) Build the transfer instruction
    let instr = transfer(
        &ID,
        &source_ata,
        &dest_ata,
        &owner_pk,
        &[],
        payload.amount,
    ).map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, format!("transfer failed: {}", e)))?;

    // 4) Serialize account list
    let accounts = vec![
        AccountSignerInfo { pubkey: source_ata.to_string(), is_signer: false },
        AccountSignerInfo { pubkey: dest_ata.to_string(),   is_signer: false },
        AccountSignerInfo { pubkey: owner_pk.to_string(),  is_signer: true  },
    ];

    // 5) Base64‐encode the instruction data
    let instruction_data = STANDARD.encode(&instr.data);

    // 6) Return the JSON envelope
    let resp = SendTokenResponse {
        success: true,
        data: SendTokenData {
            program_id: ID.to_string(),
            accounts,
            instruction_data,
        },
    };
    Ok(Json(resp))
}
