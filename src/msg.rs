use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_lib::models::{Token, TokenAmount};

use crate::models::{TokenParams, TransferTotal, WalletTransfer};

#[cw_serde]
pub struct InstantiateMsg {
  pub acl_address: Addr,
  pub params: Option<Vec<TokenParams>>,
}

#[cw_serde]
pub enum ExecuteMsg {
  Configure {
    params: Vec<TokenParams>,
  },
  Transfer {
    recipient: Addr,
    tokens: Vec<TokenAmount>,
  },
}

#[cw_serde]
pub enum QueryMsg {
  CanRequest {
    address: Addr,
    token: Token,
    amount: Uint128,
  },
  Select {
    fields: Option<Vec<String>>,
    wallet: Option<Addr>,
  },
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct SelectResponse {
  pub token_params: Option<Vec<TokenParams>>,
  pub transfer_totals: Option<Vec<TransferTotal>>,
  pub my_last_transfer: Option<WalletTransfer>,
}

#[cw_serde]
pub struct CanRequestResponse {
  pub can_request: bool,
  pub reason: Option<String>,
}
