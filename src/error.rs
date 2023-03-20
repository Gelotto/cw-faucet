use cosmwasm_std::StdError;
use cw_lib::models::Token;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContractError {
  #[error("{0}")]
  Std(#[from] StdError),

  #[error("NotAuthorized")]
  NotAuthorized {},

  #[error("ValidationError")]
  ValidationError {},

  #[error("WaitingForNextTransferInterval")]
  TransferRequestTooSoon {},

  #[error("InsufficientBalance")]
  InsufficientBalance { token: Token },
}
