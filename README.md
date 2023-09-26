# SEI Wrapper Contract

This is an example CosmWasm smart contract that wraps and unwraps arbitrary CW20 tokens into custom SEI tokens.

# 🔭  Overview
This contract allows wrapping any CW20 token by depositing it into the contract. When tokens are wrapped, a proportional amount of custom SEI tokens are minted to the user's wallet.

The user can also unwrap their SEI tokens back into the original CW20 tokens.

## ✍️  Contract Actions
The main actions the contract supports are:

* init(): Initializes the contract with the SEI token address
* wrap(): Transfers in CW20 tokens and mints SEI tokens
* unwrap(): Burns SEI tokens and transfers out CW20 tokens
* balances(): Views current wrapped balances

## 💫 Running Tests
The contract includes an integration test suite that can be run with:

* cargo test

This will execute wraps and unwraps using mock CW20 and SEI contracts and verify the expected token amounts.

## 🏆 Future Work
Some potential ways to extend the contract:

* Support unwrapping to a different CW20 token
* Implement governance and admin controls
* Add helper views for total wrapped supply

## 💻 Resources
* CosmWasm Docs
* CW20 Spec
