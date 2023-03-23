use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Timestamp, Uint128, Uint64};
use cw_lib::models::{Token, TokenAmount};

use crate::error::ContractError;

pub type FaucetResult<T> = Result<T, ContractError>;

#[cw_serde]
pub struct TokenParams {
  pub token: Token,
  pub interval: Uint64, // seconds
}

#[cw_serde]
pub struct WalletTransfer {
  pub token_amount: TokenAmount,
  pub last_transferred_at: Timestamp,
}

#[cw_serde]
pub struct TransferTotal {
  pub token: Token,
  pub amount: Uint128,
  pub count: Uint64,
}
