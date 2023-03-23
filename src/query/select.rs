use crate::{
  msg::SelectResponse,
  state::{TOKEN_PARAMS, TRANSFER_TOTALS, WALLET_TRANSFERS},
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
    my_last_transfer: loader.view_by_wallet("my_last_transfer", maybe_wallet, |addr| {
      WALLET_TRANSFERS.may_load(deps.storage, addr.clone())
    })?,
    transfer_totals: loader.view("transfer_totals", || {
      Ok(Some(
        TRANSFER_TOTALS
          .range(deps.storage, None, None, Order::Ascending)
          .map(|result| {
            let (_, v) = result.unwrap();
            v
          })
          .collect(),
      ))
    })?,
    token_params: loader.view("token_params", || {
      Ok(Some(
        TOKEN_PARAMS
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
