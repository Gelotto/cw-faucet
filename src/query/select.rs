use crate::{
  msg::SelectResponse,
  state::{HISTORY, PARAMS},
};
use cosmwasm_std::{Addr, Deps, Order, StdResult};
use cw_repository::client::Repository;

pub fn select(
  deps: Deps,
  maybe_fields: Option<Vec<String>>,
  maybe_wallet: Option<Addr>,
) -> StdResult<SelectResponse> {
  let loader = Repository::loader(deps.storage, &maybe_fields);
  Ok(SelectResponse {
    last_transfer: loader.view_by_wallet("last_transfer", maybe_wallet, |addr| {
      HISTORY.may_load(deps.storage, addr.clone())
    })?,
    params: loader.view("params", || {
      Ok(Some(
        PARAMS
          .range(deps.storage, None, None, Order::Ascending)
          .map(|result| {
            let (_, v) = result.unwrap();
            v
          })
          .collect(),
      ))
    })?,
  })
}
