use crate::models::{FaucetResult, TransferTotal, WalletTransfer};
use crate::msg::InstantiateMsg;
use crate::{error::ContractError, models::TokenParams};
use cosmwasm_std::{Addr, Deps, DepsMut, Env, MessageInfo, Uint64};
use cw_acl::client::Acl;
use cw_lib::models::TokenID;
use cw_storage_plus::{Item, Map};

pub const ACL_ADDRESS: Item<Addr> = Item::new("acl_address");
pub const TOKEN_PARAMS: Map<TokenID, TokenParams> = Map::new("token_params");
pub const WALLET_TRANSFERS: Map<Addr, WalletTransfer> = Map::new("wallet_transfers");
pub const TRANSFER_TOTALS: Map<TokenID, TransferTotal> = Map::new("transfer_totals");

/// Initialize contract state data.
pub fn initialize(
  deps: DepsMut,
  _env: &Env,
  _info: &MessageInfo,
  msg: &InstantiateMsg,
) -> Result<(), ContractError> {
  ACL_ADDRESS.save(deps.storage, &msg.acl_address)?;
  if let Some(params) = &msg.params {
    for params in params.iter() {
      TOKEN_PARAMS.save(
        deps.storage,
        params.token.get_id(),
        &TokenParams {
          token: params.token.clone(),
          interval: params.interval.max(Uint64::from(60u64)),
        },
      )?;
    }
  }
  Ok(())
}

/// Helper function that returns true if given wallet (principal) is authorized
/// by ACL to the given action.
pub fn is_allowed(
  deps: &Deps,
  principal: &Addr,
  action: &str,
) -> FaucetResult<bool> {
  if let Some(acl_addr) = ACL_ADDRESS.may_load(deps.storage)? {
    let acl = Acl::new(&acl_addr);
    Ok(acl.is_allowed(&deps.querier, principal, action)?)
  } else {
    // NOTE: No ACL implies that everything is 100% open to anyone
    Ok(true)
  }
}
