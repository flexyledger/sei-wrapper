use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, Uint128};
use cw20::Cw20ExecuteMsg;

use crate::state::{State, Config};
use crate::error::ContractError;

pub fn init(deps: DepsMut, _env: Env, _info: MessageInfo, msg: InitMsg) -> Result<Response, ContractError> {
  let state = State {
    config: Config {
      sei_token: msg.sei_token,
    },
  };
  
  state.save(deps.storage)?;

  Ok(Response::default())
}

pub fn execute_wrap(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {

  let cw20_contract = Cw20Contract(msg.cw20_token);
  
  // Transfer CW20 tokens to contract
  cw20_contract.transfer_from(info.sender, env.contract.address, msg.amount)?;

  // Mint SEI tokens
  let sei_contract = SeiContract(state.config.sei_token);
  sei_contract.mint(info.sender, msg.amount)?;

  Ok(Response::new()
    .add_attribute("action", "wrap")
    .add_attribute("amount", msg.amount))
}

pub fn execute_unwrap(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: ExecuteMsg) -> Result<Response, ContractError> {
  Ok(Response::default())
}