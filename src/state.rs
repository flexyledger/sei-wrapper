use cosmwasm_std::Addr;

pub struct Config {
  pub sei_token: Addr,
}

pub struct State {
  pub config: Config,
}