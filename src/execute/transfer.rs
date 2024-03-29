use crate::{
  error::ContractError,
  models::{FaucetResult, TransferTotal, WalletTransfer},
  state::{is_allowed, TOKEN_PARAMS, TRANSFER_TOTALS, WALLET_TRANSFERS},
};
use cosmwasm_std::{attr, Addr, CosmosMsg, DepsMut, Env, MessageInfo, Response, SubMsg, Uint64};
use cw_lib::{
  models::{Token, TokenAmount},
  utils::{
    funds::{build_cw20_transfer_msg, build_send_msg, get_token_balance},
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
    let maybe_history = WALLET_TRANSFERS.may_load(deps.storage, recipient.clone())?;
    let balance = get_token_balance(deps.querier, &env.contract.address, &ta.token)?;

    // abort if the faucet is drained
    if balance < ta.amount {
      return Err(ContractError::InsufficientBalance {
        token: ta.token.clone(),
      });
    }

    // increment global transfer total for this token type
    TRANSFER_TOTALS.update(
      deps.storage,
      ta.token.get_id(),
      |maybe_total| -> FaucetResult<TransferTotal> {
        if let Some(mut total) = maybe_total {
          total.count += Uint64::one();
          total.amount += ta.amount;
          Ok(total)
        } else {
          Ok(TransferTotal {
            token: ta.token.clone(),
            amount: ta.amount,
            count: Uint64::one(),
          })
        }
      },
    )?;

    // build transfer sub-messages
    if let Some(params) = TOKEN_PARAMS.may_load(deps.storage, ta.token.get_id())? {
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
    } else {
      // the token type isn't registered with the faucet by admin
      return Err(ContractError::NotAuthorized {});
    }

    // replace old transfer record for the recipient
    WALLET_TRANSFERS.save(
      deps.storage,
      recipient.clone(),
      &WalletTransfer {
        last_transferred_at: env.block.time,
        token_amount: ta.clone(),
      },
    )?;
  }

  Ok(
    Response::new()
      .add_attributes(vec![
        attr("action", "transfer"),
        attr("recipient", recipient.to_string()),
      ])
      .add_messages(send_msgs)
      .add_submessages(cw20_send_submsgs),
  )
}
