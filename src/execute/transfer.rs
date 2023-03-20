use crate::{
  error::ContractError,
  models::{FaucetResult, TokenParams, TransferHistory},
  state::{is_allowed, HISTORY, PARAMS},
};
use cosmwasm_std::{attr, Addr, CosmosMsg, DepsMut, Env, MessageInfo, Response, SubMsg};
use cw_lib::{
  models::{Token, TokenAmount},
  utils::{
    funds::{build_cw20_transfer_msg, build_send_msg},
    time::get_timedelta_seconds,
  },
};

pub fn transfer(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  recipient: Addr,
  token_amounts: &Vec<TokenAmount>,
) -> FaucetResult<Response> {
  if !is_allowed(&deps.as_ref(), &info.sender, "transfer")? {
    return Err(ContractError::NotAuthorized {});
  }

  let mut send_msgs: Vec<CosmosMsg> = vec![];
  let mut cw20_send_submsgs: Vec<SubMsg> = vec![];

  for ta in token_amounts.iter() {
    let maybe_history = HISTORY.may_load(deps.storage, recipient.clone())?;

    PARAMS.update(
      deps.storage,
      ta.token.get_id(),
      |maybe_params| -> FaucetResult<TokenParams> {
        if let Some(params) = maybe_params {
          // if there's a prior transfert, ensure that the recipient waits for
          // the necessary time interval between transfer requests.
          if let Some(history) = &maybe_history {
            let delta = get_timedelta_seconds(history.last_transferred_at, env.block.time);
            if delta < params.interval.u64() {
              return Err(ContractError::TransferRequestTooSoon {});
            }
          }
          match ta.token.clone() {
            Token::Native { denom } => {
              let msg = build_send_msg(&recipient, &denom, ta.amount)?;
              send_msgs.push(msg);
            },
            Token::Cw20 { address } => {
              let msg = build_cw20_transfer_msg(&recipient, &address, ta.amount)?;
              cw20_send_submsgs.push(msg);
            },
          }
          Ok(params)
        } else {
          // the token type isn't registered with the faucet by admin
          Err(ContractError::NotAuthorized {})
        }
      },
    )?;

    // replace old transfer record for the recipient
    HISTORY.save(
      deps.storage,
      recipient.clone(),
      &TransferHistory {
        last_transferred_at: env.block.time,
        token_amount: ta.clone(),
      },
    )?;
  }

  Ok(
    Response::new()
      .add_attributes(vec![attr("action", "transfer")])
      .add_messages(send_msgs)
      .add_submessages(cw20_send_submsgs),
  )
}
