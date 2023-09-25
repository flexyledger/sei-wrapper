// Imports 

#[cfg(test)]
mod tests {

  fn setup() -> (Addr, Addr, Addr) {
    let sei = mock_sei_contract();
    let cw20 = mock_cw20_contract();
    let wrapper = deploy_wrapper(sei);
    (sei, cw20, wrapper)
  }

  #[test]
  fn wrap_and_unwrap() {
    let (sei, cw20, wrapper) = setup();

    // Make wrap call
    let amount = Uint128(1000);
    cw20.transfer(alice, amount);
    wrapper.execute(alice, &ExecuteMsg::Wrap {
      cw20: cw20.addr(),
      amount 
    });

    // Assert SEI minted
    assert_eq!(sei.balance_of(alice), amount);

    // Make unwrap call
    wrapper.execute(alice, &ExecuteMsg::Unwrap { 
      sei: sei.addr(),
      amount, 
    });

    // Assert SEI burned and CW20 returned
    assert_eq!(sei.balance_of(alice), Uint128(0));
    assert_eq!(cw20.balance_of(alice), amount);
  }

}