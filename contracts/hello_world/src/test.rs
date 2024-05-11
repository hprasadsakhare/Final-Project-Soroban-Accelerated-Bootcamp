#![cfg(test)]

use super::*;
use soroban_sdk::{ Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LostAndFoundRegistry);
    let client = LostAndFoundRegistryClient::new(&env, &contract_id);

   // let words = client.LostAndFoundRegistry(&symbol_short!("Dev"));
    
}
