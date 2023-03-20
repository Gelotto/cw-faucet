use crate::{
  error::ContractError,
  models::{FaucetResult, TokenParams},
  state::{is_allowed, PARAMS},
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
    PARAMS.save(deps.storage, params.token.get_id(), &params)?;
  }
  Ok(Response::new().add_attributes(vec![attr("action", "configure")]))
}
