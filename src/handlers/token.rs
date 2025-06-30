use axum::Json;
use std::str::FromStr;
use axum::http::StatusCode;
use base64::Engine;

use spl_token::solana_program::pubkey::Pubkey;
use spl_token::instruction::{initialize_mint, mint_to};
use crate::{
    errors::ApiError,
    models::token::{
        CreateTokenRequest, CreateTokenResponse,
        CreateTokenData, AccountMetaInfo,
        MintTokenRequest, MintTokenResponse, MintTokenData,
    },
};

/// POST /token/create
/// Constructs an `initialize_mint` SPL Token instruction.
pub async fn create(
    Json(payload): Json<CreateTokenRequest>
) -> Result<Json<CreateTokenResponse>, ApiError> {
    // 1) Decode the base58 inputs
    let mint_pubkey = Pubkey::from_str(&payload.mint)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, format!("invalid `mint`: {}", e)))?;
    let authority = Pubkey::from_str(&payload.mintAuthority)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, format!("invalid `mintAuthority`: {}", e)))?;

    // 2) Build the instruction
    let instr = initialize_mint(
        &spl_token::ID,
        &mint_pubkey,
        &authority,
        None, // no freeze authority
        payload.decimals,
    ).map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, format!("failed to create instruction: {}", e)))?;

    // 3) Map AccountMeta → our serializable form
    let accounts = instr
        .accounts
        .into_iter()
        .map(|meta| AccountMetaInfo {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        })
        .collect::<Vec<_>>();

    // 4) Base64‐encode the instruction data
    let instruction_data = base64::engine::general_purpose::STANDARD.encode(instr.data);

    // 5) Build and return the JSON
    let resp = CreateTokenResponse {
        success: true,
        data: CreateTokenData {
            program_id: instr.program_id.to_string(),
            accounts,
            instruction_data,
        },
    };
    Ok(Json(resp))
}

/// POST /token/mint
/// Build a `mint_to` SPL Token instruction
pub async fn mint(
    Json(payload): Json<MintTokenRequest>
) -> Result<Json<MintTokenResponse>, ApiError> {
    // 1) Decode inputs
    let mint_pk = Pubkey::from_str(&payload.mint)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, format!("invalid `mint` pubkey: {}", e)))?;
    let dest_pk = Pubkey::from_str(&payload.destination)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, format!("invalid `destination` pubkey: {}", e)))?;
    let auth_pk = Pubkey::from_str(&payload.authority)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, format!("invalid `authority` pubkey: {}", e)))?;

    // 2) Build the instruction
    let instr = mint_to(
        &spl_token::ID,
        &mint_pk,
        &dest_pk,
        &auth_pk,
        &[],             // no multisig signers
        payload.amount, 
    ).map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "failed to create mint instruction"))?;

    // 3) Map accounts metadata
    let accounts = instr
        .accounts
        .into_iter()
        .map(|meta| AccountMetaInfo {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        })
        .collect();

    // 4) Base64‐encode the raw instruction data
    let instruction_data = base64::engine::general_purpose::STANDARD.encode(instr.data);

    // 5) Return the JSON envelope
    let resp = MintTokenResponse {
        success: true,
        data: MintTokenData {
            program_id: instr.program_id.to_string(),
            accounts,
            instruction_data,
        },
    };
    Ok(Json(resp))
}
