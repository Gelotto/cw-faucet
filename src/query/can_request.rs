use crate::{
  msg::CanRequestResponse,
  state::{TOKEN_PARAMS, WALLET_TRANSFERS},
};
use cosmwasm_std::{Addr, Deps, Env, StdResult, Uint128};
use cw_lib::{
  models::Token,
  utils::{funds::get_token_balance, time::get_timedelta_seconds},
};

pub fn can_request(
  deps: Deps,
  env: Env,
  address: Addr,
  token: Token,
  amount: Uint128,
) -> StdResult<CanRequestResponse> {
  let mut can_request = true;
  let mut reason: Option<String> = None;

  if deps.api.addr_validate(address.as_str()).is_err() {
    reason = Some("Invalid Juno address...".to_owned());
    can_request = false;
  } else if let Some(params) = TOKEN_PARAMS.may_load(deps.storage, token.get_id())? {
    let balance = get_token_balance(deps.querier, &env.contract.address, &token)?;
    if balance < amount {
      reason = Some("The faucet has run dry. Please contact us!".to_owned());
      can_request = false;
    } else if let Some(last_transfer) = WALLET_TRANSFERS.may_load(deps.storage, address.clone())? {
      let delta = get_timedelta_seconds(last_transfer.last_transferred_at, env.block.time);
      if delta < params.interval.u64() {
        reason = Some("Too soon!".to_owned());
        can_request = false;
      }
    }
  } else {
    reason = Some("Unrecognized token type...".to_owned());
    can_request = false;
  }

  Ok(CanRequestResponse { can_request, reason })
}
