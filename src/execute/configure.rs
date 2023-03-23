use crate::{
  error::ContractError,
  models::{FaucetResult, TokenParams},
  state::{is_allowed, TOKEN_PARAMS},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

pub fn configure(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  params_list: &Vec<TokenParams>,
) -> FaucetResult<Response> {
  if !is_allowed(&deps.as_ref(), &info.sender, "configure")? {
    return Err(ContractError::NotAuthorized {});
  }

  for params in params_list.iter() {
    let token_id = params.token.get_id();
    if params.interval.is_zero() {
      TOKEN_PARAMS.remove(deps.storage, token_id);
    } else {
      TOKEN_PARAMS.save(deps.storage, token_id, &params)?;
    }
  }
  Ok(Response::new().add_attributes(vec![attr("action", "configure")]))
}
