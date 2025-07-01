use axum::Json;
use axum::http::StatusCode;
use std::str::FromStr;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use spl_associated_token_account::get_associated_token_address;
use spl_token::{ID, instruction::transfer};
use solana_program::pubkey::Pubkey;
use solana_program::system_instruction;

use crate::{
    errors::ApiError,
    models::transfer::{SendTokenRequest, SendTokenResponse, SendTokenData, AccountSignerInfo, SendSolRequest, SendSolResponse, SendSolData},
};

/// POST /send/sol
/// Builds a SOL transfer instruction using the System program
pub async fn send_sol(
    Json(payload): Json<SendSolRequest>
) -> Result<Json<SendSolResponse>, ApiError> {
    if payload.from.is_empty() || payload.to.is_empty() {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "Missing required fields: from, to",
        ));
    }

    if payload.lamports == 0 {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "Amount must be greater than 0",
        ));
    }

    let from_pubkey = Pubkey::from_str(&payload.from)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, format!("invalid `from` pubkey: {}", e)))?;
    
    let to_pubkey = Pubkey::from_str(&payload.to)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, format!("invalid `to` pubkey: {}", e)))?;

    let instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, payload.lamports);

    let instruction_data = STANDARD.encode(&instruction.data);

    let accounts = vec![
        from_pubkey.to_string(),
        to_pubkey.to_string(),
    ];

    let resp = SendSolResponse {
        success: true,
        data: SendSolData {
            program_id: instruction.program_id.to_string(),
            accounts,
            instruction_data,
        },
    };

    Ok(Json(resp))
}

/// POST /send/token
/// Builds an SPL‐Token `transfer` instruction, moving `amount` from the owner's
/// token‐account to the destination's token‐account (both derived via ATA).
pub async fn send_token(
    Json(payload): Json<SendTokenRequest>
) -> Result<Json<SendTokenResponse>, ApiError> {
    let mint_pk = Pubkey::from_str(&payload.mint)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, format!("invalid `mint`: {}", e)))?;
    let owner_pk = Pubkey::from_str(&payload.owner)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, format!("invalid `owner`: {}", e)))?;
    let dest_owner_pk = Pubkey::from_str(&payload.destination)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, format!("invalid `destination`: {}", e)))?;

    let source_ata = get_associated_token_address(&owner_pk, &mint_pk);
    let dest_ata   = get_associated_token_address(&dest_owner_pk, &mint_pk);

    let instr = transfer(
        &ID,
        &source_ata,
        &dest_ata,
        &owner_pk,
        &[],
        payload.amount,
    ).map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, format!("transfer failed: {}", e)))?;

    let accounts = vec![
        AccountSignerInfo { pubkey: source_ata.to_string(), is_signer: false },
        AccountSignerInfo { pubkey: dest_ata.to_string(),   is_signer: false },
        AccountSignerInfo { pubkey: owner_pk.to_string(),  is_signer: true  },
    ];

    let instruction_data = STANDARD.encode(&instr.data);

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
