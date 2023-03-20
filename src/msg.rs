use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_lib::models::TokenAmount;

use crate::models::{TokenParams, TransferHistory};

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
  Select {
    fields: Option<Vec<String>>,
    wallet: Option<Addr>,
  },
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct SelectResponse {
  pub params: Option<Vec<TokenParams>>,
  pub last_transfer: Option<TransferHistory>,
}
